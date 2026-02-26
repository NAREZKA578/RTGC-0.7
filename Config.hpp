#pragma once

namespace RTGC {
    const int WINDOW_WIDTH = 1280;
    const int WINDOW_HEIGHT = 720;
    
    inline float Clamp(float value, float min, float max) {
        if (value < min) return min;
        if (value > max) return max;
        return value;
    }
}