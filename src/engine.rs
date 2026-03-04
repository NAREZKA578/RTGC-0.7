use winit::{
    event::{WindowEvent, ElementState, KeyEvent},
    keyboard::{KeyCode, PhysicalKey},
};
use std::sync::Arc;
use crate::graphics::GraphicsContext;
use crate::input::InputManager;
use crate::audio::AudioSystem;
use crate::ecs::EcsManager;
use crate::physics;
use crate::graphics::renderer::MenuState;
use crate::game::{Game, Animation, AnimationType};
use crate::profiler;

pub struct Engine {
    pub graphics_context: GraphicsContext,
    pub input_manager: InputManager,
    pub audio_system: AudioSystem,
    pub ecs_manager: EcsManager,
    pub physics_world: physics::PhysicsWorld,
    pub game: Option<Game>,
    window: Arc<winit::window::Window>,
    last_frame_time: std::time::Instant,
}

impl Engine {
    pub fn new(window: Arc<winit::window::Window>) -> Result<Self, Box<dyn std::error::Error>> {
        let graphics_context = GraphicsContext::new(window.clone())?;
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
            window,
            last_frame_time: std::time::Instant::now(),
        })
    }

    pub fn handle_window_event(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::CloseRequested => false,
            WindowEvent::KeyboardInput { event: key_event, .. } => {
                self.handle_key_event(key_event);
                true
            }
            _ => true,
        }
    }

    fn handle_key_event(&mut self, key_event: &KeyEvent) {
        // Update input manager
        self.input_manager.handle_keyboard_input(key_event);

        match (key_event.logical_key, key_event.state) {
            (winit::keyboard::Key::Named(winit::keyboard::NamedKey::Escape), ElementState::Pressed) => {
                // Handle escape key differently based on current menu state
                match self.graphics_context.renderer.menu_state {
                    MenuState::MainMenu | MenuState::Loading => {
                        // Exit the application
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
                    }
                    _ => {}
                }
            }
            _ => {}
        }
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
                self.physics_world.step();
                profiler::stop_timer("physics_step");

                // Update game if it exists
                if let Some(ref mut game) = self.game {
                    profiler::start_timer("game_update");
                    game.update();
                    profiler::stop_timer("game_update");
                }
            }
            _ => {
                // Update other systems as needed
            }
        }

        // Stop profiling the update cycle
        profiler::stop_timer("update_cycle");
    }

    pub fn render(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Start profiling the render cycle
        profiler::start_timer("render_cycle");
        
        self.graphics_context.render()?;
        
        // Stop profiling the render cycle
        profiler::stop_timer("render_cycle");
        
        Ok(())
    }
}