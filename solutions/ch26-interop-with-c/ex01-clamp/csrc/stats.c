// SPDX-License-Identifier: MIT OR Apache-2.0
// Copy of exercises/ch26-interop-with-c/ex01-clamp/csrc/stats.c.
int round_to_nearest(double value) {
    if (value >= 0.0) {
        return (int)(value + 0.5);
    }
    return (int)(value - 0.5);
}
