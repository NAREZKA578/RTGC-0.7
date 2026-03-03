/**
 * Simple test for the physics engine to verify basic functionality
 */

// Import the physics engine
const PhysicsEngine = require('./physics_engine.js');

console.log('Starting Simple Physics Engine Test...');

// Create engine with minimal settings
const engine = new PhysicsEngine({
    maxParticles: 1000,
    timeStep: 0.016,
    subSteps: 4,
    collisionResponse: true
});

console.log('✓ Engine created successfully');

// Test basic particle creation
const particleId = engine.createParticle(0, 10, 0, 0.5);
console.log(`✓ Created particle with ID: ${particleId}`);

const particle = engine.getParticle(particleId);
console.log(`✓ Particle initial position: (${particle.position.x}, ${particle.position.y}, ${particle.position.z})`);

// Run a few simulation steps
for (let i = 0; i < 10; i++) {
    engine.update(1/60);
    console.log(`Step ${i+1}: Position (${particle.position.x.toFixed(2)}, ${particle.position.y.toFixed(2)}, ${particle.position.z.toFixed(2)})`);
}

console.log(`✓ Final position: (${particle.position.x.toFixed(2)}, ${particle.position.y.toFixed(2)}, ${particle.position.z.toFixed(2)})`);
console.log(`✓ Final velocity: (${particle.velocity.x.toFixed(2)}, ${particle.velocity.y.toFixed(2)}, ${particle.velocity.z.toFixed(2)})`);

// Verify particle moved (due to gravity)
if (particle.position.y < 10) {
    console.log('✓ Gravity is working - particle fell');
} else {
    console.log('⚠ Gravity might not be working properly');
}

if (Math.abs(particle.velocity.y) > 0) {
    console.log('✓ Velocity updated correctly');
} else {
    console.log('⚠ Velocity not updated');
}

// Test material creation
const rubberId = engine.addMaterial('rubber_test', {
    density: 1100,
    friction: 0.8,
    restitution: 0.85
});
console.log(`✓ Created material with ID: ${rubberId}`);

// Test vehicle creation (car)
const car = engine.createVehicle('car');
if (car) {
    console.log('✓ Car created successfully');
    console.log(`✓ Car type: ${car.constructor.name}`);
} else {
    console.log('⚠ Failed to create car');
}

// Test excluded vehicle (train should be rejected)
const train = engine.createVehicle('train');
if (train === null) {
    console.log('✓ Train correctly rejected (as per requirements)');
} else {
    console.log('⚠ Train should have been rejected');
}

// Test other vehicles
const truck = engine.createVehicle('truck');
const motorcycle = engine.createVehicle('motorcycle');
const airplane = engine.createVehicle('airplane');
const helicopter = engine.createVehicle('helicopter');
const tank = engine.createVehicle('tank');

console.log(`✓ Truck: ${truck ? 'created' : 'failed'}`);
console.log(`✓ Motorcycle: ${motorcycle ? 'created' : 'failed'}`);
console.log(`✓ Airplane: ${airplane ? 'created' : 'failed'}`);
console.log(`✓ Helicopter: ${helicopter ? 'created' : 'failed'}`);
console.log(`✓ Tank: ${tank ? 'created' : 'failed'}`);

console.log('\nSimple test completed!');