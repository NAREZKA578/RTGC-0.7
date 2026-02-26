#pragma once
#include "../math/Vector3.hpp"
#include <vector>
#include <string>
#include <memory>
#include <cstdint>

class GameObject {
private:
    Vector3 position;
    Vector3 rotation;
    Vector3 scale;
    std::string name;
    bool active;
    uint32_t id;
    
public:
    GameObject(const std::string& objName = "GameObject");
    virtual ~GameObject() = default;
    
    virtual void Update(float dt);
    virtual void Render();
    
    // Transform
    Vector3 GetPosition() const { return position; }
    void SetPosition(const Vector3& pos) { position = pos; }
    Vector3 GetRotation() const { return rotation; }
    void SetRotation(const Vector3& rot) { rotation = rot; }
    Vector3 GetScale() const { return scale; }
    void SetScale(const Vector3& scl) { scale = scl; }
    
    // Properties
    std::string GetName() const { return name; }
    void SetName(const std::string& objName) { name = objName; }
    bool IsActive() const { return active; }
    void SetActive(bool act) { active = act; }
    uint32_t GetID() const { return id; }
    
    // Utility
    virtual std::string ToString() const;
};

class World {
private:
    std::vector<std::unique_ptr<GameObject>> objects;
    std::string name;
    bool active;
    
public:
    World(const std::string& worldName = "World");
    ~World() = default;
    
    void Update(float dt);
    void Render();
    
    GameObject* CreateObject(const std::string& name = "GameObject");
    void DestroyObject(GameObject* obj);
    void DestroyObject(uint32_t id);
    
    GameObject* FindObject(const std::string& name);
    GameObject* FindObject(uint32_t id);
    std::vector<GameObject*> FindObjectsByTag(const std::string& tag);
    
    void Clear();
    size_t GetObjectCount() const { return objects.size(); }
    
    std::string GetName() const { return name; }
    void SetName(const std::string& worldName) { name = worldName; }
    bool IsActive() const { return active; }
    void SetActive(bool act) { active = act; }
};

class WorldManager {
private:
    std::vector<std::unique_ptr<World>> worlds;
    World* activeWorld;
    
public:
    WorldManager();
    ~WorldManager() = default;
    
    void Update(float dt);
    void Render();
    
    World* CreateWorld(const std::string& name = "World");
    void DestroyWorld(World* world);
    void DestroyWorld(const std::string& name);
    
    void SetActiveWorld(World* world);
    void SetActiveWorld(const std::string& name);
    World* GetActiveWorld() const { return activeWorld; }
    
    World* FindWorld(const std::string& name);
    void Clear();
    size_t GetWorldCount() const { return worlds.size(); }
};