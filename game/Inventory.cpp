#include "Inventory.hpp"
#include <algorithm> // для std::min

bool Inventory::Add(ItemType type, int count) {
    for (auto& s : slots) {
        if (s.type == type && s.count < 99) {
            s.count = std::min(99, s.count + count);
            return true;
        }
    }
    for (auto& s : slots) {
        if (s.type == ItemType::None) {
            s.type = type;
            s.count = count;
            return true;
        }
    }
    return false;
}

bool Inventory::Use(int index, float& health) {
    if (index >= 8) return false;
    auto& s = slots[index];
    if (s.type == ItemType::Medkit && s.count > 0) {
        health = std::min(100.0f, health + 50.0f);
        s.count--;
        if (s.count <= 0) s.type = ItemType::None;
        return true;
    }
    return false;
}