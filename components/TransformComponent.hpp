#pragma once
#include "../math/Vector3.hpp"
#include "../math/PhysicsUtils.hpp"

struct TransformComponent {
    Vector3 position{0,0,0};
    Quat rotation{0,0,0,1}; // x,y,z,w
    Vector3 scale{1,1,1};
};