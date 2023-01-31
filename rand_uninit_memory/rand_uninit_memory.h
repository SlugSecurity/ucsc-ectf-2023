#pragma once

#define RANDOM_BYTES_SIZE 1024

extern char random_bytes[RANDOM_BYTES_SIZE];

// Requires at least 22K of stack memory to function. The caller must guarantee this function is never inlined across FFI boundaries.
void init_random_bytes(void);
