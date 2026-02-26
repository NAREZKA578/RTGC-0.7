#pragma once
#include "../game/Inventory.hpp"
#include "../physics/CharacterController.hpp"

struct CharacterComponent {
    float health = 100.0f;
    bool isAlive = true;
    float respawnTimer = 0.0f;
    bool shouldRespawn = false;
    bool inVehicle = false;
    entt::entity currentVehicle = entt::null;
    CharacterController* characterController = nullptr; // Указатель на физический контроллер
    Inventory inventory;
};