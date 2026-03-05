use std::sync::Arc;

// Основной интерфейс RHI (Render Hardware Interface)
pub trait Device {
    type Buffer;
    type Texture;
    type Shader;
    type PipelineState;
    type CommandBuffer;
    
    /// Создает буфер (вершинный, индексный, uniform)
    fn create_buffer(&self, desc: &BufferDesc) -> Result<Self::Buffer, RHIError>;
    
    /// Создает текстуру
    fn create_texture(&self, desc: &TextureDesc) -> Result<Self::Texture, RHIError>;
    
    /// Создает шейдер
    fn create_shader(&self, desc: &ShaderDesc) -> Result<Self::Shader, RHIError>;
    
    /// Создает pipeline state object
    fn create_pipeline(&self, desc: &PipelineDesc) -> Result<Self::PipelineState, RHIError>;
    
    /// Получает командный буфер для записи
    fn get_command_buffer(&self) -> Self::CommandBuffer;
    
    /// Отправляет команды на выполнение
    fn submit_commands(&self, cmd_buffer: Self::CommandBuffer);
    
    /// Ожидает завершения выполнения команд
    fn wait_idle(&self);
}

/// Описание буфера
#[derive(Debug, Clone)]
pub struct BufferDesc {
    pub size: u64,
    pub usage: BufferUsage,
    pub memory_type: MemoryType,
}

#[derive(Debug, Clone)]
pub enum BufferUsage {
    Vertex,
    Index,
    Uniform,
    Storage,
    Constant,
}

#[derive(Debug, Clone)]
pub enum MemoryType {
    /// Доступна GPU (быстрая)
    DeviceLocal,
    /// Доступна CPU и GPU (медленнее для GPU)
    HostVisible,
    /// Доступна CPU, копируется на GPU
    Upload,
}

/// Описание текстуры
#[derive(Debug, Clone)]
pub struct TextureDesc {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub format: TextureFormat,
    pub usage: TextureUsage,
    pub mip_levels: u32,
    pub array_layers: u32,
}

#[derive(Debug, Clone)]
pub enum TextureFormat {
    RGBA8Unorm,
    RGBA8Srgb,
    BGRA8Unorm,
    BGRA8Srgb,
    R32Float,
    R32G32Float,
    R32G32B32Float,
    R32G32B32A32Float,
    D24UnormS8Uint,
    D32FloatS8Uint,
}

#[derive(Debug, Clone)]
pub enum TextureUsage {
    Sampled,
    Storage,
    ColorAttachment,
    DepthStencilAttachment,
    TransferSrc,
    TransferDst,
}

/// Описание шейдера
#[derive(Debug, Clone)]
pub struct ShaderDesc {
    pub stage: ShaderStage,
    pub code: Vec<u32>, // SPIR-V bytecode
    pub entry_point: String,
}

#[derive(Debug, Clone)]
pub enum ShaderStage {
    Vertex,
    Fragment,
    Geometry,
    Compute,
    Hull,
    Domain,
}

/// Описание pipeline
#[derive(Debug, Clone)]
pub struct PipelineDesc {
    pub vertex_shader: Arc<dyn Shader>,
    pub fragment_shader: Arc<dyn Shader>,
    pub geometry_shader: Option<Arc<dyn Shader>>,
    pub hull_shader: Option<Arc<dyn Shader>>,
    pub domain_shader: Option<Arc<dyn Shader>>,
    pub input_layout: InputLayout,
    pub rasterizer_state: RasterizerState,
    pub blend_state: BlendState,
    pub depth_stencil_state: DepthStencilState,
    pub primitive_topology: PrimitiveTopology,
}

#[derive(Debug, Clone)]
pub struct InputLayout {
    pub elements: Vec<InputElement>,
}

#[derive(Debug, Clone)]
pub struct InputElement {
    pub semantic: String,
    pub format: InputFormat,
    pub offset: u32,
    pub binding: u32,
}

#[derive(Debug, Clone)]
pub enum InputFormat {
    Float,
    Float2,
    Float3,
    Float4,
    Byte4,
    UByte4,
    Short2,
    UShort2,
    Short4,
    UShort4,
}

#[derive(Debug, Clone)]
pub struct RasterizerState {
    pub fill_mode: FillMode,
    pub cull_mode: CullMode,
    pub front_face: FrontFace,
    pub depth_bias: f32,
    pub depth_bias_clamp: f32,
    pub slope_scaled_depth_bias: f32,
    pub depth_clip_enable: bool,
    pub scissor_enable: bool,
    pub multisample_enable: bool,
    pub antialiased_line_enable: bool,
}

#[derive(Debug, Clone)]
pub enum FillMode {
    Solid,
    Wireframe,
}

#[derive(Debug, Clone)]
pub enum CullMode {
    None,
    Front,
    Back,
}

#[derive(Debug, Clone)]
pub enum FrontFace {
    Clockwise,
    CounterClockwise,
}

#[derive(Debug, Clone)]
pub struct BlendState {
    pub render_targets: Vec<RenderTargetBlendDesc>,
}

#[derive(Debug, Clone)]
pub struct RenderTargetBlendDesc {
    pub blend_enable: bool,
    pub src_blend: Blend,
    pub dest_blend: Blend,
    pub blend_op: BlendOp,
    pub src_blend_alpha: Blend,
    pub dest_blend_alpha: Blend,
    pub blend_op_alpha: BlendOp,
    pub render_target_write_mask: u8,
}

#[derive(Debug, Clone)]
pub enum Blend {
    Zero,
    One,
    SrcColor,
    InvSrcColor,
    SrcAlpha,
    InvSrcAlpha,
    DestAlpha,
    InvDestAlpha,
    DestColor,
    InvDestColor,
    SrcAlphaSat,
    BlendFactor,
    InvBlendFactor,
    Src1Color,
    InvSrc1Color,
    Src1Alpha,
    InvSrc1Alpha,
}

#[derive(Debug, Clone)]
pub enum BlendOp {
    Add,
    Subtract,
    RevSubtract,
    Min,
    Max,
}

#[derive(Debug, Clone)]
pub struct DepthStencilState {
    pub depth_enable: bool,
    pub depth_write_mask: DepthWriteMask,
    pub depth_func: ComparisonFunc,
    pub stencil_enable: bool,
    pub stencil_read_mask: u8,
    pub stencil_write_mask: u8,
    pub front_face: StencilOpDesc,
    pub back_face: StencilOpDesc,
}

#[derive(Debug, Clone)]
pub enum DepthWriteMask {
    Zero,
    All,
}

#[derive(Debug, Clone)]
pub enum ComparisonFunc {
    Never,
    Less,
    Equal,
    LessEqual,
    Greater,
    NotEqual,
    GreaterEqual,
    Always,
}

#[derive(Debug, Clone)]
pub struct StencilOpDesc {
    pub stencil_fail_op: StencilOp,
    pub stencil_depth_fail_op: StencilOp,
    pub stencil_pass_op: StencilOp,
    pub stencil_func: ComparisonFunc,
}

#[derive(Debug, Clone)]
pub enum StencilOp {
    Keep,
    Zero,
    Replace,
    IncrSat,
    DecrSat,
    Invert,
    Incr,
    Decr,
}

#[derive(Debug, Clone)]
pub enum PrimitiveTopology {
    TriangleList,
    TriangleStrip,
    LineList,
    LineStrip,
    PointList,
}

/// Ошибка RHI
#[derive(Debug)]
pub enum RHIError {
    InvalidArgument(String),
    OutOfMemory,
    DeviceLost,
    InitializationFailed(String),
}

/// Командный буфер
pub trait CommandBuffer {
    /// Устанавливает pipeline
    fn set_pipeline(&mut self, pipeline: &dyn PipelineState);
    
    /// Устанавливает буферы вершин
    fn set_vertex_buffers(&mut self, start_slot: u32, buffers: &[&dyn Buffer]);
    
    /// Устанавливает индексный буфер
    fn set_index_buffer(&mut self, buffer: &dyn Buffer, format: IndexFormat, offset: u64);
    
    /// Устанавливает дескрипторы
    fn set_descriptor_sets(&mut self, first_set: u32, sets: &[&dyn DescriptorSet]);
    
    /// Рисует
    fn draw(&mut self, vertex_count: u32, instance_count: u32, first_vertex: u32, first_instance: u32);
    
    /// Рисует индексированные вершины
    fn draw_indexed(&mut self, index_count: u32, instance_count: u32, first_index: u32, vertex_offset: i32, first_instance: u32);
    
    /// Очищает рендер таргет
    fn clear_render_target_view(&mut self, rtv: &dyn RenderTargetView, color: [f32; 4]);
    
    /// Очищает глубинный буфер
    fn clear_depth_stencil_view(&mut self, dsv: &dyn DepthStencilView, clear_flags: ClearFlags, depth: f32, stencil: u8);
    
    /// Обновляет буфер
    fn update_buffer(&mut self, dst_buffer: &dyn Buffer, dst_offset: u64, data: &[u8]);
    
    /// Копирует буфер
    fn copy_buffer(&mut self, src_buffer: &dyn Buffer, dst_buffer: &dyn Buffer);
    
    /// Копирует текстуру
    fn copy_texture(&mut self, src_texture: &dyn Texture, dst_texture: &dyn Texture);
    
    /// Переход состояния ресурса
    fn resource_barrier(&mut self, barriers: &[ResourceBarrier]);
}

#[derive(Debug, Clone)]
pub enum IndexFormat {
    Uint16,
    Uint32,
}

pub trait RenderTargetView {}
pub trait DepthStencilView {}

#[derive(Debug, Clone)]
pub enum ClearFlags {
    Depth,
    Stencil,
    DepthStencil,
}

pub trait Buffer {}
pub trait Texture {}
pub trait Shader {}
pub trait PipelineState {}
pub trait DescriptorSet {}

#[derive(Debug, Clone)]
pub enum ResourceBarrier {
    Transition {
        resource: ResourceHandle,
        state_before: ResourceState,
        state_after: ResourceState,
    },
    UAV {
        resource: ResourceHandle,
    },
    Alias {
        resource_before: ResourceHandle,
        resource_after: ResourceHandle,
    },
}

#[derive(Debug, Clone)]
pub struct ResourceHandle {
    pub id: u64,
}

#[derive(Debug, Clone)]
pub enum ResourceState {
    Common,
    VertexAndConstantBuffer,
    IndexBuffer,
    RenderTarget,
    UnorderedAccess,
    DepthWrite,
    DepthRead,
    NonPixelShaderResource,
    PixelShaderResource,
    StreamOut,
    IndirectArgument,
    CopyDest,
    CopySource,
    ResolveDest,
    ResolveSource,
    Present,
    GenericRead,
    AccelerationStructure,
}