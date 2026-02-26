#pragma once
#include "../components/VehicleComponent.hpp"
#include "../components/CharacterComponent.hpp"
#include "../components/RenderableComponent.hpp"
#include "Vehicle.hpp"
#include "VehicleFactory.hpp"
#include "BuildingSystem.hpp"
#include "Terrain.hpp"
#include "RoadNetwork.hpp"
#include "CityGenerator.hpp"
#include "WeatherSystem.hpp"
#include "SpawnSystem.hpp"
#include "../core/ECSManager.hpp"
#include "../physics/CharacterController.hpp"

class GameLevel {
public:
    ECSManager ecs;
    Vehicle* vehicle = nullptr;
    CharacterController* characterController = nullptr;
    BuildingSystem buildings;
    Terrain terrain;
    WeatherSystem weather;
    SpawnSystem spawnSystem;
    entt::entity player;
    entt::entity playerCharacter;

    GameLevel();
    ~GameLevel();

    void Update(float dt);
    void Render();
};