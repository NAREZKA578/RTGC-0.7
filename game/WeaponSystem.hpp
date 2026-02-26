#pragma once
#include <glm/glm.hpp>
#include <string>

struct Weapon {
    std::string name;
    float damage = 0.0f;
    float fireRate = 0.0f;
    float nextFire = 0.0f;
    int ammo = 30;
    int maxAmmo = 30;
};

class WeaponSystem {
public:
    static void Shoot(Weapon& w, float dt, const glm::vec3& pos, const glm::vec3& dir);
};