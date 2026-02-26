#pragma once
#include "Window.hpp"
#include "../math/Vector3.hpp"
#include "../world/WorldConfig.hpp"
#include "../world/WorldSlots.hpp"
#include <GL/gl.h>
#include <vector>
#include <string>

class Terrain;
class CharacterController;

class Renderer {
public:
    enum class MenuState {
        LOADING_STATE,
        MAIN_MENU,
        CITY_SELECTION,
        GAME,
        SETTINGS,
        ERROR_STATE
    };

private:
    Window* m_window;
    bool m_initialized;
    MenuState m_currentState;
    struct City {
        const char* name;
        float x;
        float y;
        float z;
    };
    std::vector<City> m_cities;

    void InitializeSiberianCities();

public:
    Renderer();
    ~Renderer();

    bool Initialize();
    void Shutdown();
    void BeginFrame();
    void EndFrame();
    void Clear();

    void RenderTerrain(const Terrain& terrain);
    void RenderCharacter(const CharacterController& character);
    void RenderMenu();
    void RenderMenuSlots(const WorldSlotsManager& slots, int selectedSlot);
    void RenderWorldCreation(const WorldConfig::WorldSettings& settings);
    void RenderCitySelection(int selectedIndex);
    void RenderGame(const Vector3& cameraPos);
    void RenderLoading(float progress);
    void RenderError();

    bool ShouldClose() const { return m_window ? m_window->ShouldClose() : true; }
    void SetMenuState(MenuState state) { m_currentState = state; }
    MenuState GetMenuState() const { return m_currentState; }
    bool IsInitialized() const { return m_initialized; }
    const std::vector<City>& GetCities() const { return m_cities; }

    Window* GetWindow() const { return m_window; }
};
