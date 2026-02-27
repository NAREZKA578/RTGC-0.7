#ifndef VEHICLE_CONTROL_H
#define VEHICLE_CONTROL_H

#include "raylib.h"

typedef enum {
    VEHICLE_CAR,
    VEHICLE_TRUCK,
    VEHICLE_MOTORCYCLE,
    VEHICLE_AIRPLANE,
    VEHICLE_HELICOPTER,
    VEHICLE_TANK
} VehicleType;

typedef struct {
    Vector3 position;
    Vector3 velocity;
    Vector3 rotation;
    float mass;
    VehicleType type;
    // Additional vehicle-specific properties
    float throttle;
    float brake;
    float steering;
    bool handbrake;
    void* data;  // Pointer to vehicle-specific data
} Vehicle;

// Vehicle initialization functions
void init_car(Vehicle* v, Vector3 pos);
void init_truck(Vehicle* v, Vector3 pos);
void init_motorcycle(Vehicle* v, Vector3 pos);
void init_airplane(Vehicle* v, Vector3 pos);
void init_helicopter(Vehicle* v, Vector3 pos);
void init_tank(Vehicle* v, Vector3 pos);

// Vehicle control functions
void handle_vehicle_controls(Vehicle* v);
void update_vehicle(Vehicle* v, float delta_time);
void draw_vehicle(Vehicle* v);

#endif // VEHICLE_CONTROL_H