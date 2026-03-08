//! Поддержка геймпадов
//! 
//! Реализует ввод с геймпадов через gilrs:
//! - Обработка кнопок и осей
//! - Вибрация (rumble)
//! - Переназначение кнопок
//! - Поддержка нескольких геймпадов

use gilrs::{Gilrs, Gamepad, Event, EventType, Button, Axis};
use std::collections::HashMap;

/// Тип действия ввода
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum InputAction {
    // Движение
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
    
    // Камера/смотрение
    LookHorizontal,
    LookVertical,
    
    // Действия
    Accelerate,
    Brake,
    Handbrake,
    Jump,
    Crouch,
    
    // Взаимодействие
    Interact,
    UseItem,
    
    // Меню
    Pause,
    Menu,
    Map,
    
    // Переключения
    NextWeapon,
    PreviousWeapon,
    
    // Специальные
    Nitro,
    Horn,
    Lights,
}

/// Состояние кнопки
#[derive(Clone, Debug, Default)]
pub struct ButtonState {
    pub pressed: bool,
    pub just_pressed: bool,
    pub just_released: bool,
    pub value: f32, // Для триггеров
}

/// Конфигурация привязки кнопок для одного геймпада
#[derive(Clone, Debug)]
pub struct GamepadBinding {
    /// Кнопка для действия
    pub button: Option<Button>,
    /// Ось для действия (для аналогового ввода)
    pub axis: Option<Axis>,
    /// Инвертировать ось
    pub invert_axis: bool,
    /// Порог срабатывания оси
    pub axis_threshold: f32,
}

impl Default for GamepadBinding {
    fn default() -> Self {
        Self {
            button: None,
            axis: None,
            invert_axis: false,
            axis_threshold: 0.1,
        }
    }
}

/// Стандартная раскладка Xbox-совместимого геймпада
impl GamepadBinding {
    /// Раскладка по умолчанию (Xbox-style)
    pub fn default_layout() -> HashMap<InputAction, GamepadBinding> {
        let mut bindings = HashMap::new();
        
        // D-pad / Левый стик - движение
        bindings.insert(InputAction::MoveForward, GamepadBinding {
            button: Some(Button::DPadUp),
            axis: Some(Axis::LeftStickY),
            invert_axis: true,
            ..Default::default()
        });
        
        bindings.insert(InputAction::MoveBackward, GamepadBinding {
            button: Some(Button::DPadDown),
            axis: Some(Axis::LeftStickY),
            invert_axis: false,
            ..Default::default()
        });
        
        bindings.insert(InputAction::MoveLeft, GamepadBinding {
            button: Some(Button::DPadLeft),
            axis: Some(Axis::LeftStickX),
            invert_axis: true,
            ..Default::default()
        });
        
        bindings.insert(InputAction::MoveRight, GamepadBinding {
            button: Some(Button::DPadRight),
            axis: Some(Axis::LeftStickX),
            invert_axis: false,
            ..Default::default()
        });
        
        // Правый стик - камера
        bindings.insert(InputAction::LookHorizontal, GamepadBinding {
            axis: Some(Axis::RightStickX),
            ..Default::default()
        });
        
        bindings.insert(InputAction::LookVertical, GamepadBinding {
            axis: Some(Axis::RightStickY),
            invert_axis: true,
            ..Default::default()
        });
        
        // Триггеры - газ/тормоз
        bindings.insert(InputAction::Accelerate, GamepadBinding {
            button: Some(Button::RightTrigger2),
            axis: Some(Axis::RightTrigger2),
            ..Default::default()
        });
        
        bindings.insert(InputAction::Brake, GamepadBinding {
            button: Some(Button::LeftTrigger2),
            axis: Some(Axis::LeftTrigger2),
            ..Default::default()
        });
        
        // Кнопки действий
        bindings.insert(InputAction::Handbrake, GamepadBinding {
            button: Some(Button::South), // A на Xbox, Cross на PS
            ..Default::default()
        });
        
        bindings.insert(InputAction::Jump, GamepadBinding {
            button: Some(Button::East), // B на Xbox, Circle на PS
            ..Default::default()
        });
        
        bindings.insert(InputAction::Crouch, GamepadBinding {
            button: Some(Button::West), // X на Xbox, Square на PS
            ..Default::default()
        });
        
        bindings.insert(InputAction::Interact, GamepadBinding {
            button: Some(Button::North), // Y на Xbox, Triangle на PS
            ..Default::default()
        });
        
        // Меню
        bindings.insert(InputAction::Pause, GamepadBinding {
            button: Some(Button::Mode), // Start/Menu
            ..Default::default()
        });
        
        bindings.insert(InputAction::Menu, GamepadBinding {
            button: Some(Button::Select), // Back/View
            ..Default::default()
        });
        
        // Бамперы - смена оружия
        bindings.insert(InputAction::NextWeapon, GamepadBinding {
            button: Some(Button::RightTrigger),
            ..Default::default()
        });
        
        bindings.insert(InputAction::PreviousWeapon, GamepadBinding {
            button: Some(Button::LeftTrigger),
            ..Default::default()
        });
        
        // Специальные
        bindings.insert(InputAction::Nitro, GamepadBinding {
            button: Some(Button::RightThumb),
            ..Default::default()
        });
        
        bindings.insert(InputAction::Horn, GamepadBinding {
            button: Some(Button::LeftThumb),
            ..Default::default()
        });
        
        bindings
    }
}

/// Состояние одного геймпада
pub struct GamepadState {
    gamepad: Gamepad,
    name: String,
    bindings: HashMap<InputAction, GamepadBinding>,
    button_states: HashMap<InputAction, ButtonState>,
    connected: bool,
}

impl GamepadState {
    pub fn new(gamepad: Gamepad, name: String) -> Self {
        Self {
            gamepad,
            name,
            bindings: GamepadBinding::default_layout(),
            button_states: HashMap::new(),
            connected: true,
        }
    }

    /// Обновление состояния из событий
    pub fn update(&mut self, event: &Event) {
        if event.gamepad != self.gamepad {
            return;
        }

        match &event.event {
            EventType::ButtonPressed(button, _) => {
                self.handle_button_press(*button);
            }
            EventType::ButtonReleased(button, _) => {
                self.handle_button_release(*button);
            }
            EventType::ButtonChanged(button, value, _) => {
                self.handle_button_change(*button, *value);
            }
            EventType::AxisChanged(axis, value, _) => {
                self.handle_axis_change(*axis, *value);
            }
            EventType::Connected => {
                self.connected = true;
            }
            EventType::Disconnected => {
                self.connected = false;
            }
            _ => {}
        }
    }

    fn handle_button_press(&mut self, button: Button) {
        for (action, binding) in &self.bindings {
            if binding.button == Some(button) {
                let state = self.button_states.entry(action.clone()).or_default();
                if !state.pressed {
                    state.just_pressed = true;
                    state.pressed = true;
                }
            }
        }
    }

    fn handle_button_release(&mut self, button: Button) {
        for (action, binding) in &self.bindings {
            if binding.button == Some(button) {
                let state = self.button_states.entry(action.clone()).or_default();
                state.just_released = true;
                state.pressed = false;
                state.value = 0.0;
            }
        }
    }

    fn handle_button_change(&mut self, button: Button, value: f32) {
        for (action, binding) in &self.bindings {
            if binding.button == Some(button) {
                let state = self.button_states.entry(action.clone()).or_default();
                state.value = value;
                state.pressed = value > binding.axis_threshold;
            }
        }
    }

    fn handle_axis_change(&mut self, axis: Axis, value: f32) {
        for (action, binding) in &self.bindings {
            if binding.axis == Some(axis) {
                let adjusted_value = if binding.invert_axis { -value } else { value };
                
                let state = self.button_states.entry(action.clone()).or_default();
                state.value = adjusted_value;
                state.pressed = adjusted_value.abs() > binding.axis_threshold;
            }
        }
    }

    /// Сброс флагов just_pressed/just_released (вызывать каждый кадр)
    pub fn reset_frame_flags(&mut self) {
        for state in self.button_states.values_mut() {
            state.just_pressed = false;
            state.just_released = false;
        }
    }

    /// Проверка нажатия действия
    pub fn is_action_pressed(&self, action: &InputAction) -> bool {
        self.button_states.get(action).map(|s| s.pressed).unwrap_or(false)
    }

    /// Проверка только что нажатого действия
    pub fn is_action_just_pressed(&self, action: &InputAction) -> bool {
        self.button_states.get(action).map(|s| s.just_pressed).unwrap_or(false)
    }

    /// Проверка только что отпущенного действия
    pub fn is_action_just_released(&self, action: &InputAction) -> bool {
        self.button_states.get(action).map(|s| s.just_released).unwrap_or(false)
    }

    /// Получение значения действия (для аналоговых входов)
    pub fn get_action_value(&self, action: &InputAction) -> f32 {
        self.button_states.get(action).map(|s| s.value).unwrap_or(0.0)
    }

    /// Переназначение кнопки
    pub fn rebind(&mut self, action: InputAction, button: Button) {
        if let Some(binding) = self.bindings.get_mut(&action) {
            binding.button = Some(button);
            binding.axis = None;
        }
    }

    /// Переназначение оси
    pub fn rebind_axis(&mut self, action: InputAction, axis: Axis, invert: bool) {
        if let Some(binding) = self.bindings.get_mut(&action) {
            binding.axis = Some(axis);
            binding.invert_axis = invert;
            binding.button = None;
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }

    pub fn gamepad(&self) -> Gamepad {
        self.gamepad
    }
}

/// Менеджер геймпадов
pub struct GamepadManager {
    gilrs: Gilrs,
    gamepads: HashMap<i32, GamepadState>,
    vibration_enabled: bool,
}

impl GamepadManager {
    pub fn new() -> Result<Self, String> {
        let gilrs = Gilrs::new()
            .map_err(|e| format!("Failed to initialize gilrs: {}", e))?;
        
        Ok(Self {
            gilrs,
            gamepads: HashMap::new(),
            vibration_enabled: true,
        })
    }

    /// Обновление менеджера (вызывать каждый кадр)
    pub fn update(&mut self) {
        // Обработка событий подключения/отключения
        while let Some(event) = self.gilrs.next_event() {
            match &event.event {
                EventType::Connected => {
                    let gp = self.gilrs.gamepad(event.gamepad.id);
                    let state = GamepadState::new(
                        event.gamepad,
                        gp.name().to_string(),
                    );
                    self.gamepads.insert(event.gamepad.id, state);
                }
                EventType::Disconnected => {
                    self.gamepads.remove(&event.gamepad.id);
                }
                _ => {
                    // Обновляем состояние соответствующего геймпада
                    if let Some(state) = self.gamepads.get_mut(&event.gamepad.id) {
                        state.update(&event);
                    }
                }
            }
        }
    }

    /// Сброс флагов кадра для всех геймпадов
    pub fn reset_frame_flags(&mut self) {
        for state in self.gamepads.values_mut() {
            state.reset_frame_flags();
        }
    }

    /// Получить первый подключенный геймпад
    pub fn primary_gamepad(&self) -> Option<&GamepadState> {
        self.gamepads.values().find(|gp| gp.is_connected())
    }

    /// Получить геймпад по ID
    pub fn get_gamepad(&self, id: i32) -> Option<&GamepadState> {
        self.gamepads.get(&id)
    }

    /// Количество подключенных геймпадов
    pub fn connected_count(&self) -> usize {
        self.gamepads.values().filter(|gp| gp.is_connected()).count()
    }

    /// Включить/выключить вибрацию
    pub fn set_vibration_enabled(&mut self, enabled: bool) {
        self.vibration_enabled = enabled;
    }

    /// Вибрация геймпада
    pub fn rumble(&mut self, id: i32, strong: f32, weak: f32, duration: f32) {
        if !self.vibration_enabled {
            return;
        }

        if let Some(gp) = self.gilrs.gamepad_mut(id) {
            gp.rumble(strong, weak, duration);
        }
    }

    /// Вибрация первого геймпада
    pub fn rumble_primary(&mut self, strong: f32, weak: f32, duration: f32) {
        if let Some(id) = self.gamepads.keys().next().copied() {
            self.rumble(id, strong, weak, duration);
        }
    }

    /// Отключить вибрацию на всех геймпадах
    pub fn stop_all_rumble(&mut self) {
        for id in self.gamepads.keys().copied() {
            if let Some(gp) = self.gilrs.gamepad_mut(id) {
                gp.stop_rumble();
            }
        }
    }

    /// Список всех геймпадов
    pub fn list_gamepads(&self) -> Vec<String> {
        self.gamepads.values().map(|gp| gp.name().to_string()).collect()
    }
}

impl Default for GamepadManager {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            gilrs: Gilrs::new().unwrap(),
            gamepads: HashMap::new(),
            vibration_enabled: true,
        })
    }
}

/// Комбинированный ввод (клавиатура + геймпад)
pub struct CombinedInput {
    gamepad_manager: GamepadManager,
    // Здесь можно добавить обработку клавиатуры
    keyboard_state: HashMap<InputAction, ButtonState>,
}

impl CombinedInput {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            gamepad_manager: GamepadManager::new()?,
            keyboard_state: HashMap::new(),
        })
    }

    pub fn update(&mut self) {
        self.gamepad_manager.update();
    }

    pub fn reset_frame_flags(&mut self) {
        self.gamepad_manager.reset_frame_flags();
        
        for state in self.keyboard_state.values_mut() {
            state.just_pressed = false;
            state.just_released = false;
        }
    }

    /// Проверка действия (геймпад имеет приоритет)
    pub fn is_action_pressed(&self, action: &InputAction) -> bool {
        // Сначала проверяем геймпад
        if let Some(gp) = self.gamepad_manager.primary_gamepad() {
            if gp.is_action_pressed(action) {
                return true;
            }
        }
        
        // Затем клавиатуру
        self.keyboard_state.get(action).map(|s| s.pressed).unwrap_or(false)
    }

    pub fn is_action_just_pressed(&self, action: &InputAction) -> bool {
        if let Some(gp) = self.gamepad_manager.primary_gamepad() {
            if gp.is_action_just_pressed(action) {
                return true;
            }
        }
        
        self.keyboard_state.get(action).map(|s| s.just_pressed).unwrap_or(false)
    }

    pub fn get_action_value(&self, action: &InputAction) -> f32 {
        if let Some(gp) = self.gamepad_manager.primary_gamepad() {
            let value = gp.get_action_value(action);
            if value.abs() > 0.01 {
                return value;
            }
        }
        
        self.keyboard_state.get(action).map(|s| if s.pressed { 1.0 } else { 0.0 }).unwrap_or(0.0)
    }

    pub fn gamepad_manager(&self) -> &GamepadManager {
        &self.gamepad_manager
    }

    pub fn gamepad_manager_mut(&mut self) -> &mut GamepadManager {
        &mut self.gamepad_manager
    }
}

impl Default for CombinedInput {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_bindings() {
        let bindings = GamepadBinding::default_layout();
        
        assert!(bindings.contains_key(&InputAction::MoveForward));
        assert!(bindings.contains_key(&InputAction::Accelerate));
        assert!(bindings.contains_key(&InputAction::Pause));
    }

    #[test]
    fn test_button_state_default() {
        let state = ButtonState::default();
        assert!(!state.pressed);
        assert!(!state.just_pressed);
        assert!(!state.just_released);
        assert_eq!(state.value, 0.0);
    }

    #[test]
    fn test_input_action_count() {
        // Просто проверяем что enum определены
        let actions = vec![
            InputAction::MoveForward,
            InputAction::Accelerate,
            InputAction::Pause,
        ];
        assert_eq!(actions.len(), 3);
    }
}
