# Project context for coding agents

## User and strategic objective

- The repository belongs to Grzegorz Aleksander Klementowski.
- Address him in Polish unless he explicitly switches to English.
- Treat this repository as part of his technical portfolio for securing a
  software-development position.
- Optimise work for learning, technical credibility and employability, with
  particular emphasis on Rust, Linux/Unix, Git, testing, documentation and
  GitHub Actions.
- Give direct, practical recommendations and explain the reasoning behind
  technical choices.

## Repository purpose

`rust_excercises` is a Cargo workspace containing Rust algorithm exercises,
small CLI utilities, experiments and selected portfolio projects. It is not one
single application.

The repository is used to practise and demonstrate:

- idiomatic stable Rust;
- algorithms and safe text/number conversion;
- `Result`, meaningful errors and input validation;
- unit and integration testing;
- maintainable CLI and domain-oriented design;
- Git history built through small, focused commits;
- automated formatting, Clippy, compilation and tests in GitHub Actions.

The root `README.md` documents the workspace and highlights its strongest
portfolio projects. There are many Cargo manifests, so inspect the relevant
crate and its local history before making changes.

## Current work snapshot

Last verified: 2026-07-29. Always confirm the current branch, status and source
files before relying on this snapshot.

The active branch was `other/rust-algorithms/z5/b`, eight commits ahead of
`main`, with a clean working tree.

Work in `other/rust-algorithms/z5/b`:

1. `z1-char-into-u8` is implemented and tested. It converts an ASCII decimal
   digit character into `u8` and reports invalid input through
   `Result<u8, String>`.
2. `z2-add-in-written-res` is the immediate work in progress. Its test suite is
   already present, but `add_by_hand` still contains `todo!()`. The intended
   solution performs written decimal addition on arbitrarily long strings,
   reuses the digit-conversion function from task 1 and propagates errors with
   `?`.
3. `z3-digit-into-roman-numeral` has only been scaffolded and still contains
   Cargo's default example implementation. It should eventually convert
   `I, V, X, L, C, D, M` into numeric values.
4. The fourth exercise, validation and conversion of complete Roman numerals,
   is described in `other/rust-algorithms/z5/b/exercises.md` but did not yet
   have its own crate.

The nearest sensible next step is to implement `add_by_hand` test-first, then
run formatting, strict Clippy and the relevant tests.

## Engineering conventions

- Prefer idiomatic, explicit and readable stable Rust.
- Use strong types and meaningful domain structures where the problem warrants
  them, without overengineering small exercises.
- Return `Result` with useful errors instead of panicking in production paths.
- Add tests for normal behaviour, invalid input, boundaries and regressions.
- Run `cargo fmt`, `cargo clippy` and `cargo test` in proportion to the changed
  scope.
- The workspace CI applies strict Clippy profiles and treats warnings as
  failures.
- If the workspace `target` directory is not writable in an agent environment,
  use a crate-specific directory under `/tmp` through `CARGO_TARGET_DIR`.
- Temporary debugging output should not be mistaken for intended final design;
  ensure it is removed before a portfolio commit.

## Git conventions

- Inspect `git status` and the relevant diff before and after changes.
- Never run `git add .`.
- Stage only deliberate, coherent files.
- Do not amend, rebase, force-push or discard changes without explicit
  permission.
- Prefer small, focused commits with an imperative technical subject.
- For portfolio-relevant changes, add a short commit body explaining the
  behaviour, validation, error handling and tests when that context is useful.
- Never commit secrets, build artefacts or editor-generated files.

## Portfolio priorities

Algorithm exercises should show clear reasoning and disciplined testing, but
should not be overengineered. Larger projects should additionally demonstrate
architecture, documentation, error handling, examples and CI.

The repository's stronger portfolio material includes:

- `other/light-state-domain-driven`;
- `pasja_informatyki/odc-5/evaluation_mean`;
- `pasja_informatyki/odc-4/lotto_game`;
- `small_test_tools/recordbook_vcard_cli`.

When reviewing or extending the repository, distinguish between practice
exercises and projects intended as primary portfolio showcases.
