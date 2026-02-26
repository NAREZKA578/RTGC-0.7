#include "WorldConfig.hpp"
#include "../core/Logger.hpp"
#include <cmath>
#include <cstdlib>
#include <ctime>
#include <algorithm>

namespace WorldConfig {

    // Simple pseudo-random number generator
    static unsigned int seed = 0;
    static unsigned int m_w = 88675123;
    static unsigned int m_z = 362436069;

    void setSeed(unsigned int s) {
        seed = s;
        m_w = s;
        m_z = s * 362436069;
    }

    unsigned int getRandom() {
        m_z = 36969 * (m_z & 65535) + (m_z >> 16);
        m_w = 18000 * (m_w & 65535) + (m_w >> 16);
        return (m_z << 16) + m_w;
    }

    float randomFloat() {
        return (float)getRandom() / 4294967296.0f;
    }

    float WorldGenerator::PerlinNoise(float x, float z, int seedVal) {
        unsigned int oldSeed = seed;
        setSeed(seedVal);

        // Simple interpolation-based noise
        int xi = (int)floor(x);
        int zi = (int)floor(z);
        float xf = x - xi;
        float zf = z - zi;

        // Smooth interpolation
        float u = xf * xf * (3.0f - 2.0f * xf);
        float v = zf * zf * (3.0f - 2.0f * zf);

        // Generate random gradients
        float grad00 = randomFloat() * 2.0f - 1.0f;
        float grad10 = randomFloat() * 2.0f - 1.0f;
        float grad01 = randomFloat() * 2.0f - 1.0f;
        float grad11 = randomFloat() * 2.0f - 1.0f;

        float n00 = grad00 * xf + grad00 * zf;
        float n10 = grad10 * (xf - 1) + grad10 * zf;
        float n01 = grad01 * xf + grad01 * (zf - 1);
        float n11 = grad11 * (xf - 1) + grad11 * (zf - 1);

        // Interpolate
        float nx0 = n00 * (1 - u) + n10 * u;
        float nx1 = n01 * (1 - u) + n11 * u;
        float nxy = nx0 * (1 - v) + nx1 * v;

        setSeed(oldSeed);
        return nxy * 0.5f + 0.5f;
    }

    float WorldGenerator::OctaveNoise(float x, float z, int octaves, float persistence, int seedVal) {
        float total = 0.0f;
        float frequency = 1.0f;
        float amplitude = 1.0f;
        float maxValue = 0.0f;

        for (int i = 0; i < octaves; i++) {
            total += PerlinNoise(x * frequency, z * frequency, seedVal + i) * amplitude;
            maxValue += amplitude;
            amplitude *= persistence;
            frequency *= 2.0f;
        }

        return total / maxValue;
    }

    float WorldGenerator::GetHeight(float x, float z) {
        float height = 0.0f;
        float scale = 0.01f;

        switch (m_settings.terrainType) {
            case TerrainType::FLAT_PLAINS:
                height = OctaveNoise(x * scale, z * scale, 3, 0.5f, m_settings.seed);
                height = height * 0.3f;
                break;

            case TerrainType::HILLS:
                height = OctaveNoise(x * scale, z * scale, 4, 0.6f, m_settings.seed);
                height = height * 0.6f;
                break;

            case TerrainType::MOUNTAINS:
                height = OctaveNoise(x * scale * 0.5f, z * scale * 0.5f, 5, 0.7f, m_settings.seed);
                height = powf(height, 2.0f) * m_settings.mountainHeight;
                break;

            case TerrainType::TUNDRA:
                height = OctaveNoise(x * scale, z * scale, 3, 0.4f, m_settings.seed);
                height = height * 0.2f;
                break;

            case TerrainType::FOREST:
                height = OctaveNoise(x * scale * 1.5f, z * scale * 1.5f, 4, 0.5f, m_settings.seed);
                height = height * 0.4f;
                break;

            case TerrainType::MIXED:
                height = OctaveNoise(x * scale, z * scale, 5, 0.6f, m_settings.seed);
                height = height * 0.7f;
                break;
        }

        return height;
    }

    float WorldGenerator::GetMoisture(float x, float z) {
        float scale = 0.008f;
        return OctaveNoise(x * scale + 1000, z * scale + 1000, 3, 0.5f, m_settings.seed + 100);
    }

    WorldGenerator::WorldGenerator(const WorldSettings& settings)
        : m_settings(settings)
    {
        if (m_settings.seed == 0) {
            m_settings.seed = time(nullptr);
        }
        setSeed(m_settings.seed);

        Logger::Log("=== World Generator Initialized ===");
        Logger::Log("World Name: ", m_settings.worldName);
        Logger::Log("Map Size: ", m_settings.GetMapSize(), "x", m_settings.GetMapSize());
        Logger::Log("Terrain: ", m_settings.GetTerrainName());
        Logger::Log("Climate: ", m_settings.GetClimateName());
        Logger::Log("Season: ", m_settings.GetSeasonName());
        Logger::Log("Temperature: ", m_settings.GetBaseTemperature(), "°C");
        Logger::Log("Seed: ", m_settings.seed);
    }

    WorldGenerator::~WorldGenerator() {
        Logger::Log("World Generator destroyed");
    }

    void WorldGenerator::Generate() {
        Logger::Log("Starting world generation...");

        GenerateTerrain();
        GenerateCities();
        GenerateRoads();
        GenerateVegetation();

        Logger::Log("World generation complete!");
        Logger::Log("Generated ", m_cities.size(), " cities");
        Logger::Log("Generated ", m_roads.size() / 2, " road segments");
        Logger::Log("Generated ", m_vegetation.size(), " vegetation objects");
    }

    void WorldGenerator::GenerateTerrain() {
        Logger::Log("Generating terrain...");

        int mapSize = m_settings.GetMapSize();
        int waterLevel = (int)(mapSize * m_settings.waterLevel);

        // Terrain is stored externally, this is just placeholder
        Logger::Log("Terrain generated with water level at y=", waterLevel);
    }

    void WorldGenerator::GenerateCities() {
        Logger::Log("Generating cities...");

        m_cities.clear();

        int mapSize = m_settings.GetMapSize();
        int numCities = (int)(mapSize * mapSize * 0.0001f * m_settings.cityDensity);
        numCities = std::max(10, std::min(numCities, 100));

        for (int i = 0; i < numCities; i++) {
            // Find suitable location for city (not in water, not on steep mountains)
            float x, z;
            float height;
            int attempts = 0;

            do {
                x = randomFloat() * mapSize;
                z = randomFloat() * mapSize;
                height = GetHeight(x, z);
                attempts++;

                if (attempts > 100) break;
            } while (IsWater(x, z) || IsMountain(x, z));

            if (!IsWater(x, z) && !IsMountain(x, z)) {
                m_cities.push_back(Vector3(x, height, z));
            }
        }

        Logger::Log("Generated ", m_cities.size(), " cities");
    }

    void WorldGenerator::GenerateRoads() {
        Logger::Log("Generating roads...");

        m_roads.clear();

        if (!m_settings.generateRoads || m_cities.size() < 2) {
            return;
        }

        // Generate roads connecting nearby cities
        for (size_t i = 0; i < m_cities.size(); i++) {
            for (size_t j = i + 1; j < m_cities.size(); j++) {
                Vector3& city1 = m_cities[i];
                Vector3& city2 = m_cities[j];

                float distance = sqrtf(powf(city2.x - city1.x, 2) + powf(city2.z - city1.z, 2));

                // Connect cities that are relatively close
                if (distance < 150.0f) {
                    // Create road segment
                    Vector3 roadStart(city1.x, city1.y + 0.1f, city1.z);
                    Vector3 roadEnd(city2.x, city2.y + 0.1f, city2.z);

                    m_roads.push_back(roadStart);
                    m_roads.push_back(roadEnd);
                }
            }
        }

        Logger::Log("Generated ", m_roads.size() / 2, " road segments");
    }

    void WorldGenerator::GenerateVegetation() {
        Logger::Log("Generating vegetation...");

        m_vegetation.clear();

        if (!m_settings.generateVegetation) {
            return;
        }

        int mapSize = m_settings.GetMapSize();
        int numVegetation = mapSize * mapSize / 100; // 1 vegetation per 100 units

        for (int i = 0; i < numVegetation; i++) {
            float x = randomFloat() * mapSize;
            float z = randomFloat() * mapSize;

            float height = GetHeight(x, z);

            // Only place vegetation on suitable terrain
            if (!IsWater(x, z) && !IsMountain(x, z)) {
                float moisture = GetMoisture(x, z);

                // Different vegetation based on terrain and moisture
                if (m_settings.terrainType == TerrainType::FOREST && moisture > 0.5f) {
                    // Dense forest
                    m_vegetation.push_back(Vector3(x, height, z));
                } else if (moisture > 0.4f) {
                    // Normal vegetation
                    if (randomFloat() > 0.7f) {
                        m_vegetation.push_back(Vector3(x, height, z));
                    }
                }
            }
        }

        Logger::Log("Generated ", m_vegetation.size(), " vegetation objects");
    }

    float WorldGenerator::GetHeightAt(float x, float z) {
        return GetHeight(x, z);
    }

    bool WorldGenerator::IsWater(float x, float z) {
        float height = GetHeight(x, z);
        int mapSize = m_settings.GetMapSize();
        float waterLevel = mapSize * m_settings.waterLevel;

        return height < waterLevel;
    }

    bool WorldGenerator::IsMountain(float x, float z) {
        float height = GetHeight(x, z);
        int mapSize = m_settings.GetMapSize();
        float mountainThreshold = mapSize * (0.7f + m_settings.mountainHeight * 0.3f);

        return height > mountainThreshold;
    }

    std::string WorldGenerator::GetBiomeName(float x, float z) {
        if (IsWater(x, z)) {
            return "Вода/Озеро";
        } else if (IsMountain(x, z)) {
            return "Горы";
        } else {
            float moisture = GetMoisture(x, z);
            if (moisture > 0.6f) {
                return "Тайга";
            } else if (moisture > 0.4f) {
                return "Лес";
            } else if (moisture > 0.2f) {
                return "Поля";
            } else {
                return "Степь";
            }
        }
    }
}
