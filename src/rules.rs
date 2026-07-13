use crate::{Finding, Question, RuleKind};

const LEADING_PHRASES: &[&str] = &[
    "don't you",
    "do you agree",
    "would you agree",
    "wouldn't you",
    "isn't it",
    "surely",
    "how much do you enjoy",
    "how much do you love",
];

const LOADED_WORDS: &[&str] = &[
    "amazing",
    "awesome",
    "incredible",
    "fantastic",
    "terrible",
    "horrible",
    "disgusting",
    "shocking",
    "obviously",
    "clearly",
    "unfair",
    "waste",
    "failure",
];

const ABSOLUTE_WORDS: &[&str] = &["always", "never", "everyone", "nobody", "every"];

const NEGATION_WORDS: &[&str] = &[
    "not",
    "don't",
    "doesn't",
    "didn't",
    "won't",
    "isn't",
    "aren't",
    "can't",
    "shouldn't",
];

const HIDDEN_NEGATIVES: &[&str] = &[
    "unhappy",
    "unsatisfied",
    "unlikely",
    "unable",
    "uncomfortable",
    "unimportant",
    "unaware",
    "unwilling",
    "unpleasant",
    "unnecessary",
];

pub fn check(question: &Question) -> Vec<Finding> {
    let text = question.text.to_lowercase();
    let words: Vec<&str> = text
        .split(|c: char| !c.is_alphanumeric() && c != '\'')
        .filter(|w| !w.is_empty())
        .collect();

    let mut findings = Vec::new();
    let mut report = |rule: RuleKind, message: String| {
        findings.push(Finding {
            question: question.number,
            rule,
            message,
        });
    };

    if words.contains(&"and") || words.contains(&"or") {
        report(
            RuleKind::DoubleBarreled,
            "may ask two things at once, respondents can only answer one; split it".to_string(),
        );
    }

    if let Some(phrase) = LEADING_PHRASES.iter().find(|p| text.contains(*p)) {
        report(
            RuleKind::Leading,
            format!("{phrase:?} suggests its own answer, ask neutrally instead"),
        );
    }

    let loaded: Vec<&str> = words
        .iter()
        .filter(|w| LOADED_WORDS.contains(w))
        .copied()
        .collect();
    if !loaded.is_empty() {
        report(
            RuleKind::Loaded,
            format!(
                "emotionally charged wording ({}) pushes respondents toward an answer",
                loaded.join(", ")
            ),
        );
    }

    let absolutes: Vec<&str> = words
        .iter()
        .filter(|w| ABSOLUTE_WORDS.contains(w))
        .copied()
        .collect();
    if !absolutes.is_empty() {
        report(
            RuleKind::Absolute,
            format!(
                "absolute terms ({}) force extreme answers, few behaviors are always or never",
                absolutes.join(", ")
            ),
        );
    }

    let negation_count = words.iter().filter(|w| NEGATION_WORDS.contains(w)).count();
    let has_hidden_negative = words.iter().any(|w| HIDDEN_NEGATIVES.contains(w));
    if negation_count >= 2 || (negation_count >= 1 && has_hidden_negative) {
        report(
            RuleKind::DoubleNegative,
            "stacked negations make yes and no ambiguous, rephrase positively".to_string(),
        );
    }

    findings
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_text(text: &str) -> Vec<RuleKind> {
        let question = Question {
            number: 1,
            text: text.to_string(),
        };
        check(&question).into_iter().map(|f| f.rule).collect()
    }

    #[test]
    fn double_barreled_caught() {
        assert!(
            check_text("How satisfied are you with our pricing and support?")
                .contains(&RuleKind::DoubleBarreled)
        );
    }

    #[test]
    fn single_topic_passes() {
        assert!(check_text("How satisfied are you with our pricing?").is_empty());
    }

    #[test]
    fn leading_caught() {
        assert!(
            check_text("Don't you think the new design is better?").contains(&RuleKind::Leading)
        );
    }

    #[test]
    fn neutral_phrasing_passes() {
        assert!(check_text("What do you think of the new design?").is_empty());
    }

    #[test]
    fn loaded_caught() {
        assert!(
            check_text("How amazing was your onboarding experience?").contains(&RuleKind::Loaded)
        );
    }

    #[test]
    fn plain_wording_passes() {
        assert!(check_text("How was your onboarding experience?").is_empty());
    }

    #[test]
    fn absolute_caught() {
        assert!(
            check_text("Do you always read the terms of service?").contains(&RuleKind::Absolute)
        );
    }

    #[test]
    fn bounded_wording_passes() {
        assert!(check_text("How often do you read the terms of service?").is_empty());
    }

    #[test]
    fn double_negative_caught() {
        assert!(
            check_text("Do you not feel unhappy with the checkout flow?")
                .contains(&RuleKind::DoubleNegative)
        );
    }

    #[test]
    fn single_negation_passes() {
        assert!(check_text("Is there anything you did not like about checkout?").is_empty());
    }
}
