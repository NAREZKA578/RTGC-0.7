#pragma once
#include <vector>
#include <glm/glm.hpp>
#include "../core/Logger.hpp"

struct Building {
    glm::vec3 position;
    glm::vec3 size;
    std::string type; // "house", "shop", etc.
};

class CityGenerator {
    std::vector<Building> buildings;
    int citySize = 1000; // in meters
    float buildingDensity = 0.3f; // 0-1 ratio

public:
    CityGenerator(int size = 1000, float density = 0.3f);
    void Generate();
    const std::vector<Building>& GetBuildings() const { return buildings; }
};