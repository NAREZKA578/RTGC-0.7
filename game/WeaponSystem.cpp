#include "WeaponSystem.hpp"

void WeaponSystem::Shoot(Weapon& w, float dt, const glm::vec3& pos, const glm::vec3& dir) {
    if (w.nextFire <= 0 && w.ammo > 0) {
        w.ammo--;
        w.nextFire = w.fireRate;
        // Логика выстрела - создание пули, отдача и т.д.
    }
    w.nextFire -= dt;
}