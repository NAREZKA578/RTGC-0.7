#include "WorldSlots.hpp"
#include <fstream>
#include <sstream>
#include <string>
#include <vector>

#if defined(_WIN32)
#include <direct.h>
#define MKDIR(x) _mkdir(x)
#else
#include <sys/stat.h>
#define MKDIR(x) mkdir(x, 0777)
#endif

WorldSlotsManager::WorldSlotsManager() {
    LoadFromDisk();
}

void WorldSlotsManager::LoadFromDisk() {
    MKDIR("save");
    std::ifstream fin("save/world_slots.txt");
    if (!fin) {
        for (int i = 0; i < SLOT_COUNT; ++i) {
            m_slots[i] = WorldSlot();
            m_slots[i].worldName = "World " + std::to_string(i + 1);
            m_slots[i].used = false;
            m_slots[i].procedural = true;
            m_slots[i].seed = 0;
            m_slots[i].scale = 1.0f;
            m_slots[i].regionCode = "";
            m_slots[i].isRealWorld = false;
        }
        SaveToDisk();
        return;
    }

    std::string line;
    int idx = 0;
    while (std::getline(fin, line) && idx < SLOT_COUNT) {
        if (line.empty()) continue;
        if (line.find("Slot") != 0) continue;
        std::vector<std::string> parts;
        std::stringstream ss(line);
        std::string token;
        while (std::getline(ss, token, '|')) parts.push_back(token);
        WorldSlot slot;
        if (parts.size() >= 8) {
            slot.worldName = parts[1];
            slot.used = (parts[2] == "1");
            slot.procedural = (parts[3] == "1");
            slot.seed = std::stoi(parts[4]);
            slot.scale = std::stof(parts[5]);
            slot.regionCode = parts[6];
            slot.isRealWorld = (parts[7] == "1");
        }
        m_slots[idx++] = slot;
    }

    for (; idx < SLOT_COUNT; ++idx) {
        WorldSlot slot;
        slot.worldName = "World " + std::to_string(idx + 1);
        slot.used = false;
        slot.procedural = true;
        slot.seed = 0;
        slot.scale = 1.0f;
        slot.regionCode = "";
        slot.isRealWorld = false;
        m_slots[idx] = slot;
    }
}

void WorldSlotsManager::SaveToDisk() const {
    MKDIR("save");
    std::ofstream fout("save/world_slots.txt");
    if (!fout) return;

    for (int i = 0; i < SLOT_COUNT; ++i) {
        const WorldSlot& s = m_slots[i];
        fout << "Slot" << (i + 1) << "|" << s.worldName << "|" << (s.used ? "1" : "0")
             << "|" << (s.procedural ? "1" : "0") << "|" << s.seed << "|" << s.scale
             << "|" << s.regionCode << "|" << (s.isRealWorld ? "1" : "0") << "\n";
    }
}

const WorldSlotsManager::WorldSlot& WorldSlotsManager::GetSlot(int idx) const {
    return m_slots[idx];
}

WorldSlotsManager::WorldSlot& WorldSlotsManager::GetSlot(int idx) {
    return m_slots[idx];
}

void WorldSlotsManager::CreateSlot(int idx, const std::string& name) {
    if (idx < 0 || idx >= SLOT_COUNT) return;
    WorldSlot& s = m_slots[idx];
    s.used = true;
    s.worldName = name;
    s.procedural = true;
    s.seed = idx * 1337; // simple default seed per slot
    s.scale = 1.0f;
    s.isRealWorld = false;
    s.regionCode = "";
    SaveToDisk();
}

std::string WorldSlotsManager::GetSlotDisplay(int idx) const {
    const WorldSlot& s = m_slots[idx];
    if (!s.used) return "Slot " + std::to_string(idx + 1) + ": (Empty)";
    return "Slot " + std::to_string(idx + 1) + ": " + s.worldName;
}
