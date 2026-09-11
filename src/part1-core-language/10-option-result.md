<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# Option, Result & the `?` Operator

## `Option<T>`: no null, structurally

C has exactly one way to say "there might not be a value here": a pointer
that might be `NULL`. Nothing in the type system distinguishes "this
pointer is always valid" from "check before you dereference this one" —
it's the same `T*` either way, and the compiler enforces nothing. That
gap is where an entire bug class lives: the null pointer dereference.

Rust splits "always a value" (`T`) from "maybe a value" (`Option<T>`) at
the type level:

```rust,ignore
{{#include ../../examples/ch10-option-result/src/lib.rs:option_vs_null}}
```

`find_user` returns `Option<&str>`, not `&str`. There is no `&str` to
misuse until you've handled both cases — `match`, `if let`, or a
combinator (below) — because `Option<&str>` and `&str` are different
types, and nothing implicitly converts one into the other. The compiler
rejects, at compile time, exactly the mistake a null-pointer dereference
represents at runtime.

> **C engineer's mental model.** `Option<T>` costs nothing you weren't
> already paying: for a reference-sized `T` it's the same size as the
> pointer, because Rust reuses the "reference can't be zero" fact to fold
> `None` into the all-zero bit pattern (the "null pointer optimization").
> You get the type-level guarantee for free, not as a boxed wrapper.

## Combinators instead of repeated null checks

```rust,ignore
{{#include ../../examples/ch10-option-result/src/lib.rs:combinators}}
```

`.map()` and `.unwrap_or_else()` chain the "is there a value?" check
once, instead of writing it out at every step the way you would in C:

```c
const char *name = find_user(id, users, n);
char *greeting;
if (name != NULL) {
    greeting = format_greeting(name);
} else {
    greeting = strdup("Hello, stranger!");
}
```

Both versions do the same thing. The Rust version can't compile if you
forget the `None` case — the C version compiles fine if you forget the
`NULL` check, and simply crashes (or doesn't, until the day it does)
when someone eventually calls it with an id that doesn't exist.

## `Result<T, E>`: no errno, structurally

C's other convention for "this might fail" is a return code — negative
means error, and the real value (if any) comes back through an out
pointer, with the specific meaning of "negative" defined by whatever
comment is above the function (or isn't). Nothing forces a caller to
check it; `int rc = do_thing(); do_thing_that_uses_the_result();` compiles
fine even if `rc` was never inspected.

`Result<T, E>` makes the two outcomes two different, incompatible values
of one enum: `Ok(T)` or `Err(E)`. You cannot get at the `T` without
acknowledging the `Result` — same enforcement mechanism as `Option`, one
type parameter richer.

## The `?` operator

```rust,ignore
{{#include ../../examples/ch10-option-result/src/lib.rs:question_mark}}
```

`value.trim().parse()?` says: if this is `Ok(v)`, keep going with `v`; if
it's `Err(e)`, return `Err(e)` from `parse_entry` right now. Compare to
the C idiom for propagating a failure up through several calls:

```c
int parse_entry(const char *key, const char *value, struct config_entry *out) {
    long parsed;
    int rc = parse_i64(value, &parsed);
    if (rc < 0) {
        return rc; // or `goto fail;` in a longer function
    }
    out->value = parsed;
    strncpy(out->key, key, sizeof(out->key) - 1);
    return 0;
}
```

The `if (rc < 0) return rc;` (or the equivalent `goto fail;`) after every
fallible call is exactly what `?` replaces — and unlike the C version,
skipping the check isn't an option: without `?` (or an explicit `match`),
the code simply doesn't type-check, because you'd be handing a `Result`
to something that expects the value inside it.

`?` also works with `Option`-returning expressions inside a function that
itself returns `Option` — same idea, no `Err` involved.

## A brief, forward-looking word on error types

`?` can convert between error types automatically, via `From`, as long as
an caller's declared error type implements `From<TheInnerErrorType>`. This
chapter's example keeps a single error type throughout so that machinery
stays invisible; Chapter 23 (`anyhow`/`thiserror`) covers unifying several
different error types across a real application, which is where this
mechanism starts to matter directly.

## Exercise

**Exercise 10.1**, in
[`exercises/ch10-option-result/ex01-lookup-and-parse/`](https://github.com/dc0sk/rustut/tree/main/exercises/ch10-option-result/ex01-lookup-and-parse),
has you write a small `Option`-returning search and a `Result`-returning
parser that combines both types via `?`. See its `README.md`.

Next: [Chapter 11 — Collections](11-collections.md).
