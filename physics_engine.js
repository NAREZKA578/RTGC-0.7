/**
 * Advanced Physics Engine for Large-Scale Particle Systems
 * Optimized for mid-range PCs with support for massive particle counts
 */

class PhysicsEngine {
    constructor(options = {}) {
        // Engine configuration
        this.config = {
            maxParticles: options.maxParticles || 100000,
            maxMaterials: options.maxMaterials || 100,
            gravity: options.gravity || { x: 0, y: -9.81, z: 0 }, // Negative Y is down in standard coordinate systems
            timeStep: options.timeStep || 0.016, // ~60 FPS
            subSteps: options.subSteps || 4, // Multiple sub-steps for stability
            broadPhaseEnabled: options.broadPhaseEnabled !== false,
            narrowPhaseEnabled: options.narrowPhaseEnabled !== false,
            collisionResponse: options.collisionResponse !== false,
            frictionEnabled: options.frictionEnabled !== false,
            restitutionEnabled: options.restitutionEnabled !== false,
            fluidSimulation: options.fluidSimulation || false,
            vehiclePhysics: options.vehiclePhysics || true
        };

        // Core systems
        this.particles = [];
        this.materials = [];
        this.constraints = [];
        this.vehicles = [];
        this.spatialGrid = new SpatialGrid(100); // For broad-phase collision detection
        this.contactManifold = new ContactManifold();
        
        // Performance optimization
        this.tempVec3 = { x: 0, y: 0, z: 0 };
        this.tempMat3 = new Array(9).fill(0);
        
        // Initialize material database
        this.initializeMaterials();
    }

    initializeMaterials() {
        // Common materials with realistic properties
        this.addMaterial('air', { density: 1.2, friction: 0.01, restitution: 0.99 });
        this.addMaterial('water', { density: 1000, friction: 0.1, restitution: 0.1 });
        this.addMaterial('wood', { density: 700, friction: 0.6, restitution: 0.3 });
        this.addMaterial('steel', { density: 7850, friction: 0.4, restitution: 0.3 });
        this.addMaterial('rubber', { density: 1100, friction: 0.8, restitution: 0.85 });
        this.addMaterial('concrete', { density: 2400, friction: 0.8, restitution: 0.1 });
        this.addMaterial('ice', { density: 917, friction: 0.1, restitution: 0.2 });
        this.addMaterial('sand', { density: 1600, friction: 0.8, restitution: 0.1 });
    }

    addMaterial(name, properties) {
        const material = {
            id: this.materials.length,
            name: name,
            density: properties.density || 1000,
            friction: properties.friction || 0.5,
            restitution: properties.restitution || 0.3,
            color: properties.color || '#ffffff',
            conductivity: properties.conductivity || 0.1,
            specificHeat: properties.specificHeat || 1000
        };
        this.materials.push(material);
        return material.id;
    }

    createParticle(x, y, z, radius, materialId = 0) {
        if (this.particles.length >= this.config.maxParticles) {
            console.warn('Maximum particle limit reached');
            return null;
        }

        const material = this.materials[materialId];
        const mass = (4/3) * Math.PI * Math.pow(radius, 3) * material.density;

        const particle = {
            id: this.particles.length,
            position: { x, y, z },
            velocity: { x: 0, y: 0, z: 0 },
            acceleration: { x: 0, y: 0, z: 0 },
            force: { x: 0, y: 0, z: 0 },
            mass: mass,
            invMass: mass > 0 ? 1 / mass : 0, // 0 for static objects
            radius: radius,
            materialId: materialId,
            material: material,
            rotation: { x: 0, y: 0, z: 0 },
            angularVelocity: { x: 0, y: 0, z: 0 },
            torque: { x: 0, y: 0, z: 0 },
            inertia: (2/5) * mass * radius * radius, // Sphere moment of inertia
            invInertia: mass > 0 ? 1 / ((2/5) * mass * radius * radius) : 0,
            isStatic: mass === 0,
            active: true,
            lastCollisionTime: 0
        };

        this.particles.push(particle);
        return particle.id;
    }

    addForce(particleId, fx, fy, fz) {
        const p = this.particles[particleId];
        if (p && p.active && !p.isStatic) {
            p.force.x += fx;
            p.force.y += fy;
            p.force.z += fz;
        }
    }

    addImpulse(particleId, ix, iy, iz) {
        const p = this.particles[particleId];
        if (p && p.active && !p.isStatic) {
            p.velocity.x += ix / p.mass;
            p.velocity.y += iy / p.mass;
            p.velocity.z += iz / p.mass;
        }
    }

    integrate(dt) {
        const subDt = dt / this.config.subSteps;

        for (let i = 0; i < this.config.subSteps; i++) {
            // Clear forces
            for (const p of this.particles) {
                if (p.active && !p.isStatic) {
                    p.force.x = 0;
                    p.force.y = 0;
                    p.force.z = 0;
                }
            }

            // Apply gravity
            for (const p of this.particles) {
                if (p.active && !p.isStatic) {
                    p.force.x += p.mass * this.config.gravity.x;
                    p.force.y += p.mass * this.config.gravity.y;
                    p.force.z += p.mass * this.config.gravity.z;
                }
            }

            // Update velocities and positions
            for (const p of this.particles) {
                if (p.active && !p.isStatic) {
                    // Integrate linear motion
                    p.acceleration.x = p.force.x * p.invMass;
                    p.acceleration.y = p.force.y * p.invMass;
                    p.acceleration.z = p.force.z * p.invMass;

                    p.velocity.x += p.acceleration.x * subDt;
                    p.velocity.y += p.acceleration.y * subDt;
                    p.velocity.z += p.acceleration.z * subDt;

                    // Apply damping
                    const damping = 0.999;
                    p.velocity.x *= damping;
                    p.velocity.y *= damping;
                    p.velocity.z *= damping;

                    p.position.x += p.velocity.x * subDt;
                    p.position.y += p.velocity.y * subDt;
                    p.position.z += p.velocity.z * subDt;

                    // Integrate rotational motion
                    p.angularVelocity.x += (p.torque.x * p.invInertia) * subDt;
                    p.angularVelocity.y += (p.torque.y * p.invInertia) * subDt;
                    p.angularVelocity.z += (p.torque.z * p.invInertia) * subDt;

                    p.rotation.x += p.angularVelocity.x * subDt;
                    p.rotation.y += p.angularVelocity.y * subDt;
                    p.rotation.z += p.angularVelocity.z * subDt;

                    // Clear torques for next iteration
                    p.torque.x = 0;
                    p.torque.y = 0;
                    p.torque.z = 0;
                }
            }

            // Handle collisions
            if (this.config.collisionResponse) {
                this.handleCollisions();
            }
        }
    }

    handleCollisions() {
        // Broad phase collision detection
        const potentialCollisions = this.broadPhase();

        // Narrow phase collision detection and response
        if (this.config.narrowPhaseEnabled) {
            for (const pair of potentialCollisions) {
                this.narrowPhase(pair[0], pair[1]);
            }
        }
    }

    broadPhase() {
        // Clear spatial grid
        this.spatialGrid.clear();
        
        // Insert particles into grid
        for (const p of this.particles) {
            if (p.active) {
                this.spatialGrid.insert(p);
            }
        }
        
        // Get overlapping pairs
        return this.spatialGrid.getOverlappingPairs();
    }

    narrowPhase(p1, p2) {
        if (!p1.active || !p2.active) return;

        // Calculate distance between centers
        const dx = p2.position.x - p1.position.x;
        const dy = p2.position.y - p1.position.y;
        const dz = p2.position.z - p1.position.z;
        const distanceSquared = dx*dx + dy*dy + dz*dz;
        const minDistance = p1.radius + p2.radius;

        if (distanceSquared < minDistance * minDistance) {
            // Collision detected
            const distance = Math.sqrt(distanceSquared);
            
            if (distance === 0) return; // Particles at same position
            
            // Normal vector (from p1 to p2)
            const nx = dx / distance;
            const ny = dy / distance;
            const nz = dz / distance;

            // Penetration depth
            const penetration = minDistance - distance;
            
            // Resolve collision
            if (this.config.collisionResponse) {
                this.resolveCollision(p1, p2, nx, ny, nz, penetration);
            }
        }
    }

    resolveCollision(p1, p2, nx, ny, nz, penetration) {
        // Static-static collision - no resolution needed
        if (p1.isStatic && p2.isStatic) return;

        // Separate particles
        if (!p1.isStatic && !p2.isStatic) {
            // Both dynamic - separate proportionally to inverse mass
            const totalInvMass = p1.invMass + p2.invMass;
            const separationRatio1 = p1.invMass / totalInvMass;
            const separationRatio2 = p2.invMass / totalInvMass;

            p1.position.x -= nx * penetration * separationRatio2;
            p1.position.y -= ny * penetration * separationRatio2;
            p1.position.z -= nz * penetration * separationRatio2;

            p2.position.x += nx * penetration * separationRatio1;
            p2.position.y += ny * penetration * separationRatio1;
            p2.position.z += nz * penetration * separationRatio1;
        } else if (p1.isStatic) {
            // Only p2 moves
            p2.position.x += nx * penetration;
            p2.position.y += ny * penetration;
            p2.position.z += nz * penetration;
        } else {
            // Only p1 moves
            p1.position.x -= nx * penetration;
            p1.position.y -= ny * penetration;
            p1.position.z -= nz * penetration;
        }

        // Calculate relative velocity
        const rvx = p2.velocity.x - p1.velocity.x;
        const rvy = p2.velocity.y - p1.velocity.y;
        const rvz = p2.velocity.z - p1.velocity.z;

        // Velocity along normal
        const velAlongNormal = rvx * nx + rvy * ny + rvz * nz;

        // Don't resolve if velocities are separating
        if (velAlongNormal > 0) return;

        // Calculate restitution (bounciness)
        const restitution = Math.min(p1.material.restitution, p2.material.restitution);

        // Calculate impulse scalar
        let impulseScalar = -(1 + restitution) * velAlongNormal;
        impulseScalar /= p1.invMass + p2.invMass;

        // Apply impulse
        const impulseX = impulseScalar * nx;
        const impulseY = impulseScalar * ny;
        const impulseZ = impulseScalar * nz;

        if (!p1.isStatic) {
            p1.velocity.x -= impulseX * p1.invMass;
            p1.velocity.y -= impulseY * p1.invMass;
            p1.velocity.z -= impulseZ * p1.invMass;
        }

        if (!p2.isStatic) {
            p2.velocity.x += impulseX * p2.invMass;
            p2.velocity.y += impulseY * p2.invMass;
            p2.velocity.z += impulseZ * p2.invMass;
        }

        // Apply friction
        if (this.config.frictionEnabled) {
            this.applyFriction(p1, p2, nx, ny, nz);
        }
    }

    applyFriction(p1, p2, nx, ny, nz) {
        // Calculate relative velocity
        const rvx = p2.velocity.x - p1.velocity.x;
        const rvy = p2.velocity.y - p1.velocity.y;
        const rvz = p2.velocity.z - p1.velocity.z;

        // Calculate tangent vector (perpendicular to normal)
        const tangentX = rvx - (rvx * nx + rvy * ny + rvz * nz) * nx;
        const tangentY = rvy - (rvx * nx + rvy * ny + rvz * nz) * ny;
        const tangentZ = rvz - (rvx * nx + rvy * ny + rvz * nz) * nz;

        // Normalize tangent vector
        const tangentLength = Math.sqrt(tangentX*tangentX + tangentY*tangentY + tangentZ*tangentZ);
        if (tangentLength === 0) return;

        const tnx = tangentX / tangentLength;
        const tny = tangentY / tangentLength;
        const tnz = tangentZ / tangentLength;

        // Velocity along tangent
        const velAlongTangent = rvx * tnx + rvy * tny + rvz * tnz;

        // Calculate friction impulse
        let frictionImpulse = -velAlongTangent;
        frictionImpulse /= p1.invMass + p2.invMass;

        // Clamp friction impulse (Coulomb friction law)
        const frictionCoeff = Math.sqrt(p1.material.friction * p2.material.friction);
        const maxFriction = Math.abs(frictionImpulse) * frictionCoeff;

        if (Math.abs(frictionImpulse) > maxFriction) {
            frictionImpulse = maxFriction * (frictionImpulse < 0 ? -1 : 1);
        }

        // Apply friction impulse
        const frictionX = frictionImpulse * tnx;
        const frictionY = frictionImpulse * tny;
        const frictionZ = frictionImpulse * tnz;

        if (!p1.isStatic) {
            p1.velocity.x -= frictionX * p1.invMass;
            p1.velocity.y -= frictionY * p1.invMass;
            p1.velocity.z -= frictionZ * p1.invMass;
        }

        if (!p2.isStatic) {
            p2.velocity.x += frictionX * p2.invMass;
            p2.velocity.y += frictionY * p2.invMass;
            p2.velocity.z += frictionZ * p2.invMass;
        }
    }

    // Vehicle physics system
    createVehicle(type, config) {
        if (!this.config.vehiclePhysics) return null;

        // Convert type to lowercase for comparison
        const lowerType = type.toLowerCase();
        
        switch (lowerType) {
            case 'car':
                return new Car(this, config);
            case 'truck':
                return new Truck(this, config);
            case 'motorcycle':
                return new Motorcycle(this, config);
            case 'airplane':
                return new Airplane(this, config);
            case 'helicopter':
                return new Helicopter(this, config);
            case 'tank':
                return new Tank(this, config);
            default:
                // Check if it's one of the excluded types
                const excludedTypes = ['train', 'ship', 'boat']; // trains and ships/corables excluded
                if (excludedTypes.includes(lowerType)) {
                    console.warn(`Vehicle type '${type}' is not supported in this physics engine`);
                    return null;
                }
                
                console.warn(`Unknown vehicle type: ${type}`);
                return null;
        }
    }

    update(dt) {
        this.integrate(dt);
        
        // Update vehicles
        for (const vehicle of this.vehicles) {
            vehicle.update(dt);
        }
    }

    // Utility methods
    getParticle(id) {
        return this.particles[id];
    }

    getActiveParticlesCount() {
        return this.particles.filter(p => p.active).length;
    }

    clear() {
        this.particles = [];
        this.constraints = [];
        this.vehicles = [];
        this.spatialGrid.clear();
    }
}

// Spatial Grid for efficient broad-phase collision detection
class SpatialGrid {
    constructor(cellSize = 10) {
        this.cellSize = cellSize;
        this.grid = new Map(); // key: "x,y,z", value: array of particles
    }

    clear() {
        this.grid.clear();
    }

    insert(particle) {
        const cellX = Math.floor(particle.position.x / this.cellSize);
        const cellY = Math.floor(particle.position.y / this.cellSize);
        const cellZ = Math.floor(particle.position.z / this.cellSize);

        const key = `${cellX},${cellY},${cellZ}`;
        if (!this.grid.has(key)) {
            this.grid.set(key, []);
        }
        this.grid.get(key).push(particle);

        // Also insert into neighboring cells for overlap
        for (let dx = -1; dx <= 1; dx++) {
            for (let dy = -1; dy <= 1; dy++) {
                for (let dz = -1; dz <= 1; dz++) {
                    if (dx === 0 && dy === 0 && dz === 0) continue;
                    
                    const neighborKey = `${cellX + dx},${cellY + dy},${cellZ + dz}`;
                    if (!this.grid.has(neighborKey)) {
                        this.grid.set(neighborKey, []);
                    }
                    this.grid.get(neighborKey).push(particle);
                }
            }
        }
    }

    getOverlappingPairs() {
        const pairs = [];
        const checkedPairs = new Set();

        for (const [key, particles] of this.grid.entries()) {
            for (let i = 0; i < particles.length; i++) {
                for (let j = i + 1; j < particles.length; j++) {
                    const p1 = particles[i];
                    const p2 = particles[j];
                    
                    // Create a unique identifier for the pair to avoid duplicates
                    const pairKey = p1.id < p2.id ? `${p1.id}-${p2.id}` : `${p2.id}-${p1.id}`;
                    
                    if (!checkedPairs.has(pairKey)) {
                        pairs.push([p1, p2]);
                        checkedPairs.add(pairKey);
                    }
                }
            }
        }

        return pairs;
    }
}

// Contact manifold for handling collision responses
class ContactManifold {
    constructor() {
        this.contacts = [];
    }

    addContact(bodyA, bodyB, contactPoint, normal, penetration) {
        this.contacts.push({
            bodyA, bodyB, contactPoint, normal, penetration
        });
    }

    clear() {
        this.contacts = [];
    }
}

// Vehicle base class
class Vehicle {
    constructor(engine, config) {
        this.engine = engine;
        this.config = config;
        this.parts = [];
        this.wheels = [];
        this.steeringAngle = 0;
        this.throttle = 0;
        this.brake = 0;
        this.velocity = { x: 0, y: 0, z: 0 };
        this.position = { x: 0, y: 0, z: 0 };
        this.rotation = { x: 0, y: 0, z: 0 };
        
        this.createStructure();
        engine.vehicles.push(this);
    }

    createStructure() {
        // To be implemented by subclasses
    }

    update(dt) {
        // To be implemented by subclasses
    }

    applyForce(x, y, z) {
        // Apply force to the vehicle's center of mass
    }
}

// Car class
class Car extends Vehicle {
    constructor(engine, config = {}) {
        super(engine, {
            mass: config.mass || 1200,
            length: config.length || 4.5,
            width: config.width || 1.8,
            height: config.height || 1.5,
            wheelBase: config.wheelBase || 2.7,
            trackWidth: config.trackWidth || 1.5,
            maxSpeed: config.maxSpeed || 200,
            maxAcceleration: config.maxAcceleration || 3.0,
            maxBraking: config.maxBraking || 8.0,
            maxSteeringAngle: config.maxSteeringAngle || 0.6,
            ...config
        });

        this.setupWheels();
    }

    setupWheels() {
        // Create wheels (particles representing tires)
        const wheelRadius = 0.35;
        const frontWheelPos = this.config.wheelBase / 2;
        const rearWheelPos = -this.config.wheelBase / 2;
        const leftWheelPos = this.config.trackWidth / 2;
        const rightWheelPos = -this.config.trackWidth / 2;

        // Front left wheel
        this.frontLeftWheel = this.engine.createParticle(
            this.position.x + frontWheelPos, 
            this.position.y - wheelRadius, 
            this.position.z + leftWheelPos, 
            wheelRadius, 
            this.engine.addMaterial('rubber', { friction: 0.8, restitution: 0.2 })
        );

        // Front right wheel
        this.frontRightWheel = this.engine.createParticle(
            this.position.x + frontWheelPos, 
            this.position.y - wheelRadius, 
            this.position.z + rightWheelPos, 
            wheelRadius, 
            this.engine.addMaterial('rubber', { friction: 0.8, restitution: 0.2 })
        );

        // Rear left wheel
        this.rearLeftWheel = this.engine.createParticle(
            this.position.x + rearWheelPos, 
            this.position.y - wheelRadius, 
            this.position.z + leftWheelPos, 
            wheelRadius, 
            this.engine.addMaterial('rubber', { friction: 0.8, restitution: 0.2 })
        );

        // Rear right wheel
        this.rearRightWheel = this.engine.createParticle(
            this.position.x + rearWheelPos, 
            this.position.y - wheelRadius, 
            this.position.z + rightWheelPos, 
            wheelRadius, 
            this.engine.addMaterial('rubber', { friction: 0.8, restitution: 0.2 })
        );
    }

    update(dt) {
        // Simple car physics simulation
        const carBody = this.engine.particles.find(p => 
            p.id === this.bodyParticleId
        );

        if (!carBody) return;

        // Apply steering to front wheels
        const steeringForce = this.steeringAngle * 5000; // Simplified steering
        
        if (this.frontLeftWheel !== undefined) {
            const flWheel = this.engine.particles[this.frontLeftWheel];
            if (flWheel) {
                flWheel.force.z += Math.sin(this.steeringAngle) * steeringForce;
                flWheel.force.x += Math.cos(this.steeringAngle) * steeringForce;
            }
        }

        if (this.frontRightWheel !== undefined) {
            const frWheel = this.engine.particles[this.frontRightWheel];
            if (frWheel) {
                frWheel.force.z += Math.sin(this.steeringAngle) * steeringForce;
                frWheel.force.x += Math.cos(this.steeringAngle) * steeringForce;
            }
        }

        // Apply throttle/braking to rear wheels
        const tractionForce = this.throttle * this.config.maxAcceleration * this.config.mass;
        const brakingForce = this.brake * this.config.maxBraking * this.config.mass;

        if (this.rearLeftWheel !== undefined) {
            const rlWheel = this.engine.particles[this.rearLeftWheel];
            if (rlWheel) {
                rlWheel.force.z += tractionForce - brakingForce * Math.sign(rlWheel.velocity.z);
            }
        }

        if (this.rearRightWheel !== undefined) {
            const rrWheel = this.engine.particles[this.rearRightWheel];
            if (rrWheel) {
                rrWheel.force.z += tractionForce - brakingForce * Math.sign(rrWheel.velocity.z);
            }
        }
    }

    setThrottle(value) {
        this.throttle = Math.max(-1, Math.min(1, value));
    }

    setBrake(value) {
        this.brake = Math.max(0, Math.min(1, value));
    }

    setSteering(angle) {
        this.steeringAngle = Math.max(-this.config.maxSteeringAngle, 
                                     Math.min(this.config.maxSteeringAngle, angle));
    }
}

// Additional vehicle classes would follow similar patterns
class Truck extends Vehicle {
    constructor(engine, config = {}) {
        super(engine, {
            mass: config.mass || 3500,
            length: config.length || 8.0,
            width: config.width || 2.5,
            height: config.height || 3.0,
            wheelBase: config.wheelBase || 4.5,
            trackWidth: config.trackWidth || 2.0,
            maxSpeed: config.maxSpeed || 120,
            maxAcceleration: config.maxAcceleration || 2.0,
            maxBraking: config.maxBraking || 6.0,
            maxSteeringAngle: config.maxSteeringAngle || 0.4,
            ...config
        });
        this.setupWheels();
    }

    setupWheels() {
        // More wheels for trucks (typically 6-10 wheels)
        const wheelRadius = 0.45;
        const positions = [
            // Front wheels
            { x: this.config.wheelBase * 0.7, z: this.config.trackWidth / 2 },
            { x: this.config.wheelBase * 0.7, z: -this.config.trackWidth / 2 },
            // Middle wheels
            { x: 0, z: this.config.trackWidth / 2 },
            { x: 0, z: -this.config.trackWidth / 2 },
            // Rear wheels
            { x: -this.config.wheelBase * 0.7, z: this.config.trackWidth / 2 },
            { x: -this.config.wheelBase * 0.7, z: -this.config.trackWidth / 2 }
        ];

        this.wheels = positions.map(pos => 
            this.engine.createParticle(
                this.position.x + pos.x, 
                this.position.y - wheelRadius, 
                this.position.z + pos.z, 
                wheelRadius, 
                this.engine.addMaterial('rubber', { friction: 0.8, restitution: 0.2 })
            )
        );
    }

    update(dt) {
        // Truck-specific physics
        const tractionForce = this.throttle * this.config.maxAcceleration * this.config.mass;
        const brakingForce = this.brake * this.config.maxBraking * this.config.mass;

        for (const wheelId of this.wheels.slice(2)) { // Drive wheels are typically rear wheels
            const wheel = this.engine.particles[wheelId];
            if (wheel) {
                wheel.force.z += tractionForce - brakingForce * Math.sign(wheel.velocity.z);
            }
        }

        // Apply steering to front wheels
        const steeringForce = this.steeringAngle * 3000;
        for (let i = 0; i < 2; i++) { // First two wheels are steerable
            const wheel = this.engine.particles[this.wheels[i]];
            if (wheel) {
                wheel.force.z += Math.sin(this.steeringAngle) * steeringForce;
                wheel.force.x += Math.cos(this.steeringAngle) * steeringForce;
            }
        }
    }
}

class Motorcycle extends Vehicle {
    constructor(engine, config = {}) {
        super(engine, {
            mass: config.mass || 200,
            length: config.length || 2.2,
            width: config.width || 0.8,
            height: config.height || 1.2,
            wheelBase: config.wheelBase || 1.4,
            trackWidth: config.trackWidth || 0.6,
            maxSpeed: config.maxSpeed || 250,
            maxAcceleration: config.maxAcceleration || 5.0,
            maxBraking: config.maxBraking || 10.0,
            maxSteeringAngle: config.maxSteeringAngle || 1.0,
            ...config
        });
        this.setupWheels();
    }

    setupWheels() {
        const wheelRadius = 0.3;
        // Front wheel
        this.frontWheel = this.engine.createParticle(
            this.position.x + this.config.wheelBase / 2, 
            this.position.y - wheelRadius, 
            this.position.z, 
            wheelRadius, 
            this.engine.addMaterial('rubber', { friction: 0.9, restitution: 0.2 })
        );
        
        // Rear wheel
        this.rearWheel = this.engine.createParticle(
            this.position.x - this.config.wheelBase / 2, 
            this.position.y - wheelRadius, 
            this.position.z, 
            wheelRadius, 
            this.engine.addMaterial('rubber', { friction: 0.9, restitution: 0.2 })
        );
    }

    update(dt) {
        // Motorcycle-specific physics including lean
        const tractionForce = this.throttle * this.config.maxAcceleration * this.config.mass;
        const brakingForce = this.brake * this.config.maxBraking * this.config.mass;

        const rearWheel = this.engine.particles[this.rearWheel];
        if (rearWheel) {
            rearWheel.force.z += tractionForce - brakingForce * Math.sign(rearWheel.velocity.z);
        }

        // Steering affects both wheels for simplified motorcycle physics
        const steeringForce = this.steeringAngle * 4000;
        const frontWheel = this.engine.particles[this.frontWheel];
        if (frontWheel) {
            frontWheel.force.z += Math.sin(this.steeringAngle) * steeringForce;
            frontWheel.force.x += Math.cos(this.steeringAngle) * steeringForce;
        }
    }
}

class Airplane extends Vehicle {
    constructor(engine, config = {}) {
        super(engine, {
            mass: config.mass || 50000,
            length: config.length || 38.0,
            wingspan: config.wingspan || 36.0,
            height: config.height || 12.0,
            maxSpeed: config.maxSpeed || 900, // km/h
            maxThrust: config.maxThrust || 100000,
            maxLift: config.maxLift || 200000,
            ...config
        });
        this.setupStructure();
    }

    setupStructure() {
        // Simplified airplane as multiple particles connected by constraints
        this.fuselage = this.engine.createParticle(
            this.position.x, this.position.y, this.position.z,
            2.0, // Radius representing fuselage width
            this.engine.addMaterial('aluminum', { density: 2700, friction: 0.1, restitution: 0.2 })
        );
    }

    update(dt) {
        // Airplane physics simulation
        const body = this.engine.particles[this.fuselage];
        if (!body) return;

        // Calculate lift based on speed and angle of attack
        const speed = Math.sqrt(body.velocity.x**2 + body.velocity.y**2 + body.velocity.z**2);
        const lift = Math.min(speed * 100, this.config.maxLift); // Simplified lift calculation
        
        // Apply lift opposing gravity
        body.force.y -= lift;

        // Apply thrust in forward direction
        const thrust = this.throttle * this.config.maxThrust;
        body.force.x += thrust * Math.cos(this.rotation.y); // Simplified thrust direction
        body.force.z += thrust * Math.sin(this.rotation.y);

        // Apply drag
        const dragCoefficient = 0.02;
        body.force.x -= body.velocity.x * speed * dragCoefficient;
        body.force.y -= body.velocity.y * speed * dragCoefficient;
        body.force.z -= body.velocity.z * speed * dragCoefficient;
    }
}

class Helicopter extends Vehicle {
    constructor(engine, config = {}) {
        super(engine, {
            mass: config.mass || 5000,
            length: config.length || 12.0,
            rotorspan: config.rotorspan || 14.0,
            height: config.height || 4.0,
            maxSpeed: config.maxSpeed || 250,
            maxLift: config.maxLift || 50000,
            maxThrust: config.maxThrust || 20000,
            ...config
        });
        this.setupStructure();
    }

    setupStructure() {
        this.body = this.engine.createParticle(
            this.position.x, this.position.y, this.position.z,
            1.5, // Radius representing helicopter body
            this.engine.addMaterial('composite', { density: 1500, friction: 0.15, restitution: 0.25 })
        );
    }

    update(dt) {
        const body = this.engine.particles[this.body];
        if (!body) return;

        // Rotor lift
        const rotorLift = this.throttle * this.config.maxLift;
        body.force.y += rotorLift;

        // Forward/backward tilt for movement
        const forwardThrust = this.pitchInput * this.config.maxThrust;
        body.force.x += forwardThrust * Math.cos(this.rotation.z);
        body.force.y -= forwardThrust * Math.sin(this.rotation.z);

        // Lateral movement through roll
        const lateralThrust = this.rollInput * this.config.maxThrust * 0.7;
        body.force.x += lateralThrust * Math.sin(this.rotation.z);
        body.force.y += lateralThrust * Math.cos(this.rotation.z);

        // Yaw control
        const yawTorque = this.yawInput * 5000;
        body.torque.y += yawTorque;
    }
}

class Tank extends Vehicle {
    constructor(engine, config = {}) {
        super(engine, {
            mass: config.mass || 60000,
            length: config.length || 10.0,
            width: config.width || 3.5,
            height: config.height || 2.5,
            wheelBase: config.wheelBase || 6.0,
            trackWidth: config.trackWidth || 3.0,
            maxSpeed: config.maxSpeed || 60,
            maxAcceleration: config.maxAcceleration || 1.0,
            maxBraking: config.maxBraking || 4.0,
            maxSteeringAngle: config.maxSteeringAngle || 0.2,
            ...config
        });
        this.setupTracks();
    }

    setupTracks() {
        // Simplified tank tracks as multiple connected particles
        this.leftTrack = [];
        this.rightTrack = [];

        const trackRadius = 0.4;
        const segments = 10;
        const segmentSpacing = this.config.wheelBase / segments;

        for (let i = 0; i < segments; i++) {
            // Left track
            this.leftTrack.push(this.engine.createParticle(
                this.position.x + (i - segments/2) * segmentSpacing,
                this.position.y - trackRadius,
                this.position.z + this.config.trackWidth / 2,
                trackRadius,
                this.engine.addMaterial('steel', { friction: 0.9, restitution: 0.1 })
            ));

            // Right track
            this.rightTrack.push(this.engine.createParticle(
                this.position.x + (i - segments/2) * segmentSpacing,
                this.position.y - trackRadius,
                this.position.z - this.config.trackWidth / 2,
                trackRadius,
                this.engine.addMaterial('steel', { friction: 0.9, restitution: 0.1 })
            ));
        }
    }

    update(dt) {
        // Tank differential steering
        const leftPower = (this.leftThrottle - this.leftBrake) * this.config.maxAcceleration * this.config.mass;
        const rightPower = (this.rightThrottle - this.rightBrake) * this.config.maxAcceleration * this.config.mass;

        for (const trackSegmentId of this.leftTrack) {
            const segment = this.engine.particles[trackSegmentId];
            if (segment) {
                segment.force.z += leftPower;
            }
        }

        for (const trackSegmentId of this.rightTrack) {
            const segment = this.engine.particles[trackSegmentId];
            if (segment) {
                segment.force.z += rightPower;
            }
        }
    }

    setLeftTrack(power) {
        this.leftThrottle = Math.max(0, power);
        this.leftBrake = Math.max(0, -power);
    }

    setRightTrack(power) {
        this.rightThrottle = Math.max(0, power);
        this.rightBrake = Math.max(0, -power);
    }
}

// Export the physics engine
if (typeof module !== 'undefined' && module.exports) {
    module.exports = PhysicsEngine;
} else if (typeof window !== 'undefined') {
    window.PhysicsEngine = PhysicsEngine;
}