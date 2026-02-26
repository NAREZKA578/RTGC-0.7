#pragma once
#include <string>
#include <vector>
#include <functional>

enum class QuestState {
    NotStarted,
    InProgress,
    Completed,
    Failed
};

struct Quest {
    std::string name;
    std::string description;
    QuestState state = QuestState::NotStarted;
    std::function<bool()> condition;
    std::function<void()> onCompleted;
};

class QuestSystem {
    std::vector<Quest> quests;
    int activeQuestIndex = -1;

public:
    void AddQuest(const Quest& quest);
    void Update(float dt);
    void CheckConditions();
    void CompleteQuest(int index);
    const std::vector<Quest>& GetQuests() const { return quests; }
};