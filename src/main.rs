use std::io::Read;
use std::path::PathBuf;
use std::process::ExitCode;

use skewer::{lint, parse, Format};

const USAGE: &str = "\
skewer, a linter for survey questions

Usage: skewer [FILE] [--format txt|csv|json]

Reads FILE (or stdin) and reports biased or badly constructed questions.
Format is inferred from the file extension; --format overrides it.

  txt   one question per line, # starts a comment
  csv   uses a question/text/prompt column, else the first column
  json  array of strings, or of objects with a question/text/prompt field

Exit codes: 0 clean, 1 findings, 2 error";

fn main() -> ExitCode {
    match run() {
        Ok(0) => ExitCode::SUCCESS,
        Ok(_) => ExitCode::FAILURE,
        Err(message) => {
            eprintln!("skewer: {message}");
            ExitCode::from(2)
        }
    }
}

fn run() -> Result<usize, String> {
    let (path, format_flag) = parse_args()?;

    let (content, format) = match &path {
        Some(path) => {
            let content = std::fs::read_to_string(path)
                .map_err(|e| format!("cannot read {}: {e}", path.display()))?;
            let format = format_flag
                .or_else(|| Format::from_path(path))
                .unwrap_or(Format::Txt);
            (content, format)
        }
        None => {
            let mut content = String::new();
            std::io::stdin()
                .read_to_string(&mut content)
                .map_err(|e| format!("cannot read stdin: {e}"))?;
            (content, format_flag.unwrap_or(Format::Txt))
        }
    };

    let questions = parse(&content, format).map_err(|e| e.to_string())?;
    let findings = lint(&questions);

    for question in &questions {
        let hits: Vec<_> = findings
            .iter()
            .filter(|f| f.question == question.number)
            .collect();
        if hits.is_empty() {
            continue;
        }
        println!("Q{}: {}", question.number, question.text);
        for finding in hits {
            println!("  [{}] {}", finding.rule.name(), finding.message);
        }
        println!();
    }

    let flagged = questions.len() - clean_count(&questions, &findings);
    println!(
        "{} questions checked, {} flagged, {} findings",
        questions.len(),
        flagged,
        findings.len()
    );
    Ok(findings.len())
}

fn clean_count(questions: &[skewer::Question], findings: &[skewer::Finding]) -> usize {
    questions
        .iter()
        .filter(|q| findings.iter().all(|f| f.question != q.number))
        .count()
}

fn parse_args() -> Result<(Option<PathBuf>, Option<Format>), String> {
    let mut path = None;
    let mut format = None;
    let mut args = std::env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--help" | "-h" => {
                println!("{USAGE}");
                std::process::exit(0);
            }
            "--format" => {
                let value = args.next().ok_or("--format needs a value")?;
                format = Some(value.parse().map_err(|e| format!("{e}"))?);
            }
            _ if arg.starts_with('-') => return Err(format!("unknown option {arg:?}")),
            _ if path.is_some() => return Err("expected a single input file".to_string()),
            _ => path = Some(PathBuf::from(arg)),
        }
    }
    Ok((path, format))
}
