#include <curand_kernel.h>

extern "C" __global__ void init_curand_states(
    curandState* states,
    unsigned long long seed,
    int num_states
) {
    int idx = blockIdx.x * blockDim.x + threadIdx.x;
    if (idx < num_states) {
        curand_init(seed, idx, 0, &states[idx]);
    }
}
