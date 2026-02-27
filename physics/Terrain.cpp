#include "Terrain.hpp"
#include <cmath>

Terrain::Terrain(int sz, float sc) : size(sz), scale(sc), rng(42) {
    heights.resize(size * size);
}

void Terrain::Initialize() {
    for (int z = 0; z < size; ++z) {
        for (int x = 0; x < size; ++x) {
            float height = PerlinNoise(x * scale, z * scale);
            heights[z * size + x] = height;
        }
    }
}

void Terrain::Update(float dt) {
    // For now, just regenerate occasionally to simulate dynamic terrain
    // In a real implementation, we might have erosion, tectonic activity, etc.
}

float Terrain::Noise(float x, float z) const {
    // Simple pseudo-random noise function
    int n = static_cast<int>(x) + static_cast<int>(z * 57);
    n = (n << 13) ^ n;
    int nn = (n * (n * n * 60493 + 19990303) + 1376312589) & 0x7fffffff;
    return 1.0f - (nn / 1073741824.0f);
}

float Terrain::InterpolatedNoise(float x, float z) const {
    int intX = static_cast<int>(floor(x));
    float fracX = x - intX;
    int intZ = static_cast<int>(floor(z));
    float fracZ = z - intZ;

    float v1 = Noise(intX, intZ);
    float v2 = Noise(intX + 1, intZ);
    float v3 = Noise(intX, intZ + 1);
    float v4 = Noise(intX + 1, intZ + 1);

    // Cosine interpolation
    float fX = (1 - cos(fracX * M_PI)) * 0.5f;
    float fZ = (1 - cos(fracZ * M_PI)) * 0.5f;

    float i1 = v1 * (1 - fX) + v2 * fX;
    float i2 = v3 * (1 - fX) + v4 * fX;

    return i1 * (1 - fZ) + i2 * fZ;
}

float Terrain::PerlinNoise(float x, float z) const {
    float total = 0.0f;
    float persistence = 0.5f;
    int octaves = 4;
    float frequency = 1.0f;
    float amplitude = 1.0f;

    for (int i = 0; i < octaves; ++i) {
        total += InterpolatedNoise(x * frequency, z * frequency) * amplitude;
        frequency *= 2.0f;
        amplitude *= persistence;
    }

    return total;
}

float Terrain::GetHeightAt(float x, float z) const {
    // Convert world coordinates to terrain space
    x /= scale;
    z /= scale;

    // Get integer and fractional parts
    int intX = static_cast<int>(floor(x));
    int intZ = static_cast<int>(floor(z));
    float fracX = x - intX;
    float fracZ = z - intZ;

    // Clamp to terrain bounds
    intX = std::max(0, std::min(size - 1, intX));
    intZ = std::max(0, std::min(size - 1, intZ));

    // Bilinear interpolation
    int x1 = std::min(intX + 1, size - 1);
    int z1 = std::min(intZ + 1, size - 1);

    float h00 = heights[intZ * size + intX];
    float h10 = heights[intZ * size + x1];
    float h01 = heights[z1 * size + intX];
    float h11 = heights[z1 * size + x1];

    float i0 = h00 * (1 - fracX) + h10 * fracX;
    float i1 = h01 * (1 - fracX) + h11 * fracX;

    return i0 * (1 - fracZ) + i1 * fracZ;
}

Vector3 Terrain::GetNormalAt(float x, float z) const {
    // Compute normal using height differences
    float hL = GetHeightAt(x - 0.1f, z);
    float hR = GetHeightAt(x + 0.1f, z);
    float hD = GetHeightAt(x, z - 0.1f);
    float hU = GetHeightAt(x, z + 0.1f);

    Vector3 normal(
        hL - hR,  // x component
        0.2f,     // y component (delta height for 0.2 units of x/z change)
        hD - hU   // z component
    );

    return normal.Normalize();
}