#include <cuda_runtime.h>

__global__ void ComputeTraction(float* traction, int size, float dt) {
    int idx = blockIdx.x * blockDim.x + threadIdx.x;
    if (idx >= size) return;
    // Простая модель сцепления
    traction[idx] = traction[idx] * (1.0f + dt * 0.05f);
    if (traction[idx] > 1.0f) traction[idx] = 1.0f;
}

extern "C" void LaunchTractionCuda(float* h_traction, int size, float dt) {
    float *d_traction;
    size_t bytes = size * sizeof(float);
    cudaMalloc(&d_traction, bytes);
    dim3 block(256), grid((size + 255) / 256);
    ComputeTraction<<<grid, block>>>(d_traction, size, dt);
    cudaMemcpy(h_traction, d_traction, bytes, cudaMemcpyDeviceToHost);
    cudaFree(d_traction);
}