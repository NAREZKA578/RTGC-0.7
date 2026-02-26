#include "AIController.hpp"
#include <glm/gtc/quaternion.hpp> // Для glm::length

AIController::AIController(CharacterController* ctrl) : character(ctrl) {}

void AIController::Update(float dt) {
    if (!character) return;
    auto pos = character->GetPosition();
    glm::vec3 dir(target.x - pos.x, 0, target.z - pos.z);
    if (glm::length(dir) > 2.0f) { // ИСПРАВЛЕНО: glm::length
        character->Move(PxVec3(dir.x, 0, dir.z), 2.0f);
    }
}