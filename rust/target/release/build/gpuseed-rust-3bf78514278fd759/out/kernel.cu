
#include <curand_kernel.h>

// Kernel to initialize cuRAND states
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

// Kernel to generate random bytes using cuRAND
extern "C" __global__ void generate_random_bytes(
    curandState* states,
    unsigned char* output,
    int num_elements
) {
    int idx = blockIdx.x * blockDim.x + threadIdx.x;
    if (idx < num_elements) {
        curandState local_state = states[idx];
        
        // Generate 16 bytes per element using cuRAND
        for (int i = 0; i < 16; i++) {
            // Generate random unsigned int and take lower 8 bits
            unsigned int rand_val = curand(&local_state);
            output[idx * 16 + i] = (unsigned char)(rand_val & 0xFF);
        }
        
        // Save state back
        states[idx] = local_state;
    }
}
