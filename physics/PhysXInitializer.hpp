#pragma once
#include <PhysX/PxPhysicsAPI.h>
#include <characterkinematic/PxController.h>
#include <extensions/PxExtensions.h>
#include <vehicle/PxVehicleUtil.h>
#include <vehicle/PxVehicleAPI.h>
#include <extensions/PxDefaultSimulationFilterShader.h>
#include "../core/Logger.hpp"
using namespace physx;

class PhysXInitializer {
public:
    static PxFoundation* gFoundation;
    static PxPhysics* gPhysics;
    static PxDefaultCpuDispatcher* gCpuDispatcher;
    static PxScene* gScene;
    static PxMaterial* gMaterial;
    static PxPvd* gPvd;
    static bool Init();
    static void Shutdown();
private:
    static PxDefaultAllocator gAllocator;
    static PxDefaultErrorCallback gErrorCallback;
};