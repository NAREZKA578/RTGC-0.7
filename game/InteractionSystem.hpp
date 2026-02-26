#pragma once
#include "BuildingSystem.hpp"
#include "CharacterComponent.hpp"
#include "Inventory.hpp" // Добавлено для ItemType
#include "../physics/CharacterController.hpp" // Добавлено для PxVec3
#include "../components/VehicleComponent.hpp" // Для получения Vehicle из entt::entity

class InteractionSystem {
public:
    static void Update(entt::registry& registry, BuildingSystem& buildings);
};