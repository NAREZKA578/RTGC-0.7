#pragma once
#include <glm/glm.hpp>
#include <string>

struct BuildingComponent {
    glm::vec3 position{0,0,0};
    std::string type; // "house", "container", etc.
    uint32_t owner = 0;
};