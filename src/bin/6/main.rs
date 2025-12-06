use std::str::Chars;

const INPUT: &str = include_str!("./input.txt");

fn main() {
    let mut lines = INPUT.lines().rev();
    let ops_row = lines.next().unwrap();
    let ops = parse_ops_row(&ops_row);

    let mut nums_rows_iterators = lines
        .map(|nums_row| parse_nums_row(nums_row))
        .collect::<Vec<_>>();

    let result: usize = ops
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

    dbg!(result);
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
