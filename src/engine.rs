use winit::{
    event::{WindowEvent, ElementState, KeyEvent},
    keyboard::{KeyCode, PhysicalKey},
};
use std::sync::Arc;
use crate::graphics::GraphicsContext;
use crate::input::InputManager;
use crate::audio::AudioSystem;
use crate::ecs::EcsManager;
use crate::graphics::renderer::MenuState;
use crate::physics::PhysicsWorld;

pub struct Engine {
    pub graphics_context: GraphicsContext,
    pub input_manager: InputManager,
    pub audio_system: AudioSystem,
    pub ecs_manager: EcsManager,
    pub physics_world: PhysicsWorld,
    window: Arc<winit::window::Window>,
    last_frame_time: std::time::Instant,
    frame_count: u32,
    fps_counter: u32,
    fps_timer: std::time::Duration,
    target_fps: u32,
    delta_time: f32,
}

impl Engine {
    pub fn new(window: Arc<winit::window::Window>) -> Result<Self, Box<dyn std::error::Error>> {
        let graphics_context = GraphicsContext::new(window.clone())?;
        let input_manager = InputManager::new();
        let audio_system = AudioSystem::new()?;
        let ecs_manager = EcsManager::new();
        let physics_world = PhysicsWorld::new();

        Ok(Self {
            graphics_context,
            input_manager,
            audio_system,
            ecs_manager,
            physics_world,
            window,
            last_frame_time: std::time::Instant::now(),
            frame_count: 0,
            fps_counter: 0,
            fps_timer: std::time::Duration::from_secs(0),
            target_fps: 60,
            delta_time: 0.0,
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
        match (key_event.logical_key, key_event.state) {
            (winit::keyboard::Key::Named(winit::keyboard::NamedKey::Escape), ElementState::Pressed) => {
                // Handle escape key differently based on current menu state
                match self.graphics_context.renderer.menu_state {
                    MenuState::MainMenu | MenuState::Loading => {
                        // Exit the application
                    }
                    MenuState::CitySelection | MenuState::Game | MenuState::WorldCreation | MenuState::Settings => {
                        // Go back to main menu
                        self.graphics_context.renderer.menu_state = MenuState::MainMenu;
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

        // Update input manager
        self.input_manager.handle_keyboard_input(key_event);
    }

    pub fn update(&mut self) {
        let current_time = std::time::Instant::now();
        self.delta_time = (current_time - self.last_frame_time).as_secs_f32();
        self.last_frame_time = current_time;

        // FPS calculation
        self.fps_timer += std::time::Duration::from_secs_f32(self.delta_time);
        self.frame_count += 1;
        
        if self.fps_timer.as_secs_f32() >= 1.0 {
            self.fps_counter = self.frame_count;
            self.frame_count = 0;
            self.fps_timer = std::time::Duration::from_secs_f32(self.fps_timer.as_secs_f32() - 1.0);
        }

        // Update systems based on current menu state
        match self.graphics_context.renderer.menu_state {
            MenuState::Game => {
                // Update physics world
                self.physics_world.step(self.delta_time);
                
                // Update game-specific systems
            }
            _ => {
                // Update other systems as needed
            }
        }
    }

    pub fn render(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.graphics_context.render()?;
        Ok(())
    }
}