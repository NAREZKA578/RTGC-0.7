#pragma once
#include "Mesh.hpp"
#include "Shader.hpp"
#include <glm/glm.hpp>
#include <glm/gtc/quaternion.hpp>
#include <vector>

class RenderableVehicle {
public:
    std::vector<Mesh*> meshes; // Поддержка нескольких мешей
    glm::mat4 modelMatrix{1.0f};

    RenderableVehicle() = default;

    void SetMeshes(const std::vector<Mesh*>& m) { meshes = m; }
    void SetTransform(const glm::vec3& pos, const glm::quat& rot, const glm::vec3& scale = {1,1,1}) {
        modelMatrix = glm::translate(glm::mat4(1.0f), pos);
        modelMatrix *= glm::toMat4(rot);
        modelMatrix = glm::scale(modelMatrix, scale);
    }

    void Render(const Shader& shader) const {
        for (auto* mesh : meshes) {
            if (mesh) {
                glUniformMatrix4fv(glGetUniformLocation(shader.id, "uModel"), 1, GL_FALSE, &modelMatrix[0][0]);
                mesh->Render();
            }
        }
    }
};