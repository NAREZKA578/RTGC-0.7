//! ECS (Entity Component System) Module for RTGC-0.7
//! Provides entity management, component storage, and system execution

pub mod ecs_module;
pub mod world;
pub mod entity;
pub mod component;
pub mod system;
pub mod job_system;

pub use ecs_module::EcsManager;
pub use world::World;
pub use entity::Entity;
pub use component::Component;
pub use system::{System, SystemScheduler};
pub use job_system::JobSystem;
