//! Система давления в шинах и температуры
//! 
//! Реализует физику шин с учетом:
//! - Давления воздуха в шине
//! - Температуры шины (нагрев от трения)
//! - Износа протектора
//! - Влияния на сцепление с дорогой

use glam::Vec3;

/// Тип шины
#[derive(Clone, Debug, Copy, PartialEq)]
pub enum TireType {
    /// Летняя шина
    Summer,
    /// Зимняя шина
    Winter,
    /// Всесезонная шина
    AllSeason,
    /// Гоночная слик-шина
    Slick,
}

/// Состояние поверхности дороги
#[derive(Clone, Debug, Copy, PartialEq)]
pub enum RoadSurface {
    /// Сухой асфальт
    DryAsphalt,
    /// Мокрый асфальт
    WetAsphalt,
    /// Лед
    Ice,
    /// Снег
    Snow,
    /// Гравий
    Gravel,
    /// Грязь
    Dirt,
}

impl RoadSurface {
    /// Базовый коэффициент трения для поверхности
    pub fn base_friction(&self) -> f32 {
        match self {
            RoadSurface::DryAsphalt => 1.0,
            RoadSurface::WetAsphalt => 0.7,
            RoadSurface::Ice => 0.15,
            RoadSurface::Snow => 0.3,
            RoadSurface::Gravel => 0.5,
            RoadSurface::Dirt => 0.6,
        }
    }
}

/// Параметры шины
#[derive(Clone, Debug)]
pub struct TireConfig {
    /// Тип шины
    pub tire_type: TireType,
    /// Номинальное давление (PSI)
    pub nominal_pressure: f32,
    /// Максимальное давление (PSI)
    pub max_pressure: f32,
    /// Минимальное давление (PSI)
    pub min_pressure: f32,
    /// Ширина шины (мм)
    pub width: f32,
    /// Диаметр обода (дюймы)
    pub rim_diameter: f32,
    /// Процент износа протектора (0 = новая, 1 = полностью изношена)
    pub wear_level: f32,
    /// Оптимальная рабочая температура (°C)
    pub optimal_temperature: f32,
}

impl Default for TireConfig {
    fn default() -> Self {
        Self {
            tire_type: TireType::AllSeason,
            nominal_pressure: 32.0,
            max_pressure: 50.0,
            min_pressure: 20.0,
            width: 225.0,
            rim_diameter: 17.0,
            wear_level: 0.0,
            optimal_temperature: 80.0,
        }
    }
}

/// Состояние шины в реальном времени
#[derive(Clone, Debug)]
pub struct TireState {
    /// Текущее давление (PSI)
    pub pressure: f32,
    /// Текущая температура (°C)
    pub temperature: f32,
    /// Температура окружающей среды (°C)
    pub ambient_temperature: f32,
    /// Вертикальная нагрузка (N)
    pub vertical_load: f32,
    /// Продольная сила (N)
    pub longitudinal_force: f32,
    /// Боковая сила (N)
    pub lateral_force: f32,
    /// Угол скольжения (радианы)
    pub slip_angle: f32,
    /// Продольное проскальзывание (0-1)
    pub longitudinal_slip: f32,
    /// Угловая скорость колеса (рад/с)
    pub angular_velocity: f32,
    /// Эффективный радиус качения (м)
    pub effective_radius: f32,
}

impl Default for TireState {
    fn default() -> Self {
        Self {
            pressure: 32.0,
            temperature: 20.0,
            ambient_temperature: 20.0,
            vertical_load: 0.0,
            longitudinal_force: 0.0,
            lateral_force: 0.0,
            slip_angle: 0.0,
            longitudinal_slip: 0.0,
            angular_velocity: 0.0,
            effective_radius: 0.3,
        }
    }
}

/// Модель шины с физикой давления и температуры
pub struct TireModel {
    config: TireConfig,
    state: TireState,
    /// Коэффициент теплопередачи
    heat_transfer_coefficient: f32,
    /// Теплоемкость шины
    heat_capacity: f32,
}

impl TireModel {
    pub fn new(config: TireConfig) -> Self {
        let mut state = TireState::default();
        state.pressure = config.nominal_pressure;
        
        Self {
            config,
            state,
            heat_transfer_coefficient: 50.0, // W/(m²·K)
            heat_capacity: 2000.0, // J/K
        }
    }

    /// Обновление состояния шины
    /// 
    /// # Arguments
    /// * `dt` - Время шага симуляции (секунды)
    /// * `surface` - Тип поверхности дороги
    /// * `velocity` - Скорость автомобиля (м/с)
    pub fn update(&mut self, dt: f32, surface: RoadSurface, velocity: Vec3) {
        // Расчет нагрева от трения
        self.calculate_heat_generation(dt, surface);
        
        // Расчет охлаждения
        self.calculate_cooling(dt, velocity);
        
        // Обновление давления от температуры (закон Гей-Люссака)
        self.update_pressure_from_temperature();
        
        // Обновление эффективного радиуса от давления
        self.update_effective_radius();
    }

    /// Расчет тепловыделения от трения
    fn calculate_heat_generation(&mut self, dt: f32, surface: RoadSurface) {
        // Мощность трения = сила трения × скорость скольжения
        let friction_force = (self.state.longitudinal_force.powi(2) + self.state.lateral_force.powi(2)).sqrt();
        let slip_speed = velocity_magnitude(self.state.slip_angle, self.state.longitudinal_slip, self.state.angular_velocity);
        
        // Тепловыделение (Вт)
        let heat_power = friction_force * slip_speed * surface.base_friction();
        
        // Повышение температуры
        let delta_temp = (heat_power * dt) / self.heat_capacity;
        self.state.temperature += delta_temp;
    }

    /// Расчет охлаждения (конвекция + излучение)
    fn calculate_cooling(&mut self, dt: f32, velocity: Vec3) {
        // Скорость воздушного потока влияет на охлаждение
        let air_speed = velocity.length();
        
        // Коэффициент конвекции увеличивается со скоростью
        let convective_coefficient = self.heat_transfer_coefficient * (1.0 + air_speed * 0.1);
        
        // Разница температур
        let temp_diff = self.state.temperature - self.state.ambient_temperature;
        
        // Охлаждение (закон охлаждения Ньютона)
        let cooling_power = convective_coefficient * temp_diff;
        
        // Понижение температуры
        let delta_temp = (cooling_power * dt) / self.heat_capacity;
        self.state.temperature -= delta_temp.min(self.state.temperature - self.state.ambient_temperature);
    }

    /// Обновление давления от температуры
    fn update_pressure_from_temperature(&mut self) {
        // P1/T1 = P2/T2 (температура в Кельвинах)
        let temp_kelvin_initial = self.state.ambient_temperature + 273.15;
        let temp_kelvin_current = self.state.temperature + 273.15;
        
        let reference_pressure = self.config.nominal_pressure;
        let new_pressure = reference_pressure * (temp_kelvin_current / temp_kelvin_initial);
        
        // Ограничиваем давление допустимыми пределами
        self.state.pressure = new_pressure.clamp(
            self.config.min_pressure,
            self.config.max_pressure,
        );
    }

    /// Обновление эффективного радиуса качения
    fn update_effective_radius(&mut self) {
        // Радиус уменьшается при снижении давления
        let pressure_ratio = self.state.pressure / self.config.nominal_pressure;
        
        // Базовый радиус (примерно половина диаметра обода + профиль)
        let base_radius = (self.config.rim_diameter * 0.0254) / 2.0 + (self.config.width * 0.001 * 0.5);
        
        // Эффективный радиус зависит от давления и нагрузки
        self.state.effective_radius = base_radius * (0.9 + 0.1 * pressure_ratio);
        
        // Учет нагрузки (шина сплющивается)
        let load_factor = 1.0 - (self.state.vertical_load / 10000.0).min(0.1);
        self.state.effective_radius *= load_factor;
    }

    /// Расчет коэффициента сцепления
    pub fn calculate_friction_coefficient(&self, surface: RoadSurface) -> f32 {
        let mut friction = surface.base_friction();
        
        // Влияние типа шины
        let tire_factor = match (self.config.tire_type, surface) {
            (TireType::Summer, RoadSurface::Ice) => 0.5,
            (TireType::Summer, RoadSurface::Snow) => 0.6,
            (TireType::Winter, RoadSurface::Ice) => 1.2,
            (TireType::Winter, RoadSurface::Snow) => 1.3,
            (TireType::Slick, RoadSurface::DryAsphalt) => 1.5,
            _ => 1.0,
        };
        friction *= tire_factor;
        
        // Влияние температуры (оптимальная температура дает максимальное сцепление)
        let temp_efficiency = if self.state.temperature < self.config.optimal_temperature {
            0.7 + 0.3 * (self.state.temperature / self.config.optimal_temperature)
        } else {
            1.0 - 0.2 * ((self.state.temperature - self.config.optimal_temperature) / 50.0).min(1.0)
        };
        friction *= temp_efficiency;
        
        // Влияние давления (отклонение от номинального снижает сцепление)
        let pressure_ratio = self.state.pressure / self.config.nominal_pressure;
        let pressure_efficiency = 1.0 - (pressure_ratio - 1.0).abs() * 0.3;
        friction *= pressure_efficiency;
        
        // Влияние износа
        let wear_efficiency = 1.0 - self.config.wear_level * 0.2;
        friction *= wear_efficiency;
        
        friction
    }

    /// Получить текущее состояние
    pub fn state(&self) -> &TireState {
        &self.state
    }

    /// Установить внешние силы
    pub fn set_forces(&mut self, vertical: f32, longitudinal: f32, lateral: f32) {
        self.state.vertical_load = vertical;
        self.state.longitudinal_force = longitudinal;
        self.state.lateral_force = lateral;
    }

    /// Установить параметры скольжения
    pub fn set_slip(&mut self, slip_angle: f32, longitudinal_slip: f32) {
        self.state.slip_angle = slip_angle;
        self.state.longitudinal_slip = longitudinal_slip;
    }

    /// Установить угловую скорость
    pub fn set_angular_velocity(&mut self, omega: f32) {
        self.state.angular_velocity = omega;
    }

    /// Проверка критического состояния
    pub fn is_critical(&self) -> bool {
        self.state.pressure < self.config.min_pressure * 1.1
            || self.state.temperature > 120.0
            || self.config.wear_level > 0.9
    }

    /// Рекомендуемое действие
    pub fn get_recommendation(&self) -> &'static str {
        if self.state.pressure < self.config.min_pressure {
            "Критически низкое давление! Немедленно остановитесь."
        } else if self.state.pressure < self.config.nominal_pressure * 0.9 {
            "Рекомендуется подкачать шины."
        } else if self.state.temperature > 100.0 {
            "Высокая температура шин. Снизьте скорость."
        } else if self.config.wear_level > 0.8 {
            "Шины сильно изношены. Рекомендуется замена."
        } else {
            "Все в норме."
        }
    }
}

fn velocity_magnitude(slip_angle: f32, longitudinal_slip: f32, angular_velocity: f32) -> f32 {
    // Упрощенный расчет скорости скольжения
    slip_angle.abs() * 10.0 + longitudinal_slip * 5.0 + angular_velocity * 0.01
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tire_creation() {
        let config = TireConfig::default();
        let tire = TireModel::new(config.clone());
        
        assert_eq!(tire.state().pressure, config.nominal_pressure);
        assert_eq!(tire.state().temperature, 20.0);
    }

    #[test]
    fn test_pressure_increase_with_heat() {
        let mut tire = TireModel::new(TireConfig::default());
        
        // Нагреваем шину
        tire.state.temperature = 60.0;
        tire.update_pressure_from_temperature();
        
        // Давление должно увеличиться
        assert!(tire.state.pressure > 32.0);
    }

    #[test]
    fn test_friction_on_different_surfaces() {
        let tire = TireModel::new(TireConfig::default());
        
        let dry_friction = tire.calculate_friction_coefficient(RoadSurface::DryAsphalt);
        let ice_friction = tire.calculate_friction_coefficient(RoadSurface::Ice);
        
        assert!(dry_friction > ice_friction);
    }

    #[test]
    fn test_winter_tire_on_ice() {
        let mut config = TireConfig::default();
        config.tire_type = TireType::Winter;
        
        let winter_tire = TireModel::new(config);
        let summer_tire = TireModel::new(TireConfig::default());
        
        let winter_friction = winter_tire.calculate_friction_coefficient(RoadSurface::Ice);
        let summer_friction = summer_tire.calculate_friction_coefficient(RoadSurface::Ice);
        
        assert!(winter_friction > summer_friction);
    }
}
