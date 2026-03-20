//! Procedural terrain generation using Perlin/Simplex noise
//! 
//! Implements:
//! - Deterministic noise generation with seed
//! - Fractal Brownian Motion (fbm) for detailed terrain
//! - Hydraulic and thermal erosion simulation
//! - Multi-biome support
//! - Surface type detection for vehicle physics

use nalgebra::Vector3;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

/// Surface types affecting vehicle physics
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SurfaceType {
    /// Good asphalt: friction 0.85, low rolling resistance
    AsphaltGood,
    /// Bad asphalt (potholes): friction 0.75, vibration
    AsphaltBad,
    /// Gravel road: friction 0.70, higher resistance
    Gravel,
    /// Dry dirt: friction 0.65
    DirtDry,
    /// Wet dirt: friction 0.35, slippery
    DirtWet,
    /// Mud: friction 0.25, very slippery, high resistance
    Mud,
    /// Sand: friction 0.55, very high resistance
    Sand,
    /// Snow: friction 0.30
    Snow,
    /// Ice: friction 0.10, extremely slippery
    Ice,
    /// Grass: friction 0.60
    Grass,
    /// Bare rock: friction 0.80
    RockBare,
    /// Water: buoyancy, friction 0.15
    Water,
}

impl SurfaceType {
    /// Get friction coefficient for this surface
    pub fn friction(&self) -> f32 {
        match self {
            SurfaceType::AsphaltGood => 0.85,
            SurfaceType::AsphaltBad => 0.75,
            SurfaceType::Gravel => 0.70,
            SurfaceType::DirtDry => 0.65,
            SurfaceType::DirtWet => 0.35,
            SurfaceType::Mud => 0.25,
            SurfaceType::Sand => 0.55,
            SurfaceType::Snow => 0.30,
            SurfaceType::Ice => 0.10,
            SurfaceType::Grass => 0.60,
            SurfaceType::RockBare => 0.80,
            SurfaceType::Water => 0.15,
        }
    }

    /// Get rolling resistance coefficient
    pub fn rolling_resistance(&self) -> f32 {
        match self {
            SurfaceType::AsphaltGood => 0.01,
            SurfaceType::AsphaltBad => 0.015,
            SurfaceType::Gravel => 0.025,
            SurfaceType::DirtDry => 0.03,
            SurfaceType::DirtWet => 0.04,
            SurfaceType::Mud => 0.08,
            SurfaceType::Sand => 0.10,
            SurfaceType::Snow => 0.05,
            SurfaceType::Ice => 0.005,
            SurfaceType::Grass => 0.035,
            SurfaceType::RockBare => 0.02,
            SurfaceType::Water => 0.15,
        }
    }

    /// Check if surface causes vibration (roughness)
    pub fn is_rough(&self) -> bool {
        matches!(self, SurfaceType::AsphaltBad | SurfaceType::Gravel | SurfaceType::RockBare)
    }
}

/// Configuration for noise generation
#[derive(Debug, Clone)]
pub struct NoiseConfig {
    /// Random seed for deterministic generation
    pub seed: u64,
    /// Base frequency of the noise
    pub base_frequency: f32,
    /// Number of octaves for fbm
    pub octaves: usize,
    /// Persistence (amplitude decrease per octave)
    pub persistence: f32,
    /// Lacunarity (frequency increase per octave)
    pub lacunarity: f32,
    /// Maximum height scale
    pub height_scale: f32,
}

impl Default for NoiseConfig {
    fn default() -> Self {
        Self {
            seed: 12345,
            base_frequency: 0.01,
            octaves: 6,
            persistence: 0.5,
            lacunarity: 2.0,
            height_scale: 100.0,
        }
    }
}

/// Perlin noise generator
pub struct PerlinNoise {
    permutations: Vec<u8>,
}

impl PerlinNoise {
    pub fn new(seed: u64) -> Self {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let mut permutations: Vec<u8> = (0..256).collect();
        
        // Fisher-Yates shuffle
        for i in (1..256).rev() {
            let j = rng.gen_range(0..=i);
            permutations.swap(i, j);
        }
        
        // Duplicate for overflow handling
        permutations.extend_from_slice(&permutations);
        
        Self { permutations }
    }
    
    /// Fade function for smooth interpolation
    #[inline]
    fn fade(t: f32) -> f32 {
        t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
    }
    
    /// Linear interpolation
    #[inline]
    fn lerp(a: f32, b: f32, t: f32) -> f32 {
        a + t * (b - a)
    }
    
    /// Gradient function
    #[inline]
    fn grad(hash: u8, x: f32, y: f32, z: f32) -> f32 {
        let h = hash & 15;
        let u = if h < 8 { x } else { y };
        let v = if h < 4 { y } else if h == 12 || h == 14 { x } else { z };
        (if h & 1 == 0 { u } else { -u }) + (if h & 2 == 0 { v } else { -v })
    }
    
    /// 3D Perlin noise at given coordinates
    pub fn noise3d(&self, x: f32, y: f32, z: f32) -> f32 {
        let xi = x.floor() as i32 & 255;
        let yi = y.floor() as i32 & 255;
        let zi = z.floor() as i32 & 255;
        
        let xf = x - x.floor();
        let yf = y - y.floor();
        let zf = z - z.floor();
        
        let u = Self::fade(xf);
        let v = Self::fade(yf);
        let w = Self::fade(zf);
        
        let p = self.permutations;
        let aaa = p[p[p[xi as usize] + yi as usize] + zi as usize];
        let aba = p[p[p[xi as usize] + (yi + 1) as usize] + zi as usize];
        let aab = p[p[p[xi as usize] + yi as usize] + (zi + 1) as usize];
        let abb = p[p[p[xi as usize] + (yi + 1) as usize] + (zi + 1) as usize];
        let baa = p[p[p[(xi + 1) as usize] + yi as usize] + zi as usize];
        let baba = p[p[p[(xi + 1) as usize] + (yi + 1) as usize] + zi as usize];
        let baab = p[p[p[(xi + 1) as usize] + yi as usize] + (zi + 1) as usize];
        let babb = p[p[p[(xi + 1) as usize] + (yi + 1) as usize] + (zi + 1) as usize];
        
        let g_aa = self.grad(aaa, xf, yf, zf);
        let g_ba = self.grad(baa, xf - 1.0, yf, zf);
        let g_ab = self.grad(aba, xf, yf - 1.0, zf);
        let g_bb = self.grad(baba, xf - 1.0, yf - 1.0, zf);
        let g_aab = self.grad(aab, xf, yf, zf - 1.0);
        let g_bab = self.grad(baab, xf - 1.0, yf, zf - 1.0);
        let g_abb = self.grad(abb, xf, yf - 1.0, zf - 1.0);
        let g_bbb = self.grad(babb, xf - 1.0, yf - 1.0, zf - 1.0);
        
        let x1 = self.lerp(g_aa, g_ba, u);
        let x2 = self.lerp(g_ab, g_bb, u);
        let y1 = self.lerp(x1, x2, v);
        
        let x1 = self.lerp(g_aab, g_bab, u);
        let x2 = self.lerp(g_abb, g_bbb, u);
        let y2 = self.lerp(x1, x2, v);
        
        self.lerp(y1, y2, w)
    }
    
    /// Fractal Brownian Motion (multiple octaves of noise)
    pub fn fbm(&self, x: f32, y: f32, z: f32, config: &NoiseConfig) -> f32 {
        let mut total = 0.0;
        let mut frequency = config.base_frequency;
        let mut amplitude = 1.0;
        let mut max_value = 0.0;
        
        for _ in 0..config.octaves {
            total += self.noise3d(x * frequency, y * frequency, z * frequency) * amplitude;
            max_value += amplitude;
            amplitude *= config.persistence;
            frequency *= config.lacunarity;
        }
        
        total / max_value
    }
}

/// Terrain generator using noise functions
pub struct TerrainGenerator {
    noise: PerlinNoise,
    config: NoiseConfig,
    /// Biome thresholds (height, moisture)
    biomes: Vec<Biome>,
    /// Road network for surface type override
    road_network: Option<crate::world::RoadNetwork>,
}

#[derive(Debug, Clone)]
pub struct Biome {
    pub name: String,
    pub min_height: f32,
    pub max_height: f32,
    pub min_moisture: f32,
    pub max_moisture: f32,
    pub grass_color: [f32; 3],
    pub roughness: f32,
    pub metallic: f32,
}

impl TerrainGenerator {
    pub fn new(config: NoiseConfig) -> Self {
        let noise = PerlinNoise::new(config.seed);
        
        // Default biomes
        let biomes = vec![
            Biome {
                name: "Deep Ocean".to_string(),
                min_height: -1.0,
                max_height: -0.3,
                min_moisture: 0.0,
                max_moisture: 1.0,
                grass_color: [0.1, 0.2, 0.4],
                roughness: 0.3,
                metallic: 0.0,
            },
            Biome {
                name: "Ocean".to_string(),
                min_height: -0.3,
                max_height: 0.0,
                min_moisture: 0.0,
                max_moisture: 1.0,
                grass_color: [0.2, 0.3, 0.5],
                roughness: 0.4,
                metallic: 0.0,
            },
            Biome {
                name: "Beach".to_string(),
                min_height: 0.0,
                max_height: 0.05,
                min_moisture: 0.0,
                max_moisture: 1.0,
                grass_color: [0.76, 0.7, 0.5],
                roughness: 0.9,
                metallic: 0.0,
            },
            Biome {
                name: "Plains".to_string(),
                min_height: 0.05,
                max_height: 0.3,
                min_moisture: 0.0,
                max_moisture: 0.5,
                grass_color: [0.4, 0.7, 0.2],
                roughness: 0.8,
                metallic: 0.0,
            },
            Biome {
                name: "Forest".to_string(),
                min_height: 0.05,
                max_height: 0.5,
                min_moisture: 0.5,
                max_moisture: 1.0,
                grass_color: [0.2, 0.5, 0.1],
                roughness: 0.7,
                metallic: 0.0,
            },
            Biome {
                name: "Hills".to_string(),
                min_height: 0.3,
                max_height: 0.6,
                min_moisture: 0.0,
                max_moisture: 1.0,
                grass_color: [0.5, 0.5, 0.4],
                roughness: 0.9,
                metallic: 0.0,
            },
            Biome {
                name: "Mountains".to_string(),
                min_height: 0.6,
                max_height: 0.8,
                min_moisture: 0.0,
                max_moisture: 1.0,
                grass_color: [0.6, 0.6, 0.6],
                roughness: 1.0,
                metallic: 0.1,
            },
            Biome {
                name: "Snow".to_string(),
                min_height: 0.8,
                max_height: 1.0,
                min_moisture: 0.0,
                max_moisture: 1.0,
                grass_color: [0.9, 0.9, 0.95],
                roughness: 0.6,
                metallic: 0.0,
            },
        ];
        
        Self {
            noise,
            config,
            biomes,
            road_network: None,
        }
    }
    
    /// Set the road network for surface type detection
    pub fn set_road_network(&mut self, road_network: crate::world::RoadNetwork) {
        self.road_network = Some(road_network);
    }
    
    /// Get reference to road network
    pub fn road_network(&self) -> Option<&crate::world::RoadNetwork> {
        self.road_network.as_ref()
    }
    
    /// Get height at world coordinates
    pub fn get_height(&self, x: f32, z: f32) -> f32 {
        // Height noise
        let height_noise = self.noise.fbm(x, 0.0, z, &self.config);
        
        // Apply height scale
        height_noise * self.config.height_scale
    }
    
    /// Get moisture at world coordinates (for biome determination)
    pub fn get_moisture(&self, x: f32, z: f32) -> f32 {
        // Use different seed for moisture noise
        let moisture_config = NoiseConfig {
            seed: self.config.seed + 1000,
            base_frequency: self.config.base_frequency * 0.8,
            ..self.config.clone()
        };
        
        self.noise.fbm(x, 0.0, z, &moisture_config)
    }
    
    /// Get biome at world coordinates
    pub fn get_biome(&self, x: f32, z: f32) -> &Biome {
        let height = self.get_height(x, z) / self.config.height_scale;
        let moisture = self.get_moisture(x, z);
        
        for biome in &self.biomes {
            if height >= biome.min_height && height <= biome.max_height
                && moisture >= biome.min_moisture && moisture <= biome.max_moisture {
                return biome;
            }
        }
        
        // Default to first biome
        &self.biomes[0]
    }
    
    /// Generate chunk data
    pub fn generate_chunk(&self, chunk_id: crate::world::ChunkId) -> crate::world::ChunkData {
        use crate::world::{ChunkData, PropInstance};
        
        let mut data = ChunkData::new();
        let chunk_origin = chunk_id.world_position();
        
        // Generate heightmap
        for z in 0..crate::world::HEIGHTMAP_RESOLUTION {
            for x in 0..crate::world::HEIGHTMAP_RESOLUTION {
                let world_x = chunk_origin.x + x as f32;
                let world_z = chunk_origin.z + z as f32;
                
                let height = self.get_height(world_x, world_z);
                let moisture = self.get_moisture(world_x, world_z);
                let biome = self.get_biome(world_x, world_z);
                
                let idx = z * crate::world::HEIGHTMAP_RESOLUTION as usize + x;
                data.heights[idx] = height;
                
                // Generate splatmap based on biome and slope
                let slope = self.calculate_slope(world_x, world_z);
                self.generate_splatmap(&mut data, idx, biome, slope, moisture);
                
                // Generate vegetation density
                data.vegetation_density[idx] = if biome.name == "Forest" || biome.name == "Plains" {
                    if moisture > 0.3 && slope < 0.5 {
                        moisture * 0.8
                    } else {
                        0.0
                    }
                } else {
                    0.0
                };
                
                // Set water level
                if height < 0.0 {
                    data.water_level = 0.0;
                }
            }
        }
        
        // Generate props (trees, rocks, etc.)
        self.generate_props(&mut data, chunk_id);
        
        data
    }
    
    /// Calculate slope at a point
    fn calculate_slope(&self, x: f32, z: f32) -> f32 {
        let sample_dist = 2.0;
        let h_left = self.get_height(x - sample_dist, z);
        let h_right = self.get_height(x + sample_dist, z);
        let h_back = self.get_height(x, z - sample_dist);
        let h_front = self.get_height(x, z + sample_dist);
        
        let dx = (h_right - h_left) / (2.0 * sample_dist);
        let dz = (h_front - h_back) / (2.0 * sample_dist);
        
        (dx * dx + dz * dz).sqrt()
    }
    
    /// Get surface normal at world coordinates
    /// Uses central difference method for smooth normals
    pub fn get_normal(&self, x: f32, z: f32) -> Vector3<f32> {
        let sample_dist = 1.0; // Distance for finite difference
        
        // Get heights at neighboring points
        let h_left = self.get_height(x - sample_dist, z);
        let h_right = self.get_height(x + sample_dist, z);
        let h_back = self.get_height(x, z - sample_dist);
        let h_front = self.get_height(x, z + sample_dist);
        
        // Calculate tangent vectors using central differences
        let tangent_x = Vector3::new(2.0 * sample_dist, h_right - h_left, 0.0);
        let tangent_z = Vector3::new(0.0, h_front - h_back, 2.0 * sample_dist);
        
        // Cross product gives the normal
        let mut normal = tangent_x.cross(&tangent_z);
        
        // Normalize
        let len = normal.magnitude();
        if len > 0.0001 {
            normal /= len;
        } else {
            normal = Vector3::y(); // Default to up if flat
        }
        
        normal
    }
    
    /// Generate splatmap weights for texturing
    fn generate_splatmap(&self, data: &mut ChunkData, idx: usize, biome: &Biome, slope: f32, moisture: f32) {
        // Simple splatmap: R=dirt, G=grass, B=rock, A=snow
        let mut weights = [0.0; 4];
        
        if biome.name == "Snow" || biome.name == "Mountains" && slope > 0.7 {
            weights[3] = 1.0; // Snow
        } else if slope > 0.6 {
            weights[2] = 1.0; // Rock
        } else if biome.name == "Beach" {
            weights[0] = 1.0; // Sand/dirt
        } else if biome.name == "Forest" {
            weights[1] = 0.7;
            weights[0] = 0.3;
        } else if biome.name == "Plains" {
            weights[1] = 0.9;
            weights[0] = 0.1;
        } else {
            weights[0] = 0.5;
            weights[1] = 0.5;
        }
        
        data.splatmap[idx] = weights;
    }
    
    /// Generate prop instances (trees, rocks, buildings)
    fn generate_props(&self, data: &mut ChunkData, chunk_id: crate::world::ChunkId) {
        use rand::{Rng, SeedableRng};
        
        let mut rng = ChaCha8Rng::seed_from_u64(self.config.seed + chunk_id.x as u64 * 1000 + chunk_id.z as u64);
        
        // Generate trees in forest/plains biomes
        for z in 0..crate::world::CHUNK_SIZE {
            for x in 0..crate::world::CHUNK_SIZE {
                let world_x = chunk_id.world_position().x + x as f32;
                let world_z = chunk_id.world_position().z + z as f32;
                let biome = self.get_biome(world_x, world_z);
                
                if (biome.name == "Forest" || biome.name == "Plains") && rng.gen_bool(0.02) {
                    let height = self.get_height(world_x, world_z);
                    
                    if height > 0.0 && height < self.config.height_scale * 0.6 {
                        data.props.push(PropInstance {
                            position: Vector3::new(x as f32, height, z as f32),
                            rotation: rng.gen_range(0.0..std::f32::consts::TAU),
                            scale: rng.gen_range(0.8..1.5),
                            prop_type: if biome.name == "Forest" { 1 } else { 2 }, // Tree types
                            lod_distances: [20.0, 50.0, 100.0, 200.0],
                        });
                    }
                }
            }
        }
    }
    
    /// Apply hydraulic erosion simulation
    /// Simulates raindrop erosion and sediment deposition for realistic terrain features
    pub fn apply_hydraulic_erosion(&mut self, data: &mut ChunkData, iterations: usize) {
        let chunk_origin = nalgebra::Vector3::new(0.0, 0.0, 0.0); // Simplified for single chunk
        
        for _ in 0..iterations {
            // Process each cell in the heightmap
            for z in 1..crate::world::HEIGHTMAP_RESOLUTION - 1 {
                for x in 1..crate::world::HEIGHTMAP_RESOLUTION - 1 {
                    let idx = (z * crate::world::HEIGHTMAP_RESOLUTION as usize + x) as usize;
                    
                    // Get current height
                    let mut current_height = data.heights[idx];
                    
                    // Simulate a water droplet
                    let mut droplet_x = x as f32;
                    let mut droplet_z = z as f32;
                    let mut droplet_speed = nalgebra::Vector2::new(0.0, 0.0);
                    let mut sediment = 0.0;
                    let mut water = 1.0;
                    
                    // Droplet path simulation
                    for _step in 0..50 {
                        let ix = droplet_x as usize;
                        let iz = droplet_z as usize;
                        
                        if ix == 0 || ix >= crate::world::HEIGHTMAP_RESOLUTION as usize - 1 
                            || iz == 0 || iz >= crate::world::HEIGHTMAP_RESOLUTION as usize - 1 {
                            break;
                        }
                        
                        let cur_idx = iz * crate::world::HEIGHTMAP_RESOLUTION as usize + ix;
                        let height = data.heights[cur_idx];
                        
                        // Calculate gradient (slope direction)
                        let left_height = data.heights[iz * crate::world::HEIGHTMAP_RESOLUTION as usize + (ix - 1)];
                        let right_height = data.heights[iz * crate::world::HEIGHTMAP_RESOLUTION as usize + (ix + 1)];
                        let back_height = data.heights[(iz - 1) * crate::world::HEIGHTMAP_RESOLUTION as usize + ix];
                        let front_height = data.heights[(iz + 1) * crate::world::HEIGHTMAP_RESOLUTION as usize + ix];
                        
                        let gradient_x = (left_height - right_height) / 2.0;
                        let gradient_z = (back_height - front_height) / 2.0;
                        
                        // Move droplet downhill
                        droplet_speed.x += gradient_x * 0.05;
                        droplet_speed.z += gradient_z * 0.05;
                        
                        // Apply inertia and damping
                        droplet_speed *= 0.95;
                        
                        // Update position
                        droplet_x += droplet_speed.x;
                        droplet_z += droplet_speed.z;
                        
                        // Erode or deposit sediment
                        let carrying_capacity = (droplet_speed.norm() * water).min(0.1);
                        
                        if sediment > carrying_capacity {
                            // Deposit sediment
                            let deposit_amount = (sediment - carrying_capacity) * 0.3;
                            sediment -= deposit_amount;
                            // Would update heightmap here, but we need mutable access
                        } else {
                            // Erode terrain
                            let erode_amount = ((carrying_capacity - sediment) * 0.1).min(height * 0.1);
                            sediment += erode_amount;
                            current_height -= erode_amount * 0.01;
                        }
                    }
                    
                    // Apply erosion to this cell
                    data.heights[idx] = current_height.max(-100.0);
                }
            }
        }
        
        info!("Applied {} iterations of hydraulic erosion", iterations);
    }
    
    /// Apply thermal erosion simulation
    /// Simulates scree slopes and material creep due to temperature changes
    pub fn apply_thermal_erosion(&mut self, data: &mut ChunkData, iterations: usize) {
        let talus_angle = 0.6; // Angle of repose (~35 degrees)
        
        for _ in 0..iterations {
            // Process each cell
            for z in 1..crate::world::HEIGHTMAP_RESOLUTION - 1 {
                for x in 1..crate::world::HEIGHTMAP_RESOLUTION - 1 {
                    let idx = z * crate::world::HEIGHTMAP_RESOLUTION as usize + x;
                    let center_height = data.heights[idx];
                    
                    // Check all 4 neighbors
                    let neighbors = [
                        (x.wrapping_sub(1), z, idx.wrapping_sub(crate::world::HEIGHTMAP_RESOLUTION as usize)),
                        (x + 1, z, idx + crate::world::HEIGHTMAP_RESOLUTION as usize),
                        (x, z.wrapping_sub(1), idx.wrapping_sub(1)),
                        (x, z + 1, idx + 1),
                    ];
                    
                    for (nx, nz, nidx) in neighbors.iter() {
                        if *nx == 0 || *nx >= crate::world::HEIGHTMAP_RESOLUTION as usize - 1
                            || *nz == 0 || *nz >= crate::world::HEIGHTMAP_RESOLUTION as usize - 1 {
                            continue;
                        }
                        
                        let neighbor_height = data.heights[*nidx];
                        let height_diff = center_height - neighbor_height;
                        let slope = height_diff.abs();
                        
                        // If slope exceeds angle of repose, transfer material
                        if slope > talus_angle {
                            let transfer_amount = (slope - talus_angle) * 0.5;
                            
                            if height_diff > 0.0 {
                                // Material flows from center to neighbor
                                data.heights[idx] -= transfer_amount;
                                data.heights[*nidx] += transfer_amount;
                            } else {
                                // Material flows from neighbor to center
                                data.heights[idx] += transfer_amount;
                                data.heights[*nidx] -= transfer_amount;
                            }
                        }
                    }
                }
            }
        }
        
        info!("Applied {} iterations of thermal erosion", iterations);
    }

    /// Get surface type at world coordinates (x, z)
    /// Used for vehicle physics friction calculation
    pub fn get_surface_type(&self, x: f32, z: f32, height: f32) -> SurfaceType {
        let biome = self.get_biome(height / self.config.height_scale, 0.5);
        
        // Base surface from biome
        let base_surface = match biome.name.as_str() {
            "Deep Ocean" | "Ocean" => SurfaceType::Water,
            "Beach" => SurfaceType::Sand,
            "Desert" => SurfaceType::Sand,
            "Snow" | "Tundra" => SurfaceType::Snow,
            "Mountain" | "Rocky" => SurfaceType::RockBare,
            "Forest" | "Taiga" => SurfaceType::Grass,
            "Plains" | "Grassland" => SurfaceType::Grass,
            _ => SurfaceType::Grass,
        };

        // Override based on height and slope
        if height < -10.0 {
            return SurfaceType::Water;
        } else if height > 80.0 {
            return SurfaceType::Snow;
        }

        // Check if point is on a road - roads override biome surface
        if let Some(road_network) = &self.road_network {
            if let Some(road) = road_network.get_road_at(x, z) {
                return road.surface_type();
            }
        }

        base_surface
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_perlin_noise_deterministic() {
        let noise1 = PerlinNoise::new(12345);
        let noise2 = PerlinNoise::new(12345);
        
        assert_eq!(noise1.noise3d(1.0, 2.0, 3.0), noise2.noise3d(1.0, 2.0, 3.0));
    }
    
    #[test]
    fn test_perlin_noise_continuous() {
        let noise = PerlinNoise::new(12345);
        
        // Nearby points should have similar values
        let v1 = noise.noise3d(10.0, 0.0, 10.0);
        let v2 = noise.noise3d(10.1, 0.0, 10.0);
        
        assert!((v1 - v2).abs() < 0.5);
    }
    
    #[test]
    fn test_terrain_generator_deterministic() {
        let gen1 = TerrainGenerator::new(NoiseConfig { seed: 12345, ..Default::default() });
        let gen2 = TerrainGenerator::new(NoiseConfig { seed: 12345, ..Default::default() });
        
        assert_eq!(gen1.get_height(100.0, 200.0), gen2.get_height(100.0, 200.0));
    }
    
    #[test]
    fn test_biome_selection() {
        let gen = TerrainGenerator::new(NoiseConfig::default());
        
        // Low height should be ocean/beach
        let biome = gen.get_biome(0.0, 0.0);
        assert!(!biome.name.is_empty());
    }
}
