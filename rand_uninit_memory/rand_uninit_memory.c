#pragma GCC diagnostic ignored "-Wuninitialized"
#pragma GCC diagnostic ignored "-Wmaybe-uninitialized"

#include "rand_uninit_memory.h"

#define LARGE_STACK_OFFSET 20480 // 20 KiB

char random_bytes[RANDOM_BYTES_SIZE];

void __attribute__((noinline)) init_random_bytes(void) {
    volatile char uninit_bytes[RANDOM_BYTES_SIZE + LARGE_STACK_OFFSET];

    for (int i = 0; i < RANDOM_BYTES_SIZE; i++) {
        random_bytes[i] = uninit_bytes[i];
        uninit_bytes[i] = '\0';
    }
}
