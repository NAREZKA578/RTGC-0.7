#pragma once
#include <string>
#include <array>

// Lightweight storage for up to 5 world slots (MVP warmup).
// Each slot can be either empty or hold a world definition (name + minimal meta).

class WorldSlotsManager {
public:
    struct WorldSlot {
        std::string worldName;
        bool used;
        bool procedural; // whether this world is procedurally generated
        int seed;
        float scale; // 1.0f means 1 unit == 1 meter
        bool isRealWorld; // placeholder flag for real-world data source
        std::string regionCode; // e.g., OSM region code, used for future real data

        WorldSlot() : worldName(""), used(false), procedural(true), seed(0), scale(1.0f), isRealWorld(false), regionCode("") {}
    };

    static constexpr int SLOT_COUNT = 5;

    WorldSlotsManager();

    void LoadFromDisk();
    void SaveToDisk() const;

    const WorldSlot& GetSlot(int idx) const;
    WorldSlot& GetSlot(int idx);

    bool IsSlotUsed(int idx) const { return (idx >= 0 && idx < SLOT_COUNT) ? m_slots[idx].used : false; }
    void CreateSlot(int idx, const std::string& name);

    int GetSlotCount() const { return SLOT_COUNT; }

    std::string GetSlotDisplay(int idx) const;

private:
    std::array<WorldSlot, SLOT_COUNT> m_slots;
};
