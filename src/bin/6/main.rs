use std::str::Chars;

const INPUT: &str = include_str!("./input.txt");

fn main() {
    part1();
    part2();
}

fn part2() {
    let mut lines = INPUT.lines().rev();
    let ops_row = lines.next().unwrap();
    let ops = parse_ops_row(ops_row);

    let mut char_iters: Vec<_> = lines.rev().map(|row| row.chars()).collect();
    let mut num_cols = NumCols {
        char_iters: &mut char_iters,
    };

    let part2_result: usize = ops
        .map(|operation| {
            let init = match operation {
                Operation::Add => 0,
                Operation::Multiply => 1,
            };

            num_cols
                .by_ref()
                .take_while(|num| num != &0)
                .fold(init, |acc, num| match operation {
                    Operation::Add => acc + num,
                    Operation::Multiply => acc * num,
                })
        })
        .sum();

    dbg!(part2_result);
}

fn part1() {
    let mut lines = INPUT.lines().rev();
    let ops_row = lines.next().unwrap();
    let ops = parse_ops_row(ops_row);

    let mut nums_rows_iterators = lines.map(parse_nums_row).collect::<Vec<_>>();

    let part1_result: usize = ops
        .map(|operation| {
            let mut result = match operation {
                Operation::Add => 0,
                Operation::Multiply => 1,
            };

            for num_iter in nums_rows_iterators.iter_mut() {
                match operation {
                    Operation::Add => result += num_iter.next().unwrap(),
                    Operation::Multiply => result *= num_iter.next().unwrap(),
                }
            }

            result
        })
        .sum();

    dbg!(part1_result);
}

struct NumCols<'a, 'b>
where
    'b: 'a,
{
    char_iters: &'a mut [Chars<'b>],
}

impl Iterator for NumCols<'_, '_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = 0;

        for char_iter in self.char_iters.iter_mut() {
            let char = char_iter.next()?;
            if let Some(digit) = char
                .to_digit(10)
                .map(|digit| usize::try_from(digit).unwrap())
            {
                result = result * 10 + digit
            }
        }

        Some(result)
    }
}

fn parse_nums_row(row: &str) -> impl Iterator<Item = usize> {
    NumsRowIter { chars: row.chars() }
}

fn parse_ops_row(row: &str) -> impl Iterator<Item = Operation> {
    row.chars().flat_map(Operation::try_from)
}

enum Operation {
    Add,
    Multiply,
}

impl TryFrom<char> for Operation {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let result = match value {
            '+' => Self::Add,
            '*' => Self::Multiply,
            _ => return Err(()),
        };

        Ok(result)
    }
}

struct NumsRowIter<'a> {
    chars: Chars<'a>,
}

impl<'a> Iterator for NumsRowIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = None;

        loop {
            let next = match self.chars.next() {
                Some(next) => next,
                None => return result,
            };

            match next
                .to_digit(10)
                .map(|digit| usize::try_from(digit).unwrap())
            {
                Some(digit) => {
                    result = Some(result.unwrap_or(0) * 10 + digit);
                }
                None if result.is_some() => return result,
                _ => {}
            }
        }
    }
}
