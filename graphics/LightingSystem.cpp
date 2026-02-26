#include "LightingSystem.hpp"
#include "Logger.hpp"

LightingSystem::LightingSystem(Shader* shader) : lightingShader(shader) {
    LOG(L"Система освещения инициализирована");

    // Добавление основного источника света
    Light mainLight;
    mainLight.position = glm::vec3(10, 10, 10);
    mainLight.color = glm::vec3(1, 1, 1);
    mainLight.intensity = 1.0f;
    mainLight.range = 50.0f;
    lights.push_back(mainLight);
}

void LightingSystem::AddLight(const Light& light) {
    lights.push_back(light);
}

void LightingSystem::SetLightUniforms(Shader& shader) const {
    shader.Use();
    glUniform1i(glGetUniformLocation(shader.id, "uLightCount"), static_cast<int>(lights.size()));

    for (size_t i = 0; i < lights.size(); ++i) {
        std::string lightName = "uLights[" + std::to_string(i) + "]";
        glUniform3fv(glGetUniformLocation(shader.id, (lightName + ".position").c_str()), 1, &lights[i].position[0]);
        glUniform3fv(glGetUniformLocation(shader.id, (lightName + ".color").c_str()), 1, &lights[i].color[0]);
        glUniform1f(glGetUniformLocation(shader.id, (lightName + ".intensity").c_str()), lights[i].intensity);
        glUniform1f(glGetUniformLocation(shader.id, (lightName + ".range").c_str()), lights[i].range);
    }
}

void LightingSystem::Update(float dt) {
    // Анимация света
    float time = static_cast<float>(glfwGetTime());
    for (auto& light : lights) {
        light.position.x = 10.0f + sinf(time) * 5.0f;
        light.position.z = 10.0f + cosf(time) * 5.0f;
    }
}