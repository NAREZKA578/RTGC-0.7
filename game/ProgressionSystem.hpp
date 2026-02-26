#pragma once
#include <string>

class ProgressionSystem {
    int experience = 0;
    int level = 1;
    int skillPoints = 0;

public:
    void AddExperience(int exp);
    void LevelUp();
    void SpendSkillPoint(const std::string& skill);
    int GetLevel() const { return level; }
    int GetExperience() const { return experience; }
    int GetSkillPoints() const { return skillPoints; }
};