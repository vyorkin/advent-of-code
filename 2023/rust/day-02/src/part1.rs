#[derive(Debug, PartialEq, Clone, Copy)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, PartialEq, Clone)]
struct Game {
    id: u32,
    rounds: Vec<Vec<(u32, Color)>>,
}

impl Game {
    pub fn new(
        id: u32,
        rounds: Vec<Vec<(u32, Color)>>,
    ) -> Self {
        Self { id, rounds }
    }

    pub fn is_possible(&self, bag: &Bag) -> bool {
        self.rounds.iter().flatten().all(|&(n, color)| {
            match color {
                Color::Red => n <= bag.red,
                Color::Green => n <= bag.green,
                Color::Blue => n <= bag.blue,
            }
        })
    }
}

struct Bag {
    red: u32,
    green: u32,
    blue: u32,
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    todo!("day 01 - part 1");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() -> miette::Result<()> {}

    #[test]
    fn test_process() -> miette::Result<()> {
        todo!("day 01 - test not implemented yet");
        let input = "";
        assert_eq!("", process(input)?);
        Ok(())
    }
}
