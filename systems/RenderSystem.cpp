#include "RenderSystem.hpp"

void RenderSystem::Render(entt::registry& registry, const class Shader& shader, const glm::mat4& view, const glm::mat4& proj) {
    auto viewable = registry.view<RenderableComponent, TransformComponent>(); // Ищем объекты с обоими компонентами
    for (auto entity : viewable) {
        auto& [r, t] = viewable.get<RenderableComponent, TransformComponent>(entity); // Получаем оба компонента
        if (r.renderable) {
            // ИСПРАВЛЕНО: Устанавливаем матрицу модели из TransformComponent
            glm::mat4 model = glm::translate(glm::mat4(1.0f), t.position.ToGLM()); // Конвертируем Vector3 в glm
            model = model * glm::toMat4(t.rotation);
            model = glm::scale(model, t.scale.ToGLM()); // Конвертируем Vector3 в glm
            glUniformMatrix4fv(glGetUniformLocation(shader.id, "uModel"), 1, GL_FALSE, &model[0][0]);
            r.renderable->Render(shader);
        }
    }
}