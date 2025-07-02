use std::collections::HashMap;

const DIRECTIONS: [[(i32, i32); 3]; 8] = [
    [(0, 1), (0, 2), (0, 3)],
    [(0, -1), (0, -2), (0, -3)],
    [(1, 1), (2, 2), (3, 3)],
    [(1, -1), (2, -2), (3, -3)],
    [(-1, 1), (-2, 2), (-3, 3)],
    [(-1, -1), (-2, -2), (-3, -3)],
    [(1, 0), (2, 0), (3, 0)],
    [(-1, 0), (-2, 0), (-3, 0)],
];

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let positions = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                ((x as i32, y as i32), c)
            })
        })
        .collect::<HashMap<(i32, i32), char>>();

    let mas = ['M', 'A', 'S'];

    let result: usize = positions
        .iter()
        .filter(|(_, c)| **c == 'X')
        .map(|((x, y), _)| {
            DIRECTIONS
                .iter()
                .map(|offsets| {
                    offsets
                        .iter()
                        .map(|(dx, dy)| {
                            positions.get(&(x + dx, y + dy))
                        })
                        .enumerate()
                        .all(|(ix, c)| mas.get(ix) == c)
                })
                .filter(|b| *b)
                .count()
        })
        .sum();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_simple() -> miette::Result<()> {
        let input: &'static str = "..X...
.SAMX.
.A..A.
XMAS.S
.X....";

        assert_eq!("4", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input: &'static str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!("18", process(input)?);
        Ok(())
    }
}
