#pragma once
#include <glm/glm.hpp>
#include <vector>
#include "../core/Logger.hpp"

class RoadNetwork {
    std::vector<std::pair<glm::vec3, glm::vec3>> roads; // pairs of start/end points
    float roadWidth = 5.0f;

public:
    RoadNetwork(float width = 5.0f);
    void Generate(const std::vector<std::pair<glm::vec3, glm::vec3>>& roads);
    void Render();
    const std::vector<std::pair<glm::vec3, glm::vec3>>& GetRoads() const { return roads; }
};