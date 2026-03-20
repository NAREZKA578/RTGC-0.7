use winit::{
    event::{WindowEvent, ElementState, KeyEvent, MouseButton},
    keyboard::{KeyCode, PhysicalKey},
};
use std::sync::Arc;
use crate::graphics::{GraphicsContext, gl_context::GlContext, TextureQuality, MaterialManager};
use crate::graphics::mesh::Mesh;
use crate::input::InputManager;
use crate::audio::AudioSystem;
use crate::ecs::EcsManager;
use crate::physics;
use crate::graphics::renderer::MenuState;
use crate::game::{WeatherSystem, DayNightCycle, Cargo, Winch, MissionGenerator, Mission};
use crate::graphics::particles::ParticleSystem;
use crate::graphics::debug_renderer::DebugRenderer;
use crate::profiler;
use crate::ui::HudManager;
use crate::assets::VehicleLoader;
use crate::world::{OpenWorld, ChunkId, generate_chunk_mesh, TerrainVertex, CHUNK_SIZE, HEIGHTMAP_RESOLUTION};
use crate::world::{Settlement, RoadNetwork, BuildingPlacer};
use nalgebra::{Vector3, UnitQuaternion, Matrix4};
use crate::physics::Vehicle;

// Fixed timestep for physics (60 Hz)
const PHYSICS_TIMESTEP: f32 = 1.0 / 60.0;

pub struct Engine {
    pub graphics_context: GraphicsContext,
    pub input_manager: InputManager,
    pub audio_system: AudioSystem,
    pub ecs_manager: EcsManager,
    pub physics_world: physics::PhysicsWorld,
    gl_context: GlContext,
    last_frame_time: std::time::Instant,
    // C1: Fixed timestep accumulator
    physics_accumulator: f32,
    physics_timestep: f32,
    // HUD Manager
    hud_manager: HudManager,
    // Material Manager
    material_manager: MaterialManager,
    // МИР-0: OpenWorld вместо ручного TerrainGenerator
    open_world: Option<OpenWorld>,
    world_seed: u64,
    settlements: Vec<Settlement>,
    road_network: Option<RoadNetwork>,
    mission_generator: Option<MissionGenerator>,
    current_mission: Option<Mission>,
    vehicle_chassis_id: Option<usize>,
    chunk_mesh_data: Option<(Vec<f32>, Vec<u32>)>,  // vertices, indices for terrain
    // Sprint 5: Weather, Day/Night, Particles, Debug
    weather_system: WeatherSystem,
    day_night_cycle: DayNightCycle,
    particle_system: ParticleSystem,
    debug_renderer: DebugRenderer,
    debug_mode: bool,
    // Sprint 4: Cargo & Winch
    cargo: Option<Cargo>,
    winch: Winch,
    // Vehicle control state
    vehicle_throttle: f32,
    vehicle_steering: f32,
    vehicle_brake: f32,
    // Задача 1: Vehicle как поле Engine
    vehicle: Option<Vehicle>,
    // Игра-4: Vehicle damage
    vehicle_health: f32,
    // Игра-5: Fuel
    vehicle_fuel: f32,
    // Сохр-1: Save timer
    save_timer: f32,
    // Ввод-1: Mouse position for menu
    mouse_x: f32,
    mouse_y: f32,
}

impl Engine {
    pub fn new(event_loop: &winit::event_loop::EventLoop<()>) -> Result<Self, Box<dyn std::error::Error>> {
        // === МИР-0: Создаём OpenWorld с процедурной генерацией ===
        let world_seed = 12345u64;  // Можно сделать настраиваемым
        
        // 1. Создаём OpenWorld (включает TerrainGenerator внутри)
        let mut open_world = OpenWorld::new(world_seed);
        
        // 2. Генерируем дорожную сеть и поселения
        use crate::world::{Settlement, RoadNetwork};
        
        // Генерируем несколько поселений вокруг старта
        let mut settlements = Vec::new();
        for gx in -2..=2 {
            for gz in -2..=2 {
                if let Some(settlement) = Settlement::generate(world_seed, gx, gz) {
                    settlements.push(settlement);
                }
            }
        }
        
        // Строим дорожную сеть между поселениями
        let road_network = RoadNetwork::generate_from_settlements(&settlements, world_seed);
        
        // Передаём дорожную сеть в генератор terrain для влияния на высоты
        open_world.generator_mut().set_road_network(road_network.clone());
        
        // 3. Создаём MissionGenerator из инфраструктуры
        let mission_generator = Some(MissionGenerator::new(settlements.clone(), road_network.clone()));
        
        // 4. Генерируем стартовый чанк для физики
        let chunk_id = ChunkId::new(0, 0);
        let chunk_data = open_world.generator().generate_chunk(chunk_id);
        
        // 5. Convert heights to format expected by RigidBody::new_terrain
        let mut height_map: Vec<Vec<f32>> = Vec::with_capacity(HEIGHTMAP_RESOLUTION as usize);
        for z in 0..HEIGHTMAP_RESOLUTION as usize {
            let mut row = Vec::with_capacity(HEIGHTMAP_RESOLUTION as usize);
            for x in 0..HEIGHTMAP_RESOLUTION as usize {
                let idx = z * HEIGHTMAP_RESOLUTION as usize + x;
                row.push(chunk_data.heights[idx]);
            }
            height_map.push(row);
        }
        
        // Create physics world first (no GL context needed)
        let mut physics_world = physics::PhysicsWorld::new();
        
        // 6. Create terrain body and add to physics world
        let terrain_body = physics::RigidBody::new_terrain(
            Vector3::zeros(),
            height_map,
            Vector3::new(CHUNK_SIZE as f32, 1.0, CHUNK_SIZE as f32),
        );
        physics_world.add_body(terrain_body);
        
        // 7. Create vehicle chassis and Vehicle physics
        let vehicle_config = VehicleLoader::create_default_vehicle("starter");
        let chassis_half_extents = Vector3::new(
            vehicle_config.body_config.dimensions[0] / 2.0,
            vehicle_config.body_config.dimensions[1] / 2.0,
            vehicle_config.body_config.dimensions[2] / 2.0,
        );
        let mut chassis = physics::RigidBody::new_box(
            Vector3::new(0.0, 10.0, 0.0),  // Start above ground
            vehicle_config.body_config.mass_kg,
            chassis_half_extents,
        );
        chassis.collision_layer = physics::LAYER_VEHICLE;
        chassis.collision_mask = physics::LAYER_WORLD | physics::LAYER_CARGO;
        chassis.enable_ccd = true;
        let chassis_id = physics_world.add_body(chassis);
        
        // Исп-2: Создать Vehicle для управления физикой
        let vc = crate::assets::VehicleLoader::to_vehicle_config(&vehicle_config);
        let mut v = Vehicle::new(vc);
        v.set_position(Vector3::new(0.0, 10.0, 0.0));
        let vehicle = Some(v);
        
        // 8. Generate terrain mesh data for renderer
        let (vertices, indices) = generate_chunk_mesh(&chunk_data, 0);
        let flat_vertices: Vec<f32> = vertices.iter()
            .flat_map(|v| [
                v.position[0], v.position[1], v.position[2],
                v.normal[0], v.normal[1], v.normal[2],
                0.5,  // moisture placeholder
                0.0,  // slope placeholder
                0.0, 0.0,  // texcoord
            ])
            .collect();
        
        // Now create OpenGL context and graphics
        let gl_context = GlContext::new(event_loop)?;
        let window = Arc::new(gl_context.window.clone());
        let gl = gl_context.gl.clone();
        
        let graphics_context = GraphicsContext::new(window, gl.clone())?;
        let input_manager = InputManager::new();
        let audio_system = AudioSystem::new()?;
        
        // Исп-6: Загрузить звуки при старте
        let _ = audio_system.engine.lock().unwrap().load_sound("assets/audio/engine_idle.wav");
        let _ = audio_system.engine.lock().unwrap().load_sound("assets/audio/engine_accel.wav");
        let _ = audio_system.engine.lock().unwrap().load_sound("assets/audio/brake.wav");
        let _ = audio_system.engine.lock().unwrap().load_sound("assets/audio/crash.wav");
        let _ = audio_system.engine.lock().unwrap().load_sound("assets/audio/winch.wav");
        
        let ecs_manager = EcsManager::new();
        
        // Create terrain mesh
        let terrain_mesh = Mesh::new_terrain(&gl, &flat_vertices, &indices)?;
        let mut renderer = &mut graphics_context.renderer;
        renderer.set_terrain_mesh(terrain_mesh);
        
        // Create vehicle box mesh
        renderer.create_vehicle_box_mesh(chassis_half_extents)?;
        drop(renderer);  // Release borrow
        
        Ok(Self {
            graphics_context,
            input_manager,
            audio_system,
            ecs_manager,
            physics_world,
            gl_context,
            last_frame_time: std::time::Instant::now(),
            physics_accumulator: 0.0,
            physics_timestep: PHYSICS_TIMESTEP,
            hud_manager: HudManager::new(),
            material_manager: MaterialManager::new(),
            open_world: Some(open_world),
            world_seed,
            settlements,
            road_network: Some(road_network),
            mission_generator,
            current_mission: None,
            vehicle_chassis_id: Some(chassis_id),
            chunk_mesh_data: Some((flat_vertices, indices)),
            // Sprint 5: Weather, Day/Night, Particles, Debug
            weather_system: WeatherSystem::new(12345),
            day_night_cycle: DayNightCycle::new(10.0, 600.0), // 10:00 AM, 10 min day
            particle_system: ParticleSystem::new(2000),
            debug_renderer: DebugRenderer::new(),
            debug_mode: false,
            // Sprint 4: Cargo & Winch
            cargo: None,
            winch: Winch::new(chassis_id),
            // Vehicle control state - initialized to zero
            vehicle_throttle: 0.0,
            vehicle_steering: 0.0,
            vehicle_brake: 0.0,
            vehicle,
            // Игра-4: Vehicle damage - start at full health
            vehicle_health: 1.0,
            // Игра-5: Fuel - start with full tank
            vehicle_fuel: 1.0,
            // Сохр-1: Save timer
            save_timer: 0.0,
            // Ввод-1: Mouse position
            mouse_x: 0.0,
            mouse_y: 0.0,
        })
    }

    pub fn handle_window_event(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::CloseRequested => false,
            WindowEvent::Resized(physical_size) => {
                // Update viewport when window is resized
                if physical_size.width > 0 && physical_size.height > 0 {
                    let _ = self.gl_context.resize(physical_size.width, physical_size.height);
                    self.graphics_context.resize(*physical_size);
                }
                true
            }
            // Ввод-1: Track mouse position for menu interaction
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_x = position.x as f32;
                self.mouse_y = position.y as f32;
            }
            // Ввод-1: Handle mouse clicks for menu
            WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Left, .. } => {
                self.handle_menu_click(self.mouse_x, self.mouse_y);
            }
            WindowEvent::KeyboardInput { event: key_event, .. } => {
                if !self.handle_key_event(key_event) {
                    return false;
                }
                true
            }
            _ => true,
        }
    }

    fn handle_key_event(&mut self, key_event: &KeyEvent) -> bool {
        // Update input manager
        self.input_manager.handle_keyboard_input(key_event);

        match (key_event.logical_key, key_event.state) {
            (winit::keyboard::Key::Named(winit::keyboard::NamedKey::Escape), ElementState::Pressed) => {
                // Handle escape key differently based on current menu state
                match self.graphics_context.renderer.menu_state {
                    MenuState::MainMenu | MenuState::Loading => {
                        // Exit the application - return false to signal caller to exit
                        return false;
                    }
                    MenuState::InGame => {
                        // Ввод-2: Пауза внутри игры при Escape
                        self.graphics_context.renderer.menu_state = MenuState::Paused;
                    }
                    MenuState::Paused => {
                        // Вернуться в игру из паузы
                        self.graphics_context.renderer.menu_state = MenuState::InGame;
                    }
                    MenuState::CitySelection | MenuState::WorldCreation | MenuState::Settings => {
                        // Go back to main menu
                        self.graphics_context.renderer.menu_state = MenuState::MainMenu;
                    }
                }
            }
            (winit::keyboard::Key::Named(winit::keyboard::NamedKey::ArrowUp), ElementState::Pressed) |
            (winit::keyboard::Key::Named(winit::keyboard::NamedKey::ArrowUp), ElementState::Released) => {
                if key_event.state == ElementState::Pressed {
                    self.vehicle_throttle = 1.0;
                } else {
                    self.vehicle_throttle = 0.0;
                }
            }
            (winit::keyboard::Key::Named(winit::keyboard::NamedKey::ArrowDown), ElementState::Pressed) |
            (winit::keyboard::Key::Named(winit::keyboard::NamedKey::ArrowDown), ElementState::Released) => {
                if key_event.state == ElementState::Pressed {
                    self.vehicle_throttle = -1.0;
                } else {
                    self.vehicle_throttle = 0.0;
                }
            }
            (winit::keyboard::Key::Named(winit::keyboard::NamedKey::ArrowLeft), ElementState::Pressed) |
            (winit::keyboard::Key::Named(winit::keyboard::NamedKey::ArrowLeft), ElementState::Released) => {
                if key_event.state == ElementState::Pressed {
                    self.vehicle_steering = -1.0;
                } else {
                    self.vehicle_steering = 0.0;
                }
            }
            (winit::keyboard::Key::Named(winit::keyboard::NamedKey::ArrowRight), ElementState::Pressed) |
            (winit::keyboard::Key::Named(winit::keyboard::NamedKey::ArrowRight), ElementState::Released) => {
                if key_event.state == ElementState::Pressed {
                    self.vehicle_steering = 1.0;
                } else {
                    self.vehicle_steering = 0.0;
                }
            }
            (winit::keyboard::Key::Named(winit::keyboard::NamedKey::Space), ElementState::Pressed) |
            (winit::keyboard::Key::Named(winit::keyboard::NamedKey::Space), ElementState::Released) => {
                if key_event.state == ElementState::Pressed {
                    self.vehicle_brake = 1.0;
                } else {
                    self.vehicle_brake = 0.0;
                }
            }
            (winit::keyboard::Key::Character(ref c), ElementState::Pressed) if c == "r" || c == "R" => {
                // Reset truck position - Задача 1: Reset vehicle
                if let Some(ref mut vehicle) = self.vehicle {
                    vehicle.set_position(Vector3::new(0.0, 10.0, 0.0));
                    vehicle.body_mut().velocity = nalgebra::Vector3::zeros();
                    vehicle.body_mut().angular_velocity = nalgebra::Vector3::zeros();
                    vehicle.body_mut().rotation = UnitQuaternion::identity();
                }
                // Also sync to physics world
                if let Some(chassis_id) = self.vehicle_chassis_id {
                    if let Some(body) = self.physics_world.get_body_mut(chassis_id) {
                        body.position = Vector3::new(0.0, 10.0, 0.0);
                        body.velocity = nalgebra::Vector3::zeros();
                        body.angular_velocity = nalgebra::Vector3::zeros();
                        body.rotation = UnitQuaternion::identity();
                    }
                }
            }
            (winit::keyboard::Key::Character(ref c), ElementState::Pressed) if c == "e" || c == "E" => {
                // Activate cargo action (pickup/drop)
                self.winch.activate_action();
            }
            (winit::keyboard::Key::Character(ref c), ElementState::Pressed) if c == "c" || c == "C" => {
                // Switch camera view (first person/third person)
                if let Some(chassis_id) = self.vehicle_chassis_id {
                    if let Some(body) = self.physics_world.get_body(chassis_id) {
                        match self.graphics_context.renderer.camera.camera_type {
                            crate::graphics::camera::CameraType::FirstPerson => {
                                self.graphics_context.renderer.camera.switch_to_third_person(
                                    body.position,
                                    body.rotation
                                );
                            }
                            crate::graphics::camera::CameraType::ThirdPerson => {
                                self.graphics_context.renderer.camera.switch_to_first_person(
                                    body.position,
                                    body.rotation
                                );
                            }
                        }
                    }
                }
            }
            (winit::keyboard::Key::Character(ref c), ElementState::Pressed) if c == "1" => {
                match self.graphics_context.renderer.menu_state {
                    MenuState::MainMenu => {
                        self.graphics_context.renderer.menu_state = MenuState::WorldCreation;
                    }
                    MenuState::CitySelection => {
                        // Handle city selection
                    }
                    _ => {}
                }
            }
            (winit::keyboard::Key::Character(ref c), ElementState::Pressed) if c == "2" => {
                match self.graphics_context.renderer.menu_state {
                    MenuState::MainMenu => {
                        self.graphics_context.renderer.menu_state = MenuState::CitySelection;
                    }
                    _ => {}
                }
            }
            (winit::keyboard::Key::Character(ref c), ElementState::Pressed) if c == "3" => {
                match self.graphics_context.renderer.menu_state {
                    MenuState::MainMenu => {
                        self.graphics_context.renderer.menu_state = MenuState::Settings;
                    }
                    _ => {}
                }
            }
            (winit::keyboard::Key::Character(ref c), ElementState::Pressed) if c == "4" => {
                match self.graphics_context.renderer.menu_state {
                    MenuState::MainMenu => {
                        // Exit application
                        return false;
                    }
                    _ => {}
                }
            }
            (winit::keyboard::Key::Character(ref c), ElementState::Pressed) if c == "f" || c == "F" => {
                // Задача 5: Toggle debug mode with F key (or use F1)
                self.debug_mode = !self.debug_mode;
            }
            // Игра-6: Winch controls - Q to shoot, Z to retract, X to release
            (winit::keyboard::Key::Character(ref c), ElementState::Pressed) if c == "q" || c == "Q" => {
                if let Some(ref vehicle) = self.vehicle {
                    let forward = vehicle.body().rotation * Vector3::new(0.0, 0.0, 1.0);
                    self.winch.shoot(vehicle.position(), forward, &self.physics_world);
                }
            }
            (winit::keyboard::Key::Character(ref c), ElementState::Pressed) if c == "z" || c == "Z" => {
                self.winch.start_retract();
            }
            (winit::keyboard::Key::Character(ref c), ElementState::Pressed) if c == "x" || c == "X" => {
                self.winch.release();
            }
            _ => {}
        }
        true
    }

    // Ввод-1: Handle menu clicks
    fn handle_menu_click(&mut self, x: f32, y: f32) {
        let h = self.graphics_context.renderer.get_height() as f32;
        let w = self.graphics_context.renderer.get_width() as f32;
        
        match self.graphics_context.renderer.menu_state {
            MenuState::MainMenu => {
                // Check click on menu buttons
                if x > w/2.0 - 120.0 && x < w/2.0 + 120.0 {
                    if y > h/2.0 - 80.0 && y < h/2.0 - 40.0 {
                        // "Новая игра" - start game
                        self.graphics_context.renderer.menu_state = MenuState::InGame;
                    } else if y > h/2.0 - 30.0 && y < h/2.0 + 10.0 {
                        // "Продолжить"
                        self.graphics_context.renderer.menu_state = MenuState::InGame;
                    } else if y > h/2.0 + 20.0 && y < h/2.0 + 60.0 {
                        // "Настройки"
                        self.graphics_context.renderer.menu_state = MenuState::Settings;
                    } else if y > h/2.0 + 70.0 && y < h/2.0 + 110.0 {
                        // "Выход"
                        std::process::exit(0);
                    }
                }
            }
            MenuState::Paused => {
                // Ввод-2: Обработка кликов в меню паузы
                if x > w/2.0 - 120.0 && x < w/2.0 + 120.0 {
                    if y > h/2.0 - 40.0 && y < h/2.0 {  // "Продолжить"
                        self.graphics_context.renderer.menu_state = MenuState::InGame;
                    } else if y > h/2.0 + 10.0 && y < h/2.0 + 50.0 {  // "Настройки"
                        self.graphics_context.renderer.menu_state = MenuState::Settings;
                    } else if y > h/2.0 + 60.0 && y < h/2.0 + 100.0 {  // "Выход в меню"
                        self.graphics_context.renderer.menu_state = MenuState::MainMenu;
                    }
                }
            }
            MenuState::Settings => {
                // Settings panel click handling can be added here
            }
            _ => {}
        }
    }

    // Сохр-1: Save game state to file
    fn save_game_state(&self) {
        use crate::game::mission_save::{MissionSaveManager, WorldState};
        use nalgebra::Vector3;
        
        let mut world_state = WorldState::default();
        
        // Save vehicle position and rotation
        if let Some(ref v) = self.vehicle {
            let pos = v.position();
            let rot = v.body().rotation;
            world_state.vehicle_position = [pos.x, pos.y, pos.z];
            world_state.vehicle_rotation = [rot.i, rot.j, rot.k, rot.w];
            world_state.vehicle_fuel = self.vehicle_fuel;
        }
        
        // Save time of day and weather
        world_state.time_of_day = self.day_night_cycle.get_hour();
        world_state.weather = format!("{:?}", self.weather_system.get_state().precipitation_type);
        
        // Save using MissionSaveManager
        let save_manager = MissionSaveManager::new("saves".into());
        if let Ok(mut sm) = save_manager {
            let _ = sm.save_game(0, &crate::game::mission_save::MissionProgress {
                objectives: vec![],
                current_objective_index: 0,
                is_complete: false,
                world_state,
            });
        }
    }

    // Сохр-2: Load game state from file
    fn load_game_state(&mut self) -> bool {
        use crate::game::mission_save::MissionSaveManager;
        use nalgebra::Vector3;
        
        let save_manager = MissionSaveManager::new("saves".into());
        if let Ok(sm) = save_manager {
            if let Ok(save) = sm.load_game(0) {
                // Restore vehicle position
                if let Some(ref mut v) = self.vehicle {
                    let wp = save.world_state.vehicle_position;
                    v.set_position(Vector3::new(wp[0], wp[1], wp[2]));
                    
                    // Restore rotation
                    let wr = save.world_state.vehicle_rotation;
                    v.body_mut().rotation = nalgebra::UnitQuaternion::from_quaternion(
                        nalgebra::Quaternion::new(wr[3], wr[0], wr[1], wr[2])
                    );
                    
                    // Restore fuel
                    self.vehicle_fuel = save.world_state.vehicle_fuel.clamp(0.0, 1.0);
                }
                
                // Restore time of day
                self.day_night_cycle.set_time(save.world_state.time_of_day, 0.0);
                
                return true;
            }
        }
        false
    }

    pub fn update(&mut self) {
        let current_time = std::time::Instant::now();
        let delta_time = (current_time - self.last_frame_time).as_secs_f32();
        self.last_frame_time = current_time;

        // Start profiling the update cycle
        profiler::start_timer("update_cycle");

        // Сохр-1: Автосохранение каждые 5 минут
        self.save_timer += delta_time;
        if self.save_timer >= 300.0 {
            self.save_timer = 0.0;
            self.save_game_state();
        }

        // Update systems based on current menu state
        match self.graphics_context.renderer.menu_state {
            MenuState::InGame | MenuState::Paused => {
                // Ввод-2: При паузе не обновлять физику, но рендерить оверлей
                if self.graphics_context.renderer.menu_state == MenuState::Paused {
                    // Пропускаем обновление физики при паузе
                    profiler::stop_timer("update_cycle");
                    return;
                }
                
                // === SPRINT 1: Fixed timestep physics loop ===
                profiler::start_timer("physics_step");
                
                // Accumulate time and step physics at fixed timestep
                self.physics_accumulator += delta_time.min(0.1);  // clamp to avoid spiral of death
                
                while self.physics_accumulator >= PHYSICS_TIMESTEP {
                    // Исп-2: Обновить Vehicle перед physics step
                    if let Some(ref mut vehicle) = self.vehicle {
                        vehicle.set_controls(physics::VehicleControls::new(
                            self.vehicle_throttle,
                            self.vehicle_brake,
                            self.vehicle_steering,
                            0.0,
                        ));
                        // Получить высоту из OpenWorld
                        if let Some(ref open_world) = self.open_world {
                            let h = |x: f32, z: f32| open_world.get_generator().get_height(x, z);
                            vehicle.update(PHYSICS_TIMESTEP, h);
                        }
                        // Синхронизировать chassis body в PhysicsWorld
                        if let Some(chassis_id) = self.vehicle_chassis_id {
                            if let Some(body) = self.physics_world.get_body_mut(chassis_id) {
                                body.position = vehicle.position();
                                body.rotation = vehicle.body().rotation;
                                body.velocity = vehicle.body().velocity;
                            }
                        }
                    }
                    
                    self.physics_world.step(PHYSICS_TIMESTEP);
                    self.physics_accumulator -= PHYSICS_TIMESTEP;
                }
                
                // Interpolation alpha for smooth rendering
                let alpha = self.physics_accumulator / PHYSICS_TIMESTEP;
                
                // === SPRINT 5: Update weather and day/night cycle ===
                self.day_night_cycle.update(delta_time);
                self.weather_system.update(delta_time, self.day_night_cycle.get_hour());

                // Apply weather friction modifier to physics bodies
                let friction_mod = self.weather_system.get_friction_modifier();
                if let Some(chassis_id) = self.vehicle_chassis_id {
                    if let Some(body) = self.physics_world.get_body_mut(chassis_id) {
                        body.friction *= friction_mod;
                    }
                }

                // Граф-4: Обновить цвет неба из DayNightCycle
                let sky_top = self.day_night_cycle.get_sky_color_top();
                let sky_hor = self.day_night_cycle.get_sky_color_horizon();
                self.graphics_context.renderer.set_sky_color(sky_top, sky_hor);
                self.graphics_context.renderer.set_sun_direction(self.day_night_cycle.get_sun_direction());
                self.graphics_context.renderer.set_ambient_intensity(self.day_night_cycle.get_ambient_intensity());

                // Исп-4: Emit particles from rain and vehicle wheels
                let intensity = self.weather_system.get_precipitation_intensity();
                if intensity > 0.05 {
                    if let Some(ref vehicle) = self.vehicle {
                        self.particle_system.emit_rain(
                            vehicle.position() + nalgebra::Vector3::new(0.0, 8.0, 0.0),
                            intensity,
                            (intensity * 15.0) as usize,
                        );
                        // Пыль из под колёс
                        for wheel in vehicle.wheels() {
                            if wheel.is_in_contact && self.vehicle_throttle.abs() > 0.3 {
                                self.particle_system.emit_dust(
                                    wheel.local_position + vehicle.position(),
                                    vehicle.body().velocity * 0.5,
                                    self.vehicle_throttle.abs(),
                                );
                            }
                        }
                    }
                }
                self.particle_system.update(delta_time);

                // Исп-3: Collect debug lines when debug mode is enabled
                if self.debug_mode {
                    if let Some(ref vehicle) = self.vehicle {
                        for wheel in vehicle.wheels() {
                            let from = wheel.local_position + vehicle.position();
                            let to = from - nalgebra::Vector3::new(0.0, 0.5, 0.0);
                            self.debug_renderer.draw_wheel_ray(from, to, wheel.is_in_contact);
                        }
                    }
                }

                // Игра-4: Vehicle damage от contact events
                for event in self.physics_world.get_contact_events() {
                    if event.impact_velocity > 5.0 {
                        self.vehicle_health -= event.impact_velocity * 0.003;
                        self.vehicle_health = self.vehicle_health.clamp(0.0, 1.0);
                        // Аудио-2: Звук удара
                        if event.impact_velocity > 8.0 {
                            let pos = event.contact_point;
                            let vol = (event.impact_velocity / 20.0).min(1.0);
                            // Звук будет воспроизведён через audio_system
                        }
                    }
                }

                // Игра-5: Fuel consumption
                if self.vehicle_throttle.abs() > 0.1 && self.vehicle_fuel > 0.0 {
                    self.vehicle_fuel -= delta_time * 0.00005;
                    self.vehicle_fuel = self.vehicle_fuel.clamp(0.0, 1.0);
                    if self.vehicle_fuel <= 0.0 {
                        self.vehicle_throttle = 0.0; // заглох
                    }
                }

                // === SPRINT 4: Update winch ===
                if let Some(chassis_id) = self.vehicle_chassis_id {
                    self.winch.update(delta_time, &mut self.physics_world, &mut vec![]);
                }

                // Sync camera with vehicle chassis position and update HUD from Vehicle
                if let Some(chassis_id) = self.vehicle_chassis_id {
                    if let Some(body) = self.physics_world.get_body(chassis_id) {
                        // Use interpolated position for smooth camera follow
                        let pos = body.position;
                        let rot = body.rotation;
                        
                        // Update renderer camera
                        self.graphics_context.renderer.update_camera_for_frame(pos, rot);
                        
                        // Update renderer vehicle transform for rendering
                        self.graphics_context.renderer.set_vehicle_transform(pos, rot);
                        
                        // Update texture streaming based on truck position
                        self.graphics_context.renderer.texture_streaming.update_camera_position(nalgebra::Vector2::new(
                            pos.x,
                            pos.z,
                        ));
                        
                        // Задача 9: HUD показывает реальные данные от Vehicle
                        let hud_data = if let Some(ref vehicle) = self.vehicle {
                            let speed = vehicle.speed() * 3.6;  // m/s -> km/h
                            let mut data = crate::ui::hud::VehicleHudData {
                                speed_kmh: speed,
                                engine_rpm: 800.0 + speed * 25.0,  // placeholder RPM
                                engine_rpm_max: 3200.0,
                                gear: crate::ui::hud::GearState::Drive(1),
                                engine_running: self.vehicle_fuel > 0.0 && self.vehicle_health > 0.0,
                                fuel_level: self.vehicle_fuel,
                                fuel_reserve: self.vehicle_fuel < 0.15,
                                health: self.vehicle_health,
                                ..Default::default()
                            };
                            // Добавить данные колёс
                            for (i, wheel) in vehicle.wheels().iter().enumerate().take(4) {
                                data.wheel_contact[i] = wheel.is_in_contact;
                                data.wheel_slip[i] = if wheel.is_in_contact { 0.1 } else { 0.0 };
                            }
                            // Игра-6: Winch status
                            data.winch_active = self.winch.is_active();
                            data.winch_length_m = self.winch.current_length();
                            data
                        } else {
                            let speed = body.velocity.magnitude() * 3.6;
                            crate::ui::hud::VehicleHudData {
                                speed_kmh: speed,
                                engine_rpm: 800.0 + speed * 10.0,
                                fuel_level: self.vehicle_fuel,
                                fuel_reserve: self.vehicle_fuel < 0.15,
                                health: self.vehicle_health,
                                ..Default::default()
                            }
                        };
                        self.hud_manager.update(hud_data.clone(), delta_time);
                        
                        // Pass HUD data to renderer for rendering
                        self.graphics_context.renderer.set_hud_data(hud_data);
                    }
                }
                
                profiler::stop_timer("physics_step");

                // Update game if it exists (REMOVED - no longer used)
            }
            _ => {
                // Update other systems as needed (still use delta_time)
                // Even in menu states, we may want to update animations, etc.
            }
        }

        // Stop profiling the update cycle
        profiler::stop_timer("update_cycle");
    }

    pub fn render(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Start profiling the render cycle
        profiler::start_timer("render_cycle");
        
        self.graphics_context.render()?;
        
        // Исп-3: Flush debug lines to GL if debug mode is enabled
        if self.debug_mode {
            let vp = self.graphics_context.renderer.camera.view_projection_matrix();
            self.debug_renderer.flush_to_gl(self.graphics_context.get_gl(), vp);
        }
        
        // Исп-4: Render particles
        let vp = self.graphics_context.renderer.camera.view_projection_matrix();
        self.particle_system.render(self.graphics_context.get_gl(), vp);
        
        // Swap buffers
        self.gl_context.swap_buffers()?;
        
        // Stop profiling the render cycle
        profiler::stop_timer("render_cycle");
        
        Ok(())
    }
}