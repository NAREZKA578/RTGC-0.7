#include "ThirdPersonCamera.hpp"
#include <glm/gtx/quaternion.hpp>

ThirdPersonCamera::ThirdPersonCamera() 
    : m_position(0, 5, 10), m_target(0, 0, 0), m_distance(10.0f),
      m_pitch(-15.0f), m_yaw(0.0f), m_speed(2.0f), m_sensitivity(0.1f) {
}

void ThirdPersonCamera::Update(float dt) {
    // Update camera position based on target and offset
    glm::vec3 offset(
        m_distance * cos(glm::radians(m_pitch)) * cos(glm::radians(m_yaw)),
        m_distance * sin(glm::radians(m_pitch)),
        m_distance * cos(glm::radians(m_pitch)) * sin(glm::radians(m_yaw))
    );
    
    glm::vec3 targetVec(m_target.x, m_target.y, m_target.z);
    glm::vec3 newPos = targetVec - offset;
    
    m_position = Vector3(newPos.x, newPos.y, newPos.z);
}

void ThirdPersonCamera::ProcessInput(float dt) {
    // For now, just placeholder - input processing will happen elsewhere
}

void ThirdPersonCamera::Follow(const glm::vec3& targetPos, const glm::quat& targetRot, float dt) {
    // Update the target position
    m_target = Vector3(targetPos.x, targetPos.y, targetPos.z);
    
    // Extract yaw from quaternion (simplified approach)
    glm::vec3 forward = glm::rotate(targetRot, glm::vec3(1, 0, 0));
    m_yaw = atan2(forward.z, forward.x) * 180.0f / M_PI;
    
    // Apply smoothing to pitch/yaw if desired
    Update(dt);
}