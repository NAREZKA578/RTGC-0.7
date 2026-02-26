#pragma once
#include <array>
#include <algorithm> // для std::min

enum class ItemType {
    None,
    Medkit,
    Ammo
};

struct Item {
    ItemType type = ItemType::None;
    int count = 0;
};

class Inventory {
public:
    std::array<Item, 8> slots = {};
    bool Add(ItemType type, int count);
    bool Use(int index, float& health);
};