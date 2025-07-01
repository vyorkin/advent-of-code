use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum AocError {
    #[error(transparent)]
    #[diagnostic(code(aoc::io_error))]
    IoError(#[from] std::io::Error),

    #[error("Parse error: `{0}`")]
    #[diagnostic(code(aoc::parser_error))]
    ParseError(String),
}

mod parser {
    use super::*;

    pub fn parse(
        _input: &str,
    ) -> Result<Vec<(u32, u32)>, AocError> {
        todo!()
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let ops = parser::parse(input)?;
    let sum = ops.iter().fold(0, |acc, (x, y)| acc + x * y);
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("mul(2,3)don't()-mul(3,3)+do()mul(2,2)", "10")]
    #[case(
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        "48"
    )]
    fn test_parser(
        #[case] _input: &str,
        #[case] _expected: &str,
    ) -> miette::Result<()> {
        todo!()
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("48", process(input)?);
        Ok(())
    }
}
