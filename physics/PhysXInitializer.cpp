#include "PhysXInitializer.hpp"

PxFoundation* PhysXInitializer::gFoundation = nullptr;
PxPhysics* PhysXInitializer::gPhysics = nullptr;
PxDefaultCpuDispatcher* PhysXInitializer::gCpuDispatcher = nullptr;
PxScene* PhysXInitializer::gScene = nullptr;
PxMaterial* PhysXInitializer::gMaterial = nullptr;
PxPvd* PhysXInitializer::gPvd = nullptr;
PxDefaultAllocator PhysXInitializer::gAllocator;
PxDefaultErrorCallback PhysXInitializer::gErrorCallback;

bool PhysXInitializer::Init() {
    gFoundation = PxCreateFoundation(PX_PHYSICS_VERSION, gAllocator, gErrorCallback);
    if (!gFoundation) {
        ERROR(L"Не удалось создать PxFoundation");
        return false;
    }
    gPvd = PxCreatePvd(*gFoundation);
    PxPvdTransport* transport = PxDefaultPvdSocketTransportCreate("127.0.0.1", 5425, 10);
    gPvd->connect(*transport, PxPvdInstrumentationFlag::eALL);
    gPhysics = PxCreatePhysics(PX_PHYSICS_VERSION, *gFoundation, PxTolerancesScale(), true, gPvd);
    if (!gPhysics) {
        ERROR(L"Не удалось создать PxPhysics");
        return false;
    }
    gMaterial = gPhysics->createMaterial(0.5f, 0.5f, 0.6f);
    if (!gMaterial) {
        ERROR(L"Не удалось создать материал");
        return false;
    }
    if (!PxInitExtensions(*gPhysics, gPvd)) {
        ERROR(L"PxInitExtensions failed");
        return false;
    }
    gCpuDispatcher = PxDefaultCpuDispatcherCreate(4);
    if (!gCpuDispatcher) {
        ERROR(L"Не удалось создать CpuDispatcher");
        return false;
    }
    PxSceneDesc sceneDesc(gPhysics->getTolerancesScale());
    sceneDesc.gravity = PxVec3(0.0f, -9.81f, 0.0f);
    sceneDesc.cpuDispatcher = gCpuDispatcher;
    sceneDesc.filterShader = PxDefaultSimulationFilterShader;
    sceneDesc.broadPhaseType = PxBroadPhaseType::eMULTI_BOX;
    sceneDesc.flags |= PxSceneFlag::eENABLE_CCD;
    sceneDesc.solverBatchSize = 32;
    sceneDesc.nbContactDataBlocks = 256;
    sceneDesc.maxNbContactDataBlocksPerFrame = 512;
    gScene = gPhysics->createScene(sceneDesc);
    if (!gScene) {
        ERROR(L"Не удалось создать сцену");
        return false;
    }
    PxInitVehicleSDK(*gPhysics);
    PxVehicleSetUpdateMode(PxVehicleUpdateMode::eVELOCITY_CHANGE);
    LOG(L"PhysX успешно инициализирован");
    return true;
}

void PhysXInitializer::Shutdown() {
    if (gScene) {
        gScene->release();
        gScene = nullptr;
    }
    if (gCpuDispatcher) {
        gCpuDispatcher->release();
        gCpuDispatcher = nullptr;
    }
    if (gMaterial) {
        gMaterial->release();
        gMaterial = nullptr;
    }
    if (gPhysics) {
        PxCloseVehicleSDK();
        gPhysics->release();
        gPhysics = nullptr;
    }
    if (gPvd) {
        gPvd->release();
        gPvd = nullptr;
    }
    if (gFoundation) {
        gFoundation->release();
        gFoundation = nullptr;
    }
}