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
        branch::alt,
        bytes::complete::{tag, take, take_until},
        character::{char, complete::u32},
        combinator::map,
        multi::many0,
        sequence::{delimited, preceded, separated_pair},
    };

    use super::*;

    fn mul(input: &str) -> IResult<&str, (u32, u32), ()> {
        delimited(
            tag("mul("),
            separated_pair(u32, char(','), u32),
            char(')'),
        )
        .parse(input)
    }

    fn find_mul(
        input: &str,
    ) -> IResult<&str, (u32, u32), ()> {
        preceded(
            take_until("mul("),
            map(mul, |pair| pair),
        )
        .parse(input)
    }

    fn mul_ops(
        input: &str,
    ) -> IResult<&str, Vec<(u32, u32)>, ()> {
        let skip_one = map(take(1usize), |_| None);
        let next_mul = map(find_mul, Some);

        let (rest, matches) =
            many0(alt((next_mul, skip_one)))
                .parse(input)?;

        let results =
            matches.into_iter().flatten().collect();

        Ok((rest, results))
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
