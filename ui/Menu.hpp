#pragma once
#include <string>
#include <functional>

enum class MenuState {
    MAIN_MENU,
    SETTINGS,
    GAME_RUNNING,
    PAUSED,
    LOADING
};

class Menu {
private:
    MenuState currentState;
    bool initialized;
    
public:
    Menu();
    ~Menu();
    
    bool Initialize();
    void Shutdown();
    void Update();
    void Render();
    
    void SetState(MenuState state);
    MenuState GetState() const { return currentState; }
    
    void ShowMainMenu();
    void ShowSettings();
    void ShowLoadingScreen();
    void ShowPauseMenu();
    
    typedef std::function<void()> Callback;
    void SetStartGameCallback(Callback callback);
    void SetSettingsCallback(Callback callback);
    void SetExitCallback(Callback callback);
    
private:
    Callback onStartGame;
    Callback onOpenSettings;
    Callback onExit;
};