#include "EngineMinimal.hpp"
#include <thread>
#include <chrono>
#include <string>
#include "physics/PhysXInitializer.hpp"

Engine::Engine()
    : m_running(false)
    , m_lastTime(0.0f)
    , m_frameTime(0.0f)
    , m_frameCount(0)
    , m_fps(0.0f)
    , m_totalTime(0.0f)
    , m_loadingProgress(0.0f)
    , m_loadingStep(0)
    , m_totalLoadingSteps(5)
    , m_state(State::LOADING)
    , m_terrain(nullptr)
    , m_character(nullptr)
    , m_vehicle(nullptr)
    , m_camera(nullptr)
    , m_cities(std::make_unique<SiberianCities>())
    , m_renderer(nullptr)
    , m_menuSelectedSlot(0)
    , m_selectedCityIndex(0)
{
}

Engine::~Engine() {
    Shutdown();
}

bool Engine::Initialize() {
    Logger::Log("=== RTGC Engine Initializing ===");
    try {
        // Initialize PhysX first
        if (!PhysXInitializer::Init()) {
            Logger::Error("Failed to initialize PhysX");
            return false;
        }
        
        InitializeSystems();
        m_startTime = std::chrono::steady_clock::now();
        m_lastTime = 0.0f;
        m_running = true;
        Logger::Log("Engine initialized successfully");
        return true;
    } catch (const std::exception& e) {
        Logger::Error("Engine initialization failed: ", e.what());
        return false;
    }
}

void Engine::InitializeSystems() {
    Logger::Log("Initializing systems...");
    m_loadingStep = 0;
    m_loadingProgress = 0.0f;

    // Input system
    m_input.Init();
    UpdateLoadingProgress(0.1f);

    // Cities/world data
    m_cities->Load();
    UpdateLoadingProgress(0.2f);

    // Terrain
    m_terrain = std::make_unique<Terrain>();
    m_terrain->Initialize();
    UpdateLoadingProgress(0.3f);

    // Character
    m_character = std::make_unique<CharacterController>();
    m_character->Initialize();
    UpdateLoadingProgress(0.4f);

    // Camera
    m_camera = std::make_unique<ThirdPersonCamera>();
    UpdateLoadingProgress(0.5f);

    // Vehicle
    if (PhysXInitializer::gPhysics && PhysXInitializer::gMaterial) {
        // Create a basic vehicle
        VehicleType truckType;
        truckType.name = "Mud Truck";
        truckType.type = VehicleDriveType::Wheeled4WD;
        truckType.mass = 2000.0f;
        truckType.maxSpeed = 30.0f;
        truckType.engineTorque = 5000.0f;
        truckType.engineRPM = 3000.0f;
        truckType.modelFile = "assets/models/truck.obj";
        
        // Add some wheels
        Wheel wheel;
        wheel.position = glm::vec3(-1.5f, -0.5f, 2.0f);
        wheel.radius = 0.4f;
        wheel.width = 0.2f;
        wheel.maxBrakeTorque = 1000.0f;
        truckType.wheels.push_back(wheel);
        
        wheel.position = glm::vec3(1.5f, -0.5f, 2.0f);
        truckType.wheels.push_back(wheel);
        
        wheel.position = glm::vec3(-1.5f, -0.5f, -2.0f);
        truckType.wheels.push_back(wheel);
        
        wheel.position = glm::vec3(1.5f, -0.5f, -2.0f);
        truckType.wheels.push_back(wheel);

        m_vehicle = std::make_unique<Vehicle>(PhysXInitializer::gPhysics, PhysXInitializer::gMaterial, truckType, PxVec3(0, 5, 0));
        UpdateLoadingProgress(0.7f);
    }

    // Renderer
    m_renderer = std::make_unique<Renderer>();
    if (!m_renderer->Initialize()) {
        Logger::Error("Failed to initialize renderer");
    }
    UpdateLoadingProgress(1.0f);

    Logger::Log("All systems initialized");
}

void Engine::UpdateLoadingProgress(float progress) {
    m_loadingProgress = progress;
    m_loadingStep++;
    Logger::Log("Loading progress: ", (int)(m_loadingProgress * 100), "%");
    if (m_loadingProgress >= 1.0f) {
        CompleteLoading();
    }
}

void Engine::CompleteLoading() {
    m_loadingProgress = 1.0f;
    m_state = State::MENU;
    Logger::Log("Loading complete, transitioning to menu");
}

void Engine::EnterWorldCreationFromSlot(int slotIndex) {
    // Lazy create slot if not exist yet
    auto slot = m_worldSlots.GetSlot(slotIndex);
    if (!slot.used) {
        std::string name = "World Slot " + std::to_string(slotIndex + 1);
        m_worldSlots.CreateSlot(slotIndex, name);
        slot.worldName = name;
        slot.used = true;
    }
    m_worldSettings = WorldConfig::WorldSettings(slot.worldName, slot.mapSize, slot.terrainType, slot.climateType, slot.season, slot.cityDensity, slot.waterLevel, slot.mountainHeight, slot.generateRoads, slot.generateVegetation, slot.seed, slot.scale);
    // Fallback: if constructor above doesn't exist, just set basics
    m_worldSettings.worldName = slot.worldName;
    m_worldSettings.scale = slot.scale;
    m_worldSettings.seed = slot.seed;
    m_state = State::WORLD_CREATION;
    Logger::Log("World creation started for slot ", slotIndex + 1);
}

void Engine::EnterWorldCreation() {
    m_worldSettings = WorldConfig::WorldSettings();
    m_worldSettings.worldName = "Мой Мир";
    Logger::Log("Entered world creation menu");
}

void Engine::CreateWorld() {
    Logger::Log("Creating new world...");
    Logger::Log("World name: ", m_worldSettings.worldName);
    m_worldGenerator = std::make_unique<WorldConfig::WorldGenerator>(m_worldSettings);
    m_worldGenerator->Generate();
    if (!m_worldGenerator->GetCities().empty()) {
        Vector3 firstCity = m_worldGenerator->GetCities()[0];
        if (m_character) m_character->SetPosition(firstCity);
        Logger::Log("Character positioned at first city");
    }
    Logger::Log("World creation complete");
}

void Engine::BackToMenuFromWorldCreation() {
    m_worldGenerator.reset();
    m_state = State::MENU;
    Logger::Log("Returning from world creation to menu");
}

void Engine::Run() {
    Logger::Log("=== Starting Game Loop ===");
    int frameCount = 0;
    while (m_running) {
        try {
            auto currentTime = std::chrono::steady_clock::now();
            auto elapsed = std::chrono::duration<float>(currentTime - m_startTime).count();

            m_frameTime = elapsed - m_lastTime;
            m_lastTime = elapsed;
            m_totalTime += m_frameTime;

            if (m_frameTime > 0.1f) m_frameTime = 0.1f;
            if (m_frameTime < 0.016f) m_frameTime = 0.016f;

            m_frameCount++;
            if (m_frameCount >= 60) {
                m_fps = 60.0f / m_totalTime;
                m_frameCount = 0;
                m_totalTime = 0.0f;
            }

            if (m_renderer && m_renderer->GetWindow()) {
                if (m_renderer->GetWindow()->ShouldClose()) {
                    Logger::Log("Window close requested");
                    m_running = false;
                    break;
                }
                m_renderer->GetWindow()->PollEvents();
            }

            // Input
            // We will handle input in its own function below
            // but keep structure for compatibility
            // HandleInput(m_frameTime);
            // We'll process inputs in the next step

            // Update systems
            UpdateSystems(m_frameTime);

            if (m_renderer) {
                m_renderer->BeginFrame();
                m_renderer->Clear();
                switch (m_state) {
                case State::LOADING:
                    m_renderer->RenderLoading(m_loadingProgress);
                    break;
                case State::MENU:\n                    m_renderer->RenderMenuSlots(m_worldSlots, m_menuSelectedSlot);
                    break;
                case State::WORLD_CREATION:
                    m_renderer->RenderWorldCreation(m_worldSettings);
                    break;
                case State::CITY_SELECTION:
                    m_renderer->RenderCitySelection(0);
                    break;
                case State::GAME:
                    m_renderer->RenderGame(m_character ? m_character->GetPosition() : Vector3());
                    break;
                case State::ERROR_STATE:
                    m_renderer->RenderError();
                    break;
                default:
                    break;
                }
                m_renderer->EndFrame();
            }

            // Simple input poll
            if (m_renderer && m_renderer->GetWindow()) {
                m_renderer->GetWindow()->PollEvents();
            }

            std::this_thread::sleep_for(std::chrono::milliseconds(16));
            frameCount++;
        } catch (const std::exception& e) {
            Logger::Error("Exception in game loop: ", e.what());
            m_running = false;
        } catch (...) {
            Logger::Error("Unknown exception in game loop");
            m_running = false;
        }
    }
    Logger::Log("=== Game Loop Ended ===");
}

void Engine::HandleInput(float dt) {
    if (!m_renderer || !m_renderer->GetWindow()) return;
    Window* window = m_renderer->GetWindow();
    if (m_state == State::MENU) {
        // 1-5 slots
        for (int i = 0; i < 5; ++i) {
            if (window->IsKeyPressed('1' + i)) {
                EnterWorldCreationFromSlot(i);
                break;
            }
        }
    } else if (m_state == State::WORLD_CREATION) {
        // No detailed handling yet
        if (window->IsKeyPressed(VK_ESCAPE)) {
            BackToMenuFromWorldCreation();
        }
        if (window->IsKeyPressed(VK_RETURN)) {
            CreateWorld();
            m_state = State::GAME;
        }
    } else if (m_state == State::GAME) {
        // Basic movement handled in RenderGame path
    }

    window->UpdateInput();
}

void Engine::UpdateSystems(float dt) {
    if (m_character) m_character->Update(dt);
    if (m_terrain) m_terrain->Update(dt);
    if (m_vehicle) m_vehicle->Update(dt);
    
    // Update PhysX scene
    if (PhysXInitializer::gScene) {
        PhysXInitializer::gScene->simulate(dt);
        PhysXInitializer::gScene->fetchResults(true);
    }
    
    // Update camera to follow vehicle
    if (m_vehicle && m_camera && m_vehicle->mActor) {
        PxTransform transform = m_vehicle->mActor->getGlobalPose();
        glm::vec3 vehiclePos(transform.p.x, transform.p.y, transform.p.z);
        glm::quat vehicleRot(transform.q.w, transform.q.x, transform.q.y, transform.q.z);
        m_camera->Follow(vehiclePos, vehicleRot, dt);
    }
}

void Engine::Shutdown() {
    if (!m_running) return;
    m_running = false;
    ShutdownSystems();
    Logger::Log("Engine shutdown complete");
}

void Engine::ShutdownSystems() {
    Logger::Log("Shutting down systems...");
    if (m_renderer) {
        m_renderer->Shutdown();
        m_renderer.reset();
    }
    if (m_character) {
        m_character->Shutdown();
        m_character.reset();
    }
    if (m_terrain) {
        m_terrain.reset();
    }
    Logger::Log("All systems shutdown");
}
