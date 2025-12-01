#[derive(Clone, Debug, PartialEq, Eq)]
enum Direction {
    Left(usize),
    Right(usize),
}

const INPUT: &str = include_str!("./test.txt");

fn main() {
    let (result, _) = INPUT
        .lines()
        .map(parse_line)
        .fold((0, 50), |(score, dial), direction| {
            let dial = turn(dial, &direction);
            match dial {
                0 => (score + 1, dial),
                _ => (score, dial),
            }
        });

    println!("{result}");
}

fn parse_line(input: &str) -> Direction {
    let magnitude = input
        .chars()
        .skip(1)
        .fold(0, |acc, curr| curr.to_digit(10).unwrap() + acc * 10);

    match input.chars().next().unwrap() {
        'L' => Direction::Left(magnitude as usize),
        'R' => Direction::Right(magnitude as usize),
        _ => panic!(),
    }
}

fn turn(from: usize, to: &Direction) -> usize {
    match to {
        Direction::Left(by) => {
            let by_mod = by % 100;
            (from + 100 - by_mod) % 100
        }
        Direction::Right(by) => {
            let by_mod = by % 100;
            (from + by_mod) % 100
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Direction, parse_line, turn};

    #[test]
    fn test_turn_left() {
        assert_eq!(turn(0, &Direction::Left(1)), 99);
        assert_eq!(turn(99, &Direction::Left(99)), 0);
        assert_eq!(turn(82, &Direction::Left(30)), 52);
        assert_eq!(turn(5, &Direction::Left(10)), 95);
        assert_eq!(turn(50, &Direction::Left(68)), 82);
    }

    #[test]
    fn test_turn_right() {
        assert_eq!(turn(99, &Direction::Right(1)), 0);
        assert_eq!(turn(11, &Direction::Right(8)), 19);
        assert_eq!(turn(95, &Direction::Right(10)), 5);
        assert_eq!(turn(95, &Direction::Right(5)), 0);
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("L90"), Direction::Left(90));
        assert_eq!(parse_line("R90"), Direction::Right(90));
        assert_eq!(parse_line("L0"), Direction::Left(0));
        assert_eq!(parse_line("R3"), Direction::Right(3));
    }
}
