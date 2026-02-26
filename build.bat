@echo off
set INCLUDES=-Iinclude -Iinclude/glm -Iinclude/entt/include -Iinclude/enet/include -Iinclude/tinyobjloader -Iinclude/miniaudio -Iinclude/stb
set FLAGS=-std=c++20 -O2 %INCLUDES% -DGLFW_INCLUDE_NONE -fopenmp
echo [CUDA] Компиляция ядер...
nvcc -c src/cuda/WindCuda.cu -o WindCuda.obj -arch=sm_50
nvcc -c src/cuda/SuspensionCuda.cu -o SuspensionCuda.obj -arch=sm_50
nvcc -c src/cuda/TractionCuda.cu -o TractionCuda.obj -arch=sm_50
nvcc -c src/cuda/TerrainCuda.cu -o TerrainCuda.obj -arch=sm_50
echo [C++] Компиляция кода...
g++ %FLAGS% -c src/*.cpp src/*/*.cpp src/*/*/*.cpp
echo [LINK] Создание RTGC.exe...
g++ *.o WindCuda.obj SuspensionCuda.obj TractionCuda.obj TerrainCuda.obj -o RTGC.exe ^
-Llib ^
-lglfw3dll ^
-lPhysXFoundation_static_64 ^
-lPhysXCommon_static_64 ^
-lPhysXCooking_static_64 ^
-lPhysXExtensions_static_64 ^
-lPhysXVehicle_static_64 ^
-lenet ^
-lOpenGL32 ^
-lgdi32 ^
-lwinmm ^
-lcuda ^
-lcudart ^
-lstdc++fs ^
-fopenmp
del *.o
echo Готово! Запуск...
RTGC.exe