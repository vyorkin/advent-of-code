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
        IResult, Parser, bytes::complete::tag,
        character::complete::u32, multi::separated_list1,
    };

    use super::*;

    fn report(input: &str) -> IResult<&str, Vec<u32>, ()> {
        let mut parser = separated_list1(tag(" "), u32);
        parser.parse(input)
    }

    pub fn parse(
        input: &str,
    ) -> Result<Vec<u32>, AocError> {
        report(input).map(|x| x.1).map_err(|_| {
            AocError::ParseError(input.to_string())
        })
    }
}

fn is_safe_monotonic<F>(xs: &[u32], cmp: F) -> bool
where
    F: Fn(u32, u32) -> bool,
{
    xs.windows(2).all(|w| {
        let (prev, curr) = (w[0], w[1]);
        let diff = curr.abs_diff(prev);
        (1..=3).contains(&diff) && cmp(curr, prev)
    })
}

fn is_safe(xs: &[u32]) -> bool {
    xs.len() < 2
        || is_safe_monotonic(xs, |curr, prev| curr > prev)
        || is_safe_monotonic(xs, |curr, prev| curr < prev)
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let reports = input
        .lines()
        .map(parser::parse)
        .collect::<Result<Vec<Vec<u32>>, _>>()?;

    let sum =
        reports.iter().filter(|&r| is_safe(r)).count();

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() -> miette::Result<()> {
        let input = "7 6 4 2 1";
        let expected = vec![7, 6, 4, 2, 1];
        let actual = parser::parse(input)?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
