#pragma once
#include "../components/TransformComponent.hpp"
#include "../components/VehicleComponent.hpp"
#include "PlayerState.hpp"
#include <array>
#include <entt/entt.hpp>

class NetworkSyncSystem {
public:
    static void ReceiveAndApply(entt::registry& registry, const std::array<PlayerState, 8>& states);
};