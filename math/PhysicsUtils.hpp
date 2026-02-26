#pragma once

// Простые утилиты для преобразования типов
// В будущем заменить на GLM/PhysX версии

struct Vec3 {
    float x, y, z;
    Vec3(float x = 0, float y = 0, float z = 0) : x(x), y(y), z(z) {}
};

struct Quat {
    float x, y, z, w;
    Quat(float x = 0, float y = 0, float z = 0, float w = 1) : x(x), y(y), z(z), w(w) {}
};

inline Vec3 ToGLM(const Vec3& v) {
    return v;
}

inline Vec3 ToPhysX(const Vec3& v) {
    return v;
}

inline Quat ToGLM(const Quat& q) {
    return q;
}

inline Quat ToPhysX(const Quat& q) {
    return q;
}