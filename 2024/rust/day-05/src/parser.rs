use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{line_ending, u32},
    multi::separated_list1,
    sequence::{pair, separated_pair},
};

use crate::{Puzzle, error::AocError};

pub fn parse_puzzle(
    input: &str,
) -> IResult<&str, Puzzle, ()> {
    let rules = separated_list1(
        line_ending,
        separated_pair(u32, tag("|"), u32),
    );
    let updates = separated_list1(
        line_ending,
        separated_list1(tag(","), u32),
    );
    let empty_line = pair(line_ending, line_ending);
    separated_pair(rules, empty_line, updates)
        .map(|(rules, lines)| Puzzle { rules, lines })
        .parse(input)
}

pub fn parse(input: &str) -> Result<Puzzle, AocError> {
    parse_puzzle(input).map(|x| x.1).map_err(|e| {
        AocError::ParseError(
            e.to_string(),
            input.to_string(),
        )
    })
}

#[cfg(test)]
mod tests {
    use miette::IntoDiagnostic;

    use super::*;

    #[test]
    fn test_parser() -> miette::Result<()> {
        let input: &'static str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        let expected = Puzzle {
            rules: vec![
                (47, 53),
                (97, 13),
                (97, 61),
                (97, 47),
                (75, 29),
                (61, 13),
                (75, 53),
                (29, 13),
                (97, 29),
                (53, 29),
                (61, 53),
                (97, 53),
                (61, 29),
                (47, 13),
                (75, 47),
                (97, 75),
                (47, 61),
                (75, 61),
                (47, 29),
                (75, 13),
                (53, 13),
            ],
            lines: vec![
                vec![75, 47, 61, 53, 29],
                vec![97, 61, 53, 29, 13],
                vec![75, 29, 13],
                vec![75, 97, 47, 61, 53],
                vec![61, 13, 29],
                vec![97, 13, 75, 29, 47],
            ],
        };
        let puzzle = parse(input).into_diagnostic()?;
        assert_eq!(puzzle, expected);
        Ok(())
    }
}
