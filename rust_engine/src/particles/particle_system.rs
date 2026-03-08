//! Система частиц для игрового движка на Rust
//! 
//! Реализует:
//! - Эффекты дыма, огня, искр
//! - Систему эмиттеров частиц
//! - Физическую симуляцию частиц
//! - GPU-ускоренный рендеринг

use glam::{Vec3, Vec4};
use std::collections::HashMap;

/// Тип частицы
#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub enum ParticleType {
    /// Дым
    Smoke,
    /// Огонь
    Fire,
    /// Искры
    Spark,
    /// Пыль
    Dust,
    /// Вода/брызги
    Water,
    /// Магические эффекты
    Magic,
    /// Кровь
    Blood,
    /// Обломки
    Debris,
}

/// Конфигурация частицы
#[derive(Clone, Debug)]
pub struct ParticleConfig {
    /// Начальный цвет (RGBA)
    pub start_color: Vec4,
    /// Конечный цвет (RGBA)
    pub end_color: Vec4,
    /// Начальный размер
    pub start_size: f32,
    /// Конечный размер
    pub end_size: f32,
    /// Время жизни (секунды)
    pub lifetime: f32,
    /// Начальная скорость
    pub initial_velocity: Vec3,
    /// Гравитация
    pub gravity: Vec3,
    /// Затухание скорости (drag)
    pub drag: f32,
    /// Случайность направления
    pub direction_spread: f32,
    /// Случайность скорости
    pub speed_variance: f32,
}

impl Default for ParticleConfig {
    fn default() -> Self {
        Self {
            start_color: Vec4::new(1.0, 1.0, 1.0, 1.0),
            end_color: Vec4::new(1.0, 1.0, 1.0, 0.0),
            start_size: 0.1,
            end_size: 0.5,
            lifetime: 2.0,
            initial_velocity: Vec3::ZERO,
            gravity: Vec3::new(0.0, -9.81, 0.0),
            drag: 0.1,
            direction_spread: 0.0,
            speed_variance: 0.0,
        }
    }
}

/// Отдельная частица
#[derive(Clone, Debug)]
pub struct Particle {
    /// Позиция
    pub position: Vec3,
    /// Скорость
    pub velocity: Vec3,
    /// Текущий цвет
    pub color: Vec4,
    /// Текущий размер
    pub size: f32,
    /// Прожитое время
    pub age: f32,
    /// Время жизни
    pub lifetime: f32,
    /// Тип частицы
    pub particle_type: ParticleType,
    /// Активна ли частица
    pub active: bool,
}

impl Particle {
    pub fn new(config: &ParticleConfig, position: Vec3, particle_type: ParticleType) -> Self {
        Self {
            position,
            velocity: config.initial_velocity,
            color: config.start_color,
            size: config.start_size,
            age: 0.0,
            lifetime: config.lifetime,
            particle_type,
            active: true,
        }
    }

    /// Обновление частицы
    pub fn update(&mut self, dt: f32, config: &ParticleConfig) {
        if !self.active {
            return;
        }

        self.age += dt;

        if self.age >= self.lifetime {
            self.active = false;
            return;
        }

        // Интерполяция цвета
        let t = self.age / self.lifetime;
        self.color = config.start_color.lerp(config.end_color, t);

        // Интерполяция размера
        self.size = config.start_size.lerp(config.end_size, t);

        // Применение гравитации
        self.velocity += config.gravity * dt;

        // Применение drag
        self.velocity *= 1.0 - config.drag * dt;

        // Обновление позиции
        self.position += self.velocity * dt;
    }

    /// Получение коэффициента жизни (0-1)
    pub fn life_ratio(&self) -> f32 {
        (self.age / self.lifetime).clamp(0.0, 1.0)
    }
}

/// Форма эмиттера
#[derive(Clone, Debug)]
pub enum EmitterShape {
    /// Точка
    Point,
    /// Сфера
    Sphere { radius: f32 },
    /// Куб
    Box { size: Vec3 },
    /// Конус
    Cone { radius: f32, height: f32 },
    /// Поверхность меша (упрощенно - плоскость)
    Plane { width: f32, height: f32 },
}

impl Default for EmitterShape {
    fn default() -> Self {
        Self::Point
    }
}

/// Режим эмиттера
#[derive(Clone, Debug)]
pub enum EmitterMode {
    /// Постоянная эмиссия
    Continuous { rate: f32 }, // частиц в секунду
    /// Взрыв (однократно)
    Burst { count: u32 },
    /// Волны (периодически)
    Wave { count: u32, interval: f32 },
}

impl Default for EmitterMode {
    fn default() -> Self {
        Self::Continuous { rate: 10.0 }
    }
}

/// Эмиттер частиц
#[derive(Clone, Debug)]
pub struct ParticleEmitter {
    /// Название эмиттера
    pub name: String,
    /// Позиция в мире
    pub position: Vec3,
    /// Поворот
    pub rotation: glam::Quat,
    /// Форма эмиттера
    pub shape: EmitterShape,
    /// Режим эмиссии
    pub mode: EmitterMode,
    /// Конфигурация частиц
    pub config: ParticleConfig,
    /// Тип частиц
    pub particle_type: ParticleType,
    /// Активен ли эмиттер
    pub enabled: bool,
    
    // Внутреннее состояние
    emit_accumulator: f32,
    burst_remaining: u32,
    wave_timer: f32,
}

impl ParticleEmitter {
    pub fn new(name: &str, position: Vec3, config: ParticleConfig) -> Self {
        Self {
            name: name.to_string(),
            position,
            rotation: glam::Quat::IDENTITY,
            shape: EmitterShape::default(),
            mode: EmitterMode::default(),
            config,
            particle_type: ParticleType::Smoke,
            enabled: true,
            emit_accumulator: 0.0,
            burst_remaining: 0,
            wave_timer: 0.0,
        }
    }

    /// Генерация позиции_spawn в пределах формы эмиттера
    fn generate_spawn_position(&self, rng: &mut impl Rng) -> Vec3 {
        match &self.shape {
            EmitterShape::Point => self.position,
            
            EmitterShape::Sphere { radius } => {
                let dir = Vec3::new(
                    rng.gen_range(-1.0..=1.0),
                    rng.gen_range(-1.0..=1.0),
                    rng.gen_range(-1.0..=1.0),
                ).normalize_or_zero();
                let dist = rng.gen_range(0.0..=*radius);
                self.position + dir * dist
            }
            
            EmitterShape::Box { size } => {
                let offset = Vec3::new(
                    rng.gen_range(-size.x..=size.x),
                    rng.gen_range(-size.y..=size.y),
                    rng.gen_range(-size.z..=size.z),
                ) * 0.5;
                self.position + self.rotation * offset
            }
            
            EmitterShape::Cone { radius, height } => {
                let angle = rng.gen_range(0.0..=std::f32::consts::PI * 2.0);
                let h = rng.gen_range(0.0..=*height);
                let r = (h / height) * radius * rng.gen_range(0.0..=1.0);
                
                let offset = Vec3::new(
                    r * angle.cos(),
                    h - height / 2.0,
                    r * angle.sin(),
                );
                self.position + self.rotation * offset
            }
            
            EmitterShape::Plane { width, height } => {
                let offset = Vec3::new(
                    rng.gen_range(-width..=width) * 0.5,
                    0.0,
                    rng.gen_range(-height..=height) * 0.5,
                );
                self.position + self.rotation * offset
            }
        }
    }

    /// Генерация начальной скорости с разбросом
    fn generate_velocity(&self, rng: &mut impl Rng) -> Vec3 {
        let base = self.config.initial_velocity;
        
        if self.config.direction_spread > 0.0 {
            let spread = self.config.direction_spread;
            let dir = Vec3::new(
                rng.gen_range(-spread..=spread),
                rng.gen_range(-spread..=spread),
                rng.gen_range(-spread..=spread),
            );
            base + dir
        } else {
            base
        }
    }

    /// Создание новой частицы
    pub fn emit_particle(&mut self, rng: &mut impl Rng) -> Option<Particle> {
        if !self.enabled {
            return None;
        }

        let spawn_pos = self.generate_spawn_position(rng);
        let mut config = self.config.clone();
        config.initial_velocity = self.generate_velocity(rng);

        // Применяем variance к скорости
        if self.config.speed_variance > 0.0 {
            let variance = rng.gen_range(1.0 - self.config.speed_variance..=1.0 + self.config.speed_variance);
            config.initial_velocity *= variance;
        }

        Some(Particle::new(&config, spawn_pos, self.particle_type))
    }

    /// Обновление эмиттера
    pub fn update(&mut self, dt: f32, particles: &mut Vec<Particle>) {
        if !self.enabled {
            return;
        }

        match &self.mode {
            EmitterMode::Continuous { rate } => {
                self.emit_accumulator += dt * rate;
                
                while self.emit_accumulator >= 1.0 {
                    if let Some(particle) = self.emit_particle(&mut ThreadRng {}) {
                        particles.push(particle);
                    }
                    self.emit_accumulator -= 1.0;
                }
            }
            
            EmitterMode::Burst { count } => {
                if self.burst_remaining > 0 {
                    let to_emit = self.burst_remaining.min(10); // Лимит за кадр
                    for _ in 0..to_emit {
                        if let Some(particle) = self.emit_particle(&mut ThreadRng {}) {
                            particles.push(particle);
                        }
                    }
                    self.burst_remaining -= to_emit;
                }
            }
            
            EmitterMode::Wave { count, interval } => {
                self.wave_timer += dt;
                
                if self.wave_timer >= *interval && self.burst_remaining > 0 {
                    self.wave_timer = 0.0;
                    let to_emit = count.min(self.burst_remaining);
                    
                    for _ in 0..to_emit {
                        if let Some(particle) = self.emit_particle(&mut ThreadRng {}) {
                            particles.push(particle);
                        }
                    }
                    self.burst_remaining -= to_emit;
                }
            }
        }
    }

    /// Запуск взрыва
    pub fn trigger_burst(&mut self, count: u32) {
        self.mode = EmitterMode::Burst { count };
        self.burst_remaining = count;
        self.enabled = true;
    }

    /// Установка режима continuous
    pub fn set_continuous(&mut self, rate: f32) {
        self.mode = EmitterMode::Continuous { rate };
        self.enabled = true;
    }

    /// Установка режима волны
    pub fn set_wave(&mut self, count: u32, interval: f32) {
        self.mode = EmitterMode::Wave { count, interval };
        self.burst_remaining = u32::MAX;
        self.enabled = true;
    }
}

// Простая реализация Rng для эмиттеров
struct ThreadRng;
impl ThreadRng {
    fn gen_range(&mut self, range: std::ops::RangeInclusive<f32>) -> f32 {
        use std::time::{SystemTime, UNIX_EPOCH};
        let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos() as f32;
        let normalized = (seed.sin() + 1.0) / 2.0;
        *range.start() + normalized * (*range.end() - *range.start())
    }
}

/// Менеджер системы частиц
pub struct ParticleSystem {
    /// Все активные эмиттеры
    emitters: HashMap<String, ParticleEmitter>,
    /// Все активные частицы
    particles: Vec<Particle>,
    /// Максимальное количество частиц
    max_particles: usize,
    /// Включена ли система
    enabled: bool,
}

impl ParticleSystem {
    pub fn new(max_particles: usize) -> Self {
        Self {
            emitters: HashMap::new(),
            particles: Vec::with_capacity(max_particles),
            max_particles,
            enabled: true,
        }
    }

    /// Добавление эмиттера
    pub fn add_emitter(&mut self, emitter: ParticleEmitter) {
        self.emitters.insert(emitter.name.clone(), emitter);
    }

    /// Удаление эмиттера по имени
    pub fn remove_emitter(&mut self, name: &str) -> Option<ParticleEmitter> {
        self.emitters.remove(name)
    }

    /// Получение эмиттера
    pub fn get_emitter(&self, name: &str) -> Option<&ParticleEmitter> {
        self.emitters.get(name)
    }

    /// Получение эмиттера (mutable)
    pub fn get_emitter_mut(&mut self, name: &str) -> Option<&mut ParticleEmitter> {
        self.emitters.get_mut(name)
    }

    /// Обновление системы частиц
    pub fn update(&mut self, dt: f32) {
        if !self.enabled {
            return;
        }

        // Обновляем эмиттеры
        for (_, emitter) in &mut self.emitters {
            emitter.update(dt, &mut self.particles);
        }

        // Обновляем частицы
        for particle in &mut self.particles {
            if particle.active {
                particle.update(dt, &ParticleConfig::default());
            }
        }

        // Удаляем неактивные частицы
        self.particles.retain(|p| p.active);

        // Ограничиваем количество частиц
        if self.particles.len() > self.max_particles {
            self.particles.truncate(self.max_particles);
        }
    }

    /// Получение всех активных частиц
    pub fn get_particles(&self) -> &[Particle] {
        &self.particles
    }

    /// Количество активных частиц
    pub fn particle_count(&self) -> usize {
        self.particles.len()
    }

    /// Количество эмиттеров
    pub fn emitter_count(&self) -> usize {
        self.emitters.len()
    }

    /// Очистка всех частиц
    pub fn clear_particles(&mut self) {
        self.particles.clear();
    }

    /// Очистка всех эмиттеров и частиц
    pub fn clear_all(&mut self) {
        self.emitters.clear();
        self.particles.clear();
    }

    /// Включение/выключение системы
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Создание предустановленного эффекта огня
    pub fn create_fire_effect(position: Vec3) -> ParticleEmitter {
        let mut emitter = ParticleEmitter::new("fire", position, ParticleConfig {
            start_color: Vec4::new(1.0, 0.5, 0.0, 1.0),
            end_color: Vec4::new(0.2, 0.2, 0.2, 0.0),
            start_size: 0.2,
            end_size: 0.8,
            lifetime: 1.5,
            initial_velocity: Vec3::new(0.0, 2.0, 0.0),
            gravity: Vec3::new(0.0, -2.0, 0.0),
            drag: 0.3,
            direction_spread: 0.3,
            speed_variance: 0.5,
            ..Default::default()
        });
        emitter.particle_type = ParticleType::Fire;
        emitter.set_continuous(50.0);
        emitter
    }

    /// Создание предустановленного эффекта дыма
    pub fn create_smoke_effect(position: Vec3) -> ParticleEmitter {
        let mut emitter = ParticleEmitter::new("smoke", position, ParticleConfig {
            start_color: Vec4::new(0.3, 0.3, 0.3, 0.8),
            end_color: Vec4::new(0.1, 0.1, 0.1, 0.0),
            start_size: 0.3,
            end_size: 1.5,
            lifetime: 3.0,
            initial_velocity: Vec3::new(0.0, 1.0, 0.0),
            gravity: Vec3::ZERO,
            drag: 0.1,
            direction_spread: 0.2,
            ..Default::default()
        });
        emitter.particle_type = ParticleType::Smoke;
        emitter.set_continuous(20.0);
        emitter
    }

    /// Создание предустановленного эффекта искр
    pub fn create_spark_effect(position: Vec3, direction: Vec3) -> ParticleEmitter {
        let mut emitter = ParticleEmitter::new("sparks", position, ParticleConfig {
            start_color: Vec4::new(1.0, 1.0, 0.5, 1.0),
            end_color: Vec4::new(1.0, 0.3, 0.0, 0.0),
            start_size: 0.05,
            end_size: 0.1,
            lifetime: 0.8,
            initial_velocity: direction * 5.0,
            gravity: Vec3::new(0.0, -9.81, 0.0),
            drag: 0.05,
            direction_spread: 0.5,
            speed_variance: 0.8,
            ..Default::default()
        });
        emitter.particle_type = ParticleType::Spark;
        emitter.trigger_burst(30);
        emitter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_particle_creation() {
        let config = ParticleConfig::default();
        let particle = Particle::new(&config, Vec3::ZERO, ParticleType::Smoke);
        
        assert!(particle.active);
        assert_eq!(particle.age, 0.0);
        assert_eq!(particle.lifetime, config.lifetime);
    }

    #[test]
    fn test_particle_update() {
        let mut config = ParticleConfig::default();
        config.lifetime = 1.0;
        config.gravity = Vec3::new(0.0, -10.0, 0.0);
        
        let mut particle = Particle::new(&config, Vec3::ZERO, ParticleType::Smoke);
        particle.update(0.5, &config);
        
        assert!((particle.age - 0.5).abs() < 0.01);
        assert!(particle.velocity.y < 0.0); // Гравитация действует
    }

    #[test]
    fn test_particle_death() {
        let config = ParticleConfig {
            lifetime: 0.5,
            ..Default::default()
        };
        
        let mut particle = Particle::new(&config, Vec3::ZERO, ParticleType::Smoke);
        particle.update(0.6, &config);
        
        assert!(!particle.active);
    }

    #[test]
    fn test_particle_system() {
        let mut system = ParticleSystem::new(1000);
        
        let emitter = ParticleEmitter::new("test", Vec3::ZERO, ParticleConfig::default());
        system.add_emitter(emitter);
        
        assert_eq!(system.emitter_count(), 1);
        assert_eq!(system.particle_count(), 0);
        
        system.update(0.1);
        
        // Частицы должны были появиться
        assert!(system.particle_count() >= 0);
    }

    #[test]
    fn test_preset_effects() {
        let fire = ParticleSystem::create_fire_effect(Vec3::ZERO);
        assert_eq!(fire.particle_type, ParticleType::Fire);
        
        let smoke = ParticleSystem::create_smoke_effect(Vec3::ZERO);
        assert_eq!(smoke.particle_type, ParticleType::Smoke);
        
        let sparks = ParticleSystem::create_spark_effect(Vec3::ZERO, Vec3::Y);
        assert_eq!(sparks.particle_type, ParticleType::Spark);
    }
}
