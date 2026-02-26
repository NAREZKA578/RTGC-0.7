#include "WeatherSystem.hpp"
#include "../cuda/WindCuda.h" // Подключаем CUDA-заголовок

WeatherSystem::WeatherSystem() : windField_h(new float[size * 3]) {
    cudaMalloc(&windField_d, size * 3 * sizeof(float));
    LOG(L"Погодная система инициализирована с CUDA");
}

WeatherSystem::~WeatherSystem() {
    cudaFree(windField_d);
    delete[] windField_h;
}

void WeatherSystem::Update(float dt) {
    timeOfDay += dt * 0.01f;
    if (timeOfDay >= 24.0f) timeOfDay -= 24.0f;
    isDay = (timeOfDay >= 6.0f && timeOfDay < 18.0f);
    windSpeed = 5.0f + sinf(timeOfDay * 0.5f) * 3.0f;
    windDirection += dt * 0.02f;

    // Используем CUDA для вычисления поля ветра
    LaunchWindCuda(windField_d, size, timeOfDay);

    // Копируем результат обратно на CPU
    cudaMemcpy(windField_h, windField_d, size * 3 * sizeof(float), cudaMemcpyDeviceToHost);
}

glm::vec3 WeatherSystem::GetWindAt(const glm::vec3& pos) const {
    int x = static_cast<int>(pos.x) % 256;
    int z = static_cast<int>(pos.z) % 256;
    int idx = z * 256 + x;
    // Используем данные с CPU
    return {windField_h[idx], windField_h[idx + size], windField_h[idx + 2*size]};
}