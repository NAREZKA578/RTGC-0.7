#include "GameLevel.hpp"
#include "../physics/PhysXInitializer.hpp"

GameLevel::GameLevel() : terrain(PhysXInitializer::gPhysics, PhysXInitializer::gMaterial) {
    auto vehicleType = VehicleFactory::CreateKamaz();
    // ИСПРАВЛЕНО: передаём playerId при создании транспорта
    vehicle = new Vehicle(PhysXInitializer::gPhysics, PhysXInitializer::gMaterial, vehicleType, PxVec3(0, 10, 0), 1);
    characterController = new CharacterController(PhysXInitializer::gScene, PhysXInitializer::gPhysics, PhysXInitializer::gMaterial);
    player = ecs.registry.create();
    // ИСПРАВЛЕНО: устанавливаем playerId в компоненте
    ecs.registry.emplace<VehicleComponent>(player, VehicleComponent{vehicle, true, 1});
    playerCharacter = ecs.registry.create();
    ecs.registry.emplace<CharacterComponent>(playerCharacter, CharacterComponent{});
    LOG(L"Уровень загружен");
}

void GameLevel::Update(float dt) {
    // Обновление объектов уровня
}

void GameLevel::Render() {
    // Рендеринг объектов уровня
}

GameLevel::~GameLevel() {
    delete vehicle;
    delete characterController;
}