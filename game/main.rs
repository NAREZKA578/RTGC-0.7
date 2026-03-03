use tetra::graphics::{self, Color, Rectangle, Texture};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};

const SCREEN_WIDTH: i32 = 1024;
const SCREEN_HEIGHT: i32 = 768;

struct GameState {
    vehicle_pos: Vec2<f32>,
    vehicle_vel: Vec2<f32>,
    vehicle_rotation: f32,
    vehicle_angular_vel: f32,
    terrain_heights: Vec<f32>,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        // Generate simple terrain
        let mut terrain_heights = Vec::new();
        let mut height = SCREEN_HEIGHT as f32 * 0.7;
        let mut slope_change = 0.0;

        for x in 0..SCREEN_WIDTH {
            if rand::random::<f32>() < 0.02 {
                slope_change = (rand::random::<f32>() - 0.5) * 4.0;
            }

            height += slope_change;
            height = height.max(SCREEN_HEIGHT as f32 * 0.5).min(SCREEN_HEIGHT as f32 * 0.8);
            terrain_heights.push(height);
        }

        Ok(GameState {
            vehicle_pos: Vec2::new(100.0, SCREEN_HEIGHT as f32 * 0.3),
            vehicle_vel: Vec2::new(0.0, 0.0),
            vehicle_rotation: 0.0,
            vehicle_angular_vel: 0.0,
            terrain_heights,
        })
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        // Handle input
        let mut thrust = 0.0;
        let mut steering = 0.0;

        if input::is_key_pressed(ctx, Key::W) || input::is_key_pressed(ctx, Key::Up) {
            thrust = 1.0;
        }

        if input::is_key_pressed(ctx, Key::S) || input::is_key_pressed(ctx, Key::Down) {
            thrust = -0.5;
        }

        if input::is_key_pressed(ctx, Key::A) || input::is_key_pressed(ctx, Key::Left) {
            steering = -1.0;
        }

        if input::is_key_pressed(ctx, Key::D) || input::is_key_pressed(ctx, Key::Right) {
            steering = 1.0;
        }

        // Vehicle physics
        let force_angle = self.vehicle_rotation.to_radians();
        let force_x = force_angle.cos() * thrust * 10.0;
        let force_y = force_angle.sin() * thrust * 10.0;

        self.vehicle_vel.x += force_x;
        self.vehicle_vel.y += force_y;

        // Apply friction
        self.vehicle_vel *= 0.95;

        // Apply gravity
        self.vehicle_vel.y += 0.5;

        // Update position
        self.vehicle_pos += self.vehicle_vel;

        // Steering affects rotation
        self.vehicle_angular_vel += steering * 0.05;
        self.vehicle_angular_vel *= 0.9;
        self.vehicle_rotation += self.vehicle_angular_vel;

        // Ground collision
        let ground_y = self.get_ground_height(self.vehicle_pos.x);
        if self.vehicle_pos.y > ground_y - 15.0 {
            self.vehicle_pos.y = ground_y - 15.0;
            self.vehicle_vel.y = self.vehicle_vel.y.min(0.0);

            // Apply rotation based on terrain slope
            let slope = self.get_terrain_slope(self.vehicle_pos.x);
            let target_rotation = slope.atan().to_degrees();
            let rotation_diff = (target_rotation - self.vehicle_rotation).rem_euclid(360.0);
            let clamped_diff = rotation_diff.max(-5.0).min(5.0);
            self.vehicle_rotation += clamped_diff * 0.1;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.8, 0.8, 0.9)); // Sky blue

        // Draw terrain
        for x in 0..(SCREEN_WIDTH - 1) as usize {
            let height = self.terrain_heights[x];
            let next_height = self.terrain_heights[(x + 1).min(SCREEN_WIDTH as usize - 1)];

            graphics::line(
                ctx,
                x as f32,
                height,
                (x + 1) as f32,
                next_height,
                2.0,
                Color::rgb(0.3, 0.2, 0.1),
            )?;
        }

        // Draw vehicle
        let vehicle_size = Vec2::new(30.0, 15.0);
        let rotated_vehicle = Rectangle::new(
            -vehicle_size.x / 2.0,
            -vehicle_size.y / 2.0,
            vehicle_size.x,
            vehicle_size.y,
        );

        graphics::rectangle(
            ctx,
            rotated_vehicle,
            Some(tetra::math::Vec2::new(self.vehicle_pos.x, self.vehicle_pos.y)),
            Some(self.vehicle_rotation.to_radians()),
            None,
            Color::RED,
        )
    }
}

impl GameState {
    fn get_ground_height(&self, x: f32) -> f32 {
        let x = x.max(0.0).min(SCREEN_WIDTH as f32 - 1.0);
        let idx = x as usize;

        if idx >= self.terrain_heights.len() - 1 {
            return self.terrain_heights[self.terrain_heights.len() - 1];
        }

        // Linear interpolation between two points
        let fract = x - idx as f32;
        let h1 = self.terrain_heights[idx];
        let h2 = self.terrain_heights[idx + 1];

        h1 + (h2 - h1) * fract
    }

    fn get_terrain_slope(&self, x: f32) -> f32 {
        let left = self.get_ground_height((x - 1.0).max(0.0));
        let right = self.get_ground_height((x + 1.0).min(SCREEN_WIDTH as f32 - 1.0));

        (right - left) / 2.0
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Mud Runner Clone", SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}