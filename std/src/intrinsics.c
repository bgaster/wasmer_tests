//#include <stdlib.h>

#include "intrinsics.h"

//static float* buffer;

static float buffer[1024];

// void alloc_and_set(unsigned int size) {
//     //buffer = (float*)malloc(size * sizeof(float));
// }

// void free_alloc() {
//   //  free(buffer);
// }

// void set_buffer(float * b) {
//     //buffer = b;
// }

void write_f32(int offset, float v) {
    buffer[offset] = v;
}