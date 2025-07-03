pub mod error;
pub mod parser;
pub mod part1;
pub mod part2;

#[derive(Debug, PartialEq)]
pub struct Puzzle {
    pub rules: Vec<(u32, u32)>,
    pub pages: Vec<Vec<u32>>,
}
