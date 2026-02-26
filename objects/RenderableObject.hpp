#pragma once
#include "GameObject.hpp"
#include "../graphics/Mesh.hpp"
#include "../graphics/Shader.hpp"
#include <vector>

class RenderableObject : public GameObject {
    std::vector<Mesh*> meshes;
    Shader* shader;
    glm::mat4 modelMatrix{1.0f};

public:
    RenderableObject(const std::string& name, const Vector3& pos, Shader* sh);
    ~RenderableObject() override;

    void AddMesh(Mesh* mesh);
    void SetShader(Shader* sh) { shader = sh; }
    Shader* GetShader() const { return shader; }

    void Update(float dt) override;
    void Render() override;
};