#include "raylib.h"
#include "physics_engine_c.h"
#include "vehicle_control.h"

#define MAX_PARTICLES 10000
#define MAX_VEHICLES 10

// Game state
typedef struct {
    Particle particles[MAX_PARTICLES];
    int particle_count;
    
    Vehicle vehicles[MAX_VEHICLES];
    int vehicle_count;
    int current_vehicle;
    
    Camera3D camera;
    bool gravity_enabled;
    Vector3 camera_offset;
} GameState;

GameState game_state = {0};

void init_game() {
    // Initialize physics engine
    init_physics_engine();
    
    // Setup camera
    game_state.camera.position = (Vector3){ 10.0f, 10.0f, 10.0f };
    game_state.camera.target = (Vector3){ 0.0f, 0.0f, 0.0f };
    game_state.camera.up = (Vector3){ 0.0f, 1.0f, 0.0f };
    game_state.camera.fovy = 45.0f;
    game_state.camera.projection = CAMERA_PERSPECTIVE;
    
    game_state.gravity_enabled = true;
    game_state.camera_offset = (Vector3){ 0.0f, 2.0f, 5.0f };
    
    // Create ground
    Particle* ground = &game_state.particles[game_state.particle_count++];
    init_particle(ground, (Vector3){0, -2, 0}, 10.0f);
    ground->mass = 0;  // Static
    
    // Create some initial particles
    for(int i = 0; i < 50; i++) {
        Particle* p = &game_state.particles[game_state.particle_count++];
        float x = (float)(rand() % 20 - 10);
        float y = 5.0f + (float)(rand() % 10);
        float z = (float)(rand() % 20 - 10);
        init_particle(p, (Vector3){x, y, z}, 0.3f);
    }
    
    // Create vehicles
    init_car(&game_state.vehicles[0], (Vector3){5, 1, 0});
    init_truck(&game_state.vehicles[1], (Vector3){-5, 1, 0});
    init_motorcycle(&game_state.vehicles[2], (Vector3){0, 1, 5});
    init_airplane(&game_state.vehicles[3], (Vector3){0, 5, -5});
    init_helicopter(&game_state.vehicles[4], (Vector3){5, 5, 5});
    init_tank(&game_state.vehicles[5], (Vector3){-5, 1, -5});
    
    game_state.vehicle_count = 6;
    game_state.current_vehicle = -1;  // No vehicle selected initially
}

void update_camera() {
    if(game_state.current_vehicle != -1) {
        // Follow current vehicle
        Vehicle* v = &game_state.vehicles[game_state.current_vehicle];
        game_state.camera.target = v->position;
        game_state.camera.position = (Vector3){
            v->position.x + game_state.camera_offset.x,
            v->position.y + game_state.camera_offset.y,
            v->position.z + game_state.camera_offset.z
        };
    } else {
        // Free camera mode - handled separately
    }
}

void handle_input() {
    // Switch vehicles with number keys
    if(IsKeyPressed(KEY_ONE)) game_state.current_vehicle = 0;
    if(IsKeyPressed(KEY_TWO)) game_state.current_vehicle = 1;
    if(IsKeyPressed(KEY_THREE)) game_state.current_vehicle = 2;
    if(IsKeyPressed(KEY_FOUR)) game_state.current_vehicle = 3;
    if(IsKeyPressed(KEY_FIVE)) game_state.current_vehicle = 4;
    if(IsKeyPressed(KEY_SIX)) game_state.current_vehicle = 5;
    if(IsKeyPressed(KEY_ZERO)) game_state.current_vehicle = -1;  // Free camera
    
    // Toggle gravity
    if(IsKeyPressed(KEY_G)) {
        game_state.gravity_enabled = !game_state.gravity_enabled;
        set_gravity_enabled(game_state.gravity_enabled);
    }
    
    // Reset simulation
    if(IsKeyPressed(KEY_R)) {
        game_state.particle_count = 0;
        game_state.vehicle_count = 0;
        init_game();
    }
    
    // Handle vehicle controls if a vehicle is selected
    if(game_state.current_vehicle != -1) {
        Vehicle* v = &game_state.vehicles[game_state.current_vehicle];
        handle_vehicle_controls(v);
    }
}

int main() {
    // Initialize window
    InitWindow(1024, 768, "Advanced Physics Game");
    SetTargetFPS(60);
    
    init_game();
    
    while(!WindowShouldClose()) {
        handle_input();
        update_camera();
        
        // Update physics
        update_physics_engine(game_state.particles, game_state.particle_count);
        
        // Update vehicles
        for(int i = 0; i < game_state.vehicle_count; i++) {
            update_vehicle(&game_state.vehicles[i], GetFrameTime());
        }
        
        BeginDrawing();
        ClearBackground(RAYWHITE);
        
        BeginMode3D(game_state.camera);
        
        // Draw ground
        DrawCube((Vector3){0, -2.5f, 0}, 20.0f, 1.0f, 20.0f, GRAY);
        DrawGrid(20, 1.0f);
        
        // Draw particles
        for(int i = 0; i < game_state.particle_count; i++) {
            Particle* p = &game_state.particles[i];
            if(p->active) {
                DrawSphere(p->position, p->radius, RED);
            }
        }
        
        // Draw vehicles
        for(int i = 0; i < game_state.vehicle_count; i++) {
            draw_vehicle(&game_state.vehicles[i]);
        }
        
        EndMode3D();
        
        // Draw UI
        DrawText(TextFormat("Particles: %d", game_state.particle_count), 10, 10, 20, BLACK);
        DrawText(TextFormat("Gravity: %s", game_state.gravity_enabled ? "ON" : "OFF"), 10, 40, 20, BLACK);
        DrawText("Press 1-6 to select vehicle, 0 for free cam", 10, 70, 20, BLACK);
        DrawText("G: Toggle gravity, R: Reset", 10, 100, 20, BLACK);
        
        if(game_state.current_vehicle != -1) {
            char* vehicle_names[] = {"Car", "Truck", "Motorcycle", "Airplane", "Helicopter", "Tank"};
            DrawText(TextFormat("Current: %s", vehicle_names[game_state.current_vehicle]), 10, 130, 20, BLUE);
        } else {
            DrawText("Current: Free Camera", 10, 130, 20, BLUE);
        }
        
        EndDrawing();
    }
    
    CloseWindow();
    return 0;
}