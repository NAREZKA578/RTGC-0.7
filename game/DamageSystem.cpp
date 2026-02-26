#include "DamageSystem.hpp"

void DamageSystem::ApplyDamage(DamageProfile& dp, float totalDamage, const std::string& hitPart) {
    if (hitPart == "head") dp.head.health -= totalDamage * 2.0f;
    else if (hitPart == "torso") dp.torso.health -= totalDamage;
    else if (hitPart == "arms") dp.arms.health -= totalDamage * 0.5f;
    else if (hitPart == "legs") dp.legs.health -= totalDamage * 0.7f;
}