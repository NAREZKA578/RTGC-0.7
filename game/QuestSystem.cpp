#include "QuestSystem.hpp"
#include "../core/Logger.hpp"

void QuestSystem::AddQuest(const Quest& quest) {
    quests.push_back(quest);
    LOG(L"Квест добавлен: " + std::wstring(quest.name.begin(), quest.name.end()));
}

void QuestSystem::Update(float dt) {
    CheckConditions();
}

void QuestSystem::CheckConditions() {
    for (size_t i = 0; i < quests.size(); ++i) {
        if (quests[i].state == QuestState::InProgress && quests[i].condition()) {
            CompleteQuest(i);
        }
    }
}

void QuestSystem::CompleteQuest(int index) {
    if (index >= 0 && index < static_cast<int>(quests.size())) {
        quests[index].state = QuestState::Completed;
        if (quests[index].onCompleted) {
            quests[index].onCompleted();
        }
        LOG(L"Квест завершён: " + std::wstring(quests[index].name.begin(), quests[index].name.end()));
    }
}