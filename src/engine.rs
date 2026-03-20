use winit::{
    event::{WindowEvent, ElementState, KeyEvent},
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
use crate::game::{WeatherSystem, DayNightCycle, Cargo, Winch};
use crate::graphics::particles::ParticleSystem;
use crate::graphics::debug_renderer::DebugRenderer;
use crate::profiler;
use crate::ui::HudManager;
use crate::assets::VehicleLoader;
use crate::world::{TerrainGenerator, ChunkId, generate_chunk_mesh, TerrainVertex, CHUNK_SIZE, HEIGHTMAP_RESOLUTION};
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
    // Terrain & Vehicle (Sprint 1)
    terrain_generator: Option<TerrainGenerator>,
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
}

impl Engine {
    pub fn new(event_loop: &winit::event_loop::EventLoop<()>) -> Result<Self, Box<dyn std::error::Error>> {
        // === SPRINT 1: Create terrain and vehicle ===
        
        // 1. Create terrain generator
        let terrain_gen = TerrainGenerator::new(Default::default());
        
        // 2. Generate starting chunk
        let chunk_id = ChunkId::new(0, 0);
        let chunk_data = terrain_gen.generate_chunk(chunk_id);
        
        // 3. Convert heights to format expected by RigidBody::new_terrain
        // The heightmap is HEIGHTMAP_RESOLUTION x HEIGHTMAP_RESOLUTION
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
        
        // 4. Create terrain body and add to physics world
        let terrain_body = physics::RigidBody::new_terrain(
            Vector3::zeros(),
            height_map,
            Vector3::new(CHUNK_SIZE as f32, 1.0, CHUNK_SIZE as f32),
        );
        physics_world.add_body(terrain_body);
        
        // 5. Create vehicle chassis and Vehicle physics
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
        
        // 6. Generate terrain mesh data for renderer
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
            terrain_generator: Some(terrain_gen),
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
                    MenuState::CitySelection | MenuState::InGame | MenuState::WorldCreation | MenuState::Settings => {
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
            _ => {}
        }
        true
    }

    pub fn update(&mut self) {
        let current_time = std::time::Instant::now();
        let delta_time = (current_time - self.last_frame_time).as_secs_f32();
        self.last_frame_time = current_time;

        // Start profiling the update cycle
        profiler::start_timer("update_cycle");

        // Update systems based on current menu state
        match self.graphics_context.renderer.menu_state {
            MenuState::InGame => {
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
                        if let Some(ref gen) = self.terrain_generator {
                            let h = |x: f32, z: f32| gen.get_height(x, z);
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
                                engine_running: true,
                                fuel_level: 1.0,
                                ..Default::default()
                            };
                            // Добавить данные колёс
                            for (i, wheel) in vehicle.wheels().iter().enumerate().take(4) {
                                data.wheel_contact[i] = wheel.is_in_contact;
                                data.wheel_slip[i] = if wheel.is_in_contact { 0.1 } else { 0.0 };
                            }
                            data
                        } else {
                            let speed = body.velocity.magnitude() * 3.6;
                            crate::ui::hud::VehicleHudData {
                                speed_kmh: speed,
                                engine_rpm: 800.0 + speed * 10.0,
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
        
        // Swap buffers
        self.gl_context.swap_buffers()?;
        
        // Stop profiling the render cycle
        profiler::stop_timer("render_cycle");
        
        Ok(())
    }
}