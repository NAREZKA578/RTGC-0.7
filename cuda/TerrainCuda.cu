#include <cuda_runtime.h>

__global__ void ComputeTerrain(float* terrain, int size, float dt) {
    int idx = blockIdx.x * blockDim.x + threadIdx.x;
    if (idx >= size) return;
    // Простая модель изменения высоты ландшафта
    terrain[idx] = terrain[idx] + sinf(idx * 0.01f + dt) * 0.01f;
}

extern "C" void LaunchTerrainCuda(float* h_terrain, int size, float dt) {
    float *d_terrain;
    size_t bytes = size * sizeof(float);
    cudaMalloc(&d_terrain, bytes);
    dim3 block(256), grid((size + 255) / 256);
    ComputeTerrain<<<grid, block>>>(d_terrain, size, dt);
    cudaMemcpy(h_terrain, d_terrain, bytes, cudaMemcpyDeviceToHost);
    cudaFree(d_terrain);
}