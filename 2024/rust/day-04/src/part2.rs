use std::collections::HashMap;

// M.S
// .A.
// M.S

const DIRECTIONS: [[(i32, i32); 2]; 4] = [
    [(-1, -1), (1, 1)],
    [(-1, 1), (1, -1)],
    [(1, -1), (-1, 1)],
    [(1, 1), (-1, -1)],
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

    let ms = ['M', 'S'];

    let result: usize = positions
        .iter()
        .filter(|(_, c)| **c == 'A')
        .filter(|((x, y), _)| {
            DIRECTIONS
                .iter()
                .map(|offsets| {
                    offsets
                        .iter()
                        .map(|(dx, dy)| {
                            positions.get(&(x + dx, y + dy))
                        })
                        .enumerate()
                        .all(|(ix, c)| ms.get(ix) == c)
                })
                .filter(|b| *b)
                .count()
                == 2
        })
        .count();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input: &'static str = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";
        assert_eq!("9", process(input)?);
        Ok(())
    }
}
