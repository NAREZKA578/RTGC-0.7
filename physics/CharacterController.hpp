#pragma once
#include "../core/Logger.hpp"
#include "../math/Vector3.hpp"

using namespace std;

class CharacterController {
private:
    Vector3 position;
    Vector3 velocity;
    float height = 1.8f;
    float radius = 0.4f;
    bool onGround = true;

public:
    CharacterController();
    ~CharacterController();

    void Initialize();
    void Update(float dt);
    void Shutdown();

    void Move(const Vector3& direction, float dt);
    void Jump();
    Vector3 GetPosition() const;
    void SetPosition(const Vector3& pos);

    void* GetPxController() const { return nullptr; } // Заглушка для PhysX
    bool IsOnGround() const { return onGround; }
};