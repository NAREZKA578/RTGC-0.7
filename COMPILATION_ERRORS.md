# Отчёт об ошибках компиляции RTGC-0.7

**Дата генерации:** 21 марта 2026 г.  
**Версия проекта:** 0.7.0  
**Статус:** ❌ Компиляция не удалась

---

## 📊 Статистика

- **Всего ошибок:** ~660
- **Всего предупреждений:** 130
- **Код выхода:** 101

---

## 🔴 Критические категории ошибок

### 1. RHI (Render Hardware Interface) - Проблемы Send/Sync
**Файл:** `src/graphics/rhi/gl.rs`

- `*mut c_void` не реализует `Sync` для `glow::Context`
- `RefCell<HashMap>` не реализует `Sync` для `GlPipeline`
- Не реализованы методы трейта `IDevice` для `GlDevice`
- Не реализованы методы трейта `ICommandList` для `GlCommandList`

**Пример ошибки:**
```
error[E0277]: `*mut c_void` cannot be shared between threads safely
   --> src/graphics/rhi/gl.rs:113:20
    |
113 | impl RhiDevice for GlDevice {
    |                    ^^^^^^^^
```

---

### 2. ECS - Проблемы с замыканиями и заимствованием
**Файл:** `src/ecs/ecs_module.rs`

- Captured variable cannot escape `FnMut` closure body
- Cannot move out of `self` because it is borrowed

**Пример ошибки:**
```
error[E0505]: cannot move out of `self` because it is borrowed
   --> src/ecs/ecs_module.rs:161:46
    |
161 |         self.dense_indices.iter().filter_map(move |&idx| {
```

---

### 3. Физика - Multiple mutable borrows
**Файл:** `src/physics/physics_module.rs`, `src/physics/vehicle.rs`

- Cannot borrow `self.rigid_bodies` as mutable more than once at a time
- Cannot borrow `*self` as mutable more than once at a time

**Пример ошибки:**
```
error[E0499]: cannot borrow `self.rigid_bodies` as mutable more than once at a time
    --> src/physics/physics_module.rs:1909:27
     |
1908 |         let body_a = &mut self.rigid_bodies[contact.body_a];
     |                           ----------------- first borrow
1909 |         let body_b = &mut self.rigid_bodies[contact.body_b];
     |                           ^^^^^^^^^^^^^^^^^ second borrow
```

---

### 4. Генерация мира - Borrow conflicts
**Файлы:** 
- `src/world/terrain_generator.rs`
- `src/world/lod_system.rs`
- `src/world/spatial_index.rs`
- `src/world/world_module.rs`

**Ошибки:**
- Cannot borrow `permutations` as mutable because it is also borrowed as immutable
- Cannot borrow `*self` as immutable because it is also borrowed as mutable
- Use of moved value: `streaming_config`
- Cannot call non-const associated function in constants

---

### 5. Миссии - Moved values
**Файл:** `src/game/mission_save.rs`, `src/game/mission_generator.rs`

- Borrow of moved value: `mission`
- Cannot assign twice to immutable variable `base_reward`

---

## 📁 Файлы с наибольшим количеством ошибок

| Файл | Категория | Примерное кол-во ошибок |
|------|-----------|------------------------|
| `src/graphics/rhi/gl.rs` | RHI Send/Sync | ~200 |
| `src/physics/physics_module.rs` | Borrow checker | ~150 |
| `src/physics/vehicle.rs` | Borrow checker | ~100 |
| `src/world/terrain_generator.rs` | Borrow checker | ~50 |
| `src/world/lod_system.rs` | Borrow checker | ~50 |
| `src/ecs/ecs_module.rs` | Closure borrows | ~30 |
| `src/world/spatial_index.rs` | Const evaluation | ~20 |
| `src/game/mission_save.rs` | Move semantics | ~10 |

---

## ⚠️ Предупреждения (130 шт.)

### Неиспользуемые импорты
- `std::cmp::Ordering` - `src/world/road_network.rs`
- `Path` - `src/game/mission_save.rs`
- `Quaternion`, `Vector3` - `src/network/protocol.rs`

### Неиспользуемые переменные
- `dt` (время) - множественные файлы физики
- `config` - `src/physics/vehicle.rs`
- `rng` - `src/world/russian_names.rs`, `src/world/buildings.rs`
- `main_road_dir` - `src/world/buildings.rs`

### Ambiguous glob re-exports
- `AudioConfig` - конфликт между `config::*` и `audio::*`
- `lod_system` - конфликт между `graphics::*` и `world::*`
- `DayNightCycle` - конфликт между `world::*` и `game::*`

---

## 🔧 Рекомендации по исправлению

### Краткосрочные (для быстрой компиляции):

1. **Отключить RHI модуль полностью**
   - Удалить или закомментировать `impl RhiDevice for GlDevice`
   - Использовать OpenGL через `glow` напрямую

2. **Исправить borrow checker ошибки**
   - Использовать `.clone()` для moved values
   - Разделить mutable borrows на отдельные области видимости
   - Использовать `std::cell::RefCell` вместо прямых заимствований

3. **Исправить const evaluation**
   - Заменить константы на `lazy_static!` или `once_cell`

### Долгосрочные (архитектурные):

1. **Переработать RHI архитектуру**
   - Использовать `Arc<Mutex<T>>` для shared ресурсов
   - Реализовать все методы трейтов `IDevice` и `ICommandList`

2. **Рефакторинг ECS**
   - Использовать `split_at_mut` для множественных mutable заимствований
   - Переписать `iter_mut` с правильным lifetime

3. **Физический движок**
   - Использовать индексацию вместо прямых ссылок
   - Применить `RwLock` для потокобезопасности

---

## 📝 Полный лог компиляции

Полный вывод компиляции сохранён в этом файле ниже.

---

*Конец отчёта*
