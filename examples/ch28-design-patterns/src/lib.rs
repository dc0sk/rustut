// SPDX-License-Identifier: MIT OR Apache-2.0
//! Guided example for Chapter 28 — design patterns in Rust vs. C idioms.

// ANCHOR: strategy
/// Strategy pattern: swappable behavior via a trait, instead of C's usual
/// "function pointer field in a struct." Generic + trait bound gives
/// **static** dispatch (monomorphized, zero indirection, Ch. 12); `Box<dyn
/// Compressor>` gives **dynamic** dispatch (one shared implementation, one
/// indirect call, Ch. 13) when the concrete type isn't known until
/// runtime. Either way: no raw function pointer, no null check, no `void*`
/// context argument threaded through by hand.
pub trait Compressor {
    fn compress(&self, data: &[u8]) -> Vec<u8>;
}

pub struct NoOp;
impl Compressor for NoOp {
    fn compress(&self, data: &[u8]) -> Vec<u8> {
        data.to_vec()
    }
}

/// A toy run-length encoder — good enough to prove the strategy actually
/// changes behavior, not meant to be a real compressor.
pub struct RunLength;
impl Compressor for RunLength {
    fn compress(&self, data: &[u8]) -> Vec<u8> {
        let mut out = Vec::new();
        let mut iter = data.iter().peekable();
        while let Some(&byte) = iter.next() {
            let mut count: u8 = 1;
            while iter.peek() == Some(&&byte) && count < u8::MAX {
                iter.next();
                count += 1;
            }
            out.push(count);
            out.push(byte);
        }
        out
    }
}

/// Static dispatch: `C` is resolved and monomorphized at compile time.
pub fn compress_static<C: Compressor>(strategy: &C, data: &[u8]) -> Vec<u8> {
    strategy.compress(data)
}

/// Dynamic dispatch: the concrete strategy is chosen at runtime.
pub fn compress_dynamic(strategy: &dyn Compressor, data: &[u8]) -> Vec<u8> {
    strategy.compress(data)
}
// ANCHOR_END: strategy

// ANCHOR: observer
/// Observer pattern: rather than porting a C/OOP-style callback list
/// (`Vec<Box<dyn Fn(&Event)>>`, each observer registered and invoked
/// in-line, sharing whatever state it needs via captured references or a
/// `void*`), a channel (Ch. 20) is often the more idiomatic Rust
/// replacement — "observers" are just receivers reading events off a
/// queue, decoupled from the publisher's call stack entirely, and the
/// publisher never holds a `Vec` of closures with unclear lifetimes.
use std::sync::mpsc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    Connected,
    Disconnected,
}

pub fn publish_events(events: Vec<Event>) -> mpsc::Receiver<Event> {
    let (tx, rx) = mpsc::channel();
    for event in events {
        tx.send(event).expect("receiver is still alive");
    }
    rx
}
// ANCHOR_END: observer

// ANCHOR: visitor
/// Visitor pattern: in a language without exhaustive pattern matching,
/// "do something different per variant of a closed set of types" needs
/// hand-rolled double dispatch (an `accept(Visitor*)` method on every
/// type, calling back into `visit_foo`/`visit_bar`). An exhaustive `match`
/// over an enum (Ch. 9) usually *replaces* this outright — the compiler
/// already guarantees every variant is handled, which is the entire
/// problem Visitor exists to solve in the first place.
pub enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
}

pub fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
        Shape::Rectangle { width, height } => width * height,
    }
}
// ANCHOR_END: visitor

// ANCHOR: singleton
/// Singleton, when you genuinely need one: `std::sync::OnceLock` gives a
/// lazily-initialized, thread-safe global with no hand-rolled
/// double-checked locking (the classic C pattern: a `static` pointer, a
/// mutex, and a check-lock-check-again dance to initialize exactly once).
use std::sync::OnceLock;

static CONFIG: OnceLock<String> = OnceLock::new();

pub fn global_config() -> &'static str {
    CONFIG.get_or_init(|| "default-config".to_string())
}
// ANCHOR_END: singleton

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn static_and_dynamic_dispatch_agree() {
        let data = b"aaabbbbc";
        assert_eq!(
            compress_static(&RunLength, data),
            compress_dynamic(&RunLength, data)
        );
        assert_eq!(compress_static(&NoOp, data), data);
    }

    #[test]
    fn run_length_actually_compresses_runs() {
        assert_eq!(RunLength.compress(b"aaab"), vec![3, b'a', 1, b'b']);
    }

    #[test]
    fn observer_events_arrive_in_order() {
        let rx = publish_events(vec![Event::Connected, Event::Disconnected]);
        let received: Vec<_> = rx.iter().collect();
        assert_eq!(received, vec![Event::Connected, Event::Disconnected]);
    }

    #[test]
    fn visitor_replacement_computes_area() {
        assert_eq!(
            area(&Shape::Rectangle {
                width: 3.0,
                height: 4.0
            }),
            12.0
        );
    }

    #[test]
    fn singleton_initializes_once() {
        assert_eq!(global_config(), "default-config");
        assert_eq!(global_config(), "default-config");
    }
}
