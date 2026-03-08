//! Асинхронная физика для стабильного FPS
//! 
//! Реализует физику в отдельном потоке с фиксированным шагом времени
//! для обеспечения стабильной частоты кадров независимо от сложности симуляции.

use std::sync::{Arc, Mutex};
use tokio::task;
use rapier3d::prelude::*;

/// Конфигурация физического мира
#[derive(Clone, Debug)]
pub struct PhysicsConfig {
    /// Гравитация (м/с²)
    pub gravity: Vec3,
    /// Фиксированный шаг времени (секунды)
    pub timestep: f32,
    /// Количество субшагов для стабильности
    pub substeps: usize,
    /// Максимальное количество накопленных шагов
    pub max_accumulated_steps: usize,
}

impl Default for PhysicsConfig {
    fn default() -> Self {
        Self {
            gravity: Vec3::new(0.0, -9.81, 0.0),
            timestep: 1.0 / 60.0,
            substeps: 4,
            max_accumulated_steps: 5,
        }
    }
}

/// Данные состояния физического объекта
#[derive(Clone, Debug)]
pub struct RigidBodyState {
    pub position: Vec3,
    pub rotation: Quat,
    pub linear_velocity: Vec3,
    pub angular_velocity: Vec3,
}

/// Команда для физического мира
#[derive(Clone, Debug)]
pub enum PhysicsCommand {
    /// Добавить тело
    AddRigidBody {
        id: u32,
        position: Vec3,
        rotation: Quat,
        mass: f32,
        is_static: bool,
    },
    /// Удалить тело
    RemoveRigidBody(u32),
    /// Применить силу
    ApplyForce {
        id: u32,
        force: Vec3,
        point: Option<Vec3>,
    },
    /// Установить скорость
    SetVelocity {
        id: u32,
        linear: Vec3,
        angular: Vec3,
    },
    /// Обновить трансформацию
    SetTransform {
        id: u32,
        position: Vec3,
        rotation: Quat,
    },
}

/// Результат шага физики
#[derive(Clone, Debug, Default)]
pub struct PhysicsStepResult {
    pub states: Vec<(u32, RigidBodyState)>,
    pub collisions: Vec<CollisionEvent>,
}

/// Асинхронный физический движок
pub struct AsyncPhysicsEngine {
    config: PhysicsConfig,
    
    // Физический мир Rapier
    world: Option<PhysicsPipeline>,
    islands: Option<IslandManager>,
    bodies: Option<RigidBodySet>,
    colliders: Option<ColliderSet>,
    gravity: Option<IntegrationParameters>,
    
    // Каналы для коммуникации
    command_queue: Arc<Mutex<Vec<PhysicsCommand>>>,
    state_buffer: Arc<Mutex<PhysicsStepResult>>,
    
    // Состояние
    accumulated_time: f32,
    is_running: bool,
}

impl AsyncPhysicsEngine {
    /// Создание нового асинхронного физического движка
    pub fn new(config: PhysicsConfig) -> Self {
        let mut params = IntegrationParameters::default();
        params.dt = config.timestep / config.substeps as f32;
        
        Self {
            config,
            world: Some(PhysicsPipeline::new()),
            islands: Some(IslandManager::new()),
            bodies: Some(RigidBodySet::new()),
            colliders: Some(ColliderSet::new()),
            gravity: Some(params),
            command_queue: Arc::new(Mutex::new(Vec::new())),
            state_buffer: Arc::new(Mutex::new(PhysicsStepResult::default())),
            accumulated_time: 0.0,
            is_running: false,
        }
    }

    /// Запуск асинхронного цикла физики
    pub async fn run_async(&mut self) {
        self.is_running = true;
        
        while self.is_running {
            // Обработка команд
            self.process_commands();
            
            // Шаг симуляции
            self.step();
            
            // Небольшая задержка для предотвращения 100% загрузки CPU
            tokio::time::sleep(tokio::time::Duration::from_micros(100)).await;
        }
    }

    /// Один шаг симуляции
    pub fn step(&mut self) {
        self.accumulated_time += self.config.timestep;
        
        // Ограничиваем накопленное время
        let max_step_time = self.config.timestep * self.config.max_accumulated_steps as f32;
        if self.accumulated_time > max_step_time {
            self.accumulated_time = max_step_time;
        }
        
        // Выполняем субшаги
        while self.accumulated_time >= self.config.timestep / self.config.substeps as f32 {
            self.physics_substep();
            self.accumulated_time -= self.config.timestep / self.config.substeps as f32;
        }
        
        // Сбор состояния тел
        self.collect_states();
    }

    /// Субшаг физики
    fn physics_substep(&mut self) {
        if let (Some(pipeline), Some(islands), Some(bodies), Some(colliders), Some(params)) = (
            &mut self.world,
            &mut self.islands,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.gravity,
        ) {
            pipeline.step(
                &Vec3::y() * -self.config.gravity.y,
                params,
                islands,
                bodies,
                colliders,
                &mut (),
                &(),
            );
        }
    }

    /// Обработка очереди команд
    fn process_commands(&mut self) {
        let commands = {
            let mut queue = self.command_queue.lock().unwrap();
            std::mem::take(&mut *queue)
        };
        
        for command in commands {
            self.execute_command(command);
        }
    }

    /// Выполнение отдельной команды
    fn execute_command(&mut self, command: PhysicsCommand) {
        let bodies = self.bodies.as_mut().unwrap();
        let colliders = self.colliders.as_mut().unwrap();
        
        match command {
            PhysicsCommand::AddRigidBody { id, position, rotation, mass, is_static } => {
                let body_type = if is_static {
                    RigidBodyType::Fixed
                } else {
                    RigidBodyType::Dynamic
                };
                
                let rigid_body = RigidBodyBuilder::new(body_type)
                    .translation(vector![position.x, position.y, position.z])
                    .rotation(quaternion![rotation.w, rotation.x, rotation.y, rotation.z]);
                
                let handle = bodies.insert(rigid_body.build());
                
                // Добавляем коллайдер
                if !is_static {
                    let collider = ColliderBuilder::ball(0.5).mass(mass);
                    colliders.insert_with_parent(collider.build(), handle, bodies);
                }
            }
            
            PhysicsCommand::RemoveRigidBody(id) => {
                // В реальной реализации нужно сопоставить id с handle
                // Здесь упрощенно
            }
            
            PhysicsCommand::ApplyForce { id, force, point } => {
                // Применение силы к телу
            }
            
            PhysicsCommand::SetVelocity { id, linear, angular } => {
                // Установка скорости
            }
            
            PhysicsCommand::SetTransform { id, position, rotation } => {
                // Установка трансформации
            }
        }
    }

    /// Сбор состояний всех тел
    fn collect_states(&mut self) {
        let mut result = PhysicsStepResult::default();
        
        if let Some(bodies) = &self.bodies {
            for (handle, body) in bodies.iter() {
                let pos = body.position();
                let state = RigidBodyState {
                    position: Vec3::new(pos.translation.x, pos.translation.y, pos.translation.z),
                    rotation: Quat::from_array([pos.rotation.i, pos.rotation.j, pos.rotation.k, pos.rotation.scalar]),
                    linear_velocity: Vec3::new(body.linvel().x, body.linvel().y, body.linvel().z),
                    angular_velocity: Vec3::new(body.angvel().x, body.angvel().y, body.angvel().z),
                };
                
                result.states.push((handle.index() as u32, state));
            }
        }
        
        // Сохраняем результат в буфер
        if let Ok(mut buffer) = self.state_buffer.lock() {
            *buffer = result;
        }
    }

    /// Добавление команды в очередь
    pub fn send_command(&self, command: PhysicsCommand) {
        if let Ok(mut queue) = self.command_queue.lock() {
            queue.push(command);
        }
    }

    /// Получение последнего состояния физики
    pub fn get_state(&self) -> PhysicsStepResult {
        self.state_buffer.lock().unwrap().clone()
    }

    /// Получение состояния конкретного тела
    pub fn get_body_state(&self, id: u32) -> Option<RigidBodyState> {
        let state = self.get_state();
        state.states.iter()
            .find(|(body_id, _)| *body_id == id)
            .map(|(_, s)| s.clone())
    }

    /// Остановка физического движка
    pub fn stop(&mut self) {
        self.is_running = false;
    }

    /// Проверка работы движка
    pub fn is_running(&self) -> bool {
        self.is_running
    }
}

/// Менеджер физической сцены
pub struct PhysicsSceneManager {
    engine: AsyncPhysicsEngine,
    object_mapping: std::collections::HashMap<u32, u32>,
    next_id: u32,
}

impl PhysicsSceneManager {
    pub fn new(config: PhysicsConfig) -> Self {
        Self {
            engine: AsyncPhysicsEngine::new(config),
            object_mapping: std::collections::HashMap::new(),
            next_id: 0,
        }
    }

    /// Регистрация объекта в физической сцене
    pub fn register_object(&mut self, position: Vec3, rotation: Quat, mass: f32, is_static: bool) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        
        self.engine.send_command(PhysicsCommand::AddRigidBody {
            id,
            position,
            rotation,
            mass,
            is_static,
        });
        
        self.object_mapping.insert(id, id);
        id
    }

    /// Применение силы к объекту
    pub fn apply_force(&mut self, id: u32, force: Vec3, point: Option<Vec3>) {
        self.engine.send_command(PhysicsCommand::ApplyForce { id, force, point });
    }

    /// Получение состояния объекта
    pub fn get_object_state(&self, id: u32) -> Option<RigidBodyState> {
        self.engine.get_body_state(id)
    }

    /// Запуск асинхронного цикла
    pub async fn run(&mut self) {
        self.engine.run_async().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_physics_config_default() {
        let config = PhysicsConfig::default();
        assert_eq!(config.timestep, 1.0 / 60.0);
        assert_eq!(config.substeps, 4);
        assert_eq!(config.gravity.y, -9.81);
    }

    #[test]
    fn test_physics_engine_creation() {
        let engine = AsyncPhysicsEngine::new(PhysicsConfig::default());
        assert!(!engine.is_running());
    }

    #[test]
    fn test_command_queue() {
        let engine = AsyncPhysicsEngine::new(PhysicsConfig::default());
        
        engine.send_command(PhysicsCommand::AddRigidBody {
            id: 1,
            position: Vec3::new(0.0, 10.0, 0.0),
            rotation: Quat::IDENTITY,
            mass: 1.0,
            is_static: false,
        });
        
        // Команда должна быть в очереди
        // В реальной реализации можно проверить через get_state после обработки
    }
}
