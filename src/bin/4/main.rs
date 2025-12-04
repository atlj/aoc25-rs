const INPUT: &str = include_str!("./input.txt");

fn main() {
    let mut map = parse(INPUT);

    let mut result = 0;

    loop {
        let pos_to_remove: Vec<_> = walk(&map)
            .filter(|(_, tile)| matches!(tile, Tile::Roll))
            .filter_map(|(pos, _)| {
                if adjacent_cells(&map, &pos)
                    .filter(|(_, tile)| matches!(tile, Tile::Roll))
                    .count()
                    < 4
                {
                    Some(pos)
                } else {
                    None
                }
            })
            .collect();

        if pos_to_remove.is_empty() {
            break;
        }

        result += pos_to_remove.len();

        for pos in pos_to_remove {
            map[pos.1][pos.0] = Tile::Empty;
        }
    }

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
            let x = ((isize::try_from(of_pos.0).unwrap()) + (delta_x))
                .try_into()
                .ok()?;
            let y = ((isize::try_from(of_pos.1).unwrap()) + (delta_y))
                .try_into()
                .ok()?;

            let row: &Vec<_> = map.get(y)?;
            let tile = row.get(x)?;

            Some(((x, y), tile))
        })
}
