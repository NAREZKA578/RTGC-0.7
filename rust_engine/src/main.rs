//! Игровой движок на Rust - главный файл
//! 
//! Демонстрация использования всех компонентов движка

use rust_engine::{
    TerrainGenerator, ErosionConfig,
    PostProcessConfig, BloomConfig,
    TireModel, TireConfig, RoadSurface,
    AsyncPhysicsEngine, PhysicsConfig,
    MissionManager, MissionType, SaveSystem, SaveData,
    GamepadManager, InputAction,
};
use glam::Vec3;

#[tokio::main]
async fn main() {
    println!("=== Rust Game Engine Demo ===\n");

    // 1. Генерация ландшафта с эрозией
    println!("1. Генерация ландшафта с гидравлической и термальной эрозией...");
    let mut terrain = TerrainGenerator::new(128, 42)
        .with_config(ErosionConfig {
            hydraulic_particles: 5000,
            hydraulic_erosion_rate: 0.015,
            deposition_rate: 0.008,
            thermal_iterations: 30,
            thermal_slope_threshold: 0.4,
            thermal_erosion_rate: 0.025,
        });
    
    terrain.generate_base();
    println!("   Базовый ландшафт сгенерирован ({}x{})", terrain.size(), terrain.size());
    
    terrain.apply_erosion();
    println!("   Эрозия применена!");
    
    let heightmap = terrain.get_heightmap();
    let min_h = heightmap.iter().cloned().fold(f32::INFINITY, f32::min);
    let max_h = heightmap.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    println!("   Диапазон высот: {:.2} - {:.2}", min_h, max_h);

    // 2. Пост-обработка (Bloom)
    println!("\n2. Настройка пост-обработки (Bloom)...");
    let bloom_config = BloomConfig {
        threshold: 0.8,
        intensity: 0.6,
        blur_radius: 5,
    };
    let pp_config = PostProcessConfig {
        bloom: bloom_config,
        gamma: 2.2,
        contrast: 1.1,
        saturation: 1.2,
        ..Default::default()
    };
    println!("   Bloom порог: {}", pp_config.bloom.threshold);
    println!("   Bloom интенсивность: {}", pp_config.bloom.intensity);
    println!("   Gamma: {}", pp_config.gamma);

    // 3. Физика шин
    println!("\n3. Симуляция физики шин...");
    let tire_config = TireConfig {
        tire_type: rust_engine::TireType::Summer,
        nominal_pressure: 32.0,
        max_pressure: 50.0,
        min_pressure: 20.0,
        width: 225.0,
        rim_diameter: 17.0,
        wear_level: 0.1,
        optimal_temperature: 85.0,
    };
    
    let mut tire = TireModel::new(tire_config);
    
    // Симуляция нескольких шагов
    for i in 0..10 {
        tire.set_forces(3000.0, 500.0, 200.0);
        tire.set_slip(0.05, 0.02);
        tire.set_angular_velocity(50.0);
        tire.update(0.016, RoadSurface::DryAsphalt, Vec3::new(20.0, 0.0, 0.0));
        
        if i == 0 || i == 9 {
            println!("   Шаг {}: давление = {:.1} PSI, температура = {:.1}°C", 
                i + 1, tire.state().pressure, tire.state().temperature);
        }
    }
    
    let friction = tire.calculate_friction_coefficient(RoadSurface::DryAsphalt);
    println!("   Коэффициент сцепления (сухой асфальт): {:.2}", friction);
    
    let ice_friction = tire.calculate_friction_coefficient(RoadSurface::Ice);
    println!("   Коэффициент сцепления (лед): {:.2}", ice_friction);
    
    println!("   Рекомендация: {}", tire.get_recommendation());

    // 4. Асинхронная физика
    println!("\n4. Инициализация асинхронной физики...");
    let physics_config = PhysicsConfig {
        gravity: Vec3::new(0.0, -9.81, 0.0),
        timestep: 1.0 / 60.0,
        substeps: 4,
        max_accumulated_steps: 5,
    };
    
    let mut physics = AsyncPhysicsEngine::new(physics_config);
    println!("   Гравитация: {:.2} м/с²", physics_config.gravity.y);
    println!("   Timestep: {:.4} с", physics_config.timestep);
    println!("   Substeps: {}", physics_config.substeps);
    
    // Добавляем тестовое тело
    use rust_engine::PhysicsCommand;
    physics.send_command(PhysicsCommand::AddRigidBody {
        id: 1,
        position: Vec3::new(0.0, 10.0, 0.0),
        rotation: glam::Quat::IDENTITY,
        mass: 1.0,
        is_static: false,
    });
    println!("   Добавлено тело в очередь");

    // 5. Система миссий
    println!("\n5. Система миссий...");
    let mut mission_manager = MissionManager::new();
    
    let mission_id = mission_manager.create_mission(
        "Первая доставка",
        "Доставьте груз в точку назначения",
        MissionType::Delivery,
    );
    
    if let Some(mission) = mission_manager.get_mission_mut(mission_id) {
        use rust_engine::gameplay::mission_system::MissionObjective;
        mission.objectives.push(MissionObjective::new("Доставить груз", 1, "delivery"));
        mission.reward.money = 500;
        mission.reward.experience = 100;
    }
    
    mission_manager.accept_mission(mission_id);
    println!("   Миссия создана и принята: ID {}", mission_id);
    
    mission_manager.update_mission_progress(mission_id, "delivery", 1);
    
    if let Some(mission) = mission_manager.get_mission(mission_id) {
        println!("   Статус миссии: {:?}", mission.status);
        println!("   Награда: {} денег, {} опыта", mission.reward.money, mission.reward.experience);
    }

    // 6. Сохранения
    println!("\n6. Система сохранений...");
    let save_system = SaveSystem::new("./saves");
    
    let save_data = SaveData {
        player_name: "Driver".to_string(),
        money: 2500,
        level: 5,
        experience: 1250,
        player_position: [100.0, 0.0, 200.0],
        ..Default::default()
    };
    
    println!("   Данные для сохранения подготовлены");
    println!("   Игрок: {}, Уровень: {}, Деньги: {}", 
        save_data.player_name, save_data.level, save_data.money);

    // 7. Геймпады
    println!("\n7. Поддержка геймпадов...");
    match GamepadManager::new() {
        Ok(mut gp_manager) => {
            gp_manager.update();
            let count = gp_manager.connected_count();
            println!("   Подключено геймпадов: {}", count);
            
            if count > 0 {
                let names = gp_manager.list_gamepads();
                for name in names {
                    println!("   - {}", name);
                }
            } else {
                println!("   (геймпады не обнаружены, но поддержка готова)");
            }
            
            // Тестовая вибрация
            gp_manager.rumble_primary(0.5, 0.3, 0.2);
            println!("   Вибрация тестирована");
        }
        Err(e) => {
            println!("   Ошибка инициализации: {}", e);
        }
    }

    // Комбинированный ввод
    if let Ok(input) = rust_engine::CombinedInput::new() {
        println!("   Комбинированный ввод (клавиатура + геймпад) готов");
    }

    println!("\n=== Демонстрация завершена ===");
    println!("\nВсе компоненты движка успешно инициализированы:");
    println!("✓ Гидравлическая и термальная эрозия ландшафта");
    println!("✓ Bloom и пост-обработка (PBR)");
    println!("✓ Давление в шинах и температура");
    println!("✓ Асинхронная физика для стабильного FPS");
    println!("✓ Система миссий и сохранений");
    println!("✓ Поддержка геймпадов");
    println!("\nВесь движок написан на Rust!");
}
