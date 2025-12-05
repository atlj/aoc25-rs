use std::ops::RangeInclusive;

const INPUT: &str = include_str!("./input.txt");

fn main() {
    let (ranges_str, ids_str) = INPUT.split_once("\n\n").unwrap();

    let ranges: Vec<_> = parse_ranges(ranges_str).collect();

    let ids = parse_ids(ids_str);

    let result = ids
        .filter(|id| ranges.iter().any(|range| range.contains(id)))
        .count();

    dbg!(result);
}

fn parse_ranges(input: &str) -> impl Iterator<Item = RangeInclusive<usize>> {
    input.lines().map(|line| {
        let (a, b) = line.split_once("-").unwrap();
        let a_int: usize = a.parse().unwrap();
        let b_int: usize = b.parse().unwrap();
        a_int..=b_int
    })
}

fn parse_ids(input: &str) -> impl Iterator<Item = usize> {
    input.lines().map(|line| line.parse().unwrap())
}
