#include "ProgressionSystem.hpp"
#include "../core/Logger.hpp"

void ProgressionSystem::AddExperience(int exp) {
    experience += exp;
    int expForNextLevel = level * 100; // Пример: 100, 200, 300...
    if (experience >= expForNextLevel) {
        LevelUp();
    }
}

void ProgressionSystem::LevelUp() {
    level++;
    skillPoints += 1; // +1 очко навыка за уровень
    experience = 0; // Сброс опыта
    LOG(L"Новый уровень: " + std::to_wstring(level));
}

void ProgressionSystem::SpendSkillPoint(const std::string& skill) {
    if (skillPoints > 0) {
        skillPoints--;
        LOG(L"Очко навыка потрачено на: " + std::wstring(skill.begin(), skill.end()));
    }
}