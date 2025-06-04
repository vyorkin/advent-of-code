#[tracing::instrument]
fn recover_calibration_value(line: &str) -> u32 {
    let mut digits =
        line.chars().filter_map(|c| c.to_digit(10));

    let x = digits.next().expect("first digit expected");
    let y = digits.next_back().unwrap_or(x);

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
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";
        assert_eq!("142", process(input)?);
        Ok(())
    }
}
