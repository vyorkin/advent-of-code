use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum AocError {
    #[error("Parse error:\n`{0}`.\nInput: `{1}`")]
    #[diagnostic(code(aoc::parser_error))]
    ParseError(String, String),
}
