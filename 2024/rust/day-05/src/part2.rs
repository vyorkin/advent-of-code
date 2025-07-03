#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    todo!("day ?? - part 2");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        todo!("day ?? - test not implemented yet");
        let input = "";
        assert_eq!("", process(input)?);
        Ok(())
    }
}
