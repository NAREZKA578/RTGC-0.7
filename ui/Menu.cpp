#include "Menu.hpp"
#include "../core/Logger.hpp"
#include <iostream>
#include <thread>
#include <chrono>

Menu::Menu() : currentState(MenuState::MAIN_MENU), initialized(false) {
}

Menu::~Menu() {
    Shutdown();
}

bool Menu::Initialize() {
    Logger::Log("Menu initialize");
    initialized = true;
    return true;
}

void Menu::Shutdown() {
    if (!initialized) return;
    
    Logger::Log("Menu shutdown");
    initialized = false;
}

void Menu::Update() {
    if (!initialized) return;
    
    // Обработка ввода для меню
    switch (currentState) {
        case MenuState::MAIN_MENU:
            // В реальной версии здесь будет обработка ввода
            break;
        case MenuState::SETTINGS:
            // Настройки
            break;
        case MenuState::GAME_RUNNING:
            // Игра запущена, меню не активно
            break;
        case MenuState::PAUSED:
            // Пауза
            break;
        case MenuState::LOADING:
            // Загрузка
            break;
    }
}

void Menu::Render() {
    if (!initialized) return;
    
    // Очищаем экран для меню
    system("cls");
    
    switch (currentState) {
        case MenuState::MAIN_MENU:
            ShowMainMenu();
            break;
        case MenuState::SETTINGS:
            ShowSettings();
            break;
        case MenuState::GAME_RUNNING:
            // Игра рендерится отдельно
            break;
        case MenuState::PAUSED:
            ShowPauseMenu();
            break;
        case MenuState::LOADING:
            ShowLoadingScreen();
            break;
    }
}

void Menu::SetState(MenuState state) {
    Logger::Log("Menu state changed to: ", static_cast<int>(state));
    currentState = state;
}

void Menu::ShowMainMenu() {
    std::cout << "==========================================" << std::endl;
    std::cout << "           RTGC Game Engine            " << std::endl;
    std::cout << "==========================================" << std::endl;
    std::cout << std::endl;
    std::cout << "  [1] Start Game" << std::endl;
    std::cout << "  [2] Settings" << std::endl;
    std::cout << "  [3] Exit" << std::endl;
    std::cout << std::endl;
    std::cout << "==========================================" << std::endl;
    std::cout << "Press 1, 2, or 3 to select" << std::endl;
    std::cout << "==========================================" << std::endl;
}

void Menu::ShowSettings() {
    std::cout << "==========================================" << std::endl;
    std::cout << "              Settings                 " << std::endl;
    std::cout << "==========================================" << std::endl;
    std::cout << std::endl;
    std::cout << "  [1] Graphics Settings" << std::endl;
    std::cout << "  [2] Audio Settings" << std::endl;
    std::cout << "  [3] Controls" << std::endl;
    std::cout << "  [4] Back to Main Menu" << std::endl;
    std::cout << std::endl;
    std::cout << "==========================================" << std::endl;
    std::cout << "Press 1-4 to select" << std::endl;
    std::cout << "==========================================" << std::endl;
}

void Menu::ShowLoadingScreen() {
    std::cout << "==========================================" << std::endl;
    std::cout << "              Loading...               " << std::endl;
    std::cout << "==========================================" << std::endl;
    
    static int dots = 0;
    for (int i = 0; i < dots; i++) {
        std::cout << ".";
    }
    std::cout << std::endl;
    
    dots = (dots + 1) % 4;
    std::this_thread::sleep_for(std::chrono::milliseconds(500));
}

void Menu::ShowPauseMenu() {
    std::cout << "==========================================" << std::endl;
    std::cout << "              PAUSED                  " << std::endl;
    std::cout << "==========================================" << std::endl;
    std::cout << std::endl;
    std::cout << "  [1] Resume Game" << std::endl;
    std::cout << "  [2] Settings" << std::endl;
    std::cout << "  [3] Exit to Main Menu" << std::endl;
    std::cout << std::endl;
    std::cout << "==========================================" << std::endl;
    std::cout << "Press 1-3 to select" << std::endl;
    std::cout << "==========================================" << std::endl;
}

void Menu::SetStartGameCallback(Callback callback) {
    onStartGame = callback;
}

void Menu::SetSettingsCallback(Callback callback) {
    onOpenSettings = callback;
}

void Menu::SetExitCallback(Callback callback) {
    onExit = callback;
}