#pragma once
#include <string>
#include <vector>
#include "../math/Vector3.hpp"

#ifdef HUGE
#undef HUGE
#endif

namespace WorldConfig {

    enum class TerrainType {
        FLAT_PLAINS,
        HILLS,
        MOUNTAINS,
        TUNDRA,
        FOREST,
        MIXED
    };

    enum class ClimateType {
        ARCTIC,     // -40°C to -10°C
        COLD,        // -10°C to 5°C
        TEMPERATE,   // 5°C to 20°C
        HOT          // 20°C to 40°C
    };

    enum class Season {
        WINTER,
        SPRING,
        SUMMER,
        AUTUMN
    };

    enum class MapSize {
        SMALL,    // 256x256 (65k units)
        MEDIUM,   // 512x512 (262k units)
        LARGE,    // 1024x1024 (1M units)
        HUGE      // 2048x2048 (4M units)
    };

    struct WorldSettings {
        std::string worldName;
        MapSize mapSize;
        TerrainType terrainType;
        ClimateType climateType;
        Season season;
        float cityDensity;          // 0.0 to 1.0
        float waterLevel;           // 0.0 to 1.0
        float mountainHeight;       // 0.0 to 1.0
        bool generateRoads;
        bool generateVegetation;
        int seed;
        float scale; // 1.0f default; 1 unit = 1 meter - MVP scale

        WorldSettings() {
            worldName = "New World";
            mapSize = MapSize::MEDIUM;
            terrainType = TerrainType::MIXED;
            climateType = ClimateType::COLD;
            season = Season::WINTER;
            cityDensity = 0.3f;
            waterLevel = 0.3f;
            mountainHeight = 0.5f;
            generateRoads = true;
            generateVegetation = true;
            seed = 0;
            scale = 1.0f;
        }

        int GetMapSize() const {
            switch (mapSize) {
                case MapSize::SMALL: return 256;
                case MapSize::MEDIUM: return 512;
                case MapSize::LARGE: return 1024;
                case MapSize::HUGE: return 2048;
                default: return 512;
            }
        }

        std::string GetTerrainName() const {
            switch (terrainType) {
                case TerrainType::FLAT_PLAINS: return "Равнины";
                case TerrainType::HILLS: return "Холмы";
                case TerrainType::MOUNTAINS: return "Горы";
                case TerrainType::TUNDRA: return "Тундра";
                case TerrainType::FOREST: return "Тайга";
                case TerrainType::MIXED: return "Смешанный";
                default: return "Неизвестно";
            }
        }

        std::string GetClimateName() const {
            switch (climateType) {
                case ClimateType::ARCTIC: return "Арктический";
                case ClimateType::COLD: return "Холодный";
                case ClimateType::TEMPERATE: return "Умеренный";
                case ClimateType::HOT: return "Жаркий";
                default: return "Неизвестно";
            }
        }

        std::string GetSeasonName() const {
            switch (season) {
                case Season::WINTER: return "Зима";
                case Season::SPRING: return "Весна";
                case Season::SUMMER: return "Лето";
                case Season::AUTUMN: return "Осень";
                default: return "Неизвестно";
            }
        }

        float GetBaseTemperature() const {
            float baseTemp;
            switch (climateType) {
                case ClimateType::ARCTIC: baseTemp = -25.0f; break;
                case ClimateType::COLD: baseTemp = -5.0f; break;
                case ClimateType::TEMPERATE: baseTemp = 12.0f; break;
                case ClimateType::HOT: baseTemp = 30.0f; break;
                default: baseTemp = -5.0f;
            }

            switch (season) {
                case Season::WINTER: return baseTemp - 10.0f;
                case Season::SPRING: return baseTemp - 2.0f;
                case Season::SUMMER: return baseTemp + 8.0f;
                case Season::AUTUMN: return baseTemp - 2.0f;
                default: return baseTemp;
            }
        }
    };

    class WorldGenerator {
    private:
        WorldSettings m_settings;
        std::vector<Vector3> m_cities;
        std::vector<Vector3> m_roads;
        std::vector<Vector3> m_vegetation;

        float PerlinNoise(float x, float z, int seed);
        float OctaveNoise(float x, float z, int octaves, float persistence, int seed);
        float GetHeight(float x, float z);
        float GetMoisture(float x, float z);

    public:
        WorldGenerator(const WorldSettings& settings);
        ~WorldGenerator();

        void Generate();
        void GenerateTerrain();
        void GenerateCities();
        void GenerateRoads();
        void GenerateVegetation();

        const std::vector<Vector3>& GetCities() const { return m_cities; }
        const std::vector<Vector3>& GetRoads() const { return m_roads; }
        const std::vector<Vector3>& GetVegetation() const { return m_vegetation; }

        float GetHeightAt(float x, float z);
        bool IsWater(float x, float z);
        bool IsMountain(float x, float z);

        std::string GetBiomeName(float x, float z);
    };

}
