# rust_excercises

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
| fibbonaci_counter | `pasja_informatyki/fibbonaci_counter` | Scaffold for future Fibonacci exploration. |
| lotto_game | `pasja_informatyki/lotto_game` | Animated Lotto drawing with audible feedback and guarded number generation. |
| hex_color_display | `small_test_tools/hex_color_display` | Prints ANSI swatches for a list of hex colors. |
| oblicz-koszty-roweru | `small_test_tools/oblicz-koszty-roweru` | Compares electric-bike component costs between purchases. |
| program_do_mówienia_prawdy | `small_test_tools/program_do_mówienia_prawdy` | Simple loop that responds with canned truths to predefined prompts. |
| recordbook_vcard_cli | `small_test_tools/recordbook_vcard_cli` | CLI wizard that collects contact details and writes a VCF record. |
| restart_swap | `small_test_tools/restart_swap` | Wraps `sudo swapoff -a` / `sudo swapon -a` for quick swap refresh. |
| test_curl_vs_reqwest | `small_test_tools/test_curl_vs_reqwest` | Placeholder for HTTP benchmarking experiments. |
| texty-rusty-sound- | `small_test_tools/texty-rusty-sound-` | Placeholder for audio or text processing prototypes. |

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
