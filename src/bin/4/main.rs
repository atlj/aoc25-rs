use std::str::FromStr;

use aoc25_rs::Map;

const INPUT: &str = include_str!("./input.txt");

fn main() {
    part1();
    part2();
}

fn part1() {
    let map = Map::<Tile>::from_str(INPUT).unwrap();

    let result = map
        .iter()
        .filter(|(_, tile)| matches!(tile, Tile::Roll))
        .filter(|(pos, _)| {
            map.get_tiles_ring1(pos)
                .filter(|(_, tile)| matches!(tile, Tile::Roll))
                .count()
                < 4
        })
        .count();

    dbg!(result);
}

fn part2() {
    let mut map = Map::<Tile>::from_str(INPUT).unwrap();

    let mut result = 0;
    loop {
        let removed = map
            .iter()
            .filter(|(_, tile)| matches!(tile, Tile::Roll))
            .filter_map(|(pos, _)| {
                if map
                    .get_tiles_ring1(&pos)
                    .filter(|(_, tile)| matches!(tile, Tile::Roll))
                    .count()
                    < 4
                {
                    Some(pos)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        if removed.is_empty() {
            break;
        }

        result += removed.len();

        for pos in removed.into_iter() {
            match map.get_tile_mut(&pos) {
                Some(tile) => {
                    *tile = Tile::Empty;
                }
                None => panic!(),
            }
        }
    }

    dbg!(result);
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Empty,
    Roll,
}

impl TryFrom<char> for Tile {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let result = match value {
            '.' => Self::Empty,
            '@' => Self::Roll,
            _ => return Err(value),
        };

        Ok(result)
    }
}
