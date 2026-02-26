#include "HudUI.hpp"

void HudUI::Render(CharacterComponent& character) {
    ImGui::Begin("HUD", nullptr, ImGuiWindowFlags_NoDecoration | ImGuiWindowFlags_NoBackground);
    ImGui::SetWindowPos({10, 10});
    ImGui::Text("Здоровье: %.1f", character.health);
    ImGui::ProgressBar(character.health / 100.0f, ImVec2(200, 20));
    if (!character.isAlive) {
        ImGui::SetWindowSize({400, 200});
        ImGui::SetWindowPos({ImGui::GetIO().DisplaySize.x / 2 - 200, ImGui::GetIO().DisplaySize.y / 2 - 100});
        ImGui::Text("ПОГИБ");
        if (ImGui::Button("ВОСКРЕСНУТЬ")) {
            character.health = 100.0f;
            character.isAlive = true;
        }
    }
    ImGui::Text("Инвентарь:");
    for (int i = 0; i < 8; ++i) {
        auto& item = character.inventory.slots[i];
        if (item.type != ItemType::None) {
            ImGui::Text("%d: %s x%d", i,
                item.type == ItemType::Medkit ? "Аптечка" : "Патроны", item.count);
        }
    }
    ImGui::End();
}

void HudUI::Init(HWND hwnd) {
    ImGui::CreateContext();
    ImGui::StyleColorsDark();
    ImGui_ImplWin32_Init(hwnd);
    ImGui_ImplOpenGL3_Init("#version 330");
}