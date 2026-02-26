# RTGC MVP Plan

This document describes the MVP approach for RTGC with 5 world slots, procedural generation, and basic world rendering on Windows. It outlines architecture, data storage, and a release-first feature set.

## MVP Goals
- Workable main menu with 5 world slots.
- Procedural world generation for new slots.
- Ability to save and load up to 5 slots from disk under /save.
- Basic 1:1 scale framework in data model for future real-world data integration.
- Simple, consistent UI styled like Snow Runner (text-driven menu).

## Architecture Overview
- 5 world slots managed by WorldSlotsManager.
- WorldGenerator to generate city layouts and roads (stubs for MVP).
- Engine handles state machine: LOADING -> MENU -> WORLD_CREATION -> GAME.
- Renderer renders menu, world creation, and game views. Menu includes RenderMenuSlots(slots, selectedSlot).
- Save data stored under /save/world_slots.txt; directory created automatically.

## Storage Model
- WorldSlot: { worldName, used, procedural, seed, scale, isRealWorld, regionCode }
- WorldSlotsManager:
  - SLOT_COUNT = 5
  - LoadFromDisk(), SaveToDisk()
  - GetSlot(int), CreateSlot(int, string), GetSlotDisplay(int)

## World Generation (MVP)
- Procedural: generate a simple set of cities and roads with deterministic seed.
- Real-world mode: placeholder regionCode; data loaded lazily later.
- Scale: world units to meters with WorldSettings.scale; default 1.0f.

## MVP World Slots UX
- MENU shows 5 rows: Slot 1..Slot 5 with either (Empty) or WorldName.
- Press 1-5:
  - If empty: create slot and go to WORLD_CREATION for basic config.
  - If filled: load world data into memory and start GAME (after simple Generate).

## Future Enhancements (post-MVP)
- Lazy-load real-world data via streaming and region-based data sources.
- Advanced city generation, road networks, and physics integration.
- Cross-platform UI and platform-specific input handling.

## Plan Execution & Validation
- Build and run on Windows 10.
- Validate menu navigation, slot creation, world generation, and save/load.

