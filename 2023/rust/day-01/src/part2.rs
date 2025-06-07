use std::iter::from_fn;

#[tracing::instrument]
fn recover_calibration_value(line: &str) -> u32 {
    let mut s = String::from(line);

    let digits = from_fn(move || {
        let (n, skip) = if s.starts_with("one") {
            (Some(1), 2)
        } else if s.starts_with("two") {
            (Some(2), 2)
        } else if s.starts_with("three") {
            (Some(3), 4)
        } else if s.starts_with("four") {
            (Some(4), 3)
        } else if s.starts_with("five") {
            (Some(5), 3)
        } else if s.starts_with("six") {
            (Some(6), 2)
        } else if s.starts_with("seven") {
            (Some(7), 4)
        } else if s.starts_with("eight") {
            (Some(8), 4)
        } else if s.starts_with("nine") {
            (Some(9), 3)
        } else if let Some(c) = s.chars().next() {
            if let Some(n) = c.to_digit(10) {
                (Some(n), 1)
            } else {
                (Some(0), 1)
            }
        } else {
            (None, 0)
        };
        s = String::from(&s[skip..]);
        n
    })
    .filter(|&d| d != 0)
    .collect::<Vec<u32>>();

    let x = digits.first().expect("first digit expected");
    let y = digits.last().unwrap_or(x);

    format!("{x}{y}")
        .parse()
        .expect("expected to have a number")
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let answer = input
        .lines()
        .map(recover_calibration_value)
        .sum::<u32>();

    Ok(answer.to_string())
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("two1nine", "29")]
    #[case("eightwothree", "83")]
    #[case("abcone2threexyz", "13")]
    #[case("xtwone3four", "24")]
    #[case("4nineeightseven2", "42")]
    #[case("zoneight234", "14")]
    #[case("7pqrstsixteen", "76")]
    fn test_process(
        #[case] input: &str,
        #[case] expected: &str,
    ) -> miette::Result<()> {
        assert_eq!(expected, process(input)?);
        Ok(())
    }
}
