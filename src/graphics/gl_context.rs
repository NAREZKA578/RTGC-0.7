//! GL context initialization using glutin

use glow::Context;
use glutin::{
    config::{ConfigTemplateBuilder, GlConfig},
    context::{ContextApi, ContextAttributesBuilder, PossiblyCurrentContext},
    display::GetGlDisplay,
    prelude::*,
    surface::{Surface, WindowSurface},
};
use glutin_winit::{DisplayBuilder, GlProfile};
use std::num::NonZeroU32;
use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

pub struct GlContext {
    pub gl: Context,
    pub window: Window,
    pub surface: Surface<WindowSurface>,
    pub context: PossiblyCurrentContext,
    pub width: u32,
    pub height: u32,
}

impl GlContext {
    pub fn new(event_loop: &EventLoop<()>) -> Result<Self, Box<dyn std::error::Error>> {
        let window_builder = WindowBuilder::new()
            .with_title("RTGC-0.7")
            .with_inner_size(winit::dpi::LogicalSize::new(1280.0, 720.0));

        // Создаем GL дисплей и окно через glutin_winit
        let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));
        let (window, gl_display) = display_builder.build(event_loop)?;

        let window = window.ok_or("Failed to create window")?;

        // Поиск конфигурации
        let template = ConfigTemplateBuilder::new()
            .with_alpha_size(8)
            .compatible_with_native_window(window.raw_window_handle())
            .build();

        let config = unsafe {
            gl_display
                .find_configs(template)?
                .reduce(|accum, config| {
                    if config.num_samples() > accum.num_samples() {
                        config
                    } else {
                        accum
                    }
                })
                .ok_or("No valid GL config found")?
        };

        log::info!("Picked GL config: {:?}", config);

        // Создание контекста
        let context_attributes = ContextAttributesBuilder::new()
            .with_profile(GlProfile::Core)
            .with_context_api(ContextApi::OpenGl(None))
            .build(Some(window.raw_window_handle()));

        let context = unsafe {
            gl_display
                .create_context(&config, &context_attributes)?
        };

        // Создание поверхности
        let (width, height): (u32, u32) = window.inner_size().into();
        let attrs = glutin::surface::SurfaceAttributesBuilder::<WindowSurface>::new()
            .with_srgb(config.srgb_capable())
            .build(
                window.raw_window_handle(),
                NonZeroU32::new(width).unwrap(),
                NonZeroU32::new(height).unwrap(),
            );

        let surface = unsafe {
            gl_display
                .create_window_surface(&config, &attrs)?
        };

        // Делаем контекст текущим
        let context = context.make_current(&surface)?;

        // Инициализация glow
        let gl = unsafe {
            Context::from_loader_function(|s| {
                gl_display.get_proc_address(s)
            })
        };

        Ok(Self {
            gl,
            window,
            surface,
            context,
            width,
            height,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), Box<dyn std::error::Error>> {
        self.width = width;
        self.height = height;
        
        if let Some(new_width) = NonZeroU32::new(width) {
            if let Some(new_height) = NonZeroU32::new(height) {
                self.surface.resize(&self.context, new_width, new_height);
            }
        }
        
        Ok(())
    }

    pub fn swap_buffers(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.surface.swap_buffers(&self.context)?;
        Ok(())
    }
}
