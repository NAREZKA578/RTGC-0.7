#pragma once
#include "../components/CharacterComponent.hpp"
#include "../physics/CharacterController.hpp"
#include "../game/Vehicle.hpp"
#include "../game/InputManager.hpp"
#include "../game/SpawnSystem.hpp"
#include <GLFW/glfw3.h>
#include <chrono> // Для таймера

class CharacterSystem {
    static float lastInteractTime; // Добавлен анти-флуд таймер

public:
    static void Update(float dt, entt::registry& registry, CharacterController* charCtrl, Vehicle* vehicle, SpawnSystem& spawnSystem);
};