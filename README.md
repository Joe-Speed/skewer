# skewer

skewer is a command line linter for survey questions. It reads a survey and reports questions that are likely to bias your results, such as leading questions, loaded wording, and questions that ask about two things at once. It is written in Rust, compiles to a single binary, and runs offline.

An AI assistant can do these checks too, but they are simple pattern checks that do not need one. Running them locally is free and instant, and the money, credits, and tokens you would have spent can go to work that actually benefits from AI, like interpreting results or designing the study.

```
$ skewer examples/survey.txt

Q1: How satisfied are you with our pricing and customer support?
  [double-barreled] may ask two things at once, respondents can only answer one; split it

Q2: Don't you think the new dashboard is easier to use?
  [leading] "don't you" suggests its own answer, ask neutrally instead

Q3: How amazing was your onboarding experience?
  [loaded-language] emotionally charged wording (amazing) pushes respondents toward an answer

7 questions checked, 5 flagged, 5 findings
```

## Background

Badly worded survey questions produce data that looks trustworthy but is not. The common mistakes are well documented in the survey methodology literature, but the checks for them usually live in textbooks or in the heads of experienced reviewers. skewer turns those checks into a tool you can run on every survey before it goes out, in the same way you would run a linter on code.

Compared with pasting a survey into an AI chat, a linter has a few practical advantages for this kind of check. It costs nothing per run, so tokens and credits are kept for more complex questions. It gives the same answer every time, it works offline on surveys you may not want to send to a third party, and it is fast enough to run in a script or CI pipeline. AI review still makes sense for judgment calls the heuristics cannot make, which is why an optional AI pass is on the roadmap.

## Why Rust

Rust is a good fit for linters, and most modern ones (ruff, biome, oxc) are written in it. It compiles to a single self-contained binary, so users install one file rather than a language runtime and a tree of dependencies. It is fast enough that running on every save or in CI is never a cost worth thinking about. Its type system suits this problem well: each rule is an enum variant, and the compiler checks that every rule is handled everywhere it matters. And the same core can later compile to WebAssembly, which makes the planned in-browser demo possible without a server.

## Installation

You need a Rust toolchain installed. Then, from a clone of this repository:

```sh
cargo install --path .
```

## Usage

skewer takes a file path, or reads from standard input if no path is given.

```sh
skewer survey.txt
skewer survey.csv
skewer survey.json
pbpaste | skewer
skewer export.dat --format csv
```

Three input formats are supported. The format is inferred from the file extension, and the --format flag overrides it. Standard input is treated as plain text unless a format is given.

Plain text files contain one question per line. Blank lines are skipped, and lines starting with # are treated as comments.

CSV files should have a header row. skewer looks for a column named question, text, or prompt, and falls back to the first column if none of those exist.

JSON files should contain an array, either of strings or of objects with a question, text, or prompt field.

The exit code is 0 when no problems are found, 1 when there are findings, and 2 on error. This makes it usable in CI pipelines and pre-submit hooks.

## Rules

skewer currently checks for five problems.

Double-barreled questions ask about two things at once, for example "How satisfied are you with our pricing and customer support?". A single answer cannot cover both parts, so the data is uninterpretable. skewer flags questions containing "and" or "or".

Leading questions suggest their own answer, for example "Don't you think the new dashboard is easier to use?". Phrasing like this inflates agreement. skewer flags known leading openers such as "don't you" and "would you agree".

Loaded language means emotionally charged words such as "amazing", "terrible", or "unfair". These shift responses before the respondent has even considered the answer options.

Absolute terms such as "always", "never", and "everyone" force respondents toward extreme answers, since almost no behavior is truly always or never.

Double negatives, including combinations like "not unhappy", make it ambiguous what a yes or no answer means.

All of these are deliberate heuristics based on word lists and patterns. They are fast, deterministic, and work offline, but they will sometimes flag a perfectly fine question, for example an "or" that introduces legitimate answer choices. Treat findings as prompts for review rather than verdicts.

## Roadmap

Planned work includes checks on response scales (unbalanced options, missing midpoints) for structured formats, Qualtrics QSF import, an optional AI pass for judgment calls the heuristics cannot make, and a WebAssembly build with an in-browser demo.

## Contributing

New rules are welcome. A rule needs the pattern it detects, a citation to the survey methodology literature explaining why it is a problem, and tests with at least one question it catches and one it correctly passes.

If you want to contribute but are not sure where to start, ADDITIONS.md lists ideas ranging from small rules to larger projects.

## License

MIT
