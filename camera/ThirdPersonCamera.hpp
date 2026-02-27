#pragma once
#include "../math/Vector3.hpp"
#include <glm/glm.hpp>
#include <glm/gtc/quaternion.hpp>

class ThirdPersonCamera {
private:
    Vector3 m_position;
    Vector3 m_target;
    float m_distance;
    float m_pitch;
    float m_yaw;
    float m_speed;
    float m_sensitivity;

public:
    ThirdPersonCamera();
    void Update(float dt);
    void ProcessInput(float dt);
    void Follow(const glm::vec3& targetPos, const glm::quat& targetRot, float dt);
    
    Vector3 GetPosition() const { return m_position; }
    Vector3 GetTarget() const { return m_target; }
};