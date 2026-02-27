#include "Vehicle.hpp"
#include "../core/Logger.hpp"

Vehicle::Vehicle(const VehicleType& type, const Vector3& startPos) 
    : mType(type), mThrottle(0.0f), mSteering(0.0f), mBrake(0.0f) {
    
    // Create physics body for the vehicle chassis
    mActor = std::make_unique<PhysicsBody>(startPos, mType.mass, false);
    mActor->SetRestitution(0.1f);
    mActor->SetFriction(0.8f);
    
    // Initialize wheels
    mWheels = mType.wheels;
    
    Logger::Log("Created vehicle: ", mType.name, " at position: ", startPos.x, ", ", startPos.y, ", ", startPos.z);
}

void Vehicle::Update(float dt) {
    // Update wheels first
    UpdateWheels(dt);
    
    // Apply forces based on input
    ApplyEngineForces(dt);
    ApplySteering(dt);
    
    // Update the main physics body
    if (mActor) {
        mActor->Update(dt);
    }
}

void Vehicle::UpdateWheels(float dt) {
    // Simple wheel-ground contact simulation
    for (auto& wheel : mWheels) {
        // Calculate wheel world position
        Vector3 wheelWorldPos = mActor->GetPosition() + wheel.position;
        
        // Check if wheel is touching ground
        float groundHeight = 0.0f; // Should use terrain height
        wheel.grounded = (wheelWorldPos.y - wheel.radius) <= groundHeight;
        
        if (wheel.grounded) {
            // Apply simple suspension force
            float compression = (groundHeight + wheel.radius) - wheelWorldPos.y;
            if (compression > 0) {
                Vector3 suspensionForce(0, compression * 10000.0f, 0); // Stiffness
                mActor->ApplyForce(suspensionForce);
            }
        }
    }
}

void Vehicle::ApplyEngineForces(float dt) {
    if (!mActor) return;
    
    // Determine which wheels receive power based on drive type
    for (size_t i = 0; i < mWheels.size(); ++i) {
        bool powered = false;
        
        switch (mType.type) {
            case VehicleDriveType::Wheeled2WD:
                // Rear-wheel drive - typically wheels 2 and 3 (if 4 wheels)
                powered = (i >= 2);
                break;
            case VehicleDriveType::Wheeled4WD:
                // All wheels powered
                powered = true;
                break;
            case VehicleDriveType::WheeledFront:
                // Front-wheel drive - typically wheels 0 and 1
                powered = (i < 2);
                break;
            case VehicleDriveType::WheeledRear:
                // Rear-wheel drive - typically wheels 2 and 3
                powered = (i >= 2);
                break;
        }
        
        if (powered && mWheels[i].grounded) {
            // Calculate force based on throttle and RPM
            float torque = mType.engineTorque * mThrottle;
            float forceMagnitude = torque / mWheels[i].radius;
            
            // Apply force in the direction of the vehicle
            Vector3 forward = Vector3(1, 0, 0); // This should be based on vehicle orientation
            Vector3 force = forward * forceMagnitude;
            
            // Apply to the chassis at the wheel position
            mActor->ApplyForce(force);
        }
        
        // Apply brake
        if (mBrake > 0.0f) {
            float brakeForce = mBrake * mWheels[i].maxBrakeTorque / mWheels[i].radius;
            
            // Apply braking force opposite to velocity
            Vector3 vel = mActor->GetVelocity();
            if (vel.LengthSquared() > 0.001f) {
                Vector3 brakeDir = vel.Normalize();
                Vector3 brakeVec = brakeDir * (-brakeForce);
                mActor->ApplyForce(brakeVec);
            }
        }
    }
}

void Vehicle::ApplySteering(float dt) {
    if (!mActor) return;
    
    // Apply steering to front wheels (for simplicity, just change direction)
    // In a real implementation, we would calculate the actual wheel angles
    if (abs(mSteering) > 0.01f) {
        // Apply a turning moment based on steering input
        Vector3 turnAxis(0, 1, 0); // Y-axis for upright steering
        float turnForce = mSteering * 500.0f; // Steering sensitivity
        
        // Apply to front wheels if they exist
        for (size_t i = 0; i < mWheels.size(); ++i) {
            if (i < 2) { // First two wheels are front wheels
                Vector3 leverArm = mWheels[i].position;
                Vector3 torque = turnAxis.Cross(Vector3(turnForce, 0, 0));
                // Simplified: just apply a lateral force
                Vector3 lateralForce(-turnForce * mSteering, 0, 0);
                mActor->ApplyForce(lateralForce);
            }
        }
    }
}

void Vehicle::SetThrottle(float t) {
    mThrottle = std::max(-1.0f, std::min(1.0f, t)); // Clamp between -1 and 1
}

void Vehicle::SetSteering(float s) {
    mSteering = std::max(-1.0f, std::min(1.0f, s)); // Clamp between -1 and 1
}

void Vehicle::SetBrake(float b) {
    mBrake = std::max(0.0f, std::min(1.0f, b)); // Clamp between 0 and 1
}