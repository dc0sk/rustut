// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: bounds_check
fn safe_get(data: &[i32], index: usize) -> Option<i32> {
    data.get(index).copied()
}
// ANCHOR_END: bounds_check

// ANCHOR: raii_guard
struct FileGuard {
    name: &'static str,
}

impl FileGuard {
    fn open(name: &'static str) -> Self {
        println!("opening {name}");
        FileGuard { name }
    }
}

impl Drop for FileGuard {
    fn drop(&mut self) {
        // Runs automatically when `_guard` goes out of scope, on *every*
        // return path below — no goto-fail, no forgotten close().
        println!("closing {} (automatic, via Drop)", self.name);
    }
}

fn process(fail_early: bool) -> &'static str {
    let _guard = FileGuard::open("scratch.tmp");
    if fail_early {
        return "failed early — file still got closed";
    }
    "completed normally"
}
// ANCHOR_END: raii_guard

fn main() {
    let data = [10, 20, 30];
    println!("safe_get(&data, 1)  = {:?}", safe_get(&data, 1));
    println!("safe_get(&data, 99) = {:?}", safe_get(&data, 99));

    println!("{}", process(false));
    println!("{}", process(true));
}
