# Отчёт о реализации — БЛОК 2 (RHI Рефакторинг)

## Выполненные задачи

### ✅ РХИ-1: Переключить renderer на IDevice (Частично выполнено)

**Создан новый файл:** `src/graphics/renderer_rhi.rs` (332 строки)

Это **полностью новый класс `RendererRhi`**, который использует RHI абстракцию вместо прямых вызовов glow.

#### Ключевые изменения в архитектуре:

**До:**
```rust
pub struct Renderer {
    gl: Rc<glow::Context>,  // Прямая зависимость от OpenGL
    shader: Shader,          // glow-based shader
    // ... 1039 строк кода с self.gl.xxx()
}
```

**После:**
```rust
pub struct RendererRhi {
    device: Arc<dyn IDevice>,           // Абстрактный GPU интерфейс
    command_list: Option<Arc<dyn ICommandList>>,
    terrain_pipeline: Option<ResourceHandle>,
    vehicle_pipeline: Option<ResourceHandle>,
    font_texture: Option<ResourceHandle>,
    // ... backend-agnostic код
}
```

#### Реализованные методы:

| Метод | Статус | Описание |
|-------|--------|----------|
| `new()` | ✅ | Создание рендерера с device, шрифтом |
| `create_bitmap_font()` | ✅ | Процедурная генерация 128x128 текстуры |
| `render()` | ⚠️ | Каркас переключения состояний меню |
| `set_terrain_mesh()` | 🔲 | Заглушка для загрузки меша |
| `set_vehicle_transform()` | ✅ | Установка позиции/ротации машины |
| `set_hud_data()` | ✅ | Передача данных HUD |
| `render_game()` | ⚠️ | Вызов render_sky/terrain/vehicle/hud |
| `render_sky()` | 🔲 | Заглушка |
| `render_terrain()` | 🔲 | Заглушка |
| `render_vehicle()` | 🔲 | Заглушка |
| `render_hud()` | 🔲 | Заглушка |

---

### ✅ Обновление factory.rs

**Файл:** `src/graphics/rhi/gl.rs`

Добавлена функция создания GL устройства:
```rust
pub fn create_gl_device(context: Arc<Context>) -> GlDevice {
    GlDevice::new(context)
}
```

Теперь все бэкенды имеют единую точку создания:
- OpenGL: `create_gl_device(context)`
- Vulkan: `VkDevice::new(debug, validation)`
- DX12: `Dx12Device::new(debug, validation)`

---

### ✅ Экспорт в модуле

**Файл:** `src/graphics/mod.rs`

```rust
pub mod renderer_rhi;
pub use renderer_rhi::RendererRhi;
```

---

## Как использовать Vulkan после этого рефакторинга

### Шаг 1: Создать устройство через factory
```rust
use crate::graphics::rhi::{RhiFactory, RhiConfig, RhiBackend};

let config = RhiConfig {
    backend: RhiBackend::Vulkan,  // или Auto для автовыбора
    debug_enabled: cfg!(debug_assertions),
    validation_enabled: cfg!(debug_assertions),
    ..Default::default()
};

let device = RhiFactory::create_device(&config)?;
// device: Arc<dyn IDevice>
```

### Шаг 2: Создать рендерер
```rust
use crate::graphics::RendererRhi;

let renderer = RendererRhi::new(device, width, height)?;
```

### Шаг 3: Рендерить
```rust
renderer.set_terrain_mesh(terrain_mesh);
renderer.set_vehicle_transform(pos, rot);
renderer.set_hud_data(hud_data);
renderer.render()?;
```

---

## Что осталось сделать для полного переключения

### 1. Загрузка шейдеров (SPIR-V)
```rust
// В renderer_rhi.rs:
let vs_bytes = include_bytes!("../../assets/shaders/terrain.vert.spv");
let fs_bytes = include_bytes!("../../assets/shaders/terrain.frag.spv");

let vs_desc = ShaderDescription {
    stage: ShaderStage::Vertex,
    source: vs_bytes.to_vec(),
    entry_point: "main".to_string(),
};
let vs = device.create_shader(&vs_desc)?;
```

### 2. Создание Pipeline State Objects
```rust
let input_layout = InputLayout::new(vec![
    VertexAttribute { name: "position".into(), format: VertexFormat::Float32x3, offset: 0 },
    VertexAttribute { name: "normal".into(), format: VertexFormat::Float32x3, offset: 12 },
    VertexAttribute { name: "tex_coord".into(), format: VertexFormat::Float32x2, offset: 24 },
]);

let pso_desc = PipelineStateObject {
    vertex_shader: vs_handle,
    fragment_shader: Some(fs_handle),
    input_layout,
    depth_state: DepthState::default(),
    rasterizer_state: RasterizerState::default(),
    primitive_topology: PrimitiveTopology::TriangleList,
    ..Default::default()
};

let pipeline = device.create_pipeline_state(&pso_desc)?;
```

### 3. Upload mesh данных
```rust
let vb_desc = BufferDescription {
    buffer_type: BufferType::Vertex,
    size: (vertices.len() * 32) as u64, // 32 bytes per vertex
    usage: BufferUsage::VERTEX_BUFFER,
    initial_state: ResourceState::VertexBuffer,
};

let vertex_buffer = device.create_buffer(&vb_desc, Some(bytemuck::cast_slice(vertices)))?;
```

### 4. Render pass
```rust
let rp_desc = RenderPassDescription {
    color_attachments: vec![RenderAttachment {
        view: swapchain_view,
        load_op: LoadOp::Clear,
        store_op: StoreOp::Store,
        clear_value: Some(ClearValue::Color([0.1, 0.1, 0.2, 1.0])),
    }],
    depth_stencil_attachment: Some(DepthStencilAttachment {
        view: depth_view,
        depth_load_op: LoadOp::Clear,
        depth_store_op: StoreOp::Store,
        depth_clear_value: Some(1.0),
        ..Default::default()
    }),
};

cmd_list.begin_render_pass(&rp_desc);
cmd_list.set_pipeline_state(pipeline);
cmd_list.bind_vertex_buffers(0, &[(vertex_buffer, 0)]);
cmd_list.draw_indexed(index_count, 1, 0, 0, 0);
cmd_list.end_render_pass();
```

---

## Преимущества новой архитектуры

1. **Backend-agnostic код**: Один renderer работает с OpenGL, Vulkan, DX12
2. **Thread-safe**: Command lists можно записывать параллельно
3. **Явные барьеры**: Синхронизация ресурсов через resource barriers
4. **PSO-based**: Pipeline State Objects как в современных API
5. **Легкое тестирование**: Можно мокать IDevice для unit-тестов

---

## Файлы

| Файл | Изменения | Строк |
|------|-----------|-------|
| `src/graphics/rhi/gl.rs` | +5 (create_gl_device) | 454 |
| `src/graphics/renderer_rhi.rs` | Новый файл | 332 |
| `src/graphics/mod.rs` | +2 (экспорт) | 29 |
| `RHI_REFACTOR_SUMMARY.md` | Документация | 120 |

---

## Следующий приоритет

**МИР-3: Road Network** (`src/world/road_network.rs`)
- 5 типов дорог (Federal → ForestTrack)
- Граф связности поселений
- B-spline сглаживание
- Интеграция с terrain generation

**МИР-4: Buildings** (`src/world/buildings.rs`)
- BuildingBoxDesc для flat color рендеринга
- Алгоритмы расстановки в поселениях

**МИР-5: Интеграция OpenWorld в engine.rs**
- Замена ручного ChunkManager на OpenWorld::new(seed)
