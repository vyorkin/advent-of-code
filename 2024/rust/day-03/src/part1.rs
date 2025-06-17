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
    use nom::{
        IResult, Parser,
        bytes::complete::tag,
        character::complete::u32,
        sequence::{delimited, separated_pair},
    };

    use super::*;

    fn mul(input: &str) -> IResult<&str, (u32, u32), ()> {
        let mut parser = delimited(
            tag("mul("),
            separated_pair(u32, tag(","), u32),
            tag(")"),
        );
        parser.parse(input)
    }

    fn mul_ops(
        input: &str,
    ) -> IResult<&str, Vec<(u32, u32)>, ()> {
        let mut rest = input;
        let mut results = Vec::new();

        while let Some(pos) = rest.find("mul(") {
            let after = &rest[pos..];
            match mul(after) {
                Ok((next, pair)) => {
                    results.push(pair);
                    rest = next;
                }
                Err(_) => {
                    rest = &after[1..];
                }
            }
        }
        Ok(("", results))
    }

    pub fn parse(
        input: &str,
    ) -> Result<Vec<(u32, u32)>, AocError> {
        mul_ops(input).map(|x| x.1).map_err(|_| {
            AocError::ParseError(input.to_string())
        })
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
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!("161", process(input)?);
        Ok(())
    }
}
