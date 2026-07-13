# Additions

Ideas for extending skewer, roughly ordered by effort within each section. None of these are commitments. If you want to contribute one, open an issue first so the approach can be agreed before code is written. New rules need a citation to the survey methodology literature and tests, as described in the README.

## New rules

Acquiescence-prone questions. Agree/disagree formats invite yes-saying regardless of content. Detect "do you agree" and statement-plus-agreement-scale patterns and suggest construct-specific wording instead.

Jargon and reading level. Flag technical terms, acronyms, and long sentences a general audience would stumble on. A simple syllable-count readability score would go a long way.

Vague quantifiers. Words like "regularly", "often", "recently", and "sometimes" mean different things to different people. Suggest concrete timeframes such as "in the past 7 days".

Assumptive questions. Questions that presuppose a behavior or state, such as "How often do you exercise?", exclude respondents for whom the premise is false. Detect the pattern and suggest a filter question first.

Recall burden. Questions asking respondents to remember far back ("in the past year") produce unreliable answers for frequent behaviors. Flag long recall windows on everyday activities.

Hypothetical and intention questions. "Would you buy this?" answers correlate poorly with actual behavior. Flag "would you" phrasing and suggest asking about past behavior instead.

Sensitive topics without softening. Income, health, and similar questions placed bluntly cause drop-off. Detect the topic and suggest ranges or a "prefer not to say" option.

Double negatives beyond word lists. The current check uses fixed lists. A smarter pass could catch negation patterns the lists miss.

## Question set checks

These look across the whole survey rather than at one question.

Scale consistency. Flag surveys that switch between 5-point and 7-point scales, or flip scale direction midway, since both confuse respondents.

Unbalanced response options. For structured formats that include options, flag scales with more positive than negative choices, or a missing midpoint.

Question order effects. Flag sensitive or framing-heavy questions placed before the questions they could bias.

Survey length. Warn past a question count where completion rates are known to fall.

## Input formats

Qualtrics QSF import, the survey tool most researchers actually use.

Google Forms and Typeform export support.

Markdown surveys, treating headings as sections and list items as questions.

A structured YAML/JSON schema that carries response options with each question, unlocking the scale checks above.

## Output and integration

A `--json` output flag so other tools can consume findings.

SARIF output, which would let GitHub annotate survey files in pull requests.

A GitHub Action that runs skewer on survey files in a repo.

Severity levels (error, warning, suggestion) with a flag to fail only on errors.

Inline suggestions: not just "this is leading" but a proposed neutral rewrite.

## Configuration

A config file (`skewer.toml`) to disable rules, extend word lists, or set severity per rule, so teams can tune it without forking.

Custom word lists for domain-specific loaded terms.

An ignore comment convention, so a deliberately flagged question can be marked as reviewed.

## Distribution

Publish to crates.io so `cargo install skewer` works without cloning.

Prebuilt binaries on GitHub Releases for macOS, Linux, and Windows.

A Homebrew formula.

## Bigger projects

WebAssembly build with a small web page where anyone can paste a survey and see findings without installing anything. This is the single highest-impact addition for adoption.

An optional AI pass (`--ai`) that sends flagged or borderline questions to an LLM for judgment calls the heuristics cannot make, with the API key supplied by the user.

Multi-language support. The current word lists are English-only. Even one additional language would prove out the structure for more.

A benchmark corpus: a public set of questions labeled good or bad by survey methodologists, so rule changes can be measured for precision and recall instead of argued about.
