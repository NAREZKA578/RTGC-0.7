#pragma once
#include <glm/glm.hpp>
#include <vector>
#include "Shader.hpp"

struct Light {
    glm::vec3 position{0,0,0};
    glm::vec3 color{1,1,1};
    float intensity = 1.0f;
    float range = 10.0f;
};

class LightingSystem {
    std::vector<Light> lights;
    Shader* lightingShader = nullptr;

public:
    LightingSystem(Shader* shader);
    void AddLight(const Light& light);
    void SetLightUniforms(Shader& shader) const;
    void Update(float dt);
};