#include "InventoryUI.hpp"
#include <imgui.h>
#include "../core/Logger.hpp"

void InventoryUI::SetInventory(const Inventory& inventory) {
    uiItems.clear();
    for (int i = 0; i < 8; ++i) {
        auto& slot = inventory.slots[i];
        if (slot.type != ItemType::None) {
            uiItems.push_back({slot.type, slot.count});
        }
    }
}

void InventoryUI::Render() {
    if (!isVisible) return;
    ImGui::Begin("Инвентарь", nullptr, ImGuiWindowFlags_NoDecoration);
    for (size_t i = 0; i < uiItems.size(); ++i) {
        const char* name = (uiItems[i].type == ItemType::Medkit) ? "Аптечка" : "Патроны";
        if (ImGui::Button(name)) {
            selectedSlot = i;
        }
        ImGui::SameLine();
        ImGui::Text("%d", uiItems[i].count);
    }
    ImGui::End();
}

void InventoryUI::HandleInput() {
    // Обработка ввода инвентаря
}