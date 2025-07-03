#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    // - parse
    // - get only lines of pages which are in the
    //   right order
    // - for each line of pages get a number of page
    //   in the middle sum

    todo!("day 05 - part 1");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        todo!("day 05 - test not implemented yet");
        let input = "";
        assert_eq!("", process(input)?);
        Ok(())
    }
}
