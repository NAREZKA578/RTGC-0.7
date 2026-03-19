use winit::{
    event::{WindowEvent, ElementState, KeyEvent},
    keyboard::{KeyCode, PhysicalKey},
};
use std::sync::Arc;
use crate::graphics::{GraphicsContext, gl_context::GlContext, TextureQuality, MaterialManager};
use crate::input::InputManager;
use crate::audio::AudioSystem;
use crate::ecs::EcsManager;
use crate::physics;
use crate::graphics::renderer::MenuState;
use crate::game::{Game, Animation, AnimationType};
use crate::profiler;
use crate::ui::HudManager;
use crate::assets::VehicleLoader;

pub struct Engine {
    pub graphics_context: GraphicsContext,
    pub input_manager: InputManager,
    pub audio_system: AudioSystem,
    pub ecs_manager: EcsManager,
    pub physics_world: physics::PhysicsWorld,
    pub game: Option<Game>,
    gl_context: GlContext,
    last_frame_time: std::time::Instant,
    // C1: Fixed timestep accumulator
    physics_accumulator: f32,
    physics_timestep: f32,
    // HUD Manager
    hud_manager: HudManager,
    // Material Manager
    material_manager: MaterialManager,
}

impl Engine {
    pub fn new(event_loop: &winit::event_loop::EventLoop<()>) -> Result<Self, Box<dyn std::error::Error>> {
        // Создаем OpenGL контекст через glutin
        let gl_context = GlContext::new(event_loop)?;
        
        let window = Arc::new(gl_context.window.clone());
        let gl = gl_context.gl.clone();
        
        let graphics_context = GraphicsContext::new(window, gl)?;
        let input_manager = InputManager::new();
        let audio_system = AudioSystem::new()?;
        let ecs_manager = EcsManager::new();
        let physics_world = physics::PhysicsWorld::new();

        Ok(Self {
            graphics_context,
            input_manager,
            audio_system,
            ecs_manager,
            physics_world,
            game: None,
            gl_context,
            last_frame_time: std::time::Instant::now(),
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
                if let Some(ref mut game) = self.game {
                    if key_event.state == ElementState::Pressed {
                        game.set_throttle(1.0);
                    } else {
                        game.set_throttle(0.0);
                    }
                }
            }
            (winit::keyboard::Key::Named(winit::keyboard::NamedKey::ArrowDown), ElementState::Pressed) |
            (winit::keyboard::Key::Named(winit::keyboard::NamedKey::ArrowDown), ElementState::Released) => {
                if let Some(ref mut game) = self.game {
                    if key_event.state == ElementState::Pressed {
                        game.set_throttle(-1.0);
                    } else {
                        game.set_throttle(0.0);
                    }
                }
            }
            (winit::keyboard::Key::Named(winit::keyboard::NamedKey::ArrowLeft), ElementState::Pressed) |
            (winit::keyboard::Key::Named(winit::keyboard::NamedKey::ArrowLeft), ElementState::Released) => {
                if let Some(ref mut game) = self.game {
                    if key_event.state == ElementState::Pressed {
                        game.set_steering(-1.0);
                    } else {
                        game.set_steering(0.0);
                    }
                }
            }
            (winit::keyboard::Key::Named(winit::keyboard::NamedKey::ArrowRight), ElementState::Pressed) |
            (winit::keyboard::Key::Named(winit::keyboard::NamedKey::ArrowRight), ElementState::Released) => {
                if let Some(ref mut game) = self.game {
                    if key_event.state == ElementState::Pressed {
                        game.set_steering(1.0);
                    } else {
                        game.set_steering(0.0);
                    }
                }
            }
            (winit::keyboard::Key::Named(winit::keyboard::NamedKey::Space), ElementState::Pressed) |
            (winit::keyboard::Key::Named(winit::keyboard::NamedKey::Space), ElementState::Released) => {
                if let Some(ref mut game) = self.game {
                    if key_event.state == ElementState::Pressed {
                        game.set_brake(1.0);
                    } else {
                        game.set_brake(0.0);
                    }
                }
            }
            (winit::keyboard::Key::Character(ref c), ElementState::Pressed) if c == "r" || c == "R" => {
                // Reset truck position
                if let Some(ref mut game) = self.game {
                    game.reset_truck();
                }
            }
            (winit::keyboard::Key::Character(ref c), ElementState::Pressed) if c == "e" || c == "E" => {
                // Activate cargo action (pickup/drop)
                if let Some(ref mut game) = self.game {
                    game.activate_cargo_action();
                }
            }
            (winit::keyboard::Key::Character(ref c), ElementState::Pressed) if c == "c" || c == "C" => {
                // Switch camera view (first person/third person)
                if let Some(ref mut game) = self.game {
                    match self.graphics_context.renderer.camera.camera_type {
                        crate::graphics::camera::CameraType::FirstPerson => {
                            self.graphics_context.renderer.camera.switch_to_third_person(
                                game.get_truck_position(),
                                game.get_truck_rotation()
                            );
                        }
                        crate::graphics::camera::CameraType::ThirdPerson => {
                            self.graphics_context.renderer.camera.switch_to_first_person(
                                game.get_truck_position(),
                                game.get_truck_rotation()
                            );
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
                // Update physics world
                profiler::start_timer("physics_step");
                self.physics_world.step(delta_time);
                profiler::stop_timer("physics_step");

                // Update game if it exists
                if let Some(ref mut game) = self.game {
                    profiler::start_timer("game_update");
                    game.update(delta_time);
                    profiler::stop_timer("game_update");
                    
                    // Update texture streaming based on truck position
                    if let Some(ref game) = self.game {
                        let truck_position = game.get_truck_position();
                        self.graphics_context.renderer.texture_streaming.update_camera_position(nalgebra::Vector2::new(
                            truck_position.x,
                            truck_position.z,
                        ));
                    }
                }
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