use winit::window::Window;
use std::sync::Arc;
use crate::graphics::GraphicsContext;
use crate::input::InputManager;
use crate::audio::AudioSystem;
use crate::ecs::EcsManager;

pub struct Engine {
    pub graphics_context: GraphicsContext,
    pub input_manager: InputManager,
    pub audio_system: AudioSystem,
    pub ecs_manager: EcsManager,
    window: Arc<Window>,
}

impl Engine {
    pub fn new(window: Arc<Window>) -> Result<Self, Box<dyn std::error::Error>> {
        let graphics_context = GraphicsContext::new(window.clone())?;
        let input_manager = InputManager::new();
        let audio_system = AudioSystem::new()?;
        let ecs_manager = EcsManager::new();

        Ok(Self {
            graphics_context,
            input_manager,
            audio_system,
            ecs_manager,
            window,
        })
    }

    pub fn handle_window_event(&mut self, event: &winit::event::WindowEvent) -> bool {
        match event {
            winit::event::WindowEvent::CloseRequested => false,
            winit::event::WindowEvent::KeyboardInput { input, .. } => {
                self.input_manager.handle_keyboard_input(input);
                true
            }
            _ => true,
        }
    }

    pub fn render(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.graphics_context.render()?;
        Ok(())
    }
}