// SPDX-License-Identifier: MIT OR Apache-2.0
// ANCHOR: geometry_c
typedef struct {
    float x;
    float y;
} point_t;

float point_distance(point_t a, point_t b) {
    float dx = a.x - b.x;
    float dy = a.y - b.y;
    return __builtin_sqrtf(dx * dx + dy * dy);
}

int clamp_int(int value, int min, int max) {
    if (value < min) {
        return min;
    }
    if (value > max) {
        return max;
    }
    return value;
}
// ANCHOR_END: geometry_c
