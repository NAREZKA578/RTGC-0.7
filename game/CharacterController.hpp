#pragma once
#include "../math/Vector3.hpp"

class CharacterController {
private:
    Vector3 position;
    Vector3 velocity;
    float speed = 5.0f;
    
public:
    CharacterController();
    void Initialize();
    void Update(float dt);
    void Move(const Vector3& direction, float dt);
    void Shutdown();
    Vector3 GetPosition() const;
    void SetPosition(const Vector3& pos);
};
