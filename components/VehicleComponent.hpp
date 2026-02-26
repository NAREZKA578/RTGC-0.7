#pragma once
#include "../game/VehicleType.hpp"
#include "../game/Vehicle.hpp"

struct VehicleComponent {
    Vehicle* vehicle = nullptr;
    bool isLocal = false;
    uint32_t playerId = 0; // Для сетевой синхронизации

    VehicleComponent() = default;
    VehicleComponent(Vehicle* v, bool local = false, uint32_t pid = 0)
        : vehicle(v), isLocal(local), playerId(pid) {}

    void Update(float dt) {
        if (vehicle) {
            vehicle->Update(dt);
        }
    }
};