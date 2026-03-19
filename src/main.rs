use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
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
#[path = "utils/mod.rs"]
mod utils;

// Initialize tracing
use tracing_subscriber;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging (only one logger - tracing)
    utils::logger::init_logger();
    
    let event_loop = EventLoop::new()?;
    
    // Initialize engine components (creates OpenGL context via glutin)
    let mut engine = engine::Engine::new(&event_loop)?;
    
    let window = Arc::new(engine.gl_context.window.clone());
    
    // Initialize game state
    engine.game = Some(game::Game::new());
    engine.graphics_context.renderer.menu_state = graphics::renderer::MenuState::InGame;
    
    let mut frame_count = 0;
    let mut last_prof_report = std::time::Instant::now();
    
    event_loop.run(move |event, event_loop| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                if !engine.handle_window_event(event) {
                    // Print profiling report before exiting
                    profiler::print_profile_report();
                    event_loop.exit();
                }
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
                
                if let Err(e) = engine.render() {
                    eprintln!("Render error: {:?}", e);
                    // Don't panic, just log the error and continue
                }
                
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
            Event::AboutToWait => {
                // Request redraw for next frame
                window.request_redraw();
            }
            _ => {}
        }
    });
}
