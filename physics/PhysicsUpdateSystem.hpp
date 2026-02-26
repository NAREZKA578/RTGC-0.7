#pragma once
#include <entt/entt.hpp>
#include <thread>
#include <mutex>

class PhysicsUpdateSystem {
    static std::mutex mMutex;
    static std::mutex ecsMutex;
public:
    static void Update(float dt, entt::registry& registry);
};