#include "Vehicle.hpp"
#include <algorithm> // для std::min/std::max

Vehicle::Vehicle(PxPhysics* physics, PxMaterial* material, const VehicleType& vt, const PxVec3& pos, uint32_t playerId) : type(vt) {
    mActor = physics->createRigidDynamic(PxTransform(pos));
    mActor->setMass(type.mass);

    // --- НОВОЕ: Загрузка геометрии из файла ---
    std::vector<Mesh*> loadedMeshes = ModelLoader::LoadOBJ(type.modelFile);
    if (!loadedMeshes.empty()) {
        renderableVehicle = new RenderableVehicle();
        renderableVehicle->SetMeshes(loadedMeshes);

        // Создаём физическую геометрию из меша (упрощённо - используем bounding box)
        PxBoxGeometry geom(3.0f, 1.5f, 4.0f); // Заглушка
        if (loadedMeshes.size() > 0 && loadedMeshes[0]->vertices.size() > 0) {
            // Пример грубой оценки размера из первого меша
            float minX = loadedMeshes[0]->vertices[0].x, maxX = minX;
            float minY = loadedMeshes[0]->vertices[0].y, maxY = minY;
            float minZ = loadedMeshes[0]->vertices[0].z, maxZ = minZ;
            for (const auto& v : loadedMeshes[0]->vertices) {
                minX = std::min(minX, v.x); maxX = std::max(maxX, v.x);
                minY = std::min(minY, v.y); maxY = std::max(maxY, v.y);
                minZ = std::min(minZ, v.z); maxZ = std::max(maxZ, v.z);
            }
            float scaleX = (maxX - minX) / 2.0f;
            float scaleY = (maxY - minY) / 2.0f;
            float scaleZ = (maxZ - minZ) / 2.0f;
            geom = PxBoxGeometry(scaleX, scaleY, scaleZ);
        }
        PxShape* shape = physics->createShape(geom, *material, true);
        mActor->attachShape(*shape);
        shape->release();
    } else {
        // Заглушка, если модель не загрузилась
        PxBoxGeometry geom(3.0f, 1.5f, 4.0f);
        PxShape* shape = physics->createShape(geom, *material, true);
        mActor->attachShape(*shape);
        shape->release();
    }
    // --- КОНЕЦ НОВОГО ---

    switch (type.type) {
        case VehicleDriveType::Wheeled4WD:
            CreateWheeledVehicle(physics);
            break;
        case VehicleDriveType::Tracked:
            CreateTrackedVehicle(physics);
            break;
        case VehicleDriveType::Helicopter:
            CreateHelicopter(physics);
            break;
        default:
            break;
    }
    PhysXInitializer::gScene->addActor(*mActor);
    LOG(L"Транспорт создан: " + std::wstring(type.name.begin(), type.name.end()));
}

void Vehicle::Update(float dt) {
    switch (type.type) {
        case VehicleDriveType::Wheeled4WD: {
            auto* v4w = reinterpret_cast<PxVehicleDrive4W*>(mVehicle);
            if (!v4w) break;
            auto& dyn = v4w->mDriveDynData;
            dyn.setAnalogInput(PxVehicleDrive4WControl::eANALOG_INPUT_THROTTLE, throttle);
            dyn.setAnalogInput(PxVehicleDrive4WControl::eANALOG_INPUT_BRAKE, brake);
            dyn.setAnalogInput(PxVehicleDrive4WControl::eANALOG_INPUT_STEER_LEFT, -steer);
            dyn.setAnalogInput(PxVehicleDrive4WControl::eANALOG_INPUT_STEER_RIGHT, steer);
            break;
        }
        case VehicleDriveType::Tracked: {
            auto* tank = reinterpret_cast<PxVehicleDriveTank*>(mVehicle);
            if (!tank) break;
            auto& dyn = tank->mDriveDynData;
            dyn.setAnalogInput(PxVehicleDriveTankControl::eANALOG_INPUT_LEFT_THROTTLE, throttle - steer);
            dyn.setAnalogInput(PxVehicleDriveTankControl::eANALOG_INPUT_RIGHT_THROTTLE, throttle + steer);
            break;
        }
        case VehicleDriveType::Helicopter:
            if (engineOn) {
                rotorRPM = std::min(rotorRPM + dt * 50.0f, type.engineRPM);
            } else {
                rotorRPM = std::max(rotorRPM - dt * 30.0f, 0.0f);
            }
            float lift = rotorRPM * rotorRPM * 0.0001f;
            mActor->addForce(PxVec3(0, lift, 0), PxForceMode::eACCELERATION);
            PxVec3 forward = mActor->getGlobalPose().q.getBasisVector2();
            PxVec3 right = mActor->getGlobalPose().q.getBasisVector0();
            PxVec3 controlForce = forward * steer * 5000.0f + right * throttle * 3000.0f;
            mActor->addForce(controlForce, PxForceMode::eFORCE);
            break;
    }

    // --- НОВОЕ: Обновление RenderableVehicle ---
    if (renderableVehicle && mActor) {
        PxTransform t = mActor->getGlobalPose();
        glm::vec3 pos{t.p.x, t.p.y, t.p.z};
        glm::quat rot(t.q.w, t.q.x, t.q.y, t.q.z);
        renderableVehicle->SetTransform(pos, rot); // Обновляем трансформацию для рендера
    }
    // --- КОНЕЦ НОВОГО ---
}

void Vehicle::CreateWheeledVehicle(PxPhysics* physics) {
    auto* v4w = PxVehicleDrive4W::allocate(4);
    if (!v4w) return;

    PxVehicleWheelsSimData ws = PxVehicleWheelsSimData::allocate(4);
    for (size_t i = 0; i < type.wheels.size() && i < 4; ++i) {
        auto& w = type.wheels[i];
        PxWheelData wd;
        wd.mRadius = w.radius;
        wd.mWidth = w.width;
        wd.mMaxBrakeTorque = w.maxBrakeTorque;
        ws.setWheelData(i, wd);

        PxSuspensionData sd;
        sd.mMaxCompression = 0.3f;
        sd.mMaxDroop = 0.2f;
        sd.mSpringStrength = 35000.0f;
        ws.setSuspensionData(i, sd);

        ws.setWheelCentreOffset(i, PxVec3(w.position.x, w.position.y, w.position.z));
    }

    PxVehicleDriveSimData4W ds;
    PxVehicleEngineData ed;
    ed.mPeakTorque = type.engineTorque;
    ed.mMaxOmega = type.engineRPM;
    ds.setEngineData(ed);

    v4w->setup(physics, mActor, ws, ds.engine, ds.gears, PxVehicleAutoBoxData(), 0, false);
    mVehicle = v4w;
}

void Vehicle::CreateTrackedVehicle(PxPhysics* physics) {
    auto* tank = PxVehicleDriveTank::allocate(PxVehicleDriveTankControlModel::eSPECIAL);
    if (!tank) return;
    LOG(L"Инициализация гусеничного транспорта не завершена");
    mVehicle = tank;
}

void Vehicle::CreateHelicopter(PxPhysics* physics) {
    LOG(L"Физика вертолёта требует кастомной реализации");
}

Vehicle::~Vehicle() {
    if (renderableVehicle) {
        for (auto* mesh : renderableVehicle->meshes) {
            delete mesh; // Удаляем меш, созданный в ModelLoader
        }
        delete renderableVehicle; // Удаляем RenderableVehicle
    }
    if (mVehicle) {
        if (type.type == VehicleDriveType::Wheeled4WD) {
            reinterpret_cast<PxVehicleDrive4W*>(mVehicle)->free();
        } else if (type.type == VehicleDriveType::Tracked) {
            reinterpret_cast<PxVehicleDriveTank*>(mVehicle)->free();
        }
    }
}