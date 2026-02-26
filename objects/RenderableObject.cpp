#include "RenderableObject.hpp"
#include "../core/Logger.hpp"

RenderableObject::RenderableObject(const std::string& name, const Vector3& pos, Shader* sh)
    : GameObject(name, pos), shader(sh) {
    LOG(L"Создан RenderableObject: " + std::wstring(name.begin(), name.end()));
}

void RenderableObject::AddMesh(Mesh* mesh) {
    meshes.push_back(mesh);
}

void RenderableObject::Update(float dt) {
    GameObject::Update(dt);
    // Обновление модели матрицы на основе позиции, вращения и масштаба
    modelMatrix = glm::translate(glm::mat4(1.0f), position.ToGLM());
    modelMatrix *= glm::toMat4(rotation);
    modelMatrix = glm::scale(modelMatrix, scale.ToGLM());
}

void RenderableObject::Render() {
    if (!isActive || !shader) return;

    shader->Use();
    glUniformMatrix4fv(glGetUniformLocation(shader->id, "uModel"), 1, GL_FALSE, &modelMatrix[0][0]);

    for (auto* mesh : meshes) {
        if (mesh) {
            mesh->Render();
        }
    }
}

RenderableObject::~RenderableObject() {
    // Меш не удаляется, так как он может быть общим
}