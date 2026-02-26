cmake_minimum_required(VERSION 3.10)
project(RTGC)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# Include only basic files that compile
set(SOURCES
    src/main.cpp
    src/EngineMinimal.cpp
    src/math/Vector3.cpp
    src/math/Mass.cpp
    src/math/PhysicsUtils.cpp
    src/physics/CharacterController.cpp
    src/graphics/Renderer.cpp
    src/graphics/Window.cpp
    src/graphics/GPUManager.cpp
    src/core/Logger.cpp
    src/world/Terrain.cpp
    src/world/SiberianCities.cpp
    src/world/WorldConfig.cpp
    src/game/InputManager.cpp
)

set(HEADERS
    src/EngineMinimal.hpp
    src/math/Vector3.hpp
    src/math/Mass.hpp
    src/math/PhysicsUtils.hpp
    src/physics/CharacterController.hpp
    src/graphics/Renderer.hpp
    src/graphics/Window.hpp
    src/graphics/GPUManager.hpp
    src/core/Logger.hpp
    src/world/Terrain.hpp
    src/world/SiberianCities.hpp
    src/world/WorldConfig.hpp
    src/game/InputManager.hpp
)

add_executable(RTGC ${SOURCES} ${HEADERS})

target_include_directories(RTGC PRIVATE src/)

# Link Windows libraries
if(WIN32)
    target_link_libraries(RTGC opengl32 glu32 gdi32)
    target_compile_definitions(RTGC PRIVATE _CRT_SECURE_NO_WARNINGS)
    target_compile_definitions(RTGC PRIVATE WIN32_LEAN_AND_MEAN NOMINMAX)
endif()

if(CMAKE_BUILD_TYPE STREQUAL "Release")
    target_compile_options(RTGC PRIVATE -O3)
else()
    target_compile_options(RTGC PRIVATE -g)
endif()