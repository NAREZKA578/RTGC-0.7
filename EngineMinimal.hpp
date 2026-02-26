#pragma once
#include <memory>
#include "math/Vector3.hpp"
#include "world/Terrain.hpp"
#include "world/SiberianCities.hpp"
#include "world/WorldConfig.hpp"
#include "physics/CharacterController.hpp"
#include "game/InputManager.hpp"
#include "graphics/Renderer.hpp"
#include "graphics/Window.hpp"
#include "core/Logger.hpp"
#include "world/WorldSlots.hpp"

class Engine {
private:
    bool m_running;
    float m_lastTime;
    float m_frameTime;
    int m_frameCount;
    float m_fps;
    float m_totalTime;
    std::unique_ptr<Terrain> m_terrain;
    std::unique_ptr<CharacterController> m_character;
    std::unique_ptr<SiberianCities> m_cities;
    std::unique_ptr<Renderer> m_renderer;
    InputManager m_input;

    WorldSlotsManager m_worldSlots;
    int m_menuSelectedSlot;
    int m_selectedCityIndex;

    WorldConfig::WorldSettings m_worldSettings;
    std::unique_ptr<WorldConfig::WorldGenerator> m_worldGenerator;
    void InitializeSystems();
    void UpdateSystems(float dt);
    void ShutdownSystems();
    void HandleInput(float dt);
    void CompleteLoading();
    void EnterWorldCreationFromSlot(int slotIndex);

public:
    Engine();
    ~Engine();
    bool Initialize();
    void Run();
    void Shutdown();
    bool IsRunning() const { return m_running; }
    float GetFPS() const { return m_fps; }
};
