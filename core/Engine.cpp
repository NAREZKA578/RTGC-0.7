#include "Engine.hpp"
#include "../graphics/Window.hpp"
#include "../physics/PhysicsWorld.hpp"
#include "../graphics/Renderer.hpp"
#include "../audio/AudioSystem.hpp"
#include "../network/NetworkSystem.hpp"
#include "../world/WorldManager.hpp"
#include "../game/CharacterController.hpp"
#include "../game/InputManager.hpp"
#include "../world/Terrain.hpp"
#include "../core/Logger.hpp"
#include <iostream>
#include <thread>
#include <chrono>

Engine::Engine() 
    : running(false)
    , lastTime(0.0f)
    , frameTime(0.0f)
    , frameCount(0)
    , fps(0.0f)
    , totalTime(0.0f)
    , startTime(std::chrono::steady_clock::now())
    , renderer(nullptr)
    , physicsWorld(nullptr)
    , audioSystem(nullptr)
    , networkSystem(nullptr)
    , worldManager(nullptr)
    , terrain(nullptr)
    , character(nullptr)
{
}

Engine::~Engine() {
    Shutdown();
}

void Engine::InitializeSystems() {
    // Initialize core systems
    renderer = std::make_unique<Renderer>();
    physicsWorld = std::make_unique<PhysicsWorld>();
    audioSystem = std::make_unique<AudioSystem>();
    networkSystem = std::make_unique<NetworkSystem>();
    worldManager = std::make_unique<WorldManager>();
    
    // Initialize game objects
    terrain = std::make_unique<Terrain>();
    character = std::make_unique<CharacterController>();
    
    Logger::Log("Engine systems initialized");
}

void Engine::UpdateSystems(float dt) {
    // Update core systems
    physicsWorld->Update(dt);
    renderer->Update(dt);
    audioSystem->Update(dt);
    networkSystem->Update(dt);
    worldManager->Update(dt);
    
    // Update game objects
    character->Update(dt);
    terrain->Update(dt);
}

void Engine::RenderSystems() {
    renderer->BeginFrame();
    
    // Render game objects
    renderer->Render(*terrain);
    renderer->Render(*character);
    
    renderer->EndFrame();
}

void Engine::ShutdownSystems() {
    // Shutdown systems in reverse order
    terrain.reset();
    character.reset();
    
    worldManager.reset();
    networkSystem.reset();
    audioSystem.reset();
    physicsWorld.reset();
    renderer.reset();
    
    Logger::Log("Engine systems shutdown");
}

void Engine::HandleInput(float dt) {
    inputManager.Update();
    
    // Process engine-level inputs
    if (inputManager.IsKeyPressed(GLFW_KEY_ESCAPE)) {
        running = false;
    }
    
    // Pass input to subsystems
    character->HandleInput(inputManager, dt);
}

void Engine::CalculateFPS() {
    frameCount++;
    auto currentTime = std::chrono::steady_clock::now();
    float elapsed = std::chrono::duration<float>(currentTime - startTime).count();
    
    if (elapsed >= 1.0f) {
        fps = frameCount / elapsed;
        frameCount = 0;
        startTime = currentTime;
    }
}

bool Engine::Initialize() {
    try {
        Logger::Log("Initializing RTGC Engine...");
        
        // Initialize core systems
        InitializeSystems();
        
        // Initialize renderer
        if (!renderer->Initialize()) {
            Logger::LogError("Failed to initialize renderer");
            return false;
        }
        
        // Initialize physics
        if (!physicsWorld->Initialize()) {
            Logger::LogError("Failed to initialize physics world");
            return false;
        }
        
        // Initialize audio
        if (!audioSystem->Initialize()) {
            Logger::LogError("Failed to initialize audio system");
            return false;
        }
        
        // Initialize network
        if (!networkSystem->Initialize()) {
            Logger::LogError("Failed to initialize network system");
            return false;
        }
        
        // Initialize world manager
        if (!worldManager->Initialize()) {
            Logger::LogError("Failed to initialize world manager");
            return false;
        }
        
        running = true;
        lastTime = 0.0f;
        
        Logger::Log("RTGC Engine initialized successfully!");
        return true;
    } catch (const std::exception& e) {
        Logger::LogError("Exception during engine initialization: ", e.what());
        return false;
    }
}

void Engine::Run() {
    if (!running) {
        Logger::LogError("Cannot run engine: not initialized");
        return;
    }
    
    Logger::Log("Starting RTGC Engine main loop...");
    
    const float targetFrameTime = 1.0f / 60.0f; // 60 FPS target
    
    while (running) {
        auto frameStart = std::chrono::high_resolution_clock::now();
        
        // Calculate delta time
        float currentTime = std::chrono::duration<float>(std::chrono::high_resolution_clock::now().time_since_epoch()).count();
        float dt = currentTime - lastTime;
        lastTime = currentTime;
        
        // Clamp delta time to prevent large jumps
        if (dt > 0.1f) dt = 0.1f;
        
        // Update total time
        totalTime += dt;
        
        // Handle input
        HandleInput(dt);
        
        // Update systems
        UpdateSystems(dt);
        
        // Render
        RenderSystems();
        
        // Calculate FPS
        CalculateFPS();
        
        // Frame rate limiting
        auto frameEnd = std::chrono::high_resolution_clock::now();
        float frameDuration = std::chrono::duration<float>(frameEnd - frameStart).count();
        
        if (frameDuration < targetFrameTime) {
            std::this_thread::sleep_for(std::chrono::duration<float>(targetFrameTime - frameDuration));
        }
        
        frameTime = std::chrono::duration<float>(std::chrono::high_resolution_clock::now() - frameStart).count();
    }
    
    Logger::Log("RTGC Engine main loop ended");
}

void Engine::Shutdown() {
    if (!running) return;
    
    Logger::Log("Shutting down RTGC Engine...");
    
    running = false;
    ShutdownSystems();
    
    Logger::Log("RTGC Engine shutdown complete");
}

void Engine::SetWindowSize(int width, int height) {
    if (renderer) {
        renderer->SetWindowSize(width, height);
    }
}

void Engine::SetFullscreen(bool fullscreen) {
    if (renderer) {
        renderer->SetFullscreen(fullscreen);
    }
}

void Engine::SetVSync(bool enabled) {
    if (renderer) {
        renderer->SetVSync(enabled);
    }
}

void Engine::SetMasterVolume(float volume) {
    if (audioSystem) {
        audioSystem->SetMasterVolume(volume);
    }
}

bool Engine::StartServer(int port) {
    if (networkSystem) {
        return networkSystem->StartServer(port);
    }
    return false;
}

bool Engine::ConnectToServer(const std::string& address, int port) {
    if (networkSystem) {
        return networkSystem->ConnectToServer(address, port);
    }
    return false;
}

void Engine::Disconnect() {
    if (networkSystem) {
        networkSystem->Disconnect();
    }
}

void Engine::EnableDebugMode(bool enabled) {
    if (renderer) {
        renderer->EnableDebugMode(enabled);
    }
    if (physicsWorld) {
        physicsWorld->EnableDebugMode(enabled);
    }
}

void Engine::PrintPerformanceStats() {
    Logger::Log("=== Performance Stats ===");
    Logger::Log("FPS: ", fps);
    Logger::Log("Frame Time: ", frameTime * 1000.0f, " ms");
    Logger::Log("Total Runtime: ", totalTime, " s");
    Logger::Log("=========================");
}

void Engine::SaveScreenshot(const std::string& filename) {
    if (renderer) {
        renderer->SaveScreenshot(filename);
    }
}

void Engine::Pause() {
    // TODO: Implement pause functionality
}

void Engine::Resume() {
    // TODO: Implement resume functionality
}

void Engine::Reset() {
    // TODO: Implement reset functionality
}
}