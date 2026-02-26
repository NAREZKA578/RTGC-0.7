#include "NetworkSyncSystem.hpp"
#include "../components/VehicleComponent.hpp" // Для доступа к VehicleComponent

void NetworkSyncSystem::ReceiveAndApply(entt::registry& registry, const std::array<PlayerState, 8>& states) {
    auto view = registry.view<VehicleComponent>();
    for (const auto& state : states) {
        if (!state.isValid) continue;

        for (auto entity : view) {
            auto& vcomp = view.get<VehicleComponent>(entity);
            // ИСПРАВЛЕНО: Сравниваем ID игрока с ID в компоненте транспорта
            if (vcomp.vehicle && vcomp.playerId == state.playerId) {
                auto pos = physx::PxVec3(state.position.x, state.position.y, state.position.z);
                vcomp.vehicle->mActor->setGlobalPose(physx::PxTransform(pos));
            }
        }
    }
}