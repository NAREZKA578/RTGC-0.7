#pragma once
#include <entt/entt.hpp>
#include "Logger.hpp"

class ECSManager {
public:
    entt::registry registry;

    ECSManager();
    ~ECSManager();

    void Update(float dt);
};