#include <cuda_runtime.h>

__global__ void ComputeSuspension(float* suspension, int size, float dt) {
    int idx = blockIdx.x * blockDim.x + threadIdx.x;
    if (idx >= size) return;
    // Простая модель сжатия подвески
    suspension[idx] = suspension[idx] * (1.0f - dt * 0.1f);
    if (suspension[idx] < 0.01f) suspension[idx] = 0.01f;
}

extern "C" void LaunchSuspensionCuda(float* h_suspension, int size, float dt) {
    float *d_suspension;
    size_t bytes = size * sizeof(float);
    cudaMalloc(&d_suspension, bytes);
    dim3 block(256), grid((size + 255) / 256);
    ComputeSuspension<<<grid, block>>>(d_suspension, size, dt);
    cudaMemcpy(h_suspension, d_suspension, bytes, cudaMemcpyDeviceToHost);
    cudaFree(d_suspension);
}