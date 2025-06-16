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
        character::complete::{space1, u32},
        sequence::separated_pair,
    };

    use super::*;

    fn tuple(input: &str) -> IResult<&str, (u32, u32), ()> {
        let mut parser = separated_pair(u32, space1, u32);
        parser.parse(input)
    }

    pub fn parse(
        input: &str,
    ) -> Result<(u32, u32), AocError> {
        tuple(input).map(|x| x.1).map_err(|_| {
            AocError::ParseError(input.to_string())
        })
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let tuples = input
        .lines()
        .map(parser::parse)
        .collect::<Result<Vec<(u32, u32)>, _>>()?;

    let (mut ids0, mut ids1): (Vec<u32>, Vec<u32>) =
        tuples.iter().fold(
            (vec![], vec![]),
            |(mut ids0, mut ids1), (id0, id1)| {
                ids0.push(*id0);
                ids1.push(*id1);
                (ids0, ids1)
            },
        );

    ids0.sort();
    ids1.sort();

    let sum = ids0
        .iter()
        .zip(ids1)
        .fold(0u32, |acc, (a, b)| acc + a.abs_diff(b));

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() -> miette::Result<()> {
        let input = "143   142";
        let expected = (143, 142);
        let actual = parser::parse(input)?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("11", process(input)?);

        Ok(())
    }
}
