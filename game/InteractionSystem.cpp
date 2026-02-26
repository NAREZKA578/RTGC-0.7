#include "InteractionSystem.hpp"
#include "BuildingSystem.hpp"
#include "CharacterComponent.hpp"
#include "Inventory.hpp" // Добавлено для ItemType
#include "../physics/CharacterController.hpp" // Добавлено для PxVec3
#include "../game/Inventory.hpp"             // Добавлено для ItemType
#include "../components/VehicleComponent.hpp" // Для получения Vehicle из entt::entity

void InteractionSystem::Update(entt::registry& registry, BuildingSystem& buildings) {
    auto view = registry.view<CharacterComponent>();
    if (view.empty()) return;
    auto& charComp = view.get<CharacterComponent>(view.front());

    glm::vec3 pos;
    if (charComp.inVehicle && charComp.currentVehicle != entt::null) {
        auto& vcomp = registry.get<VehicleComponent>(charComp.currentVehicle);
        if (vcomp.vehicle && vcomp.vehicle->mActor) {
            auto p = vcomp.vehicle->mActor->getGlobalPose().p;
            pos = {p.x, p.y, p.z};
        }
    } else if (charComp.characterController) {
        auto p = charComp.characterController->GetPosition();
        pos = {p.x, p.y, p.z};
    }

    for (auto& b : buildings.GetObjects()) {
        float dist = glm::distance(pos, b.position);
        if (dist < 3.0f && InputManager::IsDown(GLFW_KEY_E)) {
            if (b.type == "container") {
                charComp.inventory.Add(ItemType::Medkit, 1);
                LOG(L"Взято: аптечка");
            }
        }
    }
}