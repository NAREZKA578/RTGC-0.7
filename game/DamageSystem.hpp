#pragma once
#include <string>

struct BodyPart {
    float health = 100.0f;
    float armor = 0.0f;
};

struct DamageProfile {
    BodyPart head;
    BodyPart torso;
    BodyPart arms;
    BodyPart legs;
};

class DamageSystem {
public:
    static void ApplyDamage(DamageProfile& dp, float totalDamage, const std::string& hitPart);
};