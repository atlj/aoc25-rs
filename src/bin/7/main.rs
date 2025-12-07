use std::{collections::HashSet, str::FromStr};

use aoc25_rs::{Map, Pos};

const INPUT: &str = include_str!("./input.txt");

fn main() {
    let map: Map<Tile> = Map::from_str(INPUT).unwrap();
    let start_pos = map
        .iter()
        .find_map(|(pos, tile)| {
            if matches!(tile, Tile::Start) {
                Some(pos)
            } else {
                None
            }
        })
        .expect("There is no start");

    let result = tick(&map, HashSet::from_iter([start_pos]), 0);

    dbg!(result);
}

fn tick(map: &Map<Tile>, beams_pos: HashSet<Pos>, initial_split_count: usize) -> usize {
    if beams_pos.is_empty() {
        return initial_split_count;
    }

    // Sucks we alloc here
    let (new_beams, new_split_count) = beams_pos
        .iter()
        .flat_map(|current_beam_pos| move_beam(map, current_beam_pos))
        .fold(
            (HashSet::with_capacity(beams_pos.len()), initial_split_count),
            |(mut new_beams, split_count), beam_action| match beam_action {
                BeamAction::MovedDownTo(down_pos) => {
                    new_beams.insert(down_pos);
                    (new_beams, split_count)
                }
                BeamAction::SplittedInto(left_pos, right_pos) => {
                    // Bad invariant, skill issue
                    assert!(left_pos.is_some() || right_pos.is_some());

                    left_pos.map(|pos| new_beams.insert(pos));
                    right_pos.map(|pos| new_beams.insert(pos));

                    (new_beams, split_count + 1)
                }
            },
        );

    tick(map, new_beams, new_split_count)
}

/// `None` means nowhere left to move
fn move_beam(map: &Map<Tile>, at: &Pos) -> Option<BeamAction> {
    let down_pos = Pos {
        x: at.x,
        y: at.y + 1,
    };

    let result = match map.get_tile(&down_pos)? {
        Tile::Empty => BeamAction::MovedDownTo(down_pos),
        Tile::Split => {
            let left_pos = at.x.checked_sub(1).and_then(|left_x| {
                let pos = Pos {
                    x: left_x,
                    y: at.y + 1,
                };

                map.get_tile(&pos).map(|_| pos)
            });

            let right_pos = {
                let pos = Pos {
                    x: at.x + 1,
                    y: at.y + 1,
                };

                map.get_tile(&pos).map(|_| pos)
            };

            BeamAction::SplittedInto(left_pos, right_pos)
        }
        Tile::Start => panic!("Came back to starting tile"),
    };

    Some(result)
}

#[derive(Debug)]
enum Tile {
    Start,
    Empty,
    Split,
}

#[derive(Debug)]
enum BeamAction {
    MovedDownTo(Pos),
    SplittedInto(Option<Pos>, Option<Pos>),
}

impl TryFrom<char> for Tile {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let result = match value {
            'S' => Tile::Start,
            '.' => Tile::Empty,
            '^' => Tile::Split,
            _ => return Err(value),
        };
        Ok(result)
    }
}
