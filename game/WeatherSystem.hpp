#pragma once
#include <glm/glm.hpp>
#include <cuda_runtime.h> // Для CUDA
#include "../core/Logger.hpp"

class WeatherSystem {
    float* windField_d = nullptr; // Указатель на GPU
    float* windField_h = nullptr; // Указатель на CPU
    int size = 256 * 256;
public:
    float timeOfDay = 12.0f;
    float windSpeed = 0.0f;
    float windDirection = 0.0f;
    float rainIntensity = 0.0f;
    bool isDay = true;

    WeatherSystem();
    ~WeatherSystem();
    void Update(float dt);
    glm::vec3 GetWindAt(const glm::vec3& pos) const;
};