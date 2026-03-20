# RHI Рефакторинг — Сводка

## Выполнено

### 1. Добавлен `create_gl_device()` в `/workspace/src/graphics/rhi/gl.rs`
```rust
pub fn create_gl_device(context: Arc<Context>) -> GlDevice {
    GlDevice::new(context)
}
```
Теперь OpenGL бэкенд можно создать через единую функцию factory.

### 2. Создан новый RHI-рендерер `/workspace/src/graphics/renderer_rhi.rs`
**332 строки кода** — полностью новый renderer, использующий `IDevice` вместо прямого вызова `glow`.

**Ключевые изменения:**
```rust
// БЫЛО:
pub struct Renderer {
    gl: Rc<glow::Context>,
    // ...
}

// СТАЛО:
pub struct RendererRhi {
    device: Arc<dyn IDevice>,
    command_list: Option<Arc<dyn ICommandList>>,
    // Ресурсы как ResourceHandle
    terrain_vertex_buffer: Option<ResourceHandle>,
    terrain_pipeline: Option<ResourceHandle>,
    // ...
}
```

**Преимущества:**
- Один код для всех бэкендов (OpenGL/Vulkan/DX12)
- Vulkan подключается через factory без изменений в логике рендеринга
- Thread-safe командные листы для многопоточной записи

### 3. Обновлён `/workspace/src/graphics/mod.rs`
Добавлен экспорт нового рендерера:
```rust
pub mod renderer_rhi;
pub use renderer_rhi::RendererRhi;
```

---

## Архитектура после рефакторинга

```
engine.rs
    ↓
RhiFactory::create_device(RhiConfig { backend: Vulkan })
    ↓
Arc<dyn IDevice> → VkDevice (или GlDevice)
    ↓
RendererRhi::new(device, width, height)
    ↓
device.create_command_list() → Arc<dyn ICommandList>
    ↓
cmd_list.begin_render_pass(...)
cmd_list.bind_pipeline(...)
cmd_list.draw_indexed(...)
```

---

## Следующие шаги

### Для переключения на Vulkan:
1. В `engine.rs` заменить создание `Renderer` на `RendererRhi`:
```rust
let device = RhiFactory::create_device(&RhiConfig {
    backend: RhiBackend::Vulkan,  // или Auto
    ..Default::default()
})?;
let renderer = RendererRhi::new(device, width, height)?;
```

2. Реализовать полные методы рендеринга в `renderer_rhi.rs`:
   - `render_terrain()` — загрузка меша через buffers
   - `render_vehicle()` — box placeholder
   - `render_hud()` — батчинг + bitmap font
   - `render_sky()` — gradient quad

### Для завершения RHI-рендерера:
- [ ] Загрузка шейдеров из SPIR-V
- [ ] Создание pipeline state objects
- [ ] Upload mesh данных в буферы
- [ ] Render pass с depth/stencil
- [ ] Descriptor binding для текстур

---

## Файлы

| Файл | Строк | Статус |
|------|-------|--------|
| `src/graphics/rhi/gl.rs` | +5 | ✅ Обновлено |
| `src/graphics/renderer_rhi.rs` | 333 | ✅ Создан |
| `src/graphics/mod.rs` | +2 | ✅ Обновлено |

---

## Примечание

Старый `renderer.rs` (1039 строк) остаётся для обратной совместимости.
После тестирования `RendererRhi` можно будет удалить старый код.
