#pragma once
#include "../math/Vector3.hpp"
#include "../physics/PhysicsWorld.hpp"
#include <vector>
#include <string>

struct Wheel {
    Vector3 position;
    float radius;
    float width;
    float maxBrakeTorque;
    float angularVelocity;
    float steerAngle;
    bool grounded;
    
    Wheel() : radius(0.4f), width(0.2f), maxBrakeTorque(1000.0f), 
              angularVelocity(0.0f), steerAngle(0.0f), grounded(false) {}
};

enum class VehicleDriveType {
    Wheeled2WD,
    Wheeled4WD,
    WheeledFront,
    WheeledRear
};

struct VehicleType {
    std::string name;
    VehicleDriveType type;
    float mass;
    float maxSpeed;
    float engineTorque;
    float engineRPM;
    std::string modelFile;
    std::vector<Wheel> wheels;
};

class Vehicle {
public:
    std::unique_ptr<PhysicsBody> mActor;
    VehicleType mType;
    std::vector<Wheel> mWheels;
    float mThrottle;
    float mSteering;
    float mBrake;
    
    Vehicle(const VehicleType& type, const Vector3& startPos);
    void Update(float dt);
    void SetThrottle(float t);
    void SetSteering(float s);
    void SetBrake(float b);
    
private:
    void UpdateWheels(float dt);
    void ApplyEngineForces(float dt);
    void ApplySteering(float dt);
};