#pragma once
#include <stdexcept>
#include <cmath>

class Mass {
    float value_kg = 0.0f;

public:
    Mass() = default;
    explicit Mass(float kg) : value_kg(kg) {
        if (kg < 0) {
            value_kg = 0.0f;
        }
    }

    float GetKilograms() const { return value_kg; }
    float GetPounds() const { return value_kg * 2.20462f; }

    Mass operator+(const Mass& other) const { return Mass(value_kg + other.value_kg); }
    Mass operator-(const Mass& other) const { return Mass(value_kg - other.value_kg); }
    Mass operator*(float factor) const { return Mass(value_kg * factor); }
    Mass operator/(float factor) const {
        if (std::abs(factor) < 0.0001f) {
            return Mass(0.0f);
        }
        return Mass(value_kg / factor);
    }

    Mass& operator+=(const Mass& other) { value_kg += other.value_kg; return *this; }
    Mass& operator-=(const Mass& other) { value_kg -= other.value_kg; return *this; }
    Mass& operator*=(float factor) { value_kg *= factor; return *this; }
    Mass& operator/=(float factor) {
        if (std::abs(factor) < 0.0001f) {
            return *this;
        }
        value_kg /= factor;
        return *this;
    }

    bool operator==(const Mass& other) const { return std::abs(value_kg - other.value_kg) < 0.0001f; }
    bool operator!=(const Mass& other) const { return !(*this == other); }
    bool operator<(const Mass& other) const { return value_kg < other.value_kg; }
    bool operator<=(const Mass& other) const { return (*this < other) || (*this == other); }
    bool operator>(const Mass& other) const { return !(*this <= other); }
    bool operator>=(const Mass& other) const { return !(*this < other); }
};

inline Mass operator*(float factor, const Mass& m) {
    return m * factor;
}