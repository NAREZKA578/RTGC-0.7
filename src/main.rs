use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use std::sync::Arc;
use parking_lot::Mutex;

#[path = "engine.rs"]
mod engine;
#[path = "graphics/graphics_module.rs"]
mod graphics;
#[path = "input/input_module.rs"]
mod input;
#[path = "audio/audio_module.rs"]
mod audio;
#[path = "physics/physics_module.rs"]
mod physics;
#[path = "ecs/ecs_module.rs"]
mod ecs;
#[path = "ui/ui_module.rs"]
mod ui;
#[path = "game/mission_save.rs"]
mod game;
#[path = "profiler.rs"]
mod profiler;

// Initialize tracing
use tracing_subscriber;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing subscriber
    tracing_subscriber::fmt::init();
    
    env_logger::init();
    
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Off-Road Truck Simulator")
        .with_inner_size(winit::dpi::LogicalSize::new(1920, 1080))
        .build(&event_loop)?;

    let window = Arc::new(window);
    
    // Initialize engine components
    let mut engine = engine::Engine::new(window.clone())?;
    
    // Initialize game state
    engine.game = Some(game::Game::new());
    engine.graphics_context.renderer.menu_state = graphics::renderer::MenuState::InGame;
    
    let mut frame_count = 0;
    let mut last_prof_report = std::time::Instant::now();
    
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                if !engine.handle_window_event(event) {
                    // Print profiling report before exiting
                    profiler::print_profile_report();
                    *control_flow = ControlFlow::Exit;
                }
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                engine.update();
                
                // Update camera position based on truck position and rotation
                if let Some(ref game) = engine.game {
                    engine.graphics_context.renderer.update_camera_for_frame(
                        game.get_truck_position(),
                        game.get_truck_rotation()
                    );
                }
                
                engine.render().unwrap();
                
                // Increment frame counter
                frame_count += 1;
                
                // Print profiling report every 10 seconds
                if last_prof_report.elapsed().as_secs() >= 10 {
                    profiler::print_profile_report();
                    profiler::reset_profiler(); // Reset for next period
                    frame_count = 0;
                    last_prof_report = std::time::Instant::now();
                }
            }
            _ => {}
        }
    });
}
