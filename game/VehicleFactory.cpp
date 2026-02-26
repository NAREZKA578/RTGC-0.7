#include "VehicleFactory.hpp"

VehicleType VehicleFactory::CreateKamaz() {
    VehicleType v;
    v.name = "КамАЗ-5350";
    v.type = VehicleDriveType::Wheeled4WD;
    v.mass = 8000.0f;
    v.maxSpeed = 90.0f;
    v.engineTorque = 1200.0f;
    v.wheels = {
        {{-1.8f, -0.5f, -2.5f}, 0.6f, 0.4f, 2000.0f, 0.3f},
        {{ 1.8f, -0.5f, -2.5f}, 0.6f, 0.4f, 2000.0f, 0.3f},
        {{-1.8f, -0.5f,  2.5f}, 0.6f, 0.4f, 2000.0f, 0.0f},
        {{ 1.8f, -0.5f,  2.5f}, 0.6f, 0.4f, 2000.0f, 0.0f}
    };
    v.modelFile = "assets/models/kamaz.obj"; // Путь к файлу
    return v;
}