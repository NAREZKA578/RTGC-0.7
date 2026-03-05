#[path = "audio/audio_module.rs"]
pub mod audio;
#[path = "ecs/ecs_module.rs"]
pub mod ecs;
#[path = "graphics/graphics_module.rs"]
pub mod graphics;
#[path = "input/input_module.rs"]
pub mod input;
#[path = "physics/physics_module.rs"]
pub mod physics;
#[path = "render/render_module.rs"]
pub mod render;
#[path = "ui/ui_module.rs"]
pub mod ui;
pub mod networking;
pub mod profiler;
pub mod engine;

pub use ecs::*;
pub use physics::*;
pub use graphics::*;
pub use audio::*;
pub use networking::*;
pub use ui::*;
pub use profiler::*;
pub use engine::*;

// Core engine types re-export
pub use nalgebra;
pub use rapier3d;
pub use wgpu;
pub use winit;
pub use glam;