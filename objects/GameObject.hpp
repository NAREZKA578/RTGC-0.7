#pragma once
#include <string>
#include <memory>
#include <vector>
#include <glm/glm.hpp>
#include "../math/Vector3.hpp"

class Component {
public:
    virtual ~Component() = default;
    virtual void Update(float dt) {}
    virtual void Render() {}
};

class GameObject {
protected:
    std::string name;
    Vector3 position;
    glm::quat rotation;
    Vector3 scale;
    std::vector<std::unique_ptr<Component>> components;
    bool isActive;

public:
    GameObject(const std::string& n, const Vector3& pos = {0,0,0});
    virtual ~GameObject() = default;

    void SetPosition(const Vector3& pos) { position = pos; }
    Vector3 GetPosition() const { return position; }

    void SetRotation(const glm::quat& rot) { rotation = rot; }
    glm::quat GetRotation() const { return rotation; }

    void SetScale(const Vector3& s) { scale = s; }
    Vector3 GetScale() const { return scale; }

    void AddComponent(std::unique_ptr<Component> comp);
    void RemoveComponent(const std::string& compType);

    template<typename T>
    T* GetComponent() const;

    virtual void Update(float dt);
    virtual void Render();

    void SetActive(bool active) { isActive = active; }
    bool GetActive() const { return isActive; }

    const std::string& GetName() const { return name; }
};