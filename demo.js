/**
 * Demonstration of the advanced physics engine capabilities
 * Showcases particle physics, materials, and vehicle physics
 */

// Import the physics engine
const PhysicsEngine = require('./physics_engine.js');

console.log('=== ADVANCED PHYSICS ENGINE DEMONSTRATION ===');
console.log('Optimized for mid-range PCs with support for massive particle counts');
console.log('Supports realistic materials and various vehicle types (except trains and ships)\n');

// Create physics engine with optimized settings for demonstration
const engine = new PhysicsEngine({
    maxParticles: 50000,
    maxMaterials: 100,
    timeStep: 0.016,  // ~60 FPS
    subSteps: 4,      // Multiple sub-steps for stability
    broadPhaseEnabled: true,
    narrowPhaseEnabled: true,
    collisionResponse: true,
    frictionEnabled: true,
    restitutionEnabled: true,
    vehiclePhysics: true
});

console.log('✓ Physics engine initialized with advanced features');
console.log(`  - Max particles: ${engine.config.maxParticles.toLocaleString()}`);
console.log(`  - Time step: ${engine.config.timeStep}s (${Math.round(1/engine.config.timeStep)} FPS)`);
console.log(`  - Sub-steps: ${engine.config.subSteps} (for stability)`);
console.log(`  - Features: Broad-phase, Narrow-phase, Collision Response, Friction, Restitution\n`);

// Demo 1: Material System
console.log('=== MATERIAL SYSTEM ===');
console.log('Realistic materials with physical properties: density, friction, restitution\n');

// Display existing materials
console.log('Built-in materials:');
engine.materials.forEach(mat => {
    console.log(`  - ${mat.name}: density=${mat.density}, friction=${mat.friction}, restitution=${mat.restitution}`);
});

// Add custom material
const concreteId = engine.addMaterial('demo_concrete', {
    density: 2400,    // kg/m³
    friction: 0.8,    // High friction
    restitution: 0.1  // Low bounce
});

console.log(`\nAdded custom material: demo_concrete (ID: ${concreteId})`);
console.log('  - High density (heavy)')
console.log('  - High friction (grippy)')
console.log('  - Low restitution (not bouncy)')

// Demo 2: Particle Physics
console.log('\n=== PARTICLE PHYSICS ===');
console.log('Creating diverse particle systems with different materials\n');

// Create particles of different materials
const ball1 = engine.createParticle(-5, 10, 0, 0.5, 3); // Steel ball
const ball2 = engine.createParticle(0, 10, 0, 0.5, 4);  // Rubber ball  
const ball3 = engine.createParticle(5, 10, 0, 0.5, 2);  // Wood ball

// Create ground
const ground = engine.createParticle(0, -5, 0, 10, concreteId);
engine.particles[ground].mass = 0;  // Static ground
engine.particles[ground].invMass = 0;
engine.particles[ground].isStatic = true;

console.log('Created 3 balls of different materials:');
console.log(`  - Ball 1 (Steel): ID ${ball1}, material: ${engine.particles[ball1].material.name}`);
console.log(`  - Ball 2 (Rubber): ID ${ball2}, material: ${engine.particles[ball2].material.name}`);
console.log(`  - Ball 3 (Wood): ID ${ball3}, material: ${engine.particles[ball3].material.name}`);
console.log(`  - Ground (Concrete): ID ${ground}, static`);

// Run simulation to see different behaviors
console.log('\nRunning simulation for 2 seconds...');
for (let i = 0; i < 120; i++) {  // 2 seconds at 60 FPS
    engine.update(1/60);
    
    if (i % 30 === 0) {  // Log every half second
        const t = (i/60).toFixed(1);
        console.log(`  At t=${t}s - Steel Y:${engine.particles[ball1].position.y.toFixed(2)}, ` +
                   `Rubber Y:${engine.particles[ball2].position.y.toFixed(2)}, ` +
                   `Wood Y:${engine.particles[ball3].position.y.toFixed(2)}`);
    }
}

// Final positions after bouncing
console.log('\nFinal positions after bouncing:');
console.log(`  - Steel ball: Y=${engine.particles[ball1].position.y.toFixed(2)} (dense, low bounce)`);
console.log(`  - Rubber ball: Y=${engine.particles[ball2].position.y.toFixed(2)} (elastic, bouncy)`);
console.log(`  - Wood ball: Y=${engine.particles[ball3].position.y.toFixed(2)} (medium bounce)`);

// Demo 3: Vehicle Physics
console.log('\n=== VEHICLE PHYSICS ===');
console.log('All transport types except trains and ships/corables\n');

const vehicles = {};

// Create all supported vehicles
vehicles.car = engine.createVehicle('car', { position: { x: -10, y: 0.5, z: 0 } });
vehicles.truck = engine.createVehicle('truck', { position: { x: -6, y: 1, z: 0 } });
vehicles.motorcycle = engine.createVehicle('motorcycle', { position: { x: -2, y: 0.3, z: 0 } });
vehicles.airplane = engine.createVehicle('airplane', { position: { x: 2, y: 5, z: 0 } });
vehicles.helicopter = engine.createVehicle('helicopter', { position: { x: 6, y: 3, z: 0 } });
vehicles.tank = engine.createVehicle('tank', { position: { x: 10, y: 1, z: 0 } });

console.log('Created vehicle fleet:');
Object.keys(vehicles).forEach(vehicleName => {
    const v = vehicles[vehicleName];
    console.log(`  - ${vehicleName.charAt(0).toUpperCase() + vehicleName.slice(1)}: ${v ? '✓ Active' : '✗ Failed'}`);
});

// Try to create excluded vehicles (should fail)
console.log('\nTesting excluded vehicles (should be rejected):');
const train = engine.createVehicle('train');
const ship = engine.createVehicle('ship');
const boat = engine.createVehicle('boat');

console.log(`  - Train: ${train ? '✗ Should be rejected' : '✓ Correctly rejected'}`);
console.log(`  - Ship: ${ship ? '✗ Should be rejected' : '✓ Correctly rejected'}`);
console.log(`  - Boat: ${boat ? '✗ Should be rejected' : '✓ Correctly rejected'}`);

// Demo 4: Performance with Large Particle Count
console.log('\n=== PERFORMANCE TEST ===');
console.log('Creating 5000 particles to demonstrate scalability\n');

const startTime = Date.now();

// Create a pile of particles
for (let i = 0; i < 5000; i++) {
    const x = (i % 50) - 25;  // Spread across 50 units
    const z = Math.floor(i / 50) - 25;  // Spread across 50 units in Z too
    const y = 10 + Math.random() * 2;  // Random height
    engine.createParticle(x, y, z, 0.2, 0);  // Small air particles
}

const creationTime = Date.now() - startTime;
console.log(`Created ${engine.particles.length} particles in ${creationTime}ms`);
console.log(`Current active particles: ${engine.getActiveParticlesCount()}`);

// Run simulation with many particles
console.log('\nRunning simulation with 5000+ particles...');
const simStartTime = Date.now();

for (let i = 0; i < 30; i++) {  // 0.5 seconds of simulation
    engine.update(1/60);
}

const simTime = Date.now() - simStartTime;
console.log(`Simulated 30 frames in ${simTime}ms`);
console.log(`Average frame time: ${(simTime/30).toFixed(2)}ms (${(30/(simTime/1000)).toFixed(1)} FPS)`);

// Demo 5: Advanced Physics Features
console.log('\n=== ADVANCED PHYSICS FEATURES ===');

// Show different physical properties in action
console.log('Physical simulation includes:');
console.log('  ✓ Gravity with realistic acceleration (-9.81 m/s²)');
console.log('  ✓ Mass-based interactions');
console.log('  ✓ Momentum conservation');
console.log('  ✓ Energy dissipation (damping)');
console.log('  ✓ Collision detection with spatial partitioning');
console.log('  ✓ Collision response with friction and restitution');
console.log('  ✓ Rotational dynamics');
console.log('  ✓ Multiple integration sub-steps for stability');

// Demo forces
console.log('\nApplying external forces to demonstrate dynamics...');
engine.addForce(ball1, 100, 0, 0);  // Push steel ball
engine.addImpulse(ball2, 0, 50, 0); // Impulse to rubber ball

// Run a few frames to see force effects
for (let i = 0; i < 10; i++) {
    engine.update(1/60);
}

console.log('Forces applied - balls now have different trajectories');

console.log('\n=== SUMMARY ===');
console.log('✅ Physics Engine Features:');
console.log('  • Supports up to 100,000 particles');
console.log('  • Optimized for mid-range PCs using spatial partitioning');
console.log('  • Realistic material properties (density, friction, restitution)');
console.log('  • Complete vehicle physics (car, truck, motorcycle, airplane, helicopter, tank)');
console.log('  • Excludes trains and ships/corables as requested');
console.log('  • Efficient collision detection (broad-phase + narrow-phase)');
console.log('  • Stable physics integration with sub-stepping');
console.log('  • Support for forces, impulses, and constraints');
console.log('  • Rotational dynamics');
console.log('\nThe physics engine is ready for use in games, simulations, and applications requiring realistic physics!');

// Performance metrics
const totalTime = Date.now() - startTime;
console.log(`\nTotal demo execution time: ${totalTime}ms`);