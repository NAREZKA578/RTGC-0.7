# RTGC — Выполненные задачи (БЛОК 0 + начало БЛОКА 1)

## СДЕЛАНО

### МУЛ-1 ✅ Создан network/protocol.rs
**Файл:** `src/network/protocol.rs`

Содержит только структуры данных для будущей сетевой синхронизации:
- `GameState` — полное состояние игры для передачи по сети
- `NetworkMessage` — типы сообщений клиент-сервер
- `PlayerInput` — входные данные игрока

Никакой реализации сети — только сериализуемые типы.

**Также создано:**
- `src/network/mod.rs` — модуль сети
- Обновлён `src/lib.rs` — добавлен `pub mod network`

---

### МИР-1 ✅ Создан settlement.rs (населённые пункты)
**Файл:** `src/world/settlement.rs`

Полная система генерации русских населённых пунктов:

**Типы поселений:**
- `Derevnya` (деревня): 5–30 домов, грунтовые дороги
- `Posyolok` (посёлок): 30–150 домов, асфальт в центре, АЗС
- `MalyiGorod` (малый город): 150–500 домов
- `PromGorod` (промгород): 500–2000 жителей, заводы

**Структуры:**
- `Settlement` — поселение с именем, типом, услугами
- `SettlementServices` — АЗС, ремонт, грузовой терминал
- `BuildingInstance` — здание в мире
- `BuildingType` — типы зданий (изба, хрущёвка, склад, АЗС и т.д.)

**Функции:**
- `Settlement::generate()` — детерминированная генерация из seed
- `place_buildings_in_settlement()` — расстановка зданий по улицам

---

### МИР-2 ✅ Создан russian_names.rs (словарь названий)
**Файл:** `src/world/russian_names.rs`

Генератор реалистичных русских названий:
- Префиксы: Ново-, Старо-, Красно-, Нижне- и т.д.
- Корни: -горск, -речинск, -берёзов-, -сосновк- и т.д.
- Суффиксы: -ое, -ск, -ово, -ино и т.д.

**Примеры:** Новоберёзовск, Нижнесосновка, Старокедрово

**Функции:**
- `generate_name(seed)` — одно название
- `generate_name_variants(seed, count)` — несколько вариантов

---

### Обновления модулей

**`src/world/mod.rs`:**
```rust
pub mod settlement;
pub mod russian_names;

pub use world_module::{WorldManager, OpenWorld};
pub use settlement::{Settlement, SettlementType, BuildingInstance, BuildingType, SettlementServices};
pub use russian_names::generate_name as generate_settlement_name;
```

---

## СЛЕДУЮЩИЕ ШАГИ

### РХИ-1 (приоритет) — Переключить renderer.rs на IDevice

Это главный рефакторинг который разблокирует Vulkan:

**Задача:** В `src/graphics/renderer.rs` заменить:
```rust
// БЫЛО:
gl: Rc<glow::Context>

// СТАЛО:
device: Arc<dyn IDevice>
```

Все вызовы `self.gl.xxx()` должны идти через `ICommandList`.

**Почему это важно:** После этого Vulkan подключится автоматически через `factory.rs` без изменений в логике рендеринга.

---

### МИР-3 — Road Network (дорожная сеть)

**Файл:** `src/world/road_network.rs` (новый)

Нужно создать:
- `RoadType` (FederalHighway, RegionalRoad, DirtRoad, ForestTrack)
- `RoadSegment` (сегмент дороги с waypoints)
- Алгоритм соединения поселений дорогами
- Интеграция с terrain generation (сглаживание под дорогу)

---

### МИР-4 — Buildings (строения)

**Файл:** `src/world/buildings.rs` (новый)

Нужно создать:
- Параметры box для каждого BuildingType (размеры, цвета)
- Flat color rendering для альфы
- LOD система для строений

---

### МИР-5 — Интеграция OpenWorld в engine.rs

В `engine.rs` заменить ручное управление чанками на:
```rust
world: OpenWorld,

// В update():
self.world.update(vehicle_position, dt);
```

---

## ПОЛНЫЙ СПИСОК ФАЙЛОВ

### Созданные:
| Файл | Статус |
|------|--------|
| `src/network/mod.rs` | ✅ Создан |
| `src/network/protocol.rs` | ✅ Создан |
| `src/world/settlement.rs` | ✅ Создан |
| `src/world/russian_names.rs` | ✅ Создан |

### Обновлённые:
| Файл | Изменения |
|------|-----------|
| `src/lib.rs` | Добавлен `pub mod network` |
| `src/world/mod.rs` | Добавлены settlement, russian_names, экспорты |

### Ожидают реализации:
| Файл | Задача |
|------|--------|
| `src/graphics/renderer.rs` | РХИ-1: переключить на IDevice |
| `src/world/road_network.rs` | МИР-3: дорожная сеть |
| `src/world/buildings.rs` | МИР-4: параметры строений |
| `src/engine.rs` | МИР-5: подключить OpenWorld |

---

## КОМПИЛЯЦИЯ

Для проверки компиляции выполните:
```bash
cargo check
```

**Зависимости уже есть в Cargo.toml:**
- `rand = "0.8"` ✅
- `rand_chacha = "0.3"` ✅
- `serde = { version = "1.0", features = ["derive"] }` ✅

---

## АРХИТЕКТУРА RHI

```
СЕЙЧАС:
    renderer.rs → glow::Context напрямую

ПОСЛЕ РХИ-1:
    renderer.rs → IDevice / ICommandList
                      ↓               ↓
               GlDevice (glow)    VkDevice (ash)
```

После выполнения РХИ-1 любой бэкенд (Vulkan, DX12) подключится через factory без изменений в renderer.
