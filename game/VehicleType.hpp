#pragma once
#include <string>
#include <vector>
#include <glm/glm.hpp>

enum class VehicleDriveType {
    Wheeled4WD,
    Tracked,
    Helicopter
};

struct Wheel {
    glm::vec3 position;
    float radius = 0.0f;
    float width = 0.0f;
    float maxBrakeTorque = 0.0f;
    float maxSteer = 0.3f;
};

struct VehicleType {
    std::string name;
    VehicleDriveType type;
    float mass = 0.0f;
    float maxSpeed = 0.0f;
    float engineTorque = 0.0f;
    float engineRPM = 0.0f;
    std::vector<Wheel> wheels;
    std::string modelFile; // Путь к файлу модели
};