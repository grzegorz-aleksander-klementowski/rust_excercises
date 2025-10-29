# rust_excercises
[![CI](https://github.com/grzegorz-aleksander-klementowski/Rust-Book-examples-exercises/actions/workflows/ci.yml/badge.svg)](https://github.com/grzegorz-aleksander-klementowski/Rust-Book-examples-exercises/actions/workflows/ci.yml)
[![CI](https://github.com/grzegorz-aleksander-klementowski/Rust-Book-examples-exercises/actions/workflows/ci.yml/badge.svg)](https://github.com/grzegorz-aleksander-klementowski/Rust-Book-examples-exercises/actions/workflows/ci.yml)

Cargo workspace that gathers bite-sized experiments, study notes, and quick utility prototypes. The active crates are listed in `Cargo.toml` and mirrored in the sections below.

## Getting Started
- `cargo run -p <crate_name>` executes a binary crate from the workspace root.
- `cargo test -p <crate_name>` runs the available unit tests.
- You can also `cd` into any of the directories mentioned below and use standard Cargo commands.

## Workspace Overview
| Crate | Path | Description |
| --- | --- | --- |
| light-state-domain-driven | `other/light-state-domain-driven` | Domain-driven light switch with guarded state transitions and thorough tests. |
| evaluation_mean | `pasja_informatyki/evaluation_mean` | Interactive gradebook that validates input and prints a weighted average. |
| lotto_game | `pasja_informatyki/lotto_game` | Animated Lotto drawing with audible feedback and guarded number generation. |
| hex_color_display | `small_test_tools/hex_color_display` | Prints ANSI swatches for a list of hex colors. |
| oblicz-koszty-roweru | `small_test_tools/oblicz-koszty-roweru` | Compares electric-bike component costs between purchases. |
| program_do_mówienia_prawdy | `small_test_tools/program_do_mówienia_prawdy` | Simple loop that responds with canned truths to predefined prompts. |
| recordbook_vcard_cli | `small_test_tools/recordbook_vcard_cli` | CLI wizard that collects contact details and writes a VCF record. |
| restart_swap | `small_test_tools/restart_swap` | Wraps `sudo swapoff -a` / `sudo swapon -a` for quick swap refresh. |
| test_curl_vs_reqwest | `small_test_tools/test_curl_vs_reqwest` | Placeholder for HTTP benchmarking experiments. |

## Projects

### Light-State Domain Driven Practice
`other/light-state-domain-driven`
- Domain-driven rewrite of a light switch state machine.
- The `Validate` trait guards transitions so `Light::turn_on` / `turn_off` can short circuit on invalid moves.
- Rich unit tests illustrate the allowed sequences, and the tiny CLI (`main.rs`) wires the library together.

### Pasja Informatyki Exercises
`pasja_informatyki/`
- `evaluation_mean`: interactive gradebook that validates input, collects scores, and reports a weighted average.
- `fibbonaci_counter`: work-in-progress Fibonacci playground with scaffolding for future logic.
- `lotto_game`: animated Lotto drawing simulation with audible feedback per draw and error-handled number generation.

### Small Test Tools
`small_test_tools/`
- `hex_color_display`: prints ANSI-colored swatches for a list of hex codes.
- `oblicz-koszty-roweru`: CLI helper that compares electric-bike component costs between two purchases.
- `program_do_mówienia_prawdy`: whimsical Q&A loop that responds with canned “truths” until the user exits.
- `recordbook_vcard_cli`: wizard that prompts for contact details, formats a VCF entry, and saves it to disk.
- `restart_swap`: thin wrapper around `sudo swapoff -a` / `sudo swapon -a`; run only when you actually want to refresh swap.
- `test_curl_vs_reqwest` and `texty-rusty-sound-`: placeholder crates reserved for future experiments.

## License
Distributed under the terms described in [LICENSE](LICENSE).

## Portfolio Highlight – Light-State Domain Driven Practice
I refactored an everyday light-switch example into a compact domain-driven module that showcases:
- **State-machine design**: `Light` models domain intent with explicit `State` variants and reusable transition helpers instead of ad-hoc booleans.
- **Guarded transitions**: the `Validate` trait (with a Generic Associated Type) checks each state change so `turn_on` / `turn_off` return `Result` and prevent invalid commands before any mutation occurs.
- **Ergonomic API surface**: method chaining, `Display` formatting, and clean defaults make it straightforward to integrate in CLI or service layers.
- **Testing discipline**: scenario-style tests cover happy paths, edge cases, and error handling to document behavior and lock in regressions.
- **Polish and extensibility**: rich comments outline future enhancements such as event emission and extra states, demonstrating forward planning.

This crate reflects how I approach production code: understand the domain first, encode the rules in types, and back the solution with guardrails that keep the API honest. It is small by design, but the techniques scale to more complex workflows.

## Portfolio Highlight – Pasja Informatyki: Evaluation Mean
I built an interactive gradebook that showcases disciplined CLI architecture:
- **Structured domain model**: `Gradesbook` stores `(grade, weight)` tuples; helper methods (`add`, `show_grades`, `weighted_average`) keep logic cohesive and round results to two decimals.
- **Trait-driven input & validation**: generic `Input<T>` and `Validator<T>` traits cover both floats and integers, allowing reusable parsing, range checking, and retry logic with clean separation of concerns.
- **Resilient UX flow**: prompts flush instantly, the app retries malformed input up to three times, and friendly Polish copy (via the `Messages` enum) guides the user from welcome screen to exit prompt.
- **Human-friendly output**: all collected grades are formatted in a concise table before the final weighted average is printed, so the user can verify data prior to the result.
- **Thorough test suite**: unit tests assert message wording, input validation edge cases, grade storage integrity, and the weighted-average calculation—guarding both behavior and regression risk.

This project underlines my ability to combine trait abstractions, data validation, and thoughtful UX writing into a cohesive CLI experience that feels polished while remaining easy to extend.

## Portfolio Highlight – Pasja Informatyki: Lotto Game
A Lotto simulator that blends platform-aware I/O with defensive randomness:
- **Cross-platform feedback**: the `beep` helper uses `#[cfg(target_os = "...")]` to emit system beeps on Windows or Linux, while `stop_for_seconds` times animation-friendly pauses.
- **Safe number generation**: `generate_lotto_num_for_set` retries until it finds a unique value, with guard rails to prevent infinite loops and useful error strings when the pool is exhausted.
- **Result orchestration**: `generate_lotto_set` builds a `[Option<u8>; 6]` result and validates range bounds, while `generate_lotto_set_output` converts it into a clean `[u8; 6]` array and reports partial failures.
- **Narrative messaging**: the `Message` enum handles welcome copy, countdown pacing, and per-number reveals (beep + pause) to mimic a live draw.
- **Unit-tested logic**: tests verify duplicate detection, unique generation, full-set creation, and the formatting layer—giving confidence despite the stochastic core.

The crate demonstrates mid-level Rust experience: conditional compilation, randomness with constraints, user feedback loops, and a test harness that keeps non-deterministic features under control.

## Portfolio Highlight – Small Test Tools: Hex Color Display
A micro-utility that turns ANSI escape sequences into a design-friendly preview:
- **Minimalist parsing core**: `hex_to_rgb` validates `#RRGGBB` input and uses `u8::from_str_radix` to translate hex pairs into RGB tuples without external dependencies.
- **Terminal-friendly output**: `print_color` dynamically assembles `\x1b[48;2;...m` escape codes so each line shows a colored swatch alongside the original hex string.
- **Ergonomic entry point**: the binary feeds a curated palette into the library function, demonstrating reusable library/bin separation even for tiny tools.
- **Graceful error handling**: invalid inputs are reported via `stderr`, making it easy to integrate into pipelines or shell scripts.

It’s a compact example of clean library-first design: a focused conversion core, ANSI presentation know-how, and a CLI wrapper that keeps the ergonomics friendly.

## Portfolio Highlight – Recordbook VCard CLI
QuickCVF is a guided wizard that generates ready-to-import VCF contact cards:
- **Modular architecture**: the crate is split into `config`, `input`, `output`, and `process` modules; each one owns a single responsibility and is wired together through the `ZapiskiOsobowe` struct.
- **Internationalized UX**: enums like `WiadomościDoUżytkownika` and `NagłówkiVCF` keep prompts and headers centralized, making future localization or copy tweaks straightforward.
- **Interactive flow control**: input routines solicit data with stdout flushing, retry on failures, and exit gracefully after three invalid attempts to preserve data quality.
- **File persistence**: `Wyjście::wyjście_do_pliku_cvf` writes formatted contact cards to disk and reports success or detailed error messages via displayable enums.
- **Testing strategy**: unit tests assert formatting, filename derivation, and enum display output, while integration tests ensure configuration constants and output modules remain in sync.

It’s a mid-level CLI showcase that mixes user interaction, data formatting, filesystem IO, and both unit and integration tests—highlighting my ability to ship maintainable utilities with a polish-first mindset.

## Portfolio Highlight – Small Test Tools: Restart Swap
A pragmatic helper that wraps Linux swap refresh into a safe Rust API:
- **Command encapsulation**: the `Swap` enum models `On`/`Off` states, while the `switch` helper maps variants to `swapoff`/`swapon` binaries.
- **Process management**: `execute` launches commands through `std::process::Command`, piping `stdout` to `/dev/null` and propagating `ExitStatus` or `io::Error` for upstream handling.
- **Sudo-aware workflow**: the implementation explicitly prefixes calls with `sudo`, making the intent clear and allowing callers to integrate privilege checks before invoking the utility.
- **Minimal surface area**: keeping I/O isolated to one function simplifies auditing, testing via mocks, or future enhancements (e.g., logging, dry-run support).

Even tiny admin tools benefit from Rust’s type safety and error reporting; this crate demonstrates how I wrap privileged shell operations behind a clean enum-based interface.

## Portfolio Highlight – Small Test Tools: Test Curl vs Reqwest
Scaffolding for a HTTP benchmarking lab where I plan to compare the ergonomics and performance of cURL, libcurl (via `curl` crate), and `reqwest`:
- **Workspace integration**: the crate already lives in the multi-project workspace, giving me an isolated binary target ready for iterative experiments.
- **Future benchmark focus**: the goal is to wire up `criterion`-style harnesses that measure request latency, payload handling, and concurrency trade-offs between native cURL calls and idiomatic Rust clients.
- **Design considerations**: planned modules will separate workload generation, timing probes, and result reporting so new transport libraries can be swapped in with minimal friction.
- **Developer tooling**: starter binary proves out the plumbing; upcoming iterations will add structured logging, configurable endpoints, and CSV/Markdown reports for sharing insights.

Although the implementation is a placeholder today, the project outlines how I bootstrap performance investigations—starting small, integrating with existing tooling, and planning for reproducible, data-driven comparisons.
