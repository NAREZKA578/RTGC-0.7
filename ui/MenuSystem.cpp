#include "MenuSystem.hpp"
#include <imgui.h>
#include "../core/Logger.hpp"

void MenuSystem::AddItem(const std::string& text, std::function<void()> callback) {
    menuItems.push_back(callback);
}

void MenuSystem::Render() {
    ImGui::Begin("Menu", nullptr, ImGuiWindowFlags_NoDecoration);
    switch (currentState) {
        case MenuState::MainMenu:
            ImGui::Text("RTGC - Russian Technological Game Core");
            if (ImGui::Button("Новая игра")) {
                SetState(MenuState::InGame);
            }
            if (ImGui::Button("Настройки")) {
                SetState(MenuState::Settings);
            }
            if (ImGui::Button("Выход")) {
                // Выход из игры
            }
            break;
        case MenuState::Settings:
            ImGui::Text("Настройки");
            if (ImGui::Button("Назад")) {
                SetState(MenuState::MainMenu);
            }
            break;
        case MenuState::Pause:
            ImGui::Text("Пауза");
            if (ImGui::Button("Продолжить")) {
                SetState(MenuState::InGame);
            }
            if (ImGui::Button("Выход в меню")) {
                SetState(MenuState::MainMenu);
            }
            break;
        default:
            break;
    }
    ImGui::End();
}

void MenuSystem::Update() {
    // Логика обновления меню
}

void MenuSystem::HandleInput() {
    // Обработка ввода в меню
}