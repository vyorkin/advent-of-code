use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
enum AocError {
    #[error(transparent)]
    #[diagnostic(code(aoc::io_error))]
    IoError(#[from] std::io::Error),

    #[error("Parse error:\n`{0}`.\nInput: `{1}`")]
    #[diagnostic(code(aoc::parser_error))]
    ParseError(String, String),
}

#[derive(PartialEq, Clone, Debug)]
enum Opcode {
    Do,
    Dont,
    Mul(u32, u32),
}

mod parser {
    use nom::{
        IResult, Parser,
        branch::alt,
        bytes::complete::tag,
        character::{
            char,
            complete::{anychar, u32},
        },
        combinator::value,
        multi::{many_till, many1},
        sequence::{delimited, separated_pair},
    };

    use super::*;

    fn mul_opcode(
        input: &str,
    ) -> IResult<&str, Opcode, ()> {
        delimited(
            tag("mul("),
            separated_pair(u32, char(','), u32),
            char(')'),
        )
        .map(|(x, y)| Opcode::Mul(x, y))
        .parse(input)
    }

    fn opcode(input: &str) -> IResult<&str, Opcode, ()> {
        alt((
            value(Opcode::Do, tag("do()")),
            value(Opcode::Dont, tag("don't()")),
            mul_opcode,
        ))
        .parse(input)
    }

    fn opcodes(
        input: &str,
    ) -> IResult<&str, Vec<Opcode>, ()> {
        many1(many_till(anychar, opcode).map(|(_, op)| op))
            .parse(input)
    }

    pub fn parse(
        input: &str,
    ) -> Result<Vec<Opcode>, AocError> {
        opcodes(input).map(|x| x.1).map_err(|e| {
            AocError::ParseError(
                e.to_string(),
                input.to_string(),
            )
        })
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let ops = parser::parse(input)?;
    let (_, sum) = ops.iter().fold(
        (true, 0),
        |(enabled, sum), opcode| match (enabled, opcode) {
            (false, Opcode::Do) => (true, sum),
            (true, Opcode::Do) => (true, sum),
            (false, Opcode::Dont) => (false, sum),
            (true, Opcode::Dont) => (false, sum),

            (true, Opcode::Mul(x, y)) => {
                (true, sum + x * y)
            }
            (false, Opcode::Mul(_, _)) => (false, sum),
        },
    );
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use miette::IntoDiagnostic;
    use rstest::rstest;

    use super::{parser::parse, *};

    #[rstest]
    #[case(
        "mul(2,3)don't()-mul(3,3)+do()mul(2,2)",
        vec![
            Opcode::Mul(2, 3),
            Opcode::Dont,
            Opcode::Mul(3, 3),
            Opcode::Do,
            Opcode::Mul(2, 2),
        ]
    )]
    #[case(
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        vec![
            Opcode::Mul(2, 4),
            Opcode::Dont,
            Opcode::Mul(5, 5),
            Opcode::Mul(11, 8),
            Opcode::Do,
            Opcode::Mul(8, 5),
        ]
    )]
    fn test_parser(
        #[case] input: &str,
        #[case] expected: Vec<Opcode>,
    ) -> miette::Result<()> {
        let opcodes = parse(input).into_diagnostic()?;
        assert_eq!(opcodes, expected);
        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("48", process(input)?);
        Ok(())
    }
}
