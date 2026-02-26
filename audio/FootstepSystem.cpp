#include "FootstepSystem.hpp"
#include "../core/Logger.hpp"

FootstepSystem::FootstepSystem(AudioSystem* sys) : audioSystem(sys) {
    LOG(L"Система аудио-шагов инициализирована");
}

void FootstepSystem::Update(float dt, const glm::vec3& playerPosition) {
    float distance = glm::distance(playerPosition, lastPosition);
    if (distance > stepDistance) {
        audioSystem->PlayAt("footstep", playerPosition, false);
        lastPosition = playerPosition;
    }
}