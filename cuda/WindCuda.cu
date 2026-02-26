#include <cuda_runtime.h>

__global__ void ComputeWindField(float* windX, float* windY, float* windZ, int size, float time) {
    int idx = blockIdx.x * blockDim.x + threadIdx.x;
    if (idx >= size) return;
    float x = (idx % 256) * 0.1f;
    float z = (idx / 256) * 0.1f;
    windX[idx] = sinf(x + time) * cosf(z) * 2.0f;
    windY[idx] = 0.2f + sinf(time * 0.3f) * 0.1f;
    windZ[idx] = cosf(x) * sinf(z + time) * 2.0f;
}

extern "C" void LaunchWindCuda(float* h_wind, int size, float time) {
    float *d_windX, *d_windY, *d_windZ;
    size_t bytes = size * sizeof(float);
    cudaMalloc(&d_windX, bytes); cudaMalloc(&d_windY, bytes); cudaMalloc(&d_windZ, bytes);
    dim3 block(256), grid((size + 255) / 256);
    ComputeWindField<<<grid, block>>>(d_windX, d_windY, d_windZ, size, time);
    // ИСПРАВЛЕНО: Копируем все 3 массива
    cudaMemcpy(h_wind, d_windX, bytes, cudaMemcpyDeviceToHost);
    cudaMemcpy(h_wind + size, d_windY, bytes, cudaMemcpyDeviceToHost);
    cudaMemcpy(h_wind + 2 * size, d_windZ, bytes, cudaMemcpyDeviceToHost);
    cudaFree(d_windX); cudaFree(d_windY); cudaFree(d_windZ);
}