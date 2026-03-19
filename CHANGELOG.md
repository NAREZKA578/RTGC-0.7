# Changelog

All notable changes to RTGC-0.7 will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- OpenGL RHI backend implementation (`src/graphics/rhi/gl.rs`)
- PBR lighting system with Directional, Point, and Spot lights
- ACES Tone Mapping for HDR rendering
- ECS framework with Archetype-based storage
- Audio engine based on cpal
- Input mapping system for key rebinding
- Asset loader for unified resource loading
- Vehicle physics model
- Chunk manager for world streaming
- Prop spawner for object placement
- CI/CD pipeline with GitHub Actions

### Fixed
- Helicopter Clone trait implementation for async physics
- Arena allocator missing methods (iter, as_mut_ptr, as_mut_slice, Index, split_at_mut)
- Thread pool deadlock in wait_all()
- Random number generation dependencies

### Changed
- Updated module structure for better organization
- Moved utility functions to dedicated modules (math, time, logger)

## [0.7.0] - 2024-01-01

### Added
- Initial RTGC-0.7 release
- Multi-platform RHI (Vulkan, DX12, OpenGL)
- Physics engine with rigid body dynamics
- Terrain generation and chunk streaming
- Helicopter flight model
- Basic ECS architecture
- Async physics processing
- Fracture system for destructible objects
- Dynamic weather system
- Mission save/load system

### Known Issues
- Some RHI backends incomplete
- Physics parallelization has race conditions
- Missing UI rendering
- Audio 3D positioning not implemented

---

## Version History Template

### [X.Y.Z] - YYYY-MM-DD

#### Added
- New features go here

#### Changed
- Changes to existing functionality

#### Deprecated
- Soon-to-be removed features

#### Removed
- Removed features

#### Fixed
- Bug fixes

#### Security
- Security improvements
