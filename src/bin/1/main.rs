#[derive(Clone, Debug, PartialEq, Eq)]
enum Direction {
    Left(usize),
    Right(usize),
}

const INPUT: &str = include_str!("./input.txt");

fn main() {
    let (result, _) =
        INPUT
            .lines()
            .map(parse_line)
            .fold((0, 50), |(score, original_dial), direction| {
                let (loop_count, dial) = turn(original_dial, &direction);

                println!("{original_dial:?} {direction:?} {dial:?} {loop_count:?}\n=====\n");

                (score + loop_count, dial)
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

fn turn(from: usize, to: &Direction) -> (usize, usize) {
    match to {
        Direction::Left(by) => turn_left(from, *by, 0),
        Direction::Right(by) => turn_right(from, *by, 0),
    }
}

fn turn_left(from: usize, by: usize, loop_count: usize) -> (usize, usize) {
    if let Some(result) = from.checked_sub(by) {
        return (loop_count + if result == 0 { 1 } else { 0 }, result);
    }

    let new_from = from + 100;
    turn_left(new_from, by, loop_count + if from == 0 { 0 } else { 1 })
}

fn turn_right(from: usize, by: usize, loop_count: usize) -> (usize, usize) {
    if from + by < 100 {
        return (loop_count, from + by);
    }

    turn_right(by + from - 100, 0, loop_count + 1)
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
        assert_eq!(turn(0, &Direction::Left(200)), 0);
    }

    #[test]
    fn test_turn_right() {
        assert_eq!(turn(99, &Direction::Right(1)), 0);
        assert_eq!(turn(11, &Direction::Right(8)), 19);
        assert_eq!(turn(95, &Direction::Right(10)), 5);
        assert_eq!(turn(95, &Direction::Right(5)), 0);
        assert_eq!(turn(0, &Direction::Right(200)), 0);
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("L90"), Direction::Left(90));
        assert_eq!(parse_line("R90"), Direction::Right(90));
        assert_eq!(parse_line("L0"), Direction::Left(0));
        assert_eq!(parse_line("R3"), Direction::Right(3));
    }
}
