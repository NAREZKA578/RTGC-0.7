// ЧАСТЬ 1 — HUD: ЕДИНЫЙ ЦЕНТР ИНФОРМАЦИИ
// Весь HUD хранится в одном месте, управляется единым HudManager.

use nalgebra::{Vector3, UnitQuaternion};

/// Все данные для HUD — заполняются движком, рисуются HudManager
#[derive(Debug, Clone, Default)]
pub struct VehicleHudData {
    // === Блок ДВИЖЕНИЯ ===
    pub speed_kmh: f32,              // Текущая скорость км/ч
    pub speed_max_kmh: f32,          // Максимальная скорость (для шкалы)
    pub gear: GearState,             // Передача: Park, Rev, N, 1..8
    pub engine_rpm: f32,             // Текущие обороты
    pub engine_rpm_max: f32,         // Красная зона начинается отсюда
    pub engine_running: bool,        // Двигатель запущен?

    // === Блок РЕСУРСОВ ===
    pub fuel_level: f32,             // 0.0..1.0
    pub fuel_reserve: bool,          // Резервный уровень (мигать)
    pub engine_temp: f32,            // °C, 0..120
    pub engine_overheating: bool,    // Перегрев (мигать)

    // === Блок ТРАНСМИССИИ ===
    pub diff_front_locked: bool,     // Блокировка переднего диффа
    pub diff_rear_locked: bool,      // Блокировка заднего диффа
    pub diff_center_locked: bool,    // Межосевая блокировка
    pub awd_active: bool,            // Полный привод активен
    pub low_range: bool,             // Понижающий ряд включён

    // === Блок ПОДВЕСКИ ===
    pub wheel_contact: [bool; 4],    // Какие колёса в контакте с землёй
    pub wheel_slip: [f32; 4],        // Проскальзывание 0..1 каждого колеса
    pub suspension_load: [f32; 4],   // Нагрузка подвески 0..1

    // === Блок ГРУЗА ===
    pub cargo_attached: bool,        // Груз прицеплен
    pub cargo_weight_kg: f32,        // Масса груза
    pub cargo_damage: f32,           // Повреждение груза 0..1
    pub winch_active: bool,          // Лебёдка активна
    pub winch_length_m: f32,         // Длина троса лебёдки

    // === Блок ОКРУЖЕНИЯ ===
    pub altitude_m: f32,             // Высота над уровнем моря
    pub terrain_angle_deg: f32,      // Угол наклона поверхности
    pub vehicle_roll_deg: f32,       // Крен машины (бок)
    pub vehicle_pitch_deg: f32,      // Тангаж (нос/корма)
    pub is_tipped_over: bool,        // Машина перевёрнута?
}

#[derive(Debug, Clone, PartialEq)]
pub enum GearState {
    Park,
    Reverse,
    Neutral,
    Drive(u8),  // 1..8
}

impl Default for GearState {
    fn default() -> Self {
        GearState::Neutral
    }
}

/// Конфигурация отображения HUD
#[derive(Debug, Clone)]
pub struct HudLayout {
    pub show_speed: bool,
    pub show_gear: bool,
    pub show_fuel: bool,
    pub show_diff_status: bool,
    pub show_wheel_status: bool,
    pub show_cargo: bool,
    pub show_terrain_angle: bool,
    pub compact_mode: bool,   // Мини-версия для слабых экранов
    pub show_minimap: bool,   // Правый блок с картой
}

impl Default for HudLayout {
    fn default() -> Self {
        Self {
            show_speed: true,
            show_gear: true,
            show_fuel: true,
            show_diff_status: true,
            show_wheel_status: true,
            show_cargo: true,
            show_terrain_angle: true,
            compact_mode: false,
            show_minimap: true,
        }
    }
}

/// Единый менеджер HUD — единственное место где рисуется интерфейс
pub struct HudManager {
    visible: bool,
    opacity: f32,
    layout: HudLayout,
    last_data: Option<VehicleHudData>,
    // Анимационные состояния
    flash_timer: f32,
    flash_element: Option<HudFlashElement>,
}

#[derive(Debug, Clone)]
pub enum HudFlashElement {
    FuelReserve,
    EngineOverheat,
    WheelSlip(usize),  // index 0..3
}

impl HudManager {
    pub fn new() -> Self {
        Self {
            visible: true,
            opacity: 1.0,
            layout: HudLayout::default(),
            last_data: None,
            flash_timer: 0.0,
            flash_element: None,
        }
    }

    /// Обновить данные HUD
    pub fn update(&mut self, data: VehicleHudData, dt: f32) {
        // Проверка на мигающие элементы
        if data.fuel_reserve {
            self.flash_element = Some(HudFlashElement::FuelReserve);
            self.flash_timer = 0.5;  // мигать каждые 0.5 сек
        } else if data.engine_overheating {
            self.flash_element = Some(HudFlashElement::EngineOverheat);
            self.flash_timer = 0.3;  // быстрее мигать для перегрева
        } else {
            // Проверка проскальзывания колёс
            let mut slipping_wheel = None;
            for (i, &slip) in data.wheel_slip.iter().enumerate() {
                if slip > 0.5 {
                    slipping_wheel = Some(i);
                    break;
                }
            }
            
            if let Some(idx) = slipping_wheel {
                self.flash_element = Some(HudFlashElement::WheelSlip(idx));
                self.flash_timer = 0.2;
            } else {
                self.flash_element = None;
            }
        }

        // Обновление таймера мигания
        if self.flash_timer > 0.0 {
            self.flash_timer -= dt;
            if self.flash_timer <= 0.0 {
                self.flash_timer = 0.0;
                self.flash_element = None;
            }
        }

        self.last_data = Some(data);
    }

    /// Показать/скрыть HUD
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Установить прозрачность (0.0..1.0)
    pub fn set_opacity(&mut self, opacity: f32) {
        self.opacity = opacity.clamp(0.0, 1.0);
    }

    /// Получить текущие данные
    pub fn get_data(&self) -> Option<&VehicleHudData> {
        self.last_data.as_ref()
    }

    /// Получить конфигурацию отображения
    pub fn get_layout(&self) -> &HudLayout {
        &self.layout
    }

    /// Изменить конфигурацию отображения
    pub fn set_layout(&mut self, layout: HudLayout) {
        self.layout = layout;
    }

    /// Проверить, должен ли элемент мигать сейчас
    pub fn is_flashing(&self, element: &HudFlashElement) -> bool {
        if let Some(ref flash) = self.flash_element {
            if flash == element {
                // Мигать: включено половину времени
                return self.flash_timer > 0.25;
            }
        }
        false
    }

    /// Сгенерировать VehicleHudData из параметров автомобиля (helper)
    pub fn create_vehicle_data(
        speed_kmh: f32,
        rpm: f32,
        rpm_max: f32,
        gear: GearState,
        engine_running: bool,
        fuel: f32,
        temp: f32,
    ) -> VehicleHudData {
        VehicleHudData {
            speed_kmh,
            speed_max_kmh: 120.0,  // default for trucks
            gear,
            engine_rpm: rpm,
            engine_rpm_max: rpm_max,
            engine_running,
            fuel_level: fuel,
            fuel_reserve: fuel < 0.15,
            engine_temp: temp,
            engine_overheating: temp > 100.0,
            ..Default::default()
        }
    }
}

impl Default for HudManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hud_manager_creation() {
        let hud = HudManager::new();
        assert!(hud.is_visible());
        assert_eq!(hud.get_data(), None);
    }

    #[test]
    fn test_hud_update() {
        let mut hud = HudManager::new();
        let data = VehicleHudData {
            speed_kmh: 60.0,
            engine_rpm: 2000.0,
            gear: GearState::Drive(3),
            engine_running: true,
            fuel_level: 0.5,
            ..Default::default()
        };
        
        hud.update(data.clone(), 0.016);
        
        assert_eq!(hud.get_data().unwrap().speed_kmh, 60.0);
        assert_eq!(hud.get_data().unwrap().gear, GearState::Drive(3));
    }

    #[test]
    fn test_fuel_reserve_flash() {
        let mut hud = HudManager::new();
        let data = VehicleHudData {
            fuel_level: 0.1,  // ниже 15%
            ..Default::default()
        };
        
        hud.update(data, 0.016);
        assert!(hud.flash_element.is_some());
        assert_eq!(hud.flash_element.unwrap(), HudFlashElement::FuelReserve);
    }
}
