<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# Pattern Matching

Chapter 8 introduced `enum` with a `match` already quietly doing the work
of reading a variant back out. This chapter makes `match` — and the
smaller tools built on the same pattern-matching machinery — the main
subject.

We'll use one running example, a sensor reading that can be one of four
shapes:

```rust,ignore
{{#include ../../examples/ch09-pattern-matching/src/lib.rs:sensor_reading}}
```

## Exhaustiveness: the headline feature

```rust,ignore
{{#include ../../examples/ch09-pattern-matching/src/lib.rs:match_exhaustive}}
```

Try deleting one arm from that `match` and the crate stops compiling —
`error[E0004]: non-exhaustive patterns`, naming the exact variant you
forgot. Compare that to C's `switch`:

```c
switch (reading->tag) {
    case TEMPERATURE: return format_temp(reading);
    case HUMIDITY:    return format_humidity(reading);
    // ERROR and IDLE cases forgotten — falls through to nothing,
    // silently, at runtime, with no warning from the compiler by default
}
```

A `switch` missing a `case` (or missing a `default`) just does nothing at
runtime for the cases it doesn't cover — a real, silent bug class, not a
compile error. `match`'s exhaustiveness check turns "did I forget a case"
from a runtime question into a compile-time one:

```rust,compile_fail
enum Light { Red, Yellow, Green }

fn describe(l: Light) -> &'static str {
    match l {
        Light::Red => "stop",
        Light::Green => "go",
    } // error[E0004]: non-exhaustive patterns: `Light::Yellow` not covered
}
```

## Match guards: a condition on top of the shape

```rust,ignore
{{#include ../../examples/ch09-pattern-matching/src/lib.rs:match_guard}}
```

A guard (the `if <condition>` after the pattern) adds a boolean check on
top of the pattern already matched. C's `switch` can't express this at
all — `case` labels must be compile-time constants — which is exactly why
C code reaches for an `if`/`else if` chain the moment a decision depends
on a payload's *value*, not just which variant it is. Guards are checked
top to bottom: put the stricter condition (`> 90.0`) before the looser one
(`> 75.0`), or the stricter arm is unreachable.

## `if let` and `while let`: `match` for one pattern

```rust,ignore
{{#include ../../examples/ch09-pattern-matching/src/lib.rs:if_let}}
```

`if let PATTERN = value { ... } else { ... }` is `match` with exactly one
pattern you care about and everything else falling to `else` — no `_ =>
{}` arm to write. `while let` is the same idea, repeated:

```rust,ignore
{{#include ../../examples/ch09-pattern-matching/src/lib.rs:while_let}}
```

`while let Some(reading) = pending.pop()` runs the loop body for as long
as `pop()` keeps returning `Some`, and stops the moment it returns `None`
— a common, idiomatic way to drain a collection without a separate
length check or index variable.

## Destructuring

Notice `SensorReading::Error { code, message }` in every example above:
one pattern pulls both fields out at once, bound as local variables named
after the fields. The C equivalent is a struct/union access per field —
`reading->payload.error.code`, `reading->payload.error.message` — spread
across the function body wherever each one is used. Destructuring also
works on tuples (`let (a, b) = pair;`) and on references — matching a
`&SensorReading` binds fields by reference automatically (Rust calls this
"default binding modes"), so you rarely need the explicit `ref` keyword
that older Rust code sometimes shows.

## Exercise

**Exercise 9.1**, in
[`exercises/ch09-pattern-matching/ex01-classify-readings/`](https://github.com/simonkeimer/rustut/tree/main/exercises/ch09-pattern-matching/ex01-classify-readings),
has you write a `match` with guards over a small alerting enum. See its
`README.md`.

Next: [Chapter 10 — Option, Result & the `?` Operator](10-option-result.md).
