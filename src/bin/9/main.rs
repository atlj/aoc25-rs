const INPUT: &str = include_str!("./test.txt");

fn main() {
    part2();
}

fn part2() {
    let input: Vec<_> = INPUT.lines().map(parse_line).collect();
    let filtered = pairs(&input).filter(|pair| {
        let (opposing_a, opposing_b) = opposing_pair(pair);
        input.contains(&opposing_a) && input.contains(&opposing_b)
    });

    let max_area = filtered
        .inspect(|a| {
            dbg!(a);
        })
        .map(|pair| area(&pair))
        .max()
        .unwrap();
    dbg!(max_area);
}

fn part1() {
    let input: Vec<_> = INPUT.lines().map(parse_line).collect();
    let pairs = pairs(&input);

    let max_area = pairs.map(|pair| area(&pair)).max().unwrap();

    dbg!(max_area);
}

fn opposing_pair((a, b): &(Pos, Pos)) -> (Pos, Pos) {
    match (a.0.cmp(&b.0), a.1.cmp(&b.1)) {
        (std::cmp::Ordering::Less, std::cmp::Ordering::Less)
        | (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => (
            (usize::max(a.0, b.0), usize::min(a.1, b.1)),
            (usize::min(a.0, b.0), usize::max(a.1, b.1)),
        ),
        _ => (
            (usize::max(a.0, b.0), usize::max(a.1, b.1)),
            (usize::min(a.0, b.0), usize::min(a.1, b.1)),
        ),
    }
}

fn pairs(input: &[Pos]) -> impl Iterator<Item = (Pos, Pos)> {
    input
        .iter()
        .enumerate()
        .flat_map(move |(index, a)| input.iter().skip(index + 1).map(|b| (*a, *b)))
}

fn area((a, b): &(Pos, Pos)) -> usize {
    (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1)
}

fn parse_line(line: &str) -> Pos {
    let (x, y) = line.split_once(',').unwrap();
    (x.parse().unwrap(), y.parse().unwrap())
}

type Pos = (usize, usize);
