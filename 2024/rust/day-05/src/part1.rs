use std::collections::{HashMap, HashSet};

use crate::parser;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let puzzle = parser::parse(input)?;

    let mut rules: HashMap<u32, HashSet<u32>> =
        HashMap::new();
    for (from, to) in &puzzle.rules {
        rules.entry(*from).or_default().insert(*to);
    }

    let mut rev_rules: HashMap<u32, HashSet<u32>> =
        HashMap::new();
    for (from, to) in &puzzle.rules {
        rev_rules.entry(*to).or_default().insert(*from);
    }

    // 75|29
    // 61|13
    // 75|53
    //
    // 75,47,61,53,29

    let filtered_lines: Vec<Vec<u32>> = puzzle
        .lines
        .into_iter()
        .filter(|line| {
            line.iter().enumerate().all(|(i, page)| {
                let suffix = &line[i + 1..];
                let valid_suffix = suffix.iter().all(|x| {
                    rules.get(x).is_none_or(|set| {
                        !set.contains(page)
                    })
                });

                let prefix = &line[..i];
                let valid_prefix = prefix.iter().all(|x| {
                    rev_rules.get(x).is_none_or(|set| {
                        !set.contains(page)
                    })
                });

                valid_prefix && valid_suffix
            })
        })
        .collect();

    let middles: Vec<u32> = filtered_lines
        .iter()
        .map(|line| line[line.len() / 2])
        .collect();

    let result: u32 = middles.iter().sum();
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
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

        assert_eq!("143", process(input)?);
        Ok(())
    }
}
