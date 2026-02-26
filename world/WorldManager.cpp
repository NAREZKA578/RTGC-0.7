#include "WorldManager.hpp"
#include "../core/Logger.hpp"
#include <sstream>
#include <algorithm>
#include <cstdint>

static uint32_t nextObjectID = 1;

GameObject::GameObject(const std::string& objName) 
    : name(objName), active(true), id(nextObjectID++) {
    position = Vector3(0, 0, 0);
    rotation = Vector3(0, 0, 0);
    scale = Vector3(1, 1, 1);
}

void GameObject::Update(float dt) {
    if (!active) return;
    // Базовая реализация - пустая
}

void GameObject::Render() {
    if (!active) return;
    // Базовая реализация - пустая
}

std::string GameObject::ToString() const {
    std::ostringstream oss;
    oss << "GameObject[" << id << "] '" << name << "' "
        << "Pos(" << position.x << "," << position.y << "," << position.z << ") "
        << "Active: " << (active ? "true" : "false");
    return oss.str();
}

World::World(const std::string& worldName) : name(worldName), active(true) {
    Logger::Log("World создан: ", name);
}

void World::Update(float dt) {
    if (!active) return;
    
    for (auto& obj : objects) {
        if (obj && obj->IsActive()) {
            obj->Update(dt);
        }
    }
}

void World::Render() {
    if (!active) return;
    
    for (auto& obj : objects) {
        if (obj && obj->IsActive()) {
            obj->Render();
        }
    }
}

GameObject* World::CreateObject(const std::string& objName) {
    auto obj = std::make_unique<GameObject>(objName);
    GameObject* ptr = obj.get();
    objects.push_back(std::move(obj));
    Logger::Log("GameObject создан в мире '", name, "': ", objName);
    return ptr;
}

void World::DestroyObject(GameObject* obj) {
    if (!obj) return;
    
    objects.erase(
        std::remove_if(objects.begin(), objects.end(),
            [obj](const std::unique_ptr<GameObject>& ptr) { return ptr.get() == obj; }
        ),
        objects.end()
    );
    Logger::Log("GameObject уничтожен в мире '", name, "': ", obj->GetName());
}

void World::DestroyObject(uint32_t objId) {
    DestroyObject(FindObject(objId));
}

GameObject* World::FindObject(const std::string& objName) {
    for (auto& obj : objects) {
        if (obj && obj->GetName() == objName) {
            return obj.get();
        }
    }
    return nullptr;
}

GameObject* World::FindObject(uint32_t objId) {
    for (auto& obj : objects) {
        if (obj && obj->GetID() == objId) {
            return obj.get();
        }
    }
    return nullptr;
}

std::vector<GameObject*> World::FindObjectsByTag(const std::string& tag) {
    std::vector<GameObject*> result;
    // В реальной реализации здесь был бы поиск по тегам
    return result;
}

void World::Clear() {
    objects.clear();
    Logger::Log("World '", name, "' очищен");
}

WorldManager::WorldManager() : activeWorld(nullptr) {
    Logger::Log("WorldManager создан");
}

void WorldManager::Update(float dt) {
    for (auto& world : worlds) {
        if (world && world->IsActive()) {
            world->Update(dt);
        }
    }
}

void WorldManager::Render() {
    for (auto& world : worlds) {
        if (world && world->IsActive()) {
            world->Render();
        }
    }
}

World* WorldManager::CreateWorld(const std::string& worldName) {
    auto world = std::make_unique<World>(worldName);
    World* ptr = world.get();
    worlds.push_back(std::move(world));
    
    if (!activeWorld) {
        activeWorld = ptr;
    }
    
    Logger::Log("World создан: ", worldName);
    return ptr;
}

void WorldManager::DestroyWorld(World* world) {
    if (!world) return;
    
    if (world == activeWorld) {
        activeWorld = nullptr;
    }
    
    worlds.erase(
        std::remove_if(worlds.begin(), worlds.end(),
            [world](const std::unique_ptr<World>& ptr) { return ptr.get() == world; }
        ),
        worlds.end()
    );
    
    Logger::Log("World уничтожен: ", world->GetName());
}

void WorldManager::DestroyWorld(const std::string& worldName) {
    DestroyWorld(FindWorld(worldName));
}

void WorldManager::SetActiveWorld(World* world) {
    activeWorld = world;
    if (world) {
        Logger::Log("Активный мир установлен: ", world->GetName());
    }
}

void WorldManager::SetActiveWorld(const std::string& worldName) {
    SetActiveWorld(FindWorld(worldName));
}

World* WorldManager::FindWorld(const std::string& worldName) {
    for (auto& world : worlds) {
        if (world && world->GetName() == worldName) {
            return world.get();
        }
    }
    return nullptr;
}

void WorldManager::Clear() {
    worlds.clear();
    activeWorld = nullptr;
    Logger::Log("WorldManager очищен");
}