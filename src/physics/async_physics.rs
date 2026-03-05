use crate::physics::RigidBody;
use std::sync::Arc;
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Sender, Receiver};

/// Сообщение для физического потока
pub enum PhysicsMessage {
    Step { dt: f32, sub_steps: u32 },
    SetBodies(Vec<RigidBody>),
    GetBodies,
    Shutdown,
}

/// Ответ от физического потока
pub enum PhysicsResponse {
    Bodies(Vec<RigidBody>),
    StepComplete,
    ShutdownComplete,
}

/// Асинхронный физический движок с double buffering
pub struct AsyncPhysicsEngine {
    sender: Sender<PhysicsMessage>,
    receiver: Receiver<PhysicsResponse>,
    running: Arc<AtomicBool>,
    local_bodies: Vec<RigidBody>,
    pending_bodies: Option<Vec<RigidBody>>,
}

impl AsyncPhysicsEngine {
    pub fn new() -> Self {
        let (msg_sender, msg_receiver) = channel();
        let (resp_sender, resp_receiver) = channel();
        let running = Arc::new(AtomicBool::new(true));
        let running_clone = running.clone();
        
        // Запуск физического потока
        thread::spawn(move || {
            let mut bodies: Vec<RigidBody> = Vec::new();
            
            while running_clone.load(Ordering::Relaxed) {
                match msg_receiver.recv() {
                    Ok(PhysicsMessage::Step { dt, sub_steps }) => {
                        let sub_dt = dt / sub_steps as f32;
                        
                        // Sub-stepping для стабильности
                        for _ in 0..sub_steps {
                            for body in &mut bodies {
                                body.clear_forces();
                                // Гравитация применяется в update()
                                body.update(sub_dt);
                            }
                            
                            // Здесь будет collision detection и resolution
                            // пока заглушка
                        }
                        
                        let _ = resp_sender.send(PhysicsResponse::StepComplete);
                    }
                    
                    Ok(PhysicsMessage::SetBodies(new_bodies)) => {
                        bodies = new_bodies;
                        let _ = resp_sender.send(PhysicsResponse::StepComplete);
                    }
                    
                    Ok(PhysicsMessage::GetBodies) => {
                        let _ = resp_sender.send(PhysicsResponse::Bodies(bodies.clone()));
                    }
                    
                    Ok(PhysicsMessage::Shutdown) => {
                        let _ = resp_sender.send(PhysicsResponse::ShutdownComplete);
                        break;
                    }
                    
                    Err(_) => break,
                }
            }
        });
        
        Self {
            sender: msg_sender,
            receiver: resp_receiver,
            running,
            local_bodies: Vec::new(),
            pending_bodies: None,
        }
    }
    
    /// Установить тела для симуляции (double buffer)
    pub fn set_bodies(&mut self, bodies: Vec<RigidBody>) {
        self.pending_bodies = Some(bodies);
    }
    
    /// Синхронизировать локальные данные с потоком
    pub fn sync(&mut self) {
        if let Some(pending) = self.pending_bodies.take() {
            self.local_bodies = pending.clone();
            let _ = self.sender.send(PhysicsMessage::SetBodies(pending));
            let _ = self.receiver.recv();
        } else {
            match self.receiver.try_recv() {
                Ok(PhysicsResponse::Bodies(bodies)) => {
                    self.local_bodies = bodies;
                }
                _ => {}
            }
        }
    }
    
    /// Шаг симуляции
    pub fn step(&mut self, dt: f32, sub_steps: u32) {
        let _ = self.sender.send(PhysicsMessage::Step { dt, sub_steps });
    }
    
    /// Ожидать завершения шага
    pub fn wait_for_step(&self) {
        let _ = self.receiver.recv();
    }
    
    /// Получить текущие тела
    pub fn get_bodies(&self) -> &[RigidBody] {
        &self.local_bodies
    }
    
    /// Отправить запрос на получение тел из потока
    pub fn request_bodies(&self) {
        let _ = self.sender.send(PhysicsMessage::GetBodies);
    }
    
    /// Остановить движок
    pub fn shutdown(mut self) {
        self.running.store(false, Ordering::Relaxed);
        let _ = self.sender.send(PhysicsMessage::Shutdown);
        let _ = self.receiver.recv();
    }
}

impl Default for AsyncPhysicsEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Vector3;
    
    #[test]
    fn test_async_physics_creation() {
        let engine = AsyncPhysicsEngine::new();
        assert!(engine.running.load(Ordering::Relaxed));
    }
    
    #[test]
    fn test_async_physics_step() {
        let mut engine = AsyncPhysicsEngine::new();
        
        let sphere = RigidBody::new_sphere(
            Vector3::new(0.0, 10.0, 0.0),
            1.0,
            0.5
        );
        
        engine.set_bodies(vec![sphere]);
        engine.sync();
        
        engine.step(0.016, 4);
        engine.wait_for_step();
        
        engine.request_bodies();
        engine.sync();
        
        let bodies = engine.get_bodies();
        assert_eq!(bodies.len(), 1);
        // Тело должно упасть под действием гравитации
        assert!(bodies[0].position.y < 10.0);
    }
}
