//! Модули игрового движка на Rust
//! 
//! Полный движок включает:
//! - Генерацию ландшафта с гидравлической и термальной эрозией
//! - PBR рендеринг с bloom и tone mapping
//! - Физику шин с давлением и температурой
//! - Асинхронную физику для стабильного FPS
//! - Систему миссий и сохранений
//! - Поддержку геймпадов с вибрацией
//! - Встроенный аудио движок
//! - Систему частиц

pub mod terrain;
pub mod graphics;
pub mod physics;
pub mod gameplay;
pub mod input;
pub mod audio;
pub mod particles;
pub mod save;

// Re-exports для удобства
pub use terrain::terrain_generator::{TerrainGenerator, ErosionConfig};
pub use graphics::pbr::{PostProcessor, PostProcessConfig, BloomConfig};
pub use graphics::renderer::{Renderer, Camera, Mesh};
pub use physics::tire_physics::{TireModel, TireConfig as TirePhysicsConfig, TireState, RoadSurface, TireType};
pub use physics::async_physics::{AsyncPhysicsEngine, PhysicsConfig, PhysicsCommand, RigidBodyState};
pub use gameplay::mission_system::{MissionManager, SaveSystem, Mission, SaveData, MissionType, MissionReward};
pub use input::gamepad::{GamepadManager, GamepadState, InputAction, CombinedInput};
pub use audio::audio_engine::{AudioEngine, SoundEffect, MusicTrack};
pub use particles::particle_system::{ParticleSystem, ParticleEmitter, ParticleConfig};
