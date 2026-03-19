//! Assets module for RTGC-0.7

pub mod loader;
pub mod assets_module;
pub mod asset_loader;

pub use loader::{AssetLoader, AssetHandle, AssetData, AssetType, AssetMetadata, LoaderConfig, AssetLoadError};
