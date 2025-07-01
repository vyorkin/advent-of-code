use criterion::{
    Criterion, criterion_group, criterion_main,
};
use day_04::*;

fn part1(c: &mut Criterion) {
    let mut group = c.benchmark_group("day_04::part1");
    let input = include_str!("../input1.txt");
    group.bench_with_input("part1", input, |b, input| {
        b.iter(|| part1::process(input))
    });
    group.finish();
}

fn part2(c: &mut Criterion) {
    let mut group = c.benchmark_group("day_04::part2");
    let input = include_str!("../input2.txt");
    group.bench_with_input("part2", input, |b, input| {
        b.iter(|| part2::process(input))
    });
    group.finish();
}

criterion_group!(benches, part1, part2);
criterion_main!(benches);
