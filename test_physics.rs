use nalgebra::Vector3;
use rtgc_siberian_cities::physics::{PhysicsWorld, RigidBody, Shape};

fn main() {
    // Создаем физический мир
    let mut physics_world = PhysicsWorld::new();
    
    // Создаем статический пол (плоскость на y=0)
    let ground = RigidBody::new_box(
        Vector3::new(0.0, -1.0, 0.0),  // позиция
        0.0,                            // масса (статический объект)
        Vector3::new(10.0, 1.0, 10.0)  // размеры (ширина, высота, глубина)
    );
    physics_world.add_body(ground);
    
    // Создаем несколько динамических сфер для симуляции
    for i in 0..5 {
        let sphere = RigidBody::new_sphere(
            Vector3::new(i as f32 * 2.0, 10.0 + i as f32 * 2.0, 0.0), // начальная позиция
            1.0,                                                         // масса
            0.5                                                          // радиус
        );
        physics_world.add_body(sphere);
    }
    
    // Запускаем симуляцию
    println!("Начало симуляции физики...");
    for step in 0..300 { // 300 шагов симуляции (примерно 5 секунд при 60 FPS)
        physics_world.step();
        
        // Выводим состояние первых нескольких тел каждые 60 шагов
        if step % 60 == 0 {
            println!("Шаг {}: ", step);
            for (i, body) in physics_world.rigid_bodies.iter().enumerate().take(6) {
                println!("  Тело {}: позиция = {:?}, скорость = {:?}", 
                         i, 
                         body.position, 
                         body.velocity);
            }
        }
    }
    
    println!("Симуляция завершена.");
    
    // Проверяем финальное состояние
    println!("\nФинальные позиции:");
    for (i, body) in physics_world.rigid_bodies.iter().enumerate() {
        println!("Тело {}: позиция = {:?}", i, body.position);
    }
}