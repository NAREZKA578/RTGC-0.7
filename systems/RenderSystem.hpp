#pragma once
#include "../components/RenderableComponent.hpp"
#include "../components/TransformComponent.hpp"
#include "../graphics/Shader.hpp"

class RenderSystem {
public:
    // ИСПРАВЛЕНО: Теперь использует TransformComponent для матрицы модели
    static void Render(entt::registry& registry, const class Shader& shader, const glm::mat4& view, const glm::mat4& proj);
};