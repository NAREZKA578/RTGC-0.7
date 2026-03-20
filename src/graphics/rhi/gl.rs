//! OpenGL RHI Backend for RTGC-0.7
//! 
//! Implements the RHI trait using OpenGL 4.5+ core profile

use crate::graphics::rhi::{
    RhiDevice, RhiCommandList, RhiPipeline, RhiTexture, RhiBuffer, RhiSampler,
    BufferDesc, BufferUsage, TextureDesc, TextureFormat, SamplerDesc,
    PipelineDesc, ShaderStage, PrimitiveTopology, BlendState, DepthStencilState,
    RhiResult, RhiError, IndexType, Color4f, Rect2D, Viewport, ScissorRect,
};
use glow::{HasContext, Context};
use std::sync::Arc;
use std::cell::RefCell;

/// OpenGL-specific buffer handle
#[derive(Debug, Clone)]
pub struct GlBuffer {
    pub id: u32,
    pub desc: BufferDesc,
    pub size: usize,
}

/// OpenGL-specific texture handle
#[derive(Debug, Clone)]
pub struct GlTexture {
    pub id: u32,
    pub desc: TextureDesc,
    pub format: TextureFormat,
    pub width: u32,
    pub height: u32,
    pub mip_levels: u32,
}

/// OpenGL-specific sampler handle
#[derive(Debug, Clone)]
pub struct GlSampler {
    pub id: u32,
    pub desc: SamplerDesc,
}

/// OpenGL-specific pipeline handle
#[derive(Debug, Clone)]
pub struct GlPipeline {
    pub program: u32,
    pub vertex_array: u32,
    pub desc: PipelineDesc,
    pub uniform_locations: RefCell<std::collections::HashMap<String, i32>>,
}

/// OpenGL command list state
pub struct GlCommandListState {
    pub bound_pipeline: Option<u32>,
    pub bound_vertex_buffers: Vec<Option<(u32, usize)>>, // (buffer_id, offset)
    pub bound_index_buffer: Option<(u32, IndexType)>,
    pub bound_textures: Vec<Option<u32>>,
    pub viewport: Option<Viewport>,
    pub scissor: Option<ScissorRect>,
    pub clear_color: Option<Color4f>,
}

impl GlCommandListState {
    pub fn new() -> Self {
        Self {
            bound_pipeline: None,
            bound_vertex_buffers: Vec::new(),
            bound_index_buffer: None,
            bound_textures: Vec::new(),
            viewport: None,
            scissor: None,
            clear_color: None,
        }
    }
}

/// OpenGL RHI Device
pub struct GlDevice {
    pub context: Arc<Context>,
    pub default_framebuffer: u32,
    pub buffers: Vec<Option<Arc<GlBuffer>>>,
    pub textures: Vec<Option<Arc<GlTexture>>>,
    pub samplers: Vec<Option<Arc<GlSampler>>>,
    pub pipelines: Vec<Option<Arc<GlPipeline>>>,
}

impl GlDevice {
    pub fn new(context: Arc<Context>) -> Self {
        let default_framebuffer = unsafe { context.get_parameter_i32(glow::FRAMEBUFFER_BINDING) as u32 };
        
        Self {
            context,
            default_framebuffer,
            buffers: Vec::new(),
            textures: Vec::new(),
            samplers: Vec::new(),
            pipelines: Vec::new(),
        }
    }

    fn gl_format(&self, format: TextureFormat) -> (u32, u32, u32) {
        match format {
            TextureFormat::R8Unorm => (glow::RED, glow::UNSIGNED_BYTE, glow::R8),
            TextureFormat::RG8Unorm => (glow::RG, glow::UNSIGNED_BYTE, glow::RG8),
            TextureFormat::RGBA8Unorm => (glow::RGBA, glow::UNSIGNED_BYTE, glow::RGBA8),
            TextureFormat::RGBA8Srgb => (glow::RGBA, glow::UNSIGNED_BYTE, glow::SRGB8_ALPHA8),
            TextureFormat::RGBA16Float => (glow::RGBA, glow::HALF_FLOAT, glow::RGBA16F),
            TextureFormat::RGBA32Float => (glow::RGBA, glow::FLOAT, glow::RGBA32F),
            TextureFormat::Depth32Float => (glow::DEPTH_COMPONENT, glow::FLOAT, glow::DEPTH_COMPONENT32F),
            TextureFormat::Depth24UnormStencil8 => (glow::DEPTH_STENCIL, glow::UNSIGNED_INT_24_8, glow::DEPTH24_STENCIL8),
        }
    }
}

impl RhiDevice for GlDevice {
    type Buffer = GlBuffer;
    type Texture = GlTexture;
    type Sampler = GlSampler;
    type Pipeline = GlPipeline;
    type CommandList = GlCommandList;

    fn create_buffer(&mut self, desc: &BufferDesc, initial_data: Option<&[u8]>) -> RhiResult<Arc<Self::Buffer>> {
        let gl = &self.context;
        
        let buffer_id = unsafe { gl.create_buffer() }
            .map_err(|e| RhiError::CreationFailed(format!("Failed to create buffer: {:?}", e)))?;

        let target = if desc.usage.contains(BufferUsage::INDEX) {
            glow::ELEMENT_ARRAY_BUFFER
        } else {
            glow::ARRAY_BUFFER
        };

        let size = desc.size;
        
        unsafe {
            gl.bind_buffer(target, Some(buffer_id));
            
            if let Some(data) = initial_data {
                gl.buffer_data_u8_slice(target, data, match desc.usage {
                    _ if desc.usage.contains(BufferUsage::DYNAMIC) => glow::DYNAMIC_DRAW,
                    _ if desc.usage.contains(BufferUsage::CPU_READ) => glow::STREAM_READ,
                    _ => glow::STATIC_DRAW,
                });
            } else {
                gl.buffer_data_size(target, size as i32, glow::STATIC_DRAW);
            }
            
            gl.bind_buffer(target, None);
        }

        let buffer = Arc::new(GlBuffer {
            id: buffer_id,
            desc: desc.clone(),
            size,
        });

        self.buffers.push(Some(Arc::clone(&buffer)));
        Ok(buffer)
    }

    fn create_texture(&mut self, desc: &TextureDesc, initial_data: Option<&[u8]>) -> RhiResult<Arc<Self::Texture>> {
        let gl = &self.context;
        
        let texture_id = unsafe { gl.create_texture() }
            .map_err(|e| RhiError::CreationFailed(format!("Failed to create texture: {:?}", e)))?;

        let (internal_format, format, ty) = self.gl_format(desc.format);

        unsafe {
            gl.bind_texture(glow::TEXTURE_2D, Some(texture_id));
            
            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                internal_format as i32,
                desc.width as i32,
                desc.height as i32,
                0,
                format,
                ty,
                initial_data,
            );

            gl.generate_mipmap(glow::TEXTURE_2D);
            gl.bind_texture(glow::TEXTURE_2D, None);
        }

        let texture = Arc::new(GlTexture {
            id: texture_id,
            desc: desc.clone(),
            format: desc.format,
            width: desc.width,
            height: desc.height,
            mip_levels: desc.mip_levels,
        });

        self.textures.push(Some(Arc::clone(&texture)));
        Ok(texture)
    }

    fn create_sampler(&mut self, desc: &SamplerDesc) -> RhiResult<Arc<Self::Sampler>> {
        let gl = &self.context;
        
        let sampler_id = unsafe { gl.create_sampler() }
            .map_err(|e| RhiError::CreationFailed(format!("Failed to create sampler: {:?}", e)))?;

        unsafe {
            gl.sampler_parameter_i32(sampler_id, glow::TEXTURE_MIN_FILTER, desc.min_filter as i32);
            gl.sampler_parameter_i32(sampler_id, glow::TEXTURE_MAG_FILTER, desc.mag_filter as i32);
            gl.sampler_parameter_i32(sampler_id, glow::TEXTURE_WRAP_S, desc.wrap_u as i32);
            gl.sampler_parameter_i32(sampler_id, glow::TEXTURE_WRAP_T, desc.wrap_v as i32);
        }

        let sampler = Arc::new(GlSampler {
            id: sampler_id,
            desc: desc.clone(),
        });

        self.samplers.push(Some(Arc::clone(&sampler)));
        Ok(sampler)
    }

    fn create_pipeline(&mut self, desc: &PipelineDesc) -> RhiResult<Arc<Self::Pipeline>> {
        let gl = &self.context;
        
        // TODO: Compile shaders from SPIR-V or use GLSL source
        // For now, create a placeholder program
        let program = unsafe { gl.create_program() }
            .ok_or_else(|| RhiError::CreationFailed("Failed to create program".to_string()))?;

        let vertex_array = unsafe { gl.create_vertex_array() }
            .map_err(|e| RhiError::CreationFailed(format!("Failed to create VAO: {:?}", e)))?;

        let pipeline = Arc::new(GlPipeline {
            program,
            vertex_array,
            desc: desc.clone(),
            uniform_locations: RefCell::new(std::collections::HashMap::new()),
        });

        self.pipelines.push(Some(Arc::clone(&pipeline)));
        Ok(pipeline)
    }

    fn create_command_list(&mut self) -> Self::CommandList {
        GlCommandList::new(Arc::clone(&self.context))
    }

    fn submit(&mut self, _command_list: &mut Self::CommandList) -> RhiResult<()> {
        // OpenGL executes commands immediately, so submission is a no-op
        Ok(())
    }

    fn wait_idle(&mut self) -> RhiResult<()> {
        unsafe { self.context.finish() };
        Ok(())
    }

    fn get_memory_stats(&self) -> crate::graphics::rhi::MemoryStats {
        // OpenGL doesn't provide direct memory stats
        crate::graphics::rhi::MemoryStats {
            used_buffer_memory: 0,
            used_texture_memory: 0,
            total_buffer_memory: 0,
            total_texture_memory: 0,
        }
    }
}

/// OpenGL Command List
pub struct GlCommandList {
    context: Arc<Context>,
    state: GlCommandListState,
}

impl GlCommandList {
    pub fn new(context: Arc<Context>) -> Self {
        Self {
            context,
            state: GlCommandListState::new(),
        }
    }
}

impl RhiCommandList for GlCommandList {
    fn begin(&mut self) {
        self.state = GlCommandListState::new();
    }

    fn end(&mut self) {}

    fn set_viewport(&mut self, viewport: &Viewport) {
        let gl = &self.context;
        unsafe {
            gl.viewport(viewport.x as i32, viewport.y as i32, viewport.width as i32, viewport.height as i32);
        }
        self.state.viewport = Some(viewport.clone());
    }

    fn set_scissor(&mut self, scissor: &ScissorRect) {
        let gl = &self.context;
        unsafe {
            gl.enable(glow::SCISSOR_TEST);
            gl.scissor(scissor.x as i32, scissor.y as i32, scissor.width as i32, scissor.height as i32);
        }
        self.state.scissor = Some(scissor.clone());
    }

    fn bind_pipeline(&mut self, pipeline: &Self::Pipeline) {
        let gl = &self.context;
        unsafe {
            gl.use_program(Some(pipeline.program));
            gl.bind_vertex_array(Some(pipeline.vertex_array));
        }
        self.state.bound_pipeline = Some(pipeline.program);
    }

    fn bind_vertex_buffer(&mut self, slot: u32, buffer: &Self::Buffer, offset: usize) {
        let gl = &self.context;
        let target = glow::ARRAY_BUFFER;
        
        unsafe {
            gl.bind_buffer(target, Some(buffer.id));
            // TODO: Set vertex attribute pointers based on pipeline layout
        }
        
        while self.state.bound_vertex_buffers.len() <= slot as usize {
            self.state.bound_vertex_buffers.push(None);
        }
        self.state.bound_vertex_buffers[slot as usize] = Some((buffer.id, offset));
    }

    fn bind_index_buffer(&mut self, buffer: &Self::Buffer, index_type: IndexType) {
        let gl = &self.context;
        let target = glow::ELEMENT_ARRAY_BUFFER;
        
        unsafe {
            gl.bind_buffer(target, Some(buffer.id));
        }
        
        self.state.bound_index_buffer = Some((buffer.id, index_type));
    }

    fn bind_texture(&mut self, slot: u32, texture: &Self::Texture, sampler: Option<&Self::Sampler>) {
        let gl = &self.context;
        
        unsafe {
            gl.active_texture(glow::TEXTURE0 + slot);
            gl.bind_texture(glow::TEXTURE_2D, Some(texture.id));
            
            if let Some(sampler) = sampler {
                gl.bind_sampler(slot, Some(sampler.id));
            }
        }
        
        while self.state.bound_textures.len() <= slot as usize {
            self.state.bound_textures.push(None);
        }
        self.state.bound_textures[slot as usize] = Some(texture.id);
    }

    fn set_blend_constants(&mut self, _color: &Color4f) {
        // TODO: Implement blend constants
    }

    fn clear_color(&mut self, color: &Color4f) {
        let gl = &self.context;
        unsafe {
            gl.clear_color(color.r, color.g, color.b, color.a);
            gl.clear(glow::COLOR_BUFFER_BIT);
        }
        self.state.clear_color = Some(color.clone());
    }

    fn clear_depth(&mut self, depth: f32) {
        let gl = &self.context;
        unsafe {
            gl.clear_depth_f32(depth);
            gl.clear(glow::DEPTH_BUFFER_BIT);
        }
    }

    fn clear_stencil(&mut self, stencil: i32) {
        let gl = &self.context;
        unsafe {
            gl.clear_stencil(stencil);
            gl.clear(glow::STENCIL_BUFFER_BIT);
        }
    }

    fn draw(&mut self, vertex_count: u32, instance_count: u32, first_vertex: u32, first_instance: u32) {
        let gl = &self.context;
        unsafe {
            if instance_count > 1 {
                gl.draw_arrays_instanced(
                    glow::TRIANGLES,
                    first_vertex as i32,
                    vertex_count as i32,
                    instance_count as i32,
                );
            } else {
                gl.draw_arrays(
                    glow::TRIANGLES,
                    first_vertex as i32,
                    vertex_count as i32,
                );
            }
        }
    }

    fn draw_indexed(&mut self, index_count: u32, instance_count: u32, first_index: u32, base_vertex: i32, first_instance: u32) {
        let gl = &self.context;
        let index_type = match self.state.bound_index_buffer {
            Some((_, IndexType::U16)) => glow::UNSIGNED_SHORT,
            Some((_, IndexType::U32)) => glow::UNSIGNED_INT,
            None => return, // No index buffer bound
        };
        
        let offset = (first_index * match index_type {
            glow::UNSIGNED_SHORT => 2,
            glow::UNSIGNED_INT => 4,
            _ => 1,
        }) as usize;
        
        unsafe {
            if instance_count > 1 {
                gl.draw_elements_instanced(
                    glow::TRIANGLES,
                    index_count as i32,
                    index_type,
                    offset as i32,
                    instance_count as i32,
                );
            } else {
                gl.draw_elements(
                    glow::TRIANGLES,
                    index_count as i32,
                    index_type,
                    offset as i32,
                );
            }
        }
    }

    fn dispatch(&mut self, _group_count_x: u32, _group_count_y: u32, _group_count_z: u32) {
        // Compute shaders not yet implemented in OpenGL backend
        unimplemented!("Compute shaders not yet implemented");
    }
}

/// Helper function to create a GL device from an existing glow context
pub fn create_gl_device(context: Arc<Context>) -> GlDevice {
    GlDevice::new(context)
}
