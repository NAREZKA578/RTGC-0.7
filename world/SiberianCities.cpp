#include "SiberianCities.hpp"
#include "../core/Logger.hpp"
#include <cmath>

SiberianCities::SiberianCities()
    : m_selectedCityIndex(-1)
    , m_loaded(false)
{
}

SiberianCities::~SiberianCities() {
    Logger::Log("SiberianCities destroyed");
}

void SiberianCities::InitializeDefaultCities() {
    m_cities.clear();
    
    m_cities.push_back({"Novosibirsk", Vector3(100.0f, 100.0f, 0.0f), 1600000.0f, -5.0f});
    m_cities.push_back({"Krasnoyarsk", Vector3(80.0f, 80.0f, 0.0f), 1100000.0f, -8.0f});
    m_cities.push_back({"Irkutsk", Vector3(60.0f, 120.0f, 0.0f), 620000.0f, -10.0f});
    m_cities.push_back({"Tomsk", Vector3(40.0f, 140.0f, 0.0f), 580000.0f, -6.0f});
    m_cities.push_back({"Omsk", Vector3(20.0f, 160.0f, 0.0f), 1200000.0f, -7.0f});
    m_cities.push_back({"Barnaul", Vector3(20.0f, 200.0f, 0.0f), 630000.0f, -5.0f});
    m_cities.push_back({"Kemerovo", Vector3(40.0f, 220.0f, 0.0f), 550000.0f, -6.0f});
    m_cities.push_back({"Novokuznetsk", Vector3(60.0f, 240.0f, 0.0f), 520000.0f, -7.0f});
    m_cities.push_back({"Norilsk", Vector3(80.0f, 280.0f, 0.0f), 180000.0f, -15.0f});
    m_cities.push_back({"Chita", Vector3(100.0f, 260.0f, 0.0f), 350000.0f, -12.0f});
    m_cities.push_back({"Abakan", Vector3(120.0f, 300.0f, 0.0f), 170000.0f, -4.0f});
    m_cities.push_back({"Dudinka", Vector3(140.0f, 320.0f, 0.0f), 25000.0f, -18.0f});
    m_cities.push_back({"Nazarovosibirsk", Vector3(120.0f, 340.0f, 0.0f), 120000.0f, -5.0f});
    m_cities.push_back({"Tayshet", Vector3(160.0f, 360.0f, 0.0f), 38000.0f, -9.0f});
    m_cities.push_back({"Ulan-Ude", Vector3(180.0f, 380.0f, 0.0f), 440000.0f, -11.0f});
    
    Logger::Log("Initialized ", m_cities.size(), " Siberian cities");
    m_loaded = true;
}

void SiberianCities::Load() {
    Logger::Log("Loading Siberian cities...");
    InitializeDefaultCities();
    Logger::Log("Siberian cities loaded successfully");
}

void SiberianCities::Update(float dt) {
}

const City& SiberianCities::GetCity(int index) const {
    static const City emptyCity = {"Unknown", Vector3(0, 0, 0), 0.0f, 0.0f};
    
    if (index < 0 || index >= static_cast<int>(m_cities.size())) {
        return emptyCity;
    }
    
    return m_cities[index];
}

const City* SiberianCities::GetSelectedCity() const {
    if (m_selectedCityIndex < 0 || m_selectedCityIndex >= static_cast<int>(m_cities.size())) {
        return nullptr;
    }
    return &m_cities[m_selectedCityIndex];
}

void SiberianCities::SelectCity(int index) {
    if (index >= 0 && index < static_cast<int>(m_cities.size())) {
        m_selectedCityIndex = index;
        Logger::Log("Selected city: ", m_cities[index].name);
    }
}

float SiberianCities::GetDistanceBetweenCities(int cityA, int cityB) const {
    if (cityA < 0 || cityA >= static_cast<int>(m_cities.size()) ||
        cityB < 0 || cityB >= static_cast<int>(m_cities.size())) {
        return 0.0f;
    }
    
    return m_cities[cityA].position.Distance(m_cities[cityB].position);
}

void SiberianCities::PrintCityList() const {
    Logger::Log("=== Siberian Cities List ===");
    for (size_t i = 0; i < m_cities.size(); ++i) {
        const City& city = m_cities[i];
        Logger::Log("[", static_cast<int>(i), "] ", city.name, 
                   " | Pop: ", static_cast<int>(city.population),
                   " | Temp: ", city.temperature, "°C");
    }
    Logger::Log("===========================");
}
