#include "GameObject.hpp"
#include "../core/Logger.hpp"

GameObject::GameObject(const std::string& n, const Vector3& pos)
    : name(n), position(pos), rotation(1,0,0,0), scale(1,1,1), isActive(true) {
    LOG(L"Создан GameObject: " + std::wstring(name.begin(), name.end()));
}

void GameObject::AddComponent(std::unique_ptr<Component> comp) {
    components.push_back(std::move(comp));
}

void GameObject::RemoveComponent(const std::string& compType) {
    // Реализация удаления компонента по типу
    auto it = std::find_if(components.begin(), components.end(),
        [&compType](const std::unique_ptr<Component>& comp) {
            // Здесь нужно добавить механизм идентификации типа компонента
            return true; // Заглушка
        });
    if (it != components.end()) {
        components.erase(it);
    }
}

void GameObject::Update(float dt) {
    if (!isActive) return;
    for (auto& comp : components) {
        comp->Update(dt);
    }
}

void GameObject::Render() {
    if (!isActive) return;
    for (auto& comp : components) {
        comp->Render();
    }
}