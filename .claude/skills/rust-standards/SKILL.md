---
name: rust-standards
description: Code standards for all Rust written in this repo. Load before writing or reviewing any Rust code here.
---

# Rust standards for skewer

The overriding rule: **the least code that solves the problem.** Every line must earn its place. When two designs work, pick the one with fewer concepts, fewer types, fewer lines. No speculative abstraction: no trait, generic, or module created for a future that hasn't arrived.

## Naming

- Descriptive names, full words: `item_count`, not `x` or `cnt`. Names must sound natural and human when read aloud; if a name needs decoding (`negative_un_words`), rename it (`hidden_negatives`).
- Follow standard Rust casing: `PascalCase` for types, `snake_case` for functions and variables, `SCREAMING_SNAKE_CASE` for constants. Acronyms count as one word (`IoError`, not `IOError`).
- The clarity rule applies in every case style: `HIDDEN_NEGATIVES` is a fine constant name, `NEGATIVE_UN_WORDS` is not.
- Never prefix a binding with `_` to silence an unused warning. An unused binding is a bug or dead code: delete it or use it. `_` alone in destructuring/match arms is fine.
- Functions are verbs (`parse_question`), types are nouns (`Finding`), predicates read as questions (`is_loaded`).
- No abbreviations unless universal (`id`, `max`).

## Comments

- Comments explain **why**, never what. If a comment describes what the next line does, delete it and make the code clearer instead.
- No section-title or banner comments (`// ---- helpers ----`, `// Constructors`). Structure comes from modules and ordering, not headings.
- Public API gets `///` doc comments: one sentence, plus an example when usage isn't obvious. Keep them minimal, no restating the signature.
- If code needs a paragraph of explanation, the design is wrong. Fix the design.

## Functions

- One job per function. If the name needs "and", split it.
- Small enough to read without scrolling. Extract when logic nests past two levels.
- Borrow, don't own: take `&str` not `String`, `&[T]` not `Vec<T>`, unless ownership is genuinely needed.
- Return `Result` or `Option`; never `panic!`, `unwrap`, or `expect` in library code. `unwrap` is acceptable only in tests and in `main` after human-readable error reporting is impossible.

## Types

- Model the domain with enums and structs; no magic strings or numbers. A rule kind is an enum variant, not a `&str`.
- Derive, don't hand-write: `#[derive(Debug, Clone, PartialEq)]` and friends wherever they suffice.
- Newtypes only when they prevent a real confusion, not decoration.

## Errors

- One error enum per crate (or `thiserror` if dependencies allow), propagated with `?`.
- Error messages state what failed and with what input, lowercase, no trailing period.

## Idiom

- Iterator chains (`map`, `filter`, `collect`) over index loops.
- `match` over `if`-chains when branching on structure or more than two cases.
- `Option`/`Result` combinators (`map`, `and_then`, `unwrap_or`) over manual `match` when the intent stays readable, but never stack more than three combinators; use `match` past that.
- Implement standard traits (`Display`, `FromStr`, `Iterator`) instead of inventing `to_string_custom`-style methods.

## Architecture

- Flat until it hurts: start with `main.rs` + `lib.rs`; add a module only when a file exceeds ~300 lines or a boundary is real (rules vs. report vs. cli).
- Core logic lives in `lib.rs` (pure, no I/O); `main.rs` is a thin CLI shell. The library never prints, never exits.
- Dependencies are a cost. Justify each one; prefer std until it's clearly losing.

## Gates

Code isn't done until all three pass clean:

```
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

Every rule the linter implements has at least one test with a bad question it catches and a good question it passes.
