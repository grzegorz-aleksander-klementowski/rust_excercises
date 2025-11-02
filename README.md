```markdown
# rust_excercises

A collection of Rust exercises, examples and small utilities, organized into five main groups (sorted here by approximate size on disk):

1. **Advent of Code 2024**
2. **Small Test Tools**
3. **CQRS-ES Examples**
4. **Pasja Informatyki**
5. **No Boilerplate Light State**

Each group contains one or more standalone Cargo crates. You can build or run any of them with `cargo build` / `cargo run` from its directory.

---

## Table of Contents

- [Advent of Code 2024](#advent-of-code-2024)
- [Small Test Tools](#small-test-tools)
- [CQRS-ES Examples](#cqrs-es-examples)
- [Pasja Informatyki](#pasja-informatyki)
- [No Boilerplate Light State](#no-boilerplate-light-state)
- [Contributing](#contributing)
- [License](#license)

---

## Advent of Code 2024

Solution code and inputs for **AoC 2024**, days 1 through 7:

```

advent\_of\_code\_2024/
├── day\_1\_historian\_hysteria
├── day\_2\_Rednosed\_reports
├── day\_3\_mull\_it\_over
├── day\_4\_ceres\_search
├── day\_5\_print\_queue
├── day\_6\_guard\_gallivant
└── day\_7  ← (starter / empty)

````

Each day’s folder contains:

- `Cargo.toml` / `Cargo.lock`
- `src/main.rs` (plus backups / part1 variants)
- an `input/` directory with your puzzle data
- for days 2 & 3, a `zapiski/` folder with notes

**Running a solution**
```bash
cd advent_of_code_2024/day_<n>_<name>
cargo run --release
````

---

## Small Test Tools

A suite of small CLI tools under `small_test_tools/`:

| Tool                                | Description                                                    |
| ----------------------------------- | -------------------------------------------------------------- |
| `automation_mouse_and_keyboard`     | Automate simple mouse & keyboard actions                       |
| `hex_color_display`                 | Preview hex colors in your terminal                            |
| `nowy_zapis_w_spisie_osob`          | Manage entries in a personal registry (with integration tests) |
| `oblicz-koszty-roweru`              | Calculate bicycle costs based on input parameters              |
| `odnowienie_pamieci_zamiennej_swap` | Refresh swap‐space memory pages                                |
| `program_do_mówienia_prawdy`        | A simple “truth‐telling” CLI                                   |
| `remove_first_word_of_filename`     | Strip the first word from filenames                            |
| `test_curl_vs_reqwest`              | Compare HTTP performance: cURL vs. libcurl vs. reqwest         |
| `zamień_znak`                       | Replace a single character in a string                         |

**Build & run**

```bash
cd small_test_tools/<crate_name>
cargo run
```

**Run tests** (if available)

```bash
cargo test
```

---

## CQRS-ES Examples

Example of **CQRS** + **Event Sourcing** under `cqrs-es_examples/kubex_wholesaler`:

* `src/main.rs` – simple wholesaler domain
* `notes.txt` – design notes

Shows separation of write models (commands/events) and read models for a sample domain.

```bash
cd cqrs-es_examples/kubex_wholesaler
cargo run
```

---

## Pasja Informatyki

Hands-on exercises inspired by the “Pasja Informatyki” blog:

* **evaluation\_mean** – compute average evaluation scores
* **fibbonaci\_counter** – count Fibonacci numbers
* **lotto\_game** – simulate a lottery (with a small random-number library)

```bash
cd pasja_informatyki/<crate_name>
cargo run
```

---

## No Boilerplate Light State

A minimal “light state” management example:

```
no_boilerplate_light_state/
├── src/lib.rs
└── src/main.rs
```

Demonstrates a zero-boilerplate approach to state handling.

```bash
cd no_boilerplate_light_state
cargo run
```

---

## Contributing

Contributions, issues and feature requests are welcome! Please open an issue or submit a pull request.

---

## License

This project is licensed under the terms of the [LICENSE](LICENSE) file.

```
```

