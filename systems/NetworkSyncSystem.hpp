#pragma once
#include "../components/TransformComponent.hpp"
#include "../components/VehicleComponent.hpp"
#include "../network/PlayerState.hpp"
#include <array>

class NetworkSyncSystem {
public:
    // ИСПРАВЛЕНО: Теперь синхронизирует всех игроков, а не только ID=1
    static void ReceiveAndApply(entt::registry& registry, const std::array<PlayerState, 8>& states);
};