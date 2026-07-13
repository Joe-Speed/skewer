mod input;
mod rules;

pub use input::{parse, Format};

#[derive(Debug, Clone, PartialEq)]
pub struct Question {
    pub number: usize,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Finding {
    pub question: usize,
    pub rule: RuleKind,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RuleKind {
    DoubleBarreled,
    Leading,
    Loaded,
    Absolute,
    DoubleNegative,
}

impl RuleKind {
    pub fn name(self) -> &'static str {
        match self {
            RuleKind::DoubleBarreled => "double-barreled",
            RuleKind::Leading => "leading",
            RuleKind::Loaded => "loaded-language",
            RuleKind::Absolute => "absolute",
            RuleKind::DoubleNegative => "double-negative",
        }
    }
}

pub fn lint(questions: &[Question]) -> Vec<Finding> {
    questions.iter().flat_map(rules::check).collect()
}

#[derive(Debug)]
pub enum Error {
    Csv(csv::Error),
    Json(serde_json::Error),
    JsonShape,
    UnknownFormat(String),
    NoQuestions,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Csv(e) => write!(f, "invalid csv: {e}"),
            Error::Json(e) => write!(f, "invalid json: {e}"),
            Error::JsonShape => write!(
                f,
                "json must be an array of strings, or of objects with a question/text/prompt field"
            ),
            Error::UnknownFormat(s) => write!(f, "unknown format {s:?}, expected txt, csv or json"),
            Error::NoQuestions => write!(f, "no questions found in input"),
        }
    }
}

impl std::error::Error for Error {}

impl From<csv::Error> for Error {
    fn from(e: csv::Error) -> Error {
        Error::Csv(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Error {
        Error::Json(e)
    }
}
