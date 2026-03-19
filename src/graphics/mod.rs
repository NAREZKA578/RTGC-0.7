//! Graphics Module for RTGC-0.7
//! Provides rendering, camera, shaders, meshes, textures, and RHI abstraction

pub mod renderer;
pub mod camera;
pub mod shader;
pub mod mesh;
pub mod texture;
pub mod lod_system;
pub mod texture_streaming;
pub mod lighting;
pub mod rhi;
pub mod material;
pub mod particles;
pub mod debug_renderer;

pub use renderer::{Renderer, MenuState};
pub use camera::Camera;
pub use shader::Shader;
pub use mesh::Mesh;
pub use texture::Texture;
pub use lod_system::LodSystem;
pub use texture_streaming::TextureStreamer;
pub use lighting::{Light, LightManager, LightingConfig};
pub use rhi::{RhiFactory, RhiConfig, IDevice, GraphicsBackend, RhiManager};
pub use material::{Material, MaterialManager, MaterialLayers, MaterialParams, TextureQuality, MaterialStats};
