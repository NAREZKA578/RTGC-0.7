#include "CharacterSystem.hpp"
#include "../components/CharacterComponent.hpp"
#include "../physics/CharacterController.hpp"
#include "../game/Vehicle.hpp"
#include "../game/InputManager.hpp"
#include "../game/SpawnSystem.hpp"
#include <GLFW/glfw3.h>
#include <thread> // Для sleep_for
#include <chrono> // Для sleep_for

float CharacterSystem::lastInteractTime = 0.0f;

void CharacterSystem::Update(float dt, entt::registry& registry, CharacterController* charCtrl, Vehicle* vehicle, SpawnSystem& spawnSystem) {
    auto view = registry.view<CharacterComponent>();
    for (auto entity : view) {
        auto& charComp = view.get<CharacterComponent>(entity);

        if (InputManager::IsDown(GLFW_KEY_F)) {
            // ИСПРАВЛЕНО: Убрана функция Sleep, добавлен анти-флуд
            if (glfwGetTime() - lastInteractTime > 0.3) { // 300ms cooldown
                if (charComp.inVehicle) {
                    charComp.inVehicle = false;
                    charComp.currentVehicle = entt::null;
                    LOG(L"Вышли из транспорта");
                } else if (DistanceToVehicle(charCtrl, vehicle) < 5.0f) {
                    charComp.inVehicle = true;
                    // ИСПРАВЛЕНО: Ищем entity с VehicleComponent
                    auto vehicle_view = registry.view<VehicleComponent>();
                    if (!vehicle_view.empty()) {
                        charComp.currentVehicle = vehicle_view.front();
                    }
                    LOG(L"Вошли в транспорт");
                }
                lastInteractTime = static_cast<float>(glfwGetTime());
            }
        }

        if (!charComp.inVehicle && charComp.isAlive) {
            HandleMovement(dt, charCtrl);
            HandleDamage(charComp);
        }

        if (!charComp.isAlive) {
            charComp.respawnTimer += dt;
            if (charComp.respawnTimer >= 5.0f) {
                charComp.health = 100.0f;
                charComp.isAlive = true;
                charComp.shouldRespawn = true;
                charComp.respawnTimer = 0.0f;
                LOG(L"Персонаж возродился");
            }
        }
    }
}

// Вспомогательные функции теперь определены как статические внутри cpp файла
static float DistanceToVehicle(CharacterController* ctrl, Vehicle* vehicle) {
    PxVec3 p1 = ctrl->GetPosition();
    PxVec3 p2 = vehicle->mActor->getGlobalPose().p;
    return p1.distance(p2);
}

static void HandleMovement(float dt, CharacterController* ctrl) {
    PxVec3 moveDir(0, 0, 0);
    if (InputManager::IsDown(GLFW_KEY_W)) moveDir.z -= 1;
    if (InputManager::IsDown(GLFW_KEY_S)) moveDir.z += 1;
    if (InputManager::IsDown(GLFW_KEY_A)) moveDir.x -= 1;
    if (InputManager::IsDown(GLFW_KEY_D)) moveDir.x += 1;

    float speed = InputManager::IsDown(GLFW_KEY_LEFT_SHIFT) ? 6.0f : 3.0f;
    if (moveDir.magnitudeSquared() > 0.1f) {
        moveDir.normalize();
    }
    ctrl->Move(moveDir * speed, dt); // ИСПРАВЛЕНО: передаём dt отдельно
}

static void HandleDamage(CharacterComponent& charComp) {
    if (charComp.health <= 0) {
        charComp.isAlive = false;
        LOG(L"Персонаж погиб");
    }
}