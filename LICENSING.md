# Licensing

This repository mixes two kinds of content under two different licenses. The
split follows the [REUSE Software specification](https://reuse.software/) —
machine-checkable via `reuse lint` — and is authoritative in `REUSE.toml`.
This file is the plain-language summary.

| What | Where | License |
|---|---|---|
| Book prose (chapters, appendix) | `src/**` | [CC BY-SA 4.0](LICENSES/CC-BY-SA-4.0.txt) |
| Guided example code | `examples/**` | [MIT](LICENSES/MIT.txt) OR [Apache-2.0](LICENSES/Apache-2.0.txt) |
| Exercise stubs & tests | `exercises/**` | MIT OR Apache-2.0 |
| Reference solutions | `solutions/**` | MIT OR Apache-2.0 |
| Tooling, CI, config (`*.toml`, workflows, this file, `AGENTS.md`) | repo root, `.github/**` | MIT OR Apache-2.0 |

## Why two licenses

**CC BY-SA 4.0** is the right fit for prose: it requires attribution and
that derivative books/translations stay open under the same terms, which is
what you want for teaching material. It is the *wrong* fit for code — its
share-alike terms don't compose cleanly with software licensing and it is
not what the Rust ecosystem expects when code is copied into another
project.

**MIT OR Apache-2.0** is the dual license used by the Rust compiler itself
and by the overwhelming majority of published crates. Every example,
exercise, and solution in this repository is ordinary Rust source you should
feel free to copy into your own projects under that license, no different
from copying a snippet out of any other MIT/Apache-2.0 crate.

## Mechanics

Every source file under `src/**` carries an HTML-comment SPDX header; every
`.rs`/`.toml` file elsewhere carries a `//`/`#` SPDX header. `REUSE.toml`
maps any file that can't easily carry its own header. Run `reuse lint`
(`pipx install reuse`) to verify the whole tree is correctly licensed — CI
runs this on every push.
