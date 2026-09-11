// SPDX-License-Identifier: MIT OR Apache-2.0
// Complete — nothing to fix in this file. The exercise is in src/lib.rs.
int round_to_nearest(double value) {
    if (value >= 0.0) {
        return (int)(value + 0.5);
    }
    return (int)(value - 0.5);
}
