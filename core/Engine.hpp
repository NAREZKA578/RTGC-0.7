#pragma once
#include "../math/Vector3.hpp"
#include "../physics/PhysicsWorld.hpp"
#include "../graphics/Renderer.hpp"
#include "../audio/AudioSystem.hpp"
#include "../network/NetworkSystem.hpp"
#include "../world/WorldManager.hpp"
#include "../game/CharacterController.hpp"
#include "../game/InputManager.hpp"
#include "../world/Terrain.hpp"
#include "../core/Logger.hpp"
#include <memory>
#include <chrono>

class Engine {
private:
    bool running;
    float lastTime;
    float frameTime;
    int frameCount;
    float fps;
    float totalTime;
    std::chrono::steady_clock::time_point startTime;
    
    // Core systems
    std::unique_ptr<Renderer> renderer;
    std::unique_ptr<PhysicsWorld> physicsWorld;
    std::unique_ptr<AudioSystem> audioSystem;
    std::unique_ptr<NetworkSystem> networkSystem;
    std::unique_ptr<WorldManager> worldManager;
    
    // Game objects
    std::unique_ptr<Terrain> terrain;
    std::unique_ptr<CharacterController> character;
    InputManager inputManager;
    
    void InitializeSystems();
    void UpdateSystems(float dt);
    void RenderSystems();
    void ShutdownSystems();
    void HandleInput(float dt);
    void CalculateFPS();
    
public:
    Engine();
    ~Engine();
    
    bool Initialize();
    void Run();
    void Shutdown();
    
    // Configuration
    void SetWindowSize(int width, int height);
    void SetFullscreen(bool fullscreen);
    void SetVSync(bool enabled);
    void SetMasterVolume(float volume);
    
    // Game state
    void Pause();
    void Resume();
    void Reset();
    
    // Network
    bool StartServer(int port = 1234);
    bool ConnectToServer(const std::string& address, int port = 1234);
    void Disconnect();
    
    // Debug
    void EnableDebugMode(bool enabled);
    void PrintPerformanceStats();
    void SaveScreenshot(const std::string& filename);
    
    // Getters
    bool IsRunning() const { return running; }
    bool IsPaused() const { return false; } // TODO: Implement pause state
    float GetFPS() const { return fps; }
    float GetFrameTime() const { return frameTime; }
    
    Renderer* GetRenderer() const { return renderer.get(); }
    PhysicsWorld* GetPhysicsWorld() const { return physicsWorld.get(); }
    AudioSystem* GetAudioSystem() const { return audioSystem.get(); }
    NetworkSystem* GetNetworkSystem() const { return networkSystem.get(); }
    WorldManager* GetWorldManager() const { return worldManager.get(); }
    CharacterController* GetCharacter() const { return character.get(); }
    Terrain* GetTerrain() const { return terrain.get(); }
};