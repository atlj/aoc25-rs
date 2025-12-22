const INPUT: &str = include_str!("./input.txt");

fn main() {
    let input: Vec<_> = INPUT.lines().map(parse_line).collect();
    let pairs = pairs(&input);

    let max_area = pairs.map(|pair| area(&pair)).max().unwrap();

    dbg!(max_area);
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
