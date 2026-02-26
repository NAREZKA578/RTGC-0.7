#pragma once
#include <string>
#include <vector>
#include <functional>

enum class MenuState {
    MainMenu,
    Settings,
    Pause,
    InGame
};

class MenuSystem {
    MenuState currentState = MenuState::MainMenu;
    std::vector<std::function<void()>> menuItems;
    int selectedItem = 0;

public:
    void Render();
    void Update();
    void HandleInput();
    void SetState(MenuState state) { currentState = state; }
    void AddItem(const std::string& text, std::function<void()> callback);
};