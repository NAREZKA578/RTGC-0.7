#pragma once
#include "../math/Vector3.hpp"
#include <vector>
#include <string>

struct City {
    std::string name;
    Vector3 position;
    float population;
    float temperature;
};

class SiberianCities {
private:
    std::vector<City> m_cities;
    int m_selectedCityIndex;
    bool m_loaded;
    
    void InitializeDefaultCities();
    
public:
    SiberianCities();
    ~SiberianCities();
    
    void Load();
    void Update(float dt);
    
    int GetCityCount() const { return static_cast<int>(m_cities.size()); }
    const City& GetCity(int index) const;
    const City* GetSelectedCity() const;
    void SelectCity(int index);
    const std::vector<City>& GetAllCities() const { return m_cities; }
    bool IsLoaded() const { return m_loaded; }
    
    float GetDistanceBetweenCities(int cityA, int cityB) const;
    void PrintCityList() const;
};
