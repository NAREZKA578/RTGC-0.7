use nalgebra::Vector3;

pub enum LodModel {
    HighPoly { vertices: Vec<Vector3<f32>>, indices: Vec<u32> },
    MediumPoly { vertices: Vec<Vector3<f32>>, indices: Vec<u32> },
    LowPoly { vertices: Vec<Vector3<f32>>, indices: Vec<u32> },
    Billboard { texture_id: u32, size: f32 },
}

pub struct LodObject {
    pub position: Vector3<f32>,
    pub lod_distances: [f32; 3], // [high_to_med, med_to_low, low_to_billboard]
    pub lod_models: [LodModel; 4], // [high, medium, low, billboard/none]
    pub current_lod: usize,
}

impl LodObject {
    pub fn new(
        position: Vector3<f32>,
        lod_distances: [f32; 3],
        lod_models: [LodModel; 4],
    ) -> Self {
        Self {
            position,
            lod_distances,
            lod_models,
            current_lod: 0, // Start with highest detail
        }
    }

    pub fn update_lod(&mut self, camera_position: &Vector3<f32>) {
        let distance = (self.position - camera_position).magnitude();
        
        // Determine appropriate LOD level based on distance thresholds
        self.current_lod = if distance < self.lod_distances[0] {
            0 // High poly
        } else if distance < self.lod_distances[1] {
            1 // Medium poly
        } else if distance < self.lod_distances[2] {
            2 // Low poly
        } else {
            3 // Billboard or none
        };
    }

    pub fn get_current_model(&self) -> &LodModel {
        &self.lod_models[self.current_lod]
    }

    pub fn get_render_distance(&self) -> f32 {
        // Return the furthest distance at which this object should render
        self.lod_distances[2]
    }
}

pub struct LodManager {
    pub objects: Vec<LodObject>,
}

impl LodManager {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add_object(&mut self, lod_object: LodObject) {
        self.objects.push(lod_object);
    }

    pub fn update_all_lods(&mut self, camera_position: &Vector3<f32>) {
        for obj in &mut self.objects {
            obj.update_lod(camera_position);
        }
    }

    pub fn get_objects_in_view(&self, camera_position: &Vector3<f32>, view_distance: f32) -> Vec<(usize, &LodModel)> {
        let mut visible_objects = Vec::new();
        
        for (index, obj) in self.objects.iter().enumerate() {
            let distance = (obj.position - camera_position).magnitude();
            
            // Only include objects that are within the view distance and have a model to render
            if distance < view_distance.min(obj.get_render_distance()) {
                visible_objects.push((index, obj.get_current_model()));
            }
        }
        
        visible_objects
    }
}