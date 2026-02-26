#include "CityGenerator.hpp"
#include <random>
#include <algorithm>

CityGenerator::CityGenerator(int size, float density) : citySize(size), buildingDensity(density) {}

void CityGenerator::Generate() {
    std::random_device rd;
    std::mt19937 gen(rd());
    std::uniform_real_distribution<> dis(-citySize/2.0f, citySize/2.0f);
    std::uniform_int_distribution<> typeDis(0, 2);

    int numBuildings = static_cast<int>(citySize * citySize * buildingDensity * 0.0001f); // Грубая оценка
    buildings.reserve(numBuildings);

    for (int i = 0; i < numBuildings; ++i) {
        Building b;
        b.position = glm::vec3(dis(gen), 0, dis(gen));
        b.size = glm::vec3(10 + (dis(gen) / citySize) * 20, 10 + (dis(gen) / citySize) * 10, 10 + (dis(gen) / citySize) * 20);
        int type = typeDis(gen);
        b.type = (type == 0) ? "house" : (type == 1) ? "shop" : "factory";
        buildings.push_back(b);
    }
    LOG(L"Город сгенерирован: " + std::to_wstring(buildings.size()) + L" зданий");
}