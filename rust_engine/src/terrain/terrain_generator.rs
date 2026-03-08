//! Гидравлическая и термальная эрозия ландшафта
//! 
//! Реализует симуляцию эрозии:
//! - Гидравлическая эрозия (водой)
//! - Термальная эрозия (температурное выветривание)

use noise::{NoiseFn, Perlin};
use rand::Rng;
use glam::Vec3;

/// Параметры симуляции эрозии
#[derive(Clone, Debug)]
pub struct ErosionConfig {
    /// Количество частиц воды для гидравлической эрозии
    pub hydraulic_particles: u32,
    /// Скорость эрозии водой
    pub hydraulic_erosion_rate: f32,
    /// Скорость осаждения наносов
    pub deposition_rate: f32,
    /// Количество итераций термальной эрозии
    pub thermal_iterations: u32,
    /// Порог угла склона для термальной эрозии
    pub thermal_slope_threshold: f32,
    /// Скорость термальной эрозии
    pub thermal_erosion_rate: f32,
}

impl Default for ErosionConfig {
    fn default() -> Self {
        Self {
            hydraulic_particles: 10000,
            hydraulic_erosion_rate: 0.01,
            deposition_rate: 0.005,
            thermal_iterations: 50,
            thermal_slope_threshold: 0.5,
            thermal_erosion_rate: 0.02,
        }
    }
}

/// Частица воды для симуляции гидравлической эрозии
#[derive(Clone, Debug)]
struct WaterParticle {
    position: Vec3,
    direction: Vec3,
    speed: f32,
    water_amount: f32,
    sediment: f32,
}

/// Генератор ландшафта с поддержкой эрозии
pub struct TerrainGenerator {
    noise: Perlin,
    config: ErosionConfig,
    size: usize,
    heightmap: Vec<f32>,
}

impl TerrainGenerator {
    pub fn new(size: usize, seed: u32) -> Self {
        let mut noise = Perlin::new();
        noise.set_seed(seed as i32);
        
        Self {
            noise,
            config: ErosionConfig::default(),
            size,
            heightmap: vec![0.0; size * size],
        }
    }

    pub fn with_config(mut self, config: ErosionConfig) -> Self {
        self.config = config;
        self
    }

    /// Генерация базового ландшафта с использованием шума Перлина
    pub fn generate_base(&mut self) {
        let mut rng = rand::thread_rng();
        
        for y in 0..self.size {
            for x in 0..self.size {
                let nx = x as f32 / self.size as f32;
                let ny = y as f32 / self.size as f32;
                
                // Многослойный шум для детализации
                let mut height = 0.0;
                let mut amplitude = 1.0;
                let mut frequency = 1.0;
                
                for _ in 0..4 {
                    height += self.noise.get([nx * frequency, ny * frequency]) * amplitude;
                    amplitude *= 0.5;
                    frequency *= 2.0;
                }
                
                // Добавляем немного случайности
                height += rng.gen_range(-0.05..0.05);
                
                self.heightmap[y * self.size + x] = height.max(0.0);
            }
        }
    }

    /// Гидравлическая эрозия (симуляция потока воды)
    pub fn hydraulic_erosion(&mut self) {
        let mut heightmap = self.heightmap.clone();
        let mut rng = rand::thread_rng();
        
        for _ in 0..self.config.hydraulic_particles {
            // Случайная стартовая позиция
            let mut x = rng.gen_range(0..self.size);
            let mut y = rng.gen_range(0..self.size);
            
            let mut particle = WaterParticle {
                position: Vec3::new(x as f32, y as f32, self.get_height(x, y)),
                direction: Vec3::ZERO,
                speed: 0.0,
                water_amount: 1.0,
                sediment: 0.0,
            };
            
            // Симуляция пути частицы
            for _ in 0..100 {
                let (nx, ny) = (x as i32, y as i32);
                
                // Находим направление стока (самый низкий сосед)
                let mut lowest = self.get_height(x, y);
                let mut lowest_pos = (x, y);
                
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        
                        let check_x = ((nx + dx) % self.size as i32) as usize;
                        let check_y = ((ny + dy) % self.size as i32) as usize;
                        let h = self.get_height(check_x, check_y);
                        
                        if h < lowest {
                            lowest = h;
                            lowest_pos = (check_x, check_y);
                        }
                    }
                }
                
                // Если нет уклона, останавливаемся
                if lowest_pos == (x, y) {
                    break;
                }
                
                let (next_x, next_y) = lowest_pos;
                let delta_h = self.get_height(x, y) - lowest;
                
                // Эрозия в текущей точке
                let erosion_amount = (delta_h * particle.speed * self.config.hydraulic_erosion_rate)
                    .min(particle.water_amount);
                
                if erosion_amount > 0.0 {
                    let idx = y * self.size + x;
                    heightmap[idx] -= erosion_amount;
                    particle.sediment += erosion_amount;
                }
                
                // Осаждение
                let deposit_amount = particle.sediment * self.config.deposition_rate;
                if deposit_amount > 0.0 {
                    let idx = y * self.size + x;
                    heightmap[idx] += deposit_amount;
                    particle.sediment -= deposit_amount;
                }
                
                // Перемещаем частицу
                x = next_x;
                y = next_y;
                particle.position = Vec3::new(x as f32, y as f32, self.get_height(x, y));
                particle.speed = (particle.speed + delta_h * 0.1).min(5.0);
            }
            
            // Финальное осаждение
            let idx = y * self.size + x;
            heightmap[idx] += particle.sediment;
        }
        
        self.heightmap = heightmap;
    }

    /// Термальная эрозия (сглаживание крутых склонов)
    pub fn thermal_erosion(&mut self) {
        let mut heightmap = self.heightmap.clone();
        
        for _ in 0..self.config.thermal_iterations {
            for y in 0..self.size {
                for x in 0..self.size {
                    let h = self.get_height(x, y);
                    
                    // Проверяем всех соседей
                    for dy in [-1, 0, 1] {
                        for dx in [-1, 0, 1] {
                            if dx == 0 && dy == 0 {
                                continue;
                            }
                            
                            let nx = ((x as i32 + dx) % self.size as i32) as usize;
                            let ny = ((y as i32 + dy) % self.size as i32) as usize;
                            let neighbor_h = self.get_height(nx, ny);
                            
                            let slope = h - neighbor_h;
                            
                            // Если уклон превышает порог, переносим материал
                            if slope > self.config.thermal_slope_threshold {
                                let transfer = (slope - self.config.thermal_slope_threshold) 
                                    * self.config.thermal_erosion_rate * 0.5;
                                
                                let current_idx = y * self.size + x;
                                let neighbor_idx = ny * self.size + nx;
                                
                                heightmap[current_idx] -= transfer;
                                heightmap[neighbor_idx] += transfer;
                            }
                        }
                    }
                }
            }
            
            self.heightmap = heightmap.clone();
        }
    }

    /// Применяет все виды эрозии
    pub fn apply_erosion(&mut self) {
        self.hydraulic_erosion();
        self.thermal_erosion();
    }

    fn get_height(&self, x: usize, y: usize) -> f32 {
        self.heightmap[y * self.size + x]
    }

    pub fn get_heightmap(&self) -> &[f32] {
        &self.heightmap
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terrain_generation() {
        let mut generator = TerrainGenerator::new(64, 42);
        generator.generate_base();
        
        let heightmap = generator.get_heightmap();
        assert_eq!(heightmap.len(), 64 * 64);
        
        // Проверяем, что высоты положительные
        for &h in heightmap {
            assert!(h >= 0.0);
        }
    }

    #[test]
    fn test_erosion() {
        let mut generator = TerrainGenerator::new(32, 42);
        generator.generate_base();
        
        let before = generator.get_heightmap().to_vec();
        
        generator.apply_erosion();
        
        let after = generator.get_heightmap();
        
        // После эрозии ландшафт должен измениться
        assert_ne!(before, after);
    }
}
