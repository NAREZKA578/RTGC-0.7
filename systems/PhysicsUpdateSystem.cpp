#include "PhysicsUpdateSystem.hpp"
#include "../physics/PhysXInitializer.hpp"
#include "../game/Vehicle.hpp" // Для доступа к VehicleComponent

std::mutex PhysicsUpdateSystem::mMutex;
std::mutex PhysicsUpdateSystem::ecsMutex;

void PhysicsUpdateSystem::Update(float dt, entt::registry& registry) {
    auto vehicles = registry.view<VehicleComponent>();
    for (auto entity : vehicles) {
        auto& vcomp = vehicles.get<VehicleComponent>(entity);
        if (vcomp.vehicle) {
            vcomp.vehicle->Update(dt);
        }
    }
    std::lock_guard<std::mutex> lock(mMutex);
    PhysXInitializer::gScene->simulate(dt);
    while (!PhysXInitializer::gScene->fetchResults()) {
        std::this_thread::sleep_for(std::chrono::microseconds(100));
    }
}