use nalgebra::Vector3;
use std::collections::HashMap;

pub struct SpatialHash {
    cell_size: f32,
    hash_map: HashMap<(i32, i32, i32), Vec<usize>>,
}

impl SpatialHash {
    pub fn new(cell_size: f32) -> Self {
        Self {
            cell_size,
            hash_map: HashMap::new(),
        }
    }

    fn hash_coords(&self, coords: (i32, i32, i32)) -> (i32, i32, i32) {
        // Ensure consistent hashing for negative coordinates
        let hash_coord = |x: i32| -> i32 {
            if x >= 0 {
                x % 1000000
            } else {
                ((x % 1000000) + 1000000) % 1000000
            }
        };
        
        (hash_coord(coords.0), hash_coord(coords.1), hash_coord(coords.2))
    }

    fn world_to_cell(&self, pos: &Vector3<f32>) -> (i32, i32, i32) {
        (
            (pos.x / self.cell_size).floor() as i32,
            (pos.y / self.cell_size).floor() as i32,
            (pos.z / self.cell_size).floor() as i32,
        )
    }

    pub fn clear(&mut self) {
        self.hash_map.clear();
    }

    pub fn insert(&mut self, body_index: usize, position: &Vector3<f32>) {
        let cell_coords = self.world_to_cell(position);
        let hashed_coords = self.hash_coords(cell_coords);
        
        self.hash_map.entry(hashed_coords).or_insert_with(Vec::new).push(body_index);
    }

    pub fn get_potential_collisions(&self, position: &Vector3<f32>) -> Vec<usize> {
        let mut candidates = Vec::new();
        let cell_coords = self.world_to_cell(position);

        // Check current cell and neighboring cells (3x3x3 grid)
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    let neighbor_coords = (
                        cell_coords.0 + dx,
                        cell_coords.1 + dy,
                        cell_coords.2 + dz,
                    );
                    
                    let hashed_coords = self.hash_coords(neighbor_coords);
                    
                    if let Some(bodies) = self.hash_map.get(&hashed_coords) {
                        candidates.extend_from_slice(bodies);
                    }
                }
            }
        }

        candidates
    }
}