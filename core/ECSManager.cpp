#include "ECSManager.hpp"

ECSManager::ECSManager() {
    LOG(L"ECSManager инициализирован");
}

ECSManager::~ECSManager() {
    LOG(L"ECSManager уничтожен");
}

void ECSManager::Update(float dt) {
    // Логика обновления ECS, если требуется
    // registry.update(dt); // Пример, если entt предоставляет такой метод
}