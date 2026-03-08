//! Система миссий и сохранений
//! 
//! Реализует:
//! - Систему миссий с целями и наградами
//! - Сохранение и загрузку прогресса
//! - Отслеживание статистики игрока

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use tokio::fs;

/// Тип миссии
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum MissionType {
    /// Доставить объект в точку
    Delivery,
    /// Уничтожить цель
    Destroy,
    /// Гонка на время
    Race,
    /// Исследовать область
    Explore,
    /// Собрать предметы
    Collect,
    /// Выжить в течение времени
    Survive,
}

/// Статус миссии
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum MissionStatus {
    /// Доступна для взятия
    Available,
    /// Активна (выполняется)
    Active,
    /// Завершена успешно
    Completed,
    /// Провалена
    Failed,
    /// Награда получена
    Claimed,
}

/// Цель миссии
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MissionObjective {
    /// Описание цели
    pub description: String,
    /// Текущий прогресс
    pub current: i32,
    /// Требуемый прогресс
    pub required: i32,
    /// Тип цели
    pub objective_type: String,
}

impl MissionObjective {
    pub fn new(description: &str, required: i32, objective_type: &str) -> Self {
        Self {
            description: description.to_string(),
            current: 0,
            required,
            objective_type: objective_type.to_string(),
        }
    }

    pub fn is_complete(&self) -> bool {
        self.current >= self.required
    }

    pub fn progress_percent(&self) -> f32 {
        (self.current as f32 / self.required as f32).min(1.0) * 100.0
    }
}

/// Награда за миссию
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MissionReward {
    /// Деньги
    pub money: u32,
    /// Опыт
    pub experience: u32,
    /// Предметы (id, количество)
    pub items: Vec<(String, u32)>,
    /// Репутация
    pub reputation: i32,
}

impl Default for MissionReward {
    fn default() -> Self {
        Self {
            money: 0,
            experience: 0,
            items: Vec::new(),
            reputation: 0,
        }
    }
}

/// Миссия
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Mission {
    /// Уникальный ID
    pub id: u32,
    /// Название
    pub title: String,
    /// Описание
    pub description: String,
    /// Тип миссии
    pub mission_type: MissionType,
    /// Статус
    pub status: MissionStatus,
    /// Цели
    pub objectives: Vec<MissionObjective>,
    /// Награда
    pub reward: MissionReward,
    /// Заказчик
    pub giver: String,
    /// Срок выполнения (None = без срока)
    pub time_limit: Option<f32>,
    /// Время начала (если есть лимит)
    pub start_time: Option<f32>,
}

impl Mission {
    pub fn new(id: u32, title: &str, description: &str, mission_type: MissionType) -> Self {
        Self {
            id,
            title: title.to_string(),
            description: description.to_string(),
            mission_type,
            status: MissionStatus::Available,
            objectives: Vec::new(),
            reward: MissionReward::default(),
            giver: String::new(),
            time_limit: None,
            start_time: None,
        }
    }

    pub fn with_objective(mut self, objective: MissionObjective) -> Self {
        self.objectives.push(objective);
        self
    }

    pub fn with_reward(mut self, reward: MissionReward) -> Self {
        self.reward = reward;
        self
    }

    pub fn with_giver(mut self, giver: &str) -> Self {
        self.giver = giver.to_string();
        self
    }

    /// Обновление прогресса цели
    pub fn update_objective(&mut self, objective_type: &str, value: i32) {
        if self.status != MissionStatus::Active {
            return;
        }

        for obj in &mut self.objectives {
            if obj.objective_type == objective_type {
                obj.current = obj.current.max(value);
                
                // Проверка завершения всех целей
                if self.objectives.iter().all(|o| o.is_complete()) {
                    self.status = MissionStatus::Completed;
                }
            }
        }
    }

    /// Принять миссию
    pub fn accept(&mut self) {
        if self.status == MissionStatus::Available {
            self.status = MissionStatus::Active;
        }
    }

    /// Завершить миссию
    pub fn complete(&mut self) {
        if self.status == MissionStatus::Completed {
            self.status = MissionStatus::Claimed;
        }
    }

    /// Провалить миссию
    pub fn fail(&mut self) {
        self.status = MissionStatus::Failed;
    }

    /// Проверка просрочки по времени
    pub fn check_time_limit(&mut self, current_time: f32) -> bool {
        if let (Some(limit), Some(start)) = (self.time_limit, self.start_time) {
            if current_time - start > limit && self.status == MissionStatus::Active {
                self.fail();
                return true;
            }
        }
        false
    }
}

/// Статистика игрока
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct PlayerStats {
    /// Всего пройдено миссий
    pub missions_completed: u32,
    /// Всего заработано денег
    pub total_money_earned: u32,
    /// Пройденное расстояние (км)
    pub distance_traveled: f32,
    /// Время в игре (часы)
    pub playtime_hours: f32,
    /// Средняя скорость (км/ч)
    pub average_speed: f32,
    /// Аварий
    pub crashes: u32,
    /// Побед в гонках
    pub race_wins: u32,
}

/// Данные сохранения игры
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SaveData {
    /// Версия сохранения
    pub version: u32,
    /// Имя игрока
    pub player_name: String,
    /// Текущие деньги
    pub money: u32,
    /// Уровень
    pub level: u32,
    /// Опыт
    pub experience: u32,
    /// Позиция игрока
    pub player_position: [f32; 3],
    /// Поворот игрока
    pub player_rotation: [f32; 4],
    /// Активные миссии
    pub active_missions: Vec<Mission>,
    /// Завершенные миссии (ID)
    pub completed_missions: Vec<u32>,
    /// Статистика
    pub stats: PlayerStats,
    /// Инвентарь
    pub inventory: HashMap<String, u32>,
    /// Открытый контент
    pub unlocked_content: Vec<String>,
    /// Общее время игры (секунды)
    pub total_playtime: f32,
}

impl Default for SaveData {
    fn default() -> Self {
        Self {
            version: 1,
            player_name: "Player".to_string(),
            money: 1000,
            level: 1,
            experience: 0,
            player_position: [0.0, 0.0, 0.0],
            player_rotation: [0.0, 0.0, 0.0, 1.0],
            active_missions: Vec::new(),
            completed_missions: Vec::new(),
            stats: PlayerStats::default(),
            inventory: HashMap::new(),
            unlocked_content: Vec::new(),
            total_playtime: 0.0,
        }
    }
}

/// Менеджер миссий
pub struct MissionManager {
    missions: HashMap<u32, Mission>,
    next_mission_id: u32,
}

impl MissionManager {
    pub fn new() -> Self {
        Self {
            missions: HashMap::new(),
            next_mission_id: 1,
        }
    }

    /// Добавить миссию
    pub fn add_mission(&mut self, mission: Mission) -> u32 {
        let id = mission.id;
        self.missions.insert(id, mission);
        id
    }

    /// Создать новую миссию
    pub fn create_mission(
        &mut self,
        title: &str,
        description: &str,
        mission_type: MissionType,
    ) -> u32 {
        let id = self.next_mission_id;
        self.next_mission_id += 1;

        let mission = Mission::new(id, title, description, mission_type);
        self.add_mission(mission);
        id
    }

    /// Получить миссию по ID
    pub fn get_mission(&self, id: u32) -> Option<&Mission> {
        self.missions.get(&id)
    }

    /// Получить миссию по ID (mutable)
    pub fn get_mission_mut(&mut self, id: u32) -> Option<&mut Mission> {
        self.missions.get_mut(&id)
    }

    /// Принять миссию
    pub fn accept_mission(&mut self, id: u32) -> bool {
        if let Some(mission) = self.missions.get_mut(&id) {
            mission.accept();
            true
        } else {
            false
        }
    }

    /// Получить доступные миссии
    pub fn get_available_missions(&self) -> Vec<&Mission> {
        self.missions
            .values()
            .filter(|m| m.status == MissionStatus::Available)
            .collect()
    }

    /// Получить активные миссии
    pub fn get_active_missions(&self) -> Vec<&Mission> {
        self.missions
            .values()
            .filter(|m| m.status == MissionStatus::Active)
            .collect()
    }

    /// Получить завершенные миссии (неполученные)
    pub fn get_completed_missions(&self) -> Vec<&Mission> {
        self.missions
            .values()
            .filter(|m| m.status == MissionStatus::Completed)
            .collect()
    }

    /// Обновить прогресс миссии
    pub fn update_mission_progress(&mut self, mission_id: u32, objective_type: &str, value: i32) {
        if let Some(mission) = self.missions.get_mut(&mission_id) {
            mission.update_objective(objective_type, value);
        }
    }

    /// Получить награду за миссию
    pub fn claim_reward(&mut self, id: u32) -> Option<MissionReward> {
        if let Some(mission) = self.missions.get_mut(&id) {
            if mission.status == MissionStatus::Completed {
                let reward = mission.reward.clone();
                mission.complete();
                return Some(reward);
            }
        }
        None
    }
}

impl Default for MissionManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Система сохранений
pub struct SaveSystem {
    save_directory: String,
}

impl SaveSystem {
    pub fn new(save_directory: &str) -> Self {
        Self {
            save_directory: save_directory.to_string(),
        }
    }

    /// Сохранить игру
    pub async fn save_game(&self, save_data: &SaveData, slot: u32) -> Result<(), String> {
        let filename = format!("{}/save_{}.json", self.save_directory, slot);
        
        // Создаем директорию если не существует
        if let Err(e) = std::fs::create_dir_all(&self.save_directory) {
            return Err(format!("Failed to create save directory: {}", e));
        }

        let json = serde_json::to_string_pretty(save_data)
            .map_err(|e| format!("Failed to serialize save data: {}", e))?;

        fs::write(&filename, json)
            .await
            .map_err(|e| format!("Failed to write save file: {}", e))?;

        Ok(())
    }

    /// Загрузить игру
    pub async fn load_game(&self, slot: u32) -> Result<SaveData, String> {
        let filename = format!("{}/save_{}.json", self.save_directory, slot);

        let content = fs::read_to_string(&filename)
            .await
            .map_err(|e| format!("Failed to read save file: {}", e))?;

        let save_data: SaveData = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to deserialize save data: {}", e))?;

        Ok(save_data)
    }

    /// Проверка существования сохранения
    pub fn save_exists(&self, slot: u32) -> bool {
        let filename = format!("{}/save_{}.json", self.save_directory, slot);
        Path::new(&filename).exists()
    }

    /// Удалить сохранение
    pub fn delete_save(&self, slot: u32) -> Result<(), String> {
        let filename = format!("{}/save_{}.json", self.save_directory, slot);
        
        std::fs::remove_file(&filename)
            .map_err(|e| format!("Failed to delete save file: {}", e))?;
        
        Ok(())
    }

    /// Получить список доступных сохранений
    pub fn list_saves(&self) -> Vec<u32> {
        let mut saves = Vec::new();
        
        if let Ok(entries) = std::fs::read_dir(&self.save_directory) {
            for entry in entries.flatten() {
                let filename = entry.file_name().to_string_lossy().to_string();
                if filename.starts_with("save_") && filename.ends_with(".json") {
                    if let Some(slot_str) = filename.strip_prefix("save_").and_then(|s| s.strip_suffix(".json")) {
                        if let Ok(slot) = slot_str.parse::<u32>() {
                            saves.push(slot);
                        }
                    }
                }
            }
        }
        
        saves.sort();
        saves
    }

    /// Автосохранение
    pub async fn autosave(&self, save_data: &SaveData) -> Result<(), String> {
        self.save_game(save_data, 0).await // Slot 0 reserved for autosave
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mission_creation() {
        let mut manager = MissionManager::new();
        
        let id = manager.create_mission(
            "Test Mission",
            "A test mission",
            MissionType::Delivery,
        );
        
        assert_eq!(id, 1);
        
        let mission = manager.get_mission(id).unwrap();
        assert_eq!(mission.title, "Test Mission");
        assert_eq!(mission.status, MissionStatus::Available);
    }

    #[test]
    fn test_mission_accept_and_complete() {
        let mut manager = MissionManager::new();
        
        let id = manager.create_mission(
            "Delivery",
            "Deliver package",
            MissionType::Delivery,
        );
        
        // Добавляем цель
        if let Some(mission) = manager.get_mission_mut(id) {
            mission.objectives.push(MissionObjective::new("Deliver", 1, "delivery"));
        }
        
        // Принимаем миссию
        manager.accept_mission(id);
        
        // Обновляем прогресс
        manager.update_mission_progress(id, "delivery", 1);
        
        // Проверяем статус
        let mission = manager.get_mission(id).unwrap();
        assert_eq!(mission.status, MissionStatus::Completed);
    }

    #[test]
    fn test_save_data_default() {
        let save_data = SaveData::default();
        assert_eq!(save_data.version, 1);
        assert_eq!(save_data.money, 1000);
        assert_eq!(save_data.level, 1);
    }

    #[test]
    fn test_mission_objective_progress() {
        let mut obj = MissionObjective::new("Collect items", 10, "collect");
        assert_eq!(obj.progress_percent(), 0.0);
        
        obj.current = 5;
        assert_eq!(obj.progress_percent(), 50.0);
        
        obj.current = 10;
        assert!(obj.is_complete());
        assert_eq!(obj.progress_percent(), 100.0);
    }
}
