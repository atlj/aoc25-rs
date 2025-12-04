const INPUT: &str = include_str!("./input.txt");

fn main() {
    let map = parse(INPUT);
    let result: usize = walk(&map)
        .filter(|(_, tile)| matches!(tile, Tile::Roll))
        .filter(|(pos, tile)| {
            adjacent_cells(&map, pos)
                .filter(|(_, tile)| matches!(tile, Tile::Roll))
                .count()
                < 4
        })
        .count();
    dbg!(result);
}

type Pos = (usize, usize);

type Map = Vec<Vec<Tile>>;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Empty,
    Roll,
}

fn parse(input: &str) -> Map {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    '.' => Tile::Empty,
                    '@' => Tile::Roll,
                    _ => panic!(),
                })
                .collect()
        })
        .collect()
}

fn walk(map: &Map) -> impl Iterator<Item = (Pos, &Tile)> {
    map.iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, tile)| ((x, y), tile)))
}

fn adjacent_cells<'a>(map: &'a Map, of_pos: &Pos) -> impl Iterator<Item = (Pos, &'a Tile)> {
    let adjacent_deltas: [(isize, isize); 8] = [
        // TOP
        (-1, -1),
        (0, -1),
        (1, -1),
        // MIDDLE
        (-1, 0),
        (1, 0),
        // BOTTOM
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    adjacent_deltas
        .into_iter()
        .filter_map(|(delta_x, delta_y)| {
            let x = ((of_pos.0 as isize) + (delta_x)).try_into().ok()?;
            let y = ((of_pos.1 as isize) + (delta_y)).try_into().ok()?;

            if y > map.len() || x > map[0].len() {
                return None;
            }

            let row: &Vec<_> = map.get(y)?;
            let tile = row.get(x)?;

            Some(((x, y), tile))
        })
}
