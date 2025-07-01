#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    todo!("day 04 - part 1");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        todo!("day 04 - test not implemented yet");
        let input = "";
        assert_eq!("", process(input)?);
        Ok(())
    }
}
