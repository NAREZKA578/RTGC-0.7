use crate::render::rhi::*;

/// HDR (High Dynamic Range) рендеринг и PBR (Physically Based Rendering) материалы
/// Модуль реализует полный цикл HDR рендеринга с тональной компрессией

/// Типы тональных операторов для HDR -> LDR конвертации
#[derive(Debug, Clone, Copy)]
pub enum TonemappingOperator {
    /// Reinhard - классический оператор, простой и быстрый
    Reinhard,
    /// Reinhard Extended - с контролем белого уровня
    ReinhardExtended,
    /// ACES Filmic - кинематографичный, наиболее популярный в играх
    ACESFilmic,
    /// Uncharted 2 - используется в Naughty Dog играх
    Uncharted2,
    /// Hejl-Dawson - оптимизированный ACES
    HejlDawson,
    /// Neutral - минимальное влияние на цвета
    Neutral,
}

/// Настройки пост-обработки HDR
#[derive(Debug, Clone)]
pub struct HDRPostProcessSettings {
    /// Оператор тональной компрессии
    pub tonemapping: TonemappingOperator,
    /// Экспозиция (EV stops)
    pub exposure: f32,
    /// Gamma коррекция
    pub gamma: f32,
    /// Минимальная яркость (для Reinhard Extended)
    pub white_point: f32,
    /// Bloom порог
    pub bloom_threshold: f32,
    /// Bloom интенсивность
    pub bloom_intensity: f32,
    /// Bloom радиок (в пикселях)
    pub bloom_radius: u32,
    /// Lens flare включен
    pub lens_flare_enabled: bool,
    /// Lens flare интенсивность
    pub lens_flare_intensity: f32,
    /// Chromatic aberration сила
    pub chromatic_aberration: f32,
    /// Vignette интенсивность
    pub vignette_intensity: f32,
    /// Film grain интенсивность
    pub film_grain: f32,
}

impl Default for HDRPostProcessSettings {
    fn default() -> Self {
        Self {
            tonemapping: TonemappingOperator::ACESFilmic,
            exposure: 1.0,
            gamma: 2.2,
            white_point: 1.0,
            bloom_threshold: 0.8,
            bloom_intensity: 1.0,
            bloom_radius: 7,
            lens_flare_enabled: false,
            lens_flare_intensity: 0.5,
            chromatic_aberration: 0.0,
            vignette_intensity: 0.2,
            film_grain: 0.0,
        }
    }
}

/// PBR материал с полным набором параметров
#[derive(Debug, Clone)]
pub struct PBRMaterial {
    /// Альбедо/базовый цвет текстура (sRGB)
    pub albedo_texture: Option<String>,
    /// Альбедо фактор (RGBA, sRGB)
    pub albedo_factor: [f32; 4],
    /// Металличность текстура (R канал)
    pub metallic_texture: Option<String>,
    /// Металличность фактор (0.0 = диэлектрик, 1.0 = металл)
    pub metallic_factor: f32,
    /// Шероховатость текстура (R канал)
    pub roughness_texture: Option<String>,
    /// Шероховатость фактор (0.0 = зеркало, 1.0 = матовый)
    pub roughness_factor: f32,
    /// Нормаль мап текстура
    pub normal_texture: Option<String>,
    /// Масштаб нормалей
    pub normal_scale: f32,
    /// Ambient occlusion текстура (R канал)
    pub ao_texture: Option<String>,
    /// AO сила
    pub ao_strength: f32,
    /// Emissive текстура (HDR значения возможны)
    pub emissive_texture: Option<String>,
    /// Emissive цвет (HDR)
    pub emissive_color: [f32; 3],
    /// Emissive интенсивность (ниты)
    pub emissive_intensity: f32,
    /// IOR (Index of Refraction) для диэлектриков
    pub ior: f32,
    /// Анизотропия (для brushed металлов)
    pub anisotropy: f32,
    /// Анизотропный угол вращения
    pub anisotropy_rotation: f32,
    /// Clear coat слой (для лакированных поверхностей)
    pub clear_coat: f32,
    /// Clear coat шероховатость
    pub clear_coat_roughness: f32,
    /// Transmission (для прозрачных материалов типа стекла)
    pub transmission: f32,
    /// Thickness для volume absorption
    pub thickness: f32,
    /// IOR для transmission
    pub transmission_ior: f32,
    /// Sheen (для тканей)
    pub sheen: f32,
    /// Sheen цвет
    pub sheen_color: [f32; 3],
    /// Subsurface scattering вес
    pub subsurface: f32,
    /// Subsurface цвет
    pub subsurface_color: [f32; 3],
    /// Subsurface радиус рассеивания (RGB)
    pub subsurface_radius: [f32; 3],
}

impl Default for PBRMaterial {
    fn default() -> Self {
        Self {
            albedo_texture: None,
            albedo_factor: [1.0, 1.0, 1.0, 1.0],
            metallic_texture: None,
            metallic_factor: 0.0,
            roughness_texture: None,
            roughness_factor: 1.0,
            normal_texture: None,
            normal_scale: 1.0,
            ao_texture: None,
            ao_strength: 1.0,
            emissive_texture: None,
            emissive_color: [0.0, 0.0, 0.0],
            emissive_intensity: 0.0,
            ior: 1.5, // типичный для диэлектриков
            anisotropy: 0.0,
            anisotropy_rotation: 0.0,
            clear_coat: 0.0,
            clear_coat_roughness: 0.0,
            transmission: 0.0,
            thickness: 1.0,
            transmission_ior: 1.5,
            sheen: 0.0,
            sheen_color: [1.0, 1.0, 1.0],
            subsurface: 0.0,
            subsurface_color: [1.0, 1.0, 1.0],
            subsurface_radius: [1.0, 1.0, 1.0],
        }
    }
}

impl PBRMaterial {
    /// Создает новый PBR материал с заданными параметрами
    pub fn new() -> Self {
        Self::default()
    }

    /// Устанавливает preset для распространенных материалов
    pub fn with_preset(preset: MaterialPreset) -> Self {
        match preset {
            MaterialPreset::Plastic => Self {
                albedo_factor: [0.8, 0.8, 0.8, 1.0],
                metallic_factor: 0.0,
                roughness_factor: 0.5,
                ior: 1.5,
                ..Self::default()
            },
            MaterialPreset::Metal => Self {
                albedo_factor: [0.9, 0.9, 0.9, 1.0],
                metallic_factor: 1.0,
                roughness_factor: 0.3,
                ior: 1.5,
                ..Self::default()
            },
            MaterialPreset::Wood => Self {
                albedo_factor: [0.6, 0.4, 0.2, 1.0],
                metallic_factor: 0.0,
                roughness_factor: 0.7,
                ior: 1.5,
                ..Self::default()
            },
            MaterialPreset::Glass => Self {
                albedo_factor: [1.0, 1.0, 1.0, 1.0],
                metallic_factor: 0.0,
                roughness_factor: 0.0,
                ior: 1.5,
                transmission: 1.0,
                transmission_ior: 1.5,
                thickness: 0.1,
                ..Self::default()
            },
            MaterialPreset::Fabric => Self {
                albedo_factor: [0.7, 0.7, 0.7, 1.0],
                metallic_factor: 0.0,
                roughness_factor: 1.0,
                sheen: 1.0,
                sheen_color: [0.8, 0.8, 0.8],
                ..Self::default()
            },
            MaterialPreset::Skin => Self {
                albedo_factor: [0.9, 0.7, 0.6, 1.0],
                metallic_factor: 0.0,
                roughness_factor: 0.4,
                subsurface: 1.0,
                subsurface_color: [0.9, 0.6, 0.5],
                subsurface_radius: [0.7, 0.4, 0.2], // RGB scattering
                ..Self::default()
            },
            MaterialPreset::Water => Self {
                albedo_factor: [0.0, 0.1, 0.2, 1.0],
                metallic_factor: 0.0,
                roughness_factor: 0.1,
                ior: 1.33,
                transmission: 0.9,
                transmission_ior: 1.33,
                thickness: 10.0,
                ..Self::default()
            },
            MaterialPreset::Diamond => Self {
                albedo_factor: [1.0, 1.0, 1.0, 1.0],
                metallic_factor: 0.0,
                roughness_factor: 0.0,
                ior: 2.42,
                transmission: 1.0,
                transmission_ior: 2.42,
                thickness: 0.01,
                ..Self::default()
            },
        }
    }

    /// Вычисляет F0 (Fresnel reflectance at normal incidence)
    /// Для диэлектриков: ~0.04, для металлов:等于albedo
    pub fn get_f0(&self) -> [f32; 3] {
        if self.metallic_factor >= 0.99 {
            // Металл: F0等于albedo
            [
                self.albedo_factor[0],
                self.albedo_factor[1],
                self.albedo_factor[2],
            ]
        } else {
            // Диэлектрик: вычисляем из IOR
            let ior = self.ior;
            let f0 = ((ior - 1.0) / (ior + 1.0)).powi(2);
            [f0, f0, f0]
        }
    }
}

/// Presets для распространенных материалов
#[derive(Debug, Clone, Copy)]
pub enum MaterialPreset {
    Plastic,
    Metal,
    Wood,
    Glass,
    Fabric,
    Skin,
    Water,
    Diamond,
}

/// Uniform buffer для PBR материала (GPU layout)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PBRMaterialUniforms {
    pub albedo_factor: [f32; 4],
    pub metallic_roughness: [f32; 2], // metallic, roughness
    pub normal_scale: f32,
    pub ao_strength: f32,
    pub emissive_color: [f32; 3],
    pub emissive_intensity: f32,
    pub ior: f32,
    pub anisotropy: f32,
    pub anisotropy_rotation: f32,
    pub clear_coat: f32,
    pub clear_coat_roughness: f32,
    pub transmission: f32,
    pub thickness: f32,
    pub transmission_ior: f32,
    pub sheen: f32,
    pub sheen_color: [f32; 3],
    pub subsurface: f32,
    pub subsurface_color: [f32; 3],
    pub subsurface_radius: [f32; 3],
    pub padding: [f32; 3], // выравнивание до 256 байт
}

impl From<&PBRMaterial> for PBRMaterialUniforms {
    fn from(material: &PBRMaterial) -> Self {
        Self {
            albedo_factor: material.albedo_factor,
            metallic_roughness: [material.metallic_factor, material.roughness_factor],
            normal_scale: material.normal_scale,
            ao_strength: material.ao_strength,
            emissive_color: material.emissive_color,
            emissive_intensity: material.emissive_intensity,
            ior: material.ior,
            anisotropy: material.anisotropy,
            anisotropy_rotation: material.anisotropy_rotation,
            clear_coat: material.clear_coat,
            clear_coat_roughness: material.clear_coat_roughness,
            transmission: material.transmission,
            thickness: material.thickness,
            transmission_ior: material.transmission_ior,
            sheen: material.sheen,
            sheen_color: material.sheen_color,
            subsurface: material.subsurface,
            subsurface_color: material.subsurface_color,
            subsurface_radius: material.subsurface_radius,
            padding: [0.0; 3],
        }
    }
}

/// Тональная компрессия HDR -> LDR
pub struct Tonemapper {
    operator: TonemappingOperator,
    exposure: f32,
    gamma: f32,
    white_point: f32,
}

impl Tonemapper {
    pub fn new(settings: &HDRPostProcessSettings) -> Self {
        Self {
            operator: settings.tonemapping,
            exposure: settings.exposure,
            gamma: settings.gamma,
            white_point: settings.white_point,
        }
    }

    /// Применяет тональную компрессию к HDR цвету
    #[inline]
    pub fn tonemap(&self, hdr_color: [f32; 3]) -> [f32; 3] {
        // Применяем экспозицию
        let color = [
            hdr_color[0] * self.exposure,
            hdr_color[1] * self.exposure,
            hdr_color[2] * self.exposure,
        ];

        // Применяем выбранный оператор
        let compressed = match self.operator {
            TonemappingOperator::Reinhard => self.reinhard(color),
            TonemappingOperator::ReinhardExtended => self.reinhard_extended(color),
            TonemappingOperator::ACESFilmic => self.aces_filmic(color),
            TonemappingOperator::Uncharted2 => self.uncharted2(color),
            TonemappingOperator::HejlDawson => self.hejl_dawson(color),
            TonemappingOperator::Neutral => self.neutral(color),
        };

        // Gamma коррекция
        [
            compressed[0].powf(1.0 / self.gamma),
            compressed[1].powf(1.0 / self.gamma),
            compressed[2].powf(1.0 / self.gamma),
        ]
    }

    /// Reinhard tonemapping (простой и быстрый)
    #[inline]
    fn reinhard(&self, color: [f32; 3]) -> [f32; 3] {
        [
            color[0] / (1.0 + color[0]),
            color[1] / (1.0 + color[1]),
            color[2] / (1.0 + color[2]),
        ]
    }

    /// Reinhard Extended с контролем белого уровня
    #[inline]
    fn reinhard_extended(&self, color: [f32; 3]) -> [f32; 3] {
        let white = self.white_point;
        [
            color[0] * (1.0 + color[0] / white) / (1.0 + color[0]),
            color[1] * (1.0 + color[1] / white) / (1.0 + color[1]),
            color[2] * (1.0 + color[2] / white) / (1.0 + color[2]),
        ]
    }

    /// ACES Filmic (наиболее популярный в играх)
    #[inline]
    fn aces_filmic(&self, color: [f32; 3]) -> [f32; 3] {
        let a = 2.51;
        let b = 0.03;
        let c = 2.43;
        let d = 0.59;
        let e = 0.14;

        [
            (color[0] * (a * color[0] + b)) / (color[0] * (c * color[0] + d) + e),
            (color[1] * (a * color[1] + b)) / (color[1] * (c * color[1] + d) + e),
            (color[2] * (a * color[2] + b)) / (color[2] * (c * color[2] + d) + e),
        ]
    }

    /// Uncharted 2 tonemapping
    #[inline]
    fn uncharted2(&self, color: [f32; 3]) -> [f32; 3] {
        let a = 0.15;
        let b = 0.50;
        let c = 0.10;
        let d = 0.20;
        let e = 0.02;
        let f = 0.30;
        let w = 11.2;

        let tone = |x: f32| {
            ((x * (a * x + c * b) + d * e) / (x * (a * x + b) + d * f)) - e / f
        };

        let white_scale = 1.0 / tone(w);

        [
            tone(color[0]) * white_scale,
            tone(color[1]) * white_scale,
            tone(color[2]) * white_scale,
        ]
    }

    /// Hejl-Dawson (оптимизированный ACES)
    #[inline]
    fn hejl_dawson(&self, color: [f32; 3]) -> [f32; 3] {
        let x = color;
        [
            (x[0] * (0.6 * x[0] + 0.4)) / (x[0] * (0.6 * x[0] + 0.9) + 0.17),
            (x[1] * (0.6 * x[1] + 0.4)) / (x[1] * (0.6 * x[1] + 0.9) + 0.17),
            (x[2] * (0.6 * x[2] + 0.4)) / (x[2] * (0.6 * x[2] + 0.9) + 0.17),
        ]
    }

    /// Neutral tonemapping (минимальное влияние на цвета)
    #[inline]
    fn neutral(&self, color: [f32; 3]) -> [f32; 3] {
        let start_linear = 0.8;
        let mid_in = 0.18;
        let mid_out = 0.18;

        let toe_a = 0.2;
        let toe_b = 0.3;
        let shoulder_a = 0.2;
        let shoulder_b = 0.3;

        let slope = mid_out / mid_in;

        let apply_curve = |x: f32| {
            if x < mid_in {
                x * slope
            } else {
                let t = (x - mid_in) / (1.0 - mid_in);
                let toe = t * (toe_a * t + toe_b) / (t * (toe_a * t + (1.0 - toe_a)) + (1.0 - toe_b));
                let shoulder = 1.0 - (1.0 - t) * (shoulder_a * (1.0 - t) + shoulder_b) 
                    / ((1.0 - t) * (shoulder_a * (1.0 - t) + (1.0 - shoulder_a)) + (1.0 - shoulder_b));
                
                mid_out + (shoulder - toe) * (1.0 - mid_out)
            }
        };

        [
            apply_curve(color[0]),
            apply_curve(color[1]),
            apply_curve(color[2]),
        ]
    }

    /// Обновляет настройки
    pub fn update_settings(&mut self, settings: &HDRPostProcessSettings) {
        self.operator = settings.tonemapping;
        self.exposure = settings.exposure;
        self.gamma = settings.gamma;
        self.white_point = settings.white_point;
    }
}

/// Вычисление освещения по PBR модели (Cook-Torrance BRDF)
pub mod pbr_lighting {
    use super::*;

    /// Вычисляет NDF (Normal Distribution Function) используя GGX/Trowbridge-Reitz
    #[inline]
    pub fn distribution_ggx(n_dot_h: f32, roughness: f32) -> f32 {
        let a = roughness * roughness;
        let a2 = a * a;
        let n_dot_h2 = n_dot_h * n_dot_h;

        let num = a2;
        let denom = n_dot_h2 * (a2 - 1.0) + 1.0;
        let denom_pi = std::f32::consts::PI * denom * denom;

        num / denom_pi.max(1e-10)
    }

    /// Вычисляет геометрию Smith с GGX
    #[inline]
    pub fn geometry_schlick_ggx(n_dot_v: f32, roughness: f32) -> f32 {
        let r = roughness + 1.0;
        let k = (r * r) / 8.0;

        let num = n_dot_v;
        let denom = n_dot_v * (1.0 - k) + k;

        num / denom.max(1e-10)
    }

    /// Вычисляет полную геометрию Smith
    #[inline]
    pub fn geometry_smith(n_dot_l: f32, n_dot_v: f32, roughness: f32) -> f32 {
        let ggx_v = geometry_schlick_ggx(n_dot_v, roughness);
        let ggx_l = geometry_schlick_ggx(n_dot_l, roughness);

        ggx_v * ggx_l
    }

    /// Вычисляет Fresnel используя Schlick approximation
    #[inline]
    pub fn fresnel_schlick(cos_theta: f32, f0: [f32; 3]) -> [f32; 3] {
        let pow_val = (1.0 - cos_theta).max(0.0).powi(5);
        [
            f0[0] + (1.0 - f0[0]) * pow_val,
            f0[1] + (1.0 - f0[1]) * pow_val,
            f0[2] + (1.0 - f0[2]) * pow_val,
        ]
    }

    /// Вычисляет Fresnel для GGX (с учетом roughness)
    #[inline]
    pub fn fresnel_schlick_roughness(cos_theta: f32, f0: [f32; 3], roughness: f32) -> [f32; 3] {
        let pow_val = (1.0 - cos_theta).max(0.0).powi(5);
        let rough_clamped = roughness.max(0.0).min(1.0);
        [
            f0[0] + (f32::max(f0[0], f0[1], f0[2]) - f0[0]) * pow_val * rough_clamped,
            f0[1] + (f32::max(f0[0], f0[1], f0[2]) - f0[1]) * pow_val * rough_clamped,
            f0[2] + (f32::max(f0[0], f0[1], f0[2]) - f0[2]) * pow_val * rough_clamped,
        ]
    }

    /// Полное вычисление Cook-Torrance BRDF
    #[inline]
    pub fn cook_torrance_brdf(n_dot_l: f32, n_dot_h: f32, n_dot_v: f32, l_dot_h: f32, 
                               roughness: f32, f0: [f32; 3]) -> [f32; 3] {
        // NDF
        let ndf = distribution_ggx(n_dot_h, roughness);

        // Geometry
        let geo = geometry_smith(n_dot_l, n_dot_v, roughness);

        // Fresnel
        let f = fresnel_schlick(l_dot_h.max(0.0), f0);

        // Numerator
        let numerator = [ndf * geo * f[0], ndf * geo * f[1], ndf * geo * f[2]];

        // Denominator
        let denominator = 4.0 * n_dot_v * n_dot_l;

        // BRDF
        [
            numerator[0] / denominator.max(1e-10),
            numerator[1] / denominator.max(1e-10),
            numerator[2] / denominator.max(1e-10),
        ]
    }

    /// Интегрированное освещение для точки (direct + indirect)
    pub fn compute_lighting(
        position: [f32; 3],
        normal: [f32; 3],
        view_dir: [f32; 3],
        material: &PBRMaterial,
        lights: &[crate::render::scene::Light],
        ambient: [f32; 3],
    ) -> [f32; 3] {
        use glam::{Vec3, vec3};

        let n = vec3(normal[0], normal[1], normal[2]).normalize();
        let v = vec3(view_dir[0], view_dir[1], view_dir[2]).normalize();

        // Получаем F0 из материала
        let f0 = material.get_f0();

        // Albedo без металличности
        let albedo = vec3(
            material.albedo_factor[0],
            material.albedo_factor[1],
            material.albedo_factor[2],
        );

        // Разделяем albedo для металла и диэлектрика
        let albedo_col = albedo * (1.0 - vec3(f0[0], f0[1], f0[2])) * (1.0 - material.metallic_factor);

        let mut lo = vec3(0.0, 0.0, 0.0);

        // Прямое освещение от источников
        for light in lights {
            let light_pos = vec3(light.position[0], light.position[1], light.position[2]);
            let light_color = vec3(light.color[0], light.color[1], light.color[2]) * light.intensity;

            let light_dir = (light_pos - vec3(position[0], position[1], position[2])).normalize();
            let distance = (light_pos - vec3(position[0], position[1], position[2])).length();
            
            // Attenuation для point lights
            let attenuation = 1.0 / (distance * distance + 1.0);
            
            let radiance = light_color * attenuation;

            // Cook-Torrance BRDF
            let h = (v + light_dir).normalize();
            let n_dot_l = n.dot(light_dir).max(0.0);
            let n_dot_v = n.dot(v).max(0.0);
            let n_dot_h = n.dot(h).max(0.0);
            let l_dot_h = light_dir.dot(h).max(0.0);

            if n_dot_l > 0.0 {
                let brdf = cook_torrance_brdf(
                    n_dot_l, n_dot_h, n_dot_v, l_dot_h,
                    material.roughness_factor, f0
                );

                let brdf_vec = vec3(brdf[0], brdf[1], brdf[2]);
                lo += brdf_vec * radiance * n_dot_l;
            }
        }

        // Ambient lighting (IBL будет добавлен отдельно)
        let ambient_contrib = albedo_col * ambient;

        // Итоговый цвет
        let result = ambient_contrib + lo;
        [result.x, result.y, result.z]
    }
}

/// Интерфейс для HDR рендер таргета
pub trait HDRTarget {
    /// Ширина HDR буфера
    fn width(&self) -> u32;
    /// Высота HDR буфера
    fn height(&self) -> u32;
    /// Формат HDR буфера (должен быть float)
    fn format(&self) -> TextureFormat;
    /// Получить текстуру для чтения
    fn get_texture(&self) -> &dyn Texture;
}

/// Bloom эффект для HDR рендеринга
pub struct BloomEffect {
    /// Порог яркости для bloom
    pub threshold: f32,
    /// Интенсивность bloom
    pub intensity: f32,
    /// Радиок размытия
    pub radius: u32,
    /// Количество passes размытия
    pub blur_passes: u32,
}

impl Default for BloomEffect {
    fn default() -> Self {
        Self {
            threshold: 0.8,
            intensity: 1.0,
            radius: 7,
            blur_passes: 4,
        }
    }
}

impl BloomEffect {
    pub fn new(settings: &HDRPostProcessSettings) -> Self {
        Self {
            threshold: settings.bloom_threshold,
            intensity: settings.bloom_intensity,
            radius: settings.bloom_radius,
            blur_passes: 4,
        }
    }

    /// Извлекает яркие области для bloom
    pub fn extract_bright_areas(&self, hdr_texture: &dyn Texture) -> Result<Box<dyn Texture>, RHIError> {
        // TODO: Implement shader-based bright pass extraction
        todo!("Implement bloom extraction")
    }

    /// Применяет Gaussian blur к текстуре
    pub fn gaussian_blur(&self, texture: &dyn Texture, horizontal: bool) -> Result<Box<dyn Texture>, RHIError> {
        // TODO: Implement separable Gaussian blur
        todo!("Implement gaussian blur")
    }

    /// Композитинг bloom поверх основного изображения
    pub fn composite(&self, base: &dyn Texture, bloom: &dyn Texture) -> Result<Box<dyn Texture>, RHIError> {
        // TODO: Implement additive blending of bloom
        todo!("Implement bloom composite")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reinhard_tonemapping() {
        let settings = HDRPostProcessSettings {
            tonemapping: TonemappingOperator::Reinhard,
            exposure: 1.0,
            gamma: 2.2,
            ..Default::default()
        };
        let tonemapper = Tonemapper::new(&settings);

        let hdr_white = [10.0, 10.0, 10.0];
        let ldr = tonemapper.tonemap(hdr_white);

        // Reinhard должен вернуть значения в диапазоне [0, 1]
        assert!(ldr[0] <= 1.0 && ldr[1] <= 1.0 && ldr[2] <= 1.0);
        assert!(ldr[0] > 0.0 && ldr[1] > 0.0 && ldr[2] > 0.0);
    }

    #[test]
    fn test_aces_filmic_tonemapping() {
        let settings = HDRPostProcessSettings {
            tonemapping: TonemappingOperator::ACESFilmic,
            exposure: 1.0,
            gamma: 2.2,
            ..Default::default()
        };
        let tonemapper = Tonemapper::new(&settings);

        let hdr_color = [5.0, 2.0, 1.0];
        let ldr = tonemapper.tonemap(hdr_color);

        // ACES должен сохранить относительные пропорции цветов
        assert!(ldr[0] > ldr[1] && ldr[1] > ldr[2]);
    }

    #[test]
    fn test_pbr_material_f0() {
        let dielectric = PBRMaterial {
            metallic_factor: 0.0,
            ior: 1.5,
            ..Default::default()
        };
        let f0_dielectric = dielectric.get_f0();
        
        // F0 для диэлектриков с IOR=1.5 должно быть около 0.04
        assert!((f0_dielectric[0] - 0.04).abs() < 0.01);

        let metal = PBRMaterial {
            metallic_factor: 1.0,
            albedo_factor: [0.9, 0.8, 0.7, 1.0],
            ..Default::default()
        };
        let f0_metal = metal.get_f0();
        
        // F0 для металлов должно равняться albedo
        assert!((f0_metal[0] - 0.9).abs() < 0.01);
        assert!((f0_metal[1] - 0.8).abs() < 0.01);
        assert!((f0_metal[2] - 0.7).abs() < 0.01);
    }

    #[test]
    fn test_pbr_presets() {
        let glass = PBRMaterial::with_preset(MaterialPreset::Glass);
        assert!(glass.transmission > 0.9);
        assert!(glass.roughness_factor < 0.1);

        let skin = PBRMaterial::with_preset(MaterialPreset::Skin);
        assert!(skin.subsurface > 0.5);
    }
}
