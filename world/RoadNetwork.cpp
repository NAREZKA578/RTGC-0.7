#include "RoadNetwork.hpp"

RoadNetwork::RoadNetwork(float width) : roadWidth(width) {}

void RoadNetwork::Generate(const std::vector<std::pair<glm::vec3, glm::vec3>>& inputRoads) {
    roads = inputRoads; // Просто сохраняем переданные дороги
    LOG(L"Сеть дорог сгенерирована: " + std::to_wstring(roads.size()) + L" сегментов");
}

void RoadNetwork::Render() {
    // Логика рендеринга дорог (например, отрисовка линий или геометрии)
    // for (const auto& [start, end] : roads) {
    //     // Отрисовка дороги между start и end
    // }
}