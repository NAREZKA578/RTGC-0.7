//! RHI (Render Hardware Interface) Module
//! Provides abstraction over different graphics APIs (Vulkan, DX12, OpenGL)

pub mod types;
pub mod device;
pub mod factory;
pub mod rhi_module;

#[cfg(feature = "vulkan")]
pub mod vulkan;

#[cfg(feature = "dx12")]
pub mod dx12;

pub mod gl;

pub use types::*;
pub use device::IDevice;
pub use factory::RhiFactory;
pub use rhi_module::RhiManager;

#[cfg(feature = "vulkan")]
pub use vulkan::create_vulkan_device;

#[cfg(feature = "dx12")]
pub use dx12::create_dx12_device;

pub use gl::create_gl_device;
