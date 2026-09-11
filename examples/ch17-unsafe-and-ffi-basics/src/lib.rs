// SPDX-License-Identifier: MIT OR Apache-2.0
//! Guided example for Chapter 17 — what `unsafe` actually unlocks, and a
//! minimal real FFI call.

// ANCHOR: raw_pointer
/// Dereferencing a raw pointer is one of exactly five things `unsafe`
/// unlocks. The reference `x` came from is guaranteed valid for the
/// duration of this function, so casting it to `*const i32` and
/// dereferencing it back is sound — but the compiler cannot check that
/// for you the way it checks an ordinary `&i32`; that's the whole reason
/// this requires `unsafe` and a `// SAFETY:` comment.
pub fn read_through_raw_pointer(x: &i32) -> i32 {
    let ptr: *const i32 = x;
    // SAFETY: `ptr` was derived from `x`, a live `&i32` for the entire
    // call, so it is non-null, aligned, and points at a valid `i32`.
    unsafe { *ptr }
}
// ANCHOR_END: raw_pointer

// ANCHOR: unsafe_fn
/// Calling an `unsafe fn` is the second of the five. Declaring a function
/// `unsafe` is how you push a precondition onto the *caller*: this one
/// requires `index < data.len()`, and does not check it.
///
/// # Safety
/// `index` must be less than `data.len()`.
pub unsafe fn get_unchecked_i32(data: &[i32], index: usize) -> i32 {
    // SAFETY: the caller has upheld this function's own safety
    // precondition (`index < data.len()`), per its doc comment above.
    // `unsafe_op_in_unsafe_fn` is denied workspace-wide, so this inner
    // `unsafe` block is required even though we're already in an
    // `unsafe fn`.
    unsafe { *data.get_unchecked(index) }
}
// ANCHOR_END: unsafe_fn

// ANCHOR: static_mut
/// Accessing (reading or writing) a mutable `static` is the third of the
/// five. Unlike a C global, which any thread can read or write with zero
/// compiler involvement, Rust makes every touch of a mutable static an
/// explicit, auditable `unsafe` operation — there is no way to stumble
/// into a data race on one by accident.
static mut REQUEST_COUNTER: u32 = 0;

/// # Safety
/// Caller must ensure no other thread touches `REQUEST_COUNTER`
/// concurrently with this call — this toy example has no synchronization
/// at all; Chapter 21 covers the real, safe tool for shared mutable
/// counters (`AtomicU32`).
pub unsafe fn bump_request_counter() -> u32 {
    // SAFETY: upheld by this function's own precondition, documented
    // above — the caller guarantees exclusive access for this call.
    unsafe {
        REQUEST_COUNTER += 1;
        REQUEST_COUNTER
    }
}
// ANCHOR_END: static_mut

// ANCHOR: union_field
/// Reading a `union` field is the fourth of the five — and the direct
/// echo of Chapter 8's warning about C's manual tagged-union pattern: a
/// `union` in Rust has exactly the same "nothing stops you from reading
/// the wrong variant" property C's does. Rust just makes you write
/// `unsafe` at the one place that risk actually lives, instead of it
/// being ambient risk anywhere the union is touched.
pub union FloatOrBits {
    pub float: f32,
    pub bits: u32,
}

/// # Safety
/// `value.bits` must be the last field written to `value`.
pub unsafe fn bits_of(value: &FloatOrBits) -> u32 {
    // SAFETY: caller guarantees `bits` is the currently-active field, per
    // this function's precondition above.
    unsafe { value.bits }
}
// ANCHOR_END: union_field

// ANCHOR: ffi_abs
// The fifth thing `unsafe` unlocks doesn't appear in this file: unsafe
// trait implementations (Chapter 19 covers the one you'll actually
// write — `unsafe impl Send`/`Sync`).
//
// This block itself needs `unsafe` in edition 2024 — declaring an
// `extern` block is now an unsafe operation, separate from calling what's
// inside it. Marking an individual item `safe fn` is how you vouch, once,
// that a specific C function is safe to call with any input (libc's
// `abs` on a plain `i32` can't do anything unsound), so ordinary code can
// call it without its own `unsafe` block.
unsafe extern "C" {
    safe fn abs(input: i32) -> i32;
}

pub fn absolute_value(x: i32) -> i32 {
    abs(x)
}
// ANCHOR_END: ffi_abs

/// `unsafe` does not turn off the borrow checker — it only unlocks the
/// five specific operations demonstrated above. An aliasing violation
/// inside an `unsafe` block is still rejected exactly as it would be
/// outside one:
///
/// ```rust,compile_fail
/// fn main() {
///     let mut x = 5;
///     let r1 = &x;
///     unsafe {
///         let r2 = &mut x; // still an error — `unsafe` changed nothing here
///         println!("{r1} {r2}");
///     }
/// }
/// ```
///
/// The real compiler output (rustc 1.98, stable):
///
/// ```text
/// error[E0502]: cannot borrow `x` as mutable because it is also borrowed as immutable
///  --> src/lib.rs:5:18
///   |
/// 4 |     let r1 = &x;
///   |              -- immutable borrow occurs here
/// 5 |         let r2 = &mut x; // still an error — `unsafe` changed nothing here
///   |                  ^^^^^^ mutable borrow occurs here
/// 6 |         println!("{r1} {r2}");
///   |                    -- immutable borrow later used here
/// ```
pub fn unsafe_does_not_disable_the_borrow_checker() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_pointer_read_matches_the_original() {
        let x = 42;
        assert_eq!(read_through_raw_pointer(&x), 42);
    }

    #[test]
    fn unsafe_fn_reads_in_bounds_values() {
        let data = [10, 20, 30];
        // SAFETY: index 1 is in bounds for a 3-element slice.
        let value = unsafe { get_unchecked_i32(&data, 1) };
        assert_eq!(value, 20);
    }

    #[test]
    fn static_mut_counter_increments() {
        // SAFETY: single-threaded test, no concurrent access.
        let first = unsafe { bump_request_counter() };
        // SAFETY: same as above.
        let second = unsafe { bump_request_counter() };
        assert_eq!(second, first + 1);
    }

    #[test]
    fn union_reads_back_the_written_field() {
        let value = FloatOrBits { bits: 0x3f800000 }; // 1.0f32's bit pattern
        // SAFETY: `bits` is the field that was written above.
        assert_eq!(unsafe { bits_of(&value) }, 0x3f80_0000);
    }

    #[test]
    fn ffi_abs_matches_libc_semantics() {
        assert_eq!(absolute_value(-5), 5);
        assert_eq!(absolute_value(5), 5);
        assert_eq!(absolute_value(0), 0);
    }
}
