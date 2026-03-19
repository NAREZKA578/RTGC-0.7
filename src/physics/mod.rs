//! Physics Module for RTGC-0.7
//! Provides rigid body dynamics, collision detection, constraints, and vehicle physics

pub mod physics_module;
pub mod arena_allocator;
pub mod spatial_hash;
pub mod async_physics;
pub mod thread_pool;
pub mod fracture_component;
pub mod helicopter;
pub mod advanced_vehicle;
pub mod vehicle;
pub mod deformable_terrain;

pub use physics_module::{PhysicsWorld, RigidBody, Collider, PhysicsConfig};
pub use arena_allocator::ArenaAllocator;
pub use spatial_hash::SpatialHash;
pub use async_physics::AsyncPhysicsEngine;
pub use thread_pool::ThreadPool;
pub use fracture_component::FractureComponent;
pub use helicopter::{Helicopter, HelicopterConfig, HelicopterControls};
pub use advanced_vehicle::AdvancedVehicle;
pub use vehicle::Vehicle;
pub use deformable_terrain::DeformableTerrain;
