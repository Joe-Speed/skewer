use std::path::Path;
use std::str::FromStr;

use crate::{Error, Question};

const QUESTION_FIELD_NAMES: &[&str] = &["question", "text", "prompt"];

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Format {
    Txt,
    Csv,
    Json,
}

impl Format {
    pub fn from_path(path: &Path) -> Option<Format> {
        match path.extension()?.to_str()? {
            "txt" | "text" | "md" => Some(Format::Txt),
            "csv" => Some(Format::Csv),
            "json" => Some(Format::Json),
            _ => None,
        }
    }
}

impl FromStr for Format {
    type Err = Error;

    fn from_str(s: &str) -> Result<Format, Error> {
        match s {
            "txt" => Ok(Format::Txt),
            "csv" => Ok(Format::Csv),
            "json" => Ok(Format::Json),
            other => Err(Error::UnknownFormat(other.to_string())),
        }
    }
}

pub fn parse(content: &str, format: Format) -> Result<Vec<Question>, Error> {
    let texts = match format {
        Format::Txt => parse_txt(content),
        Format::Csv => parse_csv(content)?,
        Format::Json => parse_json(content)?,
    };
    if texts.is_empty() {
        return Err(Error::NoQuestions);
    }
    Ok(texts
        .into_iter()
        .enumerate()
        .map(|(i, text)| Question {
            number: i + 1,
            text,
        })
        .collect())
}

fn parse_txt(content: &str) -> Vec<String> {
    content
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(String::from)
        .collect()
}

fn parse_csv(content: &str) -> Result<Vec<String>, Error> {
    let mut reader = csv::Reader::from_reader(content.as_bytes());
    let column = reader
        .headers()?
        .iter()
        .position(|h| QUESTION_FIELD_NAMES.contains(&h.to_lowercase().as_str()))
        .unwrap_or(0);
    let mut texts = Vec::new();
    for record in reader.records() {
        let text = record?.get(column).unwrap_or("").trim().to_string();
        if !text.is_empty() {
            texts.push(text);
        }
    }
    Ok(texts)
}

fn parse_json(content: &str) -> Result<Vec<String>, Error> {
    let value: serde_json::Value = serde_json::from_str(content)?;
    let items = value.as_array().ok_or(Error::JsonShape)?;
    items
        .iter()
        .map(|item| match item {
            serde_json::Value::String(s) => Ok(s.trim().to_string()),
            serde_json::Value::Object(fields) => QUESTION_FIELD_NAMES
                .iter()
                .find_map(|key| fields.get(*key).and_then(|v| v.as_str()))
                .map(|s| s.trim().to_string())
                .ok_or(Error::JsonShape),
            _ => Err(Error::JsonShape),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn txt_skips_blanks_and_comments() {
        let questions = parse("# survey\n\nHow was it?\n", Format::Txt).unwrap();
        assert_eq!(questions.len(), 1);
        assert_eq!(questions[0].text, "How was it?");
        assert_eq!(questions[0].number, 1);
    }

    #[test]
    fn csv_finds_question_column() {
        let content = "id,Question\n1,How was it?\n2,Would you return?\n";
        let questions = parse(content, Format::Csv).unwrap();
        assert_eq!(questions.len(), 2);
        assert_eq!(questions[1].text, "Would you return?");
    }

    #[test]
    fn csv_without_known_header_uses_first_column() {
        let content = "q\nHow was it?\n";
        let questions = parse(content, Format::Csv).unwrap();
        assert_eq!(questions[0].text, "How was it?");
    }

    #[test]
    fn json_accepts_strings_and_objects() {
        let content = r#"["How was it?", {"question": "Would you return?"}]"#;
        let questions = parse(content, Format::Json).unwrap();
        assert_eq!(questions.len(), 2);
        assert_eq!(questions[1].text, "Would you return?");
    }

    #[test]
    fn json_wrong_shape_errors() {
        assert!(matches!(
            parse(r#"{"not": "an array"}"#, Format::Json),
            Err(Error::JsonShape)
        ));
    }

    #[test]
    fn empty_input_errors() {
        assert!(matches!(parse("", Format::Txt), Err(Error::NoQuestions)));
    }
}
