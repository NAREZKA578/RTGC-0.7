#pragma once
#include "../game/Inventory.hpp"
#include <vector>

struct UIItem {
    ItemType type;
    int count = 0;
    bool isSelected = false;
};

class InventoryUI {
    std::vector<UIItem> uiItems;
    int selectedSlot = 0;
    bool isVisible = false;

public:
    void SetInventory(const Inventory& inventory);
    void Render();
    void HandleInput();
    void ToggleVisibility() { isVisible = !isVisible; }
}; 