use aoc_2022_lib::input::read_contents;
use aoc_2022_lib::puzzle::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day 1, part 1", |b| {
        b.iter(|| one::solve_part_one(black_box(&read_contents("./inputs/day1.txt"))))
    });
    c.bench_function("day 1, part 2", |b| {
        b.iter(|| one::solve_part_two(black_box(&read_contents("./inputs/day1.txt"))))
    });

    c.bench_function("day 2, part 1", |b| {
        b.iter(|| two::solve_part_one(black_box(&read_contents("./inputs/day2.txt"))))
    });
    c.bench_function("day 2, part 2", |b| {
        b.iter(|| two::solve_part_two(black_box(&read_contents("./inputs/day2.txt"))))
    });

    c.bench_function("day 3, part 1", |b| {
        b.iter(|| three::solve_part_one(black_box(&read_contents("./inputs/day3.txt"))))
    });
    c.bench_function("day 3, part 2", |b| {
        b.iter(|| three::solve_part_two(black_box(&read_contents("./inputs/day3.txt"))))
    });

    c.bench_function("day 4, part 1", |b| {
        b.iter(|| three::solve_part_one(black_box(&read_contents("./inputs/day4.txt"))))
    });
    c.bench_function("day 4, part 2", |b| {
        b.iter(|| three::solve_part_two(black_box(&read_contents("./inputs/day4.txt"))))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
