#pragma once
#include "VehicleType.hpp"
#include <PhysX/PxPhysicsAPI.h>
#include <PhysX/vehicle/PxVehicleDrive4W.h>
#include <PhysX/vehicle/PxVehicleDriveTank.h>
#include <PhysX/vehicle/PxVehicleUtil.h>
#include <array>
#include "../core/Logger.hpp"
#include "../graphics/ModelLoader.hpp" // Для загрузки меша
#include "../math/Mass.hpp" // Для массы
#include "../math/Vector3.hpp" // Для векторов
using namespace physx;

struct WheelState {
    float compression = 0, load = 0, slip = 0, lateralForce = 0;
    bool contact = false;
};

class Vehicle {
public:
    VehicleType type;
    PxRigidDynamic* mActor = nullptr;
    void* mVehicle = nullptr;
    float throttle = 0.0f;
    float brake = 0.0f;
    float steer = 0.0f;
    std::array<WheelState, 4> wheelStates = {};
    bool engineOn = false;
    float rotorRPM = 0.0f;

    // Добавлен указатель на RenderableVehicle
    class RenderableVehicle* renderableVehicle = nullptr;

    Vehicle(PxPhysics* physics, PxMaterial* material, const VehicleType& vt, const PxVec3& pos, uint32_t playerId = 0); // Добавлен playerId
    void Update(float dt);
    ~Vehicle(); // Добавлен деструктор

private:
    void CreateWheeledVehicle(PxPhysics* physics);
    void CreateTrackedVehicle(PxPhysics* physics);
    void CreateHelicopter(PxPhysics* physics);
};