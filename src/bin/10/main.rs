use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    str::FromStr,
};

const INPUT: &str = include_str!("./input.txt");

fn main() {
    let result: usize = INPUT
        .lines()
        .map(|line| Goal::from_str(line).unwrap())
        .map(|goal| goal.cost())
        .sum();

    dbg!(result);
}

#[derive(Clone, Debug)]
struct Goal {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

type MinHeap = BinaryHeap<Reverse<(usize, Vec<bool>)>>;
type History = HashSet<(usize, Vec<bool>)>;

impl Goal {
    fn cost(&self) -> usize {
        let initial_state: Vec<bool> = (0..self.lights.len()).map(|_| false).collect();

        let mut heap: MinHeap = BinaryHeap::with_capacity(1000);
        let mut history: History = HashSet::with_capacity(1000);
        self.explore_node(&initial_state, 0, &mut heap, &mut history);

        loop {
            let Reverse((cost, head)) = heap.pop().unwrap();

            if head == self.lights {
                return cost;
            }

            self.explore_node(&head, cost, &mut heap, &mut history);
        }
    }

    fn explore_node(&self, state: &[bool], cost: usize, heap: &mut MinHeap, history: &mut History) {
        for button in self.buttons.iter() {
            let new_state = {
                let mut new_state = Vec::from(state);
                for flip in button {
                    new_state[*flip] = !new_state[*flip]
                }
                new_state
            };

            if history.contains(&(cost + 1, new_state.clone())) {
                continue;
            }

            history.insert((cost + 1, new_state.clone()));

            heap.push(Reverse((cost + 1, new_state)));
        }
    }
}

impl FromStr for Goal {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (lights, rest) = parse_lights(s);
        let (buttons, rest) = parse_buttons(rest);
        let joltage = parse_joltage(rest);

        Ok(Goal {
            lights,
            buttons,
            joltage,
        })
    }
}

/// [..##..]
fn parse_lights(line: &str) -> (Vec<bool>, &str) {
    let mut chars = line.chars();

    assert!(chars.next().unwrap() == '[');

    let result: Vec<bool> = chars
        .take_while(|char| char != &']')
        .map(|char| match char {
            '.' => false,
            '#' => true,
            _ => panic!("{char}"),
        })
        .collect();

    let remainder = &line[(result.len() + 2)..];
    (result, remainder)
}

/// (1,2,3) (2,3) (4,5)
fn parse_buttons(line: &str) -> (Vec<Vec<usize>>, &str) {
    let (end_index, _) = line
        .chars()
        .enumerate()
        .find(|(_, item)| item == &'{')
        .unwrap();

    let result: Vec<Vec<usize>> = line[0..end_index]
        .split_ascii_whitespace()
        .map(parse_button)
        .collect();

    let remainder = &line[end_index..];

    (result, remainder)
}

/// (1,2,3)
fn parse_button(line: &str) -> Vec<usize> {
    line[1..(line.len() - 1)]
        .split(',')
        .map(|digit| digit.parse().unwrap())
        .collect()
}

/// {1,2,3}
fn parse_joltage(line: &str) -> Vec<usize> {
    line[1..(line.len() - 1)]
        .split(',')
        .map(|digit| digit.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::Goal;

    #[test]
    fn test_parse_button() {
        // let res = Machine::from_str("[#...##] (13,2,3) (2,3,4) {1,2}");
        // dbg!(res);
        // panic!()
    }
}
