use day_03::*;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    let input = include_str!("../input1.txt");
    part1::process(divan::black_box(input)).unwrap();
}

#[divan::bench]
fn part2() {
    let input = include_str!("../input2.txt");
    part2::process(divan::black_box(input)).unwrap();
}
