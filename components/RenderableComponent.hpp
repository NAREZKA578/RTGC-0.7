#pragma once
#include "../graphics/RenderableVehicle.hpp" // Теперь включает правильный заголовок

struct RenderableComponent {
    RenderableVehicle* renderable = nullptr;
};