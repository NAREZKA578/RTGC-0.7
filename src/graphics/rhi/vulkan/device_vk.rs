// Vulkan Backend - Device Implementation
// Implements IDevice trait for Vulkan

use crate::graphics::rhi::{
    types::*,
    device::*,
};
use std::sync::Arc;

/// Vulkan Device implementation
pub struct VkDevice {
    #[cfg(feature = "vulkan")]
    entry: ash::Entry,
    
    #[cfg(feature = "vulkan")]
    instance: ash::Instance,
    
    #[cfg(feature = "vulkan")]
    physical_device: ash::vk::PhysicalDevice,
    
    #[cfg(feature = "vulkan")]
    device: ash::Device,
    
    #[cfg(feature = "vulkan")]
    queue_family_index: u32,
    
    features: DeviceFeatures,
    limits: DeviceLimits,
    name: String,
}

unsafe impl Send for VkDevice {}
unsafe impl Sync for VkDevice {}

impl VkDevice {
    /// Create a new Vulkan device
    pub fn new(enable_validation: bool) -> RhiResult<Self> {
        #[cfg(feature = "vulkan")]
        {
            use ash::vk;
            
            // Load Vulkan entry points
            let entry = unsafe { ash::Entry::load() }
                .map_err(|e| RhiError::InitializationFailed(format!("Failed to load Vulkan library: {}", e)))?;
            
            // Create Vulkan instance
            let app_info = vk::ApplicationInfo::builder()
                .application_name(cstr!("RTGC Engine"))
                .application_version(vk::make_api_version(0, 1, 0, 0))
                .engine_name(cstr!("RTGC"))
                .engine_version(vk::make_api_version(0, 0, 7, 0))
                .api_version(vk::API_VERSION_1_3);
            
            let mut enabled_layers = Vec::new();
            let mut enabled_extensions = vec![
                cstr!(ash::extensions::khr::Surface::NAME).as_ptr(),
            ];
            
            #[cfg(target_os = "windows")]
            enabled_extensions.push(cstr!(ash::extensions::khr::Win32Surface::NAME).as_ptr());
            
            #[cfg(target_os = "linux")]
            enabled_extensions.push(cstr!(ash::extensions::khr::XlibSurface::NAME).as_ptr());
            
            if enable_validation {
                enabled_layers.push(cstr!("VK_LAYER_KHRONOS_validation").as_ptr());
                enabled_extensions.push(cstr!(ash::extensions::ext::DebugUtils::NAME).as_ptr());
            }
            
            let create_info = vk::InstanceCreateInfo::builder()
                .application_info(&app_info)
                .enabled_layer_names(&enabled_layers)
                .enabled_extension_names(&enabled_extensions);
            
            let instance = unsafe { entry.create_instance(&create_info, None) }
                .map_err(|e| RhiError::InitializationFailed(format!("Failed to create Vulkan instance: {}", e)))?;
            
            // Find physical device
            let physical_devices = unsafe { instance.enumerate_physical_devices() }
                .map_err(|e| RhiError::InitializationFailed(format!("Failed to enumerate physical devices: {}", e)))?;
            
            let physical_device = physical_devices.into_iter()
                .find(|&device| Self::is_suitable_device(&instance, device))
                .ok_or_else(|| RhiError::InitializationFailed("No suitable Vulkan device found".to_string()))?;
            
            // Get queue family index
            let queue_family_index = Self::find_queue_family(&instance, physical_device);
            
            // Create logical device
            let priorities = [1.0f32];
            let queue_info = vk::DeviceQueueCreateInfo::builder()
                .queue_family_index(queue_family_index)
                .queue_priorities(&priorities);
            
            let mut enabled_features = vk::PhysicalDeviceFeatures::builder()
                .fill_mode_non_solid(true)
                .multi_draw_indirect(true)
                .draw_indirect_first_instance(true)
                .depth_bounds(false)
                .wide_lines(false)
                .large_points(false)
                .alpha_to_one(false)
                .logic_op(false)
                .multi_viewport(false);
            
            let mut enabled_extensions = vec![
                cstr!(ash::extensions::khr::Swapchain::NAME).as_ptr(),
            ];
            
            let device_info = vk::DeviceCreateInfo::builder()
                .queue_create_infos(std::slice::from_ref(&queue_info))
                .enabled_extension_names(&enabled_extensions)
                .enabled_features(&enabled_features);
            
            let device = unsafe { instance.create_device(physical_device, &device_info, None) }
                .map_err(|e| RhiError::InitializationFailed(format!("Failed to create logical device: {}", e)))?;
            
            // Query device properties
            let device_properties = unsafe { instance.get_physical_device_properties(physical_device) };
            let name = String::from_utf8_lossy(&device_properties.device_name[..])
                .trim_end_matches('\0')
                .to_string();
            
            Ok(Self {
                entry,
                instance,
                physical_device,
                device,
                queue_family_index,
                features: Self::query_features(),
                limits: Self::query_limits(&instance, physical_device),
                name,
            })
        }
        
        #[cfg(not(feature = "vulkan"))]
        {
            Err(RhiError::Unsupported("Vulkan feature not enabled".to_string()))
        }
    }
    
    #[cfg(feature = "vulkan")]
    fn is_suitable_device(instance: &ash::Instance, device: ash::vk::PhysicalDevice) -> bool {
        use ash::vk;
        
        let props = unsafe { instance.get_physical_device_properties(device) };
        let features = unsafe { instance.get_physical_device_features(device) };
        
        // Check if discrete GPU or integrated GPU
        let is_discrete = props.device_type == vk::PhysicalDeviceType::DISCRETE_GPU;
        let is_integrated = props.device_type == vk::PhysicalDeviceType::INTEGRATED_GPU;
        
        // Must support geometry shaders and have minimum required features
        let has_required_features = features.geometry_shader == vk::TRUE &&
                                    features.multi_draw_indirect == vk::TRUE &&
                                    features.fill_mode_non_solid == vk::TRUE;
        
        (is_discrete || is_integrated) && has_required_features
    }
    
    #[cfg(feature = "vulkan")]
    fn find_queue_family(instance: &ash::Instance, physical_device: ash::vk::PhysicalDevice) -> u32 {
        use ash::vk;
        
        let queue_families = unsafe { instance.get_physical_device_queue_family_properties(physical_device) };
        
        queue_families.iter().position(|q| {
            q.queue_flags.contains(vk::QueueFlags::GRAPHICS | vk::QueueFlags::COMPUTE)
        }).unwrap_or(0) as u32
    }
    
    fn query_features() -> DeviceFeatures {
        DeviceFeatures {
            anisotropic_filtering: true,
            bc_compression: true,
            compute_shaders: true,
            geometry_shaders: true,
            tessellation: true,
            conservative_rasterization: false, // Optional in Vulkan
            multi_draw_indirect: true,
            draw_indirect_first_instance: true,
            dual_source_blending: true,
            depth_bounds_test: true,
            sample_rate_shading: true,
            texture_cube_map_array: true,
            texture_3d_as_2d_array: true,
            independent_blend: true,
            logic_op: true,
            occlusion_query: true,
            timestamp_query: true,
            pipeline_statistics_query: true,
            stream_output: false, // Not in Vulkan
            variable_rate_shading: false, // Optional
            mesh_shaders: false, // Optional (Vulkan 1.3+)
            ray_tracing: false, // Optional extension
            sampler_lod_bias: true,
            border_color_clamp: true,
        }
    }
    
    #[cfg(feature = "vulkan")]
    fn query_limits(instance: &ash::Instance, physical_device: ash::vk::PhysicalDevice) -> DeviceLimits {
        use ash::vk;
        
        let props = unsafe { instance.get_physical_device_properties(physical_device) };
        let limits = props.limits;
        
        DeviceLimits {
            max_texture_dimension_1d: limits.max_image_dimension1_d,
            max_texture_dimension_2d: limits.max_image_dimension2_d,
            max_texture_dimension_3d: limits.max_image_dimension3_d,
            max_texture_array_layers: limits.max_image_array_layers,
            max_buffer_size: limits.max_storage_buffer_range as u64,
            max_vertex_input_attributes: limits.max_vertex_input_attributes,
            max_vertex_input_bindings: limits.max_vertex_input_bindings,
            max_vertex_input_attribute_offset: limits.max_vertex_input_attribute_offset,
            max_vertex_input_binding_stride: limits.max_vertex_input_binding_stride,
            max_vertex_output_components: limits.max_vertex_output_components,
            max_fragment_input_components: limits.max_fragment_input_components,
            max_fragment_output_attachments: limits.max_fragment_output_attachments,
            max_compute_work_group_count: limits.max_compute_work_group_count,
            max_compute_work_group_invocations: limits.max_compute_work_group_invocations,
            max_compute_shared_memory_size: limits.max_compute_shared_memory_size,
            max_uniform_buffer_range: limits.max_uniform_buffer_range,
            max_storage_buffer_range: limits.max_storage_buffer_range,
            max_sampler_anisotropy: limits.max_sampler_anisotropy as f32,
            min_texel_buffer_offset_alignment: limits.min_texel_buffer_offset_alignment,
            min_uniform_buffer_offset_alignment: limits.min_uniform_buffer_offset_alignment,
            min_storage_buffer_offset_alignment: limits.min_storage_buffer_offset_alignment,
            max_descriptor_set_samplers: limits.max_descriptor_set_samplers,
            max_descriptor_set_uniform_buffers: limits.max_descriptor_set_uniform_buffers,
            max_descriptor_set_storage_buffers: limits.max_descriptor_set_storage_buffers,
            max_descriptor_set_textures: limits.max_descriptor_set_sampled_images,
            max_descriptor_set_storage_images: limits.max_descriptor_set_storage_images,
            max_per_stage_descriptor_samplers: limits.max_per_stage_descriptor_samplers,
            max_per_stage_descriptor_uniform_buffers: limits.max_per_stage_descriptor_uniform_buffers,
            max_per_stage_descriptor_storage_buffers: limits.max_per_stage_descriptor_storage_buffers,
            max_per_stage_descriptor_textures: limits.max_per_stage_descriptor_sampled_images,
            max_per_stage_descriptor_storage_images: limits.max_per_stage_descriptor_storage_images,
        }
    }
    
    #[cfg(not(feature = "vulkan"))]
    fn query_limits(_instance: &ash::Instance, _physical_device: ash::vk::PhysicalDevice) -> DeviceLimits {
        DeviceLimits::default()
    }
    
    #[cfg(feature = "vulkan")]
    fn to_vk_address_mode(address: TextureAddressMode) -> ash::vk::SamplerAddressMode {
        use ash::vk;
        
        match address {
            TextureAddressMode::Wrap => vk::SamplerAddressMode::REPEAT,
            TextureAddressMode::Clamp => vk::SamplerAddressMode::CLAMP_TO_EDGE,
            TextureAddressMode::Border => vk::SamplerAddressMode::CLAMP_TO_BORDER,
            TextureAddressMode::Mirror => vk::SamplerAddressMode::MIRRORED_REPEAT,
            TextureAddressMode::MirrorOnce => vk::SamplerAddressMode::MIRROR_CLAMP_TO_EDGE,
        }
    }
}

impl IDevice for VkDevice {
    fn get_device_name(&self) -> &str {
        &self.name
    }
    
    fn get_features(&self) -> DeviceFeatures {
        self.features.clone()
    }
    
    fn get_limits(&self) -> DeviceLimits {
        self.limits.clone()
    }
    
    fn create_buffer(&self, desc: &BufferDescription) -> RhiResult<ResourceHandle> {
        #[cfg(feature = "vulkan")]
        {
            use ash::vk;
            
            let handle = ResourceHandle::new();
            let buffer = VkBuffer::new(&self.device, self.physical_device, desc, handle)?;
            
            // Store buffer in a resource manager (TODO: implement proper resource tracking)
            // For now, we just return the handle
            
            Ok(handle)
        }
        
        #[cfg(not(feature = "vulkan"))]
        {
            Err(RhiError::Unsupported("Vulkan feature not enabled".to_string()))
        }
    }
    
    fn create_texture(&self, desc: &TextureDescription) -> RhiResult<ResourceHandle> {
        #[cfg(feature = "vulkan")]
        {
            let handle = ResourceHandle::new();
            let texture = VkTexture::new(&self.device, self.physical_device, desc, handle)?;
            Ok(handle)
        }
        
        #[cfg(not(feature = "vulkan"))]
        {
            Err(RhiError::Unsupported("Vulkan feature not enabled".to_string()))
        }
    }
    
    fn create_texture_view(
        &self,
        texture: ResourceHandle,
        desc: &TextureViewDescription,
    ) -> RhiResult<ResourceHandle> {
        #[cfg(feature = "vulkan")]
        {
            use ash::vk;
            
            let handle = ResourceHandle::new();
            
            // TODO: Get actual image from texture handle
            // For now, return error
            Err(RhiError::ResourceCreationFailed("Texture view creation requires texture lookup".to_string()))
        }
        
        #[cfg(not(feature = "vulkan"))]
        {
            Err(RhiError::Unsupported("Vulkan feature not enabled".to_string()))
        }
    }
    
    fn create_sampler(&self, desc: &SamplerDescription) -> RhiResult<ResourceHandle> {
        #[cfg(feature = "vulkan")]
        {
            use ash::vk;
            
            let handle = ResourceHandle::new();
            
            let mag_filter = match desc.mag_filter {
                FilterMode::Point => vk::Filter::NEAREST,
                FilterMode::Linear => vk::Filter::LINEAR,
                FilterMode::Anisotropic => vk::Filter::LINEAR,
            };
            
            let min_filter = match desc.min_filter {
                FilterMode::Point => vk::Filter::NEAREST,
                FilterMode::Linear => vk::Filter::LINEAR,
                FilterMode::Anisotropic => vk::Filter::LINEAR,
            };
            
            let mipmap_mode = match desc.mip_filter {
                FilterMode::Point => vk::SamplerMipmapMode::NEAREST,
                _ => vk::SamplerMipmapMode::LINEAR,
            };
            
            let address_mode_u = Self::to_vk_address_mode(desc.address_u);
            let address_mode_v = Self::to_vk_address_mode(desc.address_v);
            let address_mode_w = Self::to_vk_address_mode(desc.address_w);
            
            let sampler_info = vk::SamplerCreateInfo::builder()
                .mag_filter(mag_filter)
                .min_filter(min_filter)
                .mipmap_mode(mipmap_mode)
                .address_mode_u(address_mode_u)
                .address_mode_v(address_mode_v)
                .address_mode_w(address_mode_w)
                .mip_lod_bias(desc.mip_lod_bias)
                .anisotropy_enable(desc.anisotropic_filtering)
                .max_anisotropy(desc.max_anisotropy)
                .compare_enable(false)
                .min_lod(desc.min_lod)
                .max_lod(desc.max_lod)
                .border_color(vk::BorderColor::INT_OPAQUE_BLACK);
            
            let sampler = unsafe {
                self.device.create_sampler(&sampler_info, None)
                    .map_err(|e| RhiError::ResourceCreationFailed(format!("Failed to create sampler: {:?}", e)))?
            };
            
            Ok(handle)
        }
        
        #[cfg(not(feature = "vulkan"))]
        {
            Err(RhiError::Unsupported("Vulkan feature not enabled".to_string()))
        }
    }
    
    fn create_shader(&self, desc: &ShaderDescription) -> RhiResult<ResourceHandle> {
        #[cfg(feature = "vulkan")]
        {
            use ash::vk;
            
            let handle = ResourceHandle::new();
            
            let shader_info = vk::ShaderModuleCreateInfo::builder()
                .code(desc.code);
            
            let shader_module = unsafe {
                self.device.create_shader_module(&shader_info, None)
                    .map_err(|e| RhiError::ResourceCreationFailed(format!("Failed to create shader module: {:?}", e)))?
            };
            
            Ok(handle)
        }
        
        #[cfg(not(feature = "vulkan"))]
        {
            Err(RhiError::Unsupported("Vulkan feature not enabled".to_string()))
        }
    }
    
    fn create_pipeline_state(&self, desc: &PipelineStateObject) -> RhiResult<ResourceHandle> {
        #[cfg(feature = "vulkan")]
        {
            use ash::vk;
            
            let handle = ResourceHandle::new();
            
            // TODO: Implement full pipeline creation
            // This requires shader stages, vertex input, input assembly, 
            // viewport/scissor, rasterizer, multisample, depth/stencil, color blend
            
            Ok(handle)
        }
        
        #[cfg(not(feature = "vulkan"))]
        {
            Err(RhiError::Unsupported("Vulkan feature not enabled".to_string()))
        }
    }
    
    fn create_descriptor_heap(&self, desc: &DescriptorHeapDescription) -> RhiResult<ResourceHandle> {
        #[cfg(feature = "vulkan")]
        {
            use ash::vk;
            
            let handle = ResourceHandle::new();
            
            // In Vulkan, we create descriptor pools and sets instead of heaps
            let mut pool_sizes = Vec::new();
            
            if desc.num_samplers > 0 {
                pool_sizes.push(vk::DescriptorPoolSize {
                    ty: vk::DescriptorType::SAMPLER,
                    descriptor_count: desc.num_samplers,
                });
            }
            
            if desc.num_uniform_buffers > 0 {
                pool_sizes.push(vk::DescriptorPoolSize {
                    ty: vk::DescriptorType::UNIFORM_BUFFER,
                    descriptor_count: desc.num_uniform_buffers,
                });
            }
            
            if desc.num_storage_buffers > 0 {
                pool_sizes.push(vk::DescriptorPoolSize {
                    ty: vk::DescriptorType::STORAGE_BUFFER,
                    descriptor_count: desc.num_storage_buffers,
                });
            }
            
            if desc.num_textures > 0 {
                pool_sizes.push(vk::DescriptorPoolSize {
                    ty: vk::DescriptorType::SAMPLED_IMAGE,
                    descriptor_count: desc.num_textures,
                });
            }
            
            if desc.num_storage_textures > 0 {
                pool_sizes.push(vk::DescriptorPoolSize {
                    ty: vk::DescriptorType::STORAGE_IMAGE,
                    descriptor_count: desc.num_storage_textures,
                });
            }
            
            let total_descriptors = desc.num_samplers + desc.num_uniform_buffers + 
                                  desc.num_storage_buffers + desc.num_textures + 
                                  desc.num_storage_textures;
            
            let pool_info = vk::DescriptorPoolCreateInfo::builder()
                .pool_sizes(&pool_sizes)
                .max_sets(total_descriptors)
                .flags(vk::DescriptorPoolCreateFlags::FREE_DESCRIPTOR_SET);
            
            let descriptor_pool = unsafe {
                self.device.create_descriptor_pool(&pool_info, None)
                    .map_err(|e| RhiError::ResourceCreationFailed(format!("Failed to create descriptor pool: {:?}", e)))?
            };
            
            Ok(handle)
        }
        
        #[cfg(not(feature = "vulkan"))]
        {
            Err(RhiError::Unsupported("Vulkan feature not enabled".to_string()))
        }
    }
    
    fn create_command_list(&self, cmd_type: CommandListType) -> RhiResult<Arc<dyn ICommandList>> {
        #[cfg(feature = "vulkan")]
        {
            let cmd_list = VkCommandList::new(&self.device, self.queue_family_index, cmd_type)?;
            Ok(Arc::new(cmd_list))
        }
        
        #[cfg(not(feature = "vulkan"))]
        {
            Err(RhiError::Unsupported("Vulkan feature not enabled".to_string()))
        }
    }
    
    fn create_command_queue(&self, cmd_type: CommandListType) -> RhiResult<Arc<dyn ICommandQueue>> {
        #[cfg(feature = "vulkan")]
        {
            let queue = unsafe { self.device.get_device_queue(self.queue_family_index, 0) };
            let cmd_queue = VkCommandQueue::new(queue, cmd_type);
            Ok(Arc::new(cmd_queue))
        }
        
        #[cfg(not(feature = "vulkan"))]
        {
            Err(RhiError::Unsupported("Vulkan feature not enabled".to_string()))
        }
    }
    
    fn create_fence(&self, initial_value: u64) -> RhiResult<Arc<dyn IFence>> {
        #[cfg(feature = "vulkan")]
        {
            use ash::vk;
            
            let fence_info = vk::FenceCreateInfo::builder()
                .flags(if initial_value > 0 { 
                    vk::FenceCreateFlags::SIGNALED 
                } else { 
                    vk::FenceCreateFlags::empty() 
                });
            
            let fence = unsafe {
                self.device.create_fence(&fence_info, None)
                    .map_err(|e| RhiError::ResourceCreationFailed(format!("Failed to create fence: {:?}", e)))?
            };
            
            let vk_fence = VkFence::new(fence);
            Ok(Arc::new(vk_fence))
        }
        
        #[cfg(not(feature = "vulkan"))]
        {
            Err(RhiError::Unsupported("Vulkan feature not enabled".to_string()))
        }
    }
    
    fn create_semaphore(&self) -> RhiResult<Arc<dyn ISemaphore>> {
        #[cfg(feature = "vulkan")]
        {
            use ash::vk;
            
            let semaphore_info = vk::SemaphoreCreateInfo::builder();
            
            let semaphore = unsafe {
                self.device.create_semaphore(&semaphore_info, None)
                    .map_err(|e| RhiError::ResourceCreationFailed(format!("Failed to create semaphore: {:?}", e)))?
            };
            
            let vk_semaphore = VkSemaphore::new(semaphore);
            Ok(Arc::new(vk_semaphore))
        }
        
        #[cfg(not(feature = "vulkan"))]
        {
            Err(RhiError::Unsupported("Vulkan feature not enabled".to_string()))
        }
    }
    
    fn create_swap_chain(
        &self,
        window_handle: *mut std::ffi::c_void,
        width: u32,
        height: u32,
        format: TextureFormat,
        vsync: bool,
    ) -> RhiResult<Arc<dyn ISwapChain>> {
        #[cfg(feature = "vulkan")]
        {
            use ash::vk;
            
            // Create surface based on platform
            #[cfg(target_os = "windows")]
            let surface = unsafe {
                use ash::extensions::khr::Win32Surface;
                
                let win32_surface = Win32Surface::new(&self.entry, &self.instance);
                
                let surface_info = vk::Win32SurfaceCreateInfoKHR::builder()
                    .hwnd(window_handle)
                    .hinstance(std::ptr::null_mut());
                
                win32_surface.create_win32_surface(&surface_info, None)?
            };
            
            #[cfg(target_os = "linux")]
            let surface = unsafe {
                use ash::extensions::khr::XlibSurface;
                
                let xlib_surface = XlibSurface::new(&self.entry, &self.instance);
                
                let surface_info = vk::XlibSurfaceCreateInfoKHR::builder()
                    .window(window_handle as u64)
                    .dpy(std::ptr::null_mut());
                
                xlib_surface.create_xlib_surface(&surface_info, None)?
            };
            
            let swapchain = VkSwapChain::new(
                &self.instance,
                &self.device,
                self.physical_device,
                surface,
                width,
                height,
                format,
                vsync,
            )?;
            
            Ok(Arc::new(swapchain))
        }
        
        #[cfg(not(feature = "vulkan"))]
        {
            Err(RhiError::Unsupported("Vulkan feature not enabled".to_string()))
        }
    }
    
    fn update_buffer(
        &self,
        buffer: ResourceHandle,
        offset: u64,
        data: &[u8],
    ) -> RhiResult<()> {
        #[cfg(feature = "vulkan")]
        {
            // TODO: Implement buffer update via staging buffer or mapping
            Err(RhiError::Unsupported("Buffer update via staging buffer not yet implemented".to_string()))
        }
        
        #[cfg(not(feature = "vulkan"))]
        {
            Err(RhiError::Unsupported("Vulkan feature not enabled".to_string()))
        }
    }
    
    fn map_buffer(&self, buffer: ResourceHandle) -> RhiResult<*mut u8> {
        #[cfg(feature = "vulkan")]
        {
            // TODO: Implement buffer mapping (requires HOST_VISIBLE memory)
            Err(RhiError::Unsupported("Buffer mapping not yet implemented".to_string()))
        }
        
        #[cfg(not(feature = "vulkan"))]
        {
            Err(RhiError::Unsupported("Vulkan feature not enabled".to_string()))
        }
    }
    
    fn unmap_buffer(&self, buffer: ResourceHandle) {
        // TODO: Implement
    }
    
    fn read_back_texture(&self, texture: ResourceHandle) -> RhiResult<Vec<u8>> {
        #[cfg(feature = "vulkan")]
        {
            // TODO: Implement texture readback via staging buffer
            Err(RhiError::Unsupported("Texture readback not yet implemented".to_string()))
        }
        
        #[cfg(not(feature = "vulkan"))]
        {
            Err(RhiError::Unsupported("Vulkan feature not enabled".to_string()))
        }
    }
    
    fn destroy_resource(&self, handle: ResourceHandle) {
        #[cfg(feature = "vulkan")]
        {
            // TODO: Implement proper resource destruction with tracking
            // Need to track all created resources and destroy them properly
        }
    }
    
    fn wait_idle(&self) -> RhiResult<()> {
        #[cfg(feature = "vulkan")]
        unsafe {
            self.device.device_wait_idle()
                .map_err(|e| RhiError::DeviceLost)?;
        }
        Ok(())
    }
    
    fn get_memory_stats(&self) -> MemoryStats {
        // TODO: Query actual memory stats from Vulkan
        MemoryStats::default()
    }
}

/// Factory function to create Vulkan device
pub fn create_vulkan_device(enable_validation: bool) -> RhiResult<Box<dyn IDevice>> {
    let device = VkDevice::new(enable_validation)?;
    Ok(Box::new(device))
}

// Helper for C string literals
#[cfg(feature = "vulkan")]
fn cstr(s: &'static str) -> &'static std::ffi::CStr {
    use std::ffi::CStr;
    CStr::from_bytes_with_nul(s.as_bytes()).unwrap()
}
