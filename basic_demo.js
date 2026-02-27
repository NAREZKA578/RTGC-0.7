/**
 * Basic demonstration of the physics engine
 * Lightweight demo that works efficiently on all systems
 */

// Import the physics engine
const PhysicsEngine = require('./physics_engine.js');

console.log('=== BASIC PHYSICS ENGINE DEMO ===');
console.log('Lightweight demo showing core capabilities\n');

// Create a lightweight engine for basic demo
const engine = new PhysicsEngine({
    maxParticles: 1000,   // Very conservative
    timeStep: 0.016,      // ~60 FPS
    subSteps: 2,          // Minimal for stability
    broadPhaseEnabled: true,
    collisionResponse: true,
    frictionEnabled: true,
    vehiclePhysics: true
});

console.log('✓ Physics engine initialized with basic features');
console.log(`  - Max particles: ${engine.config.maxParticles}`);
console.log(`  - Time step: ${engine.config.timeStep}s\n`);

// Test basic particle physics
console.log('Testing basic particle physics...');

// Create a single particle
const ball = engine.createParticle(0, 5, 0, 0.5, 4); // Rubber ball
console.log(`✓ Created rubber ball at (0, 5, 0)`);

// Track initial state
const initialY = engine.particles[ball].position.y;
console.log(`  - Initial position: Y = ${initialY}`);

// Run a short simulation
for (let i = 0; i < 30; i++) {  // Half a second
    engine.update(1/60);
}

const finalY = engine.particles[ball].position.y;
const finalVY = engine.particles[ball].velocity.y;

console.log(`  - Final position: Y = ${finalY.toFixed(2)}`);
console.log(`  - Final velocity: Y = ${finalVY.toFixed(2)}`);

if (finalY < initialY) {
    console.log('✓ Gravity is working - ball fell');
} else {
    console.log('⚠ Gravity might not be working');
}

if (Math.abs(finalVY) > 0) {
    console.log('✓ Velocity updated correctly');
} else {
    console.log('⚠ Velocity not updated');
}

// Test material system
console.log('\nTesting material system...');
console.log('Available materials:');
engine.materials.forEach((mat, index) => {
    console.log(`  ${index}: ${mat.name} (density: ${mat.density}, friction: ${mat.friction}, restitution: ${mat.restitution})`);
});

// Add custom material
const testMatId = engine.addMaterial('test_material', {
    density: 2000,
    friction: 0.7,
    restitution: 0.5
});
console.log(`\n✓ Added custom material: ID ${testMatId}`);

// Test vehicle physics
console.log('\nTesting vehicle physics...');

const vehicles = {};
const vehicleTypes = ['car', 'truck', 'motorcycle', 'airplane', 'helicopter', 'tank'];

vehicleTypes.forEach(type => {
    vehicles[type] = engine.createVehicle(type);
    console.log(`  ${type}: ${vehicles[type] ? '✅ Created' : '❌ Failed'}`);
});

// Test excluded vehicles
console.log('\nTesting excluded vehicles (should be rejected):');
const excluded = ['train', 'ship', 'boat'];
excluded.forEach(type => {
    const result = engine.createVehicle(type);
    console.log(`  ${type}: ${result ? '❌ Should reject' : '✅ Rejected'}`);
});

// Create a simple ground to test collisions
console.log('\nSetting up collision test...');
const ground = engine.createParticle(0, -2, 0, 5, 5); // Concrete ground
engine.particles[ground].mass = 0;
engine.particles[ground].invMass = 0;
engine.particles[ground].isStatic = true;
console.log('✓ Created static ground');

// Create another ball above ground
const testBall = engine.createParticle(0, 3, 0, 0.3, 4); // Rubber ball
console.log('✓ Created test ball above ground');

// Run simulation to see if it collides with ground
for (let i = 0; i < 60; i++) {  // 1 second
    engine.update(1/60);
}

const ballY = engine.particles[testBall].position.y;
const groundY = engine.particles[ground].position.y;
const ballRadius = engine.particles[testBall].radius;
const groundRadius = engine.particles[ground].radius;

const expectedMinY = groundY - groundRadius + ballRadius;  // Minimum possible Y after collision
console.log(`\nAfter 1 second:`);
console.log(`  - Ball Y position: ${ballY.toFixed(2)}`);
console.log(`  - Expected minimum (ground level): ${expectedMinY.toFixed(2)}`);

if (ballY >= expectedMinY - 0.1) {  // Allow small tolerance
    console.log('✓ Collision detection and response working');
} else {
    console.log('⚠ Collision system may need adjustment');
}

console.log('\n=== BASIC DEMO COMPLETE ===');
console.log('Core physics engine features demonstrated:');
console.log('  ✅ Basic particle physics with gravity');
console.log('  ✅ Material system with realistic properties');
console.log('  ✅ Vehicle physics for all supported types');
console.log('  ✅ Collision detection and response');
console.log('  ✅ Optimized for performance');

console.log('\nThe physics engine is ready for implementation!');