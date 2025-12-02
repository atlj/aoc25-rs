use std::usize;

const INPUT: &str = include_str!("./input.txt");

type CustomRange = (usize, usize);

fn main() {
    let parsed = parse_input(INPUT);

    let total: usize = parsed
        .into_iter()
        .map(|(lower, higher)| {
            InvalidIds {
                head: lower,
                upper: higher,
            }
            .sum::<usize>()
        })
        .sum();
    println!("{}", total);
}

fn parse_input(input: &str) -> Vec<CustomRange> {
    input
        .replace("\n", "")
        .split(",")
        .map(|range_str| {
            let mut iter = range_str.split("-");
            let lower = iter.next().unwrap().parse().unwrap();
            let upper = iter.next().unwrap().parse().unwrap();
            (lower, upper)
        })
        .collect()
}

struct InvalidIds {
    head: usize,
    upper: usize,
}

impl Iterator for InvalidIds {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut head_str = self.head.to_string();

        let digit_count = head_str.len();
        if !digit_count.is_multiple_of(2) {
            self.head = 10_usize.pow(digit_count as u32);

            head_str = self.head.to_string();
        }

        let (higher, lower) = split_decimal(&head_str);

        let next_invalid_num = match higher.cmp(&lower) {
            std::cmp::Ordering::Equal => self.head,
            std::cmp::Ordering::Less => join_decimals(higher + 1, higher + 1),
            std::cmp::Ordering::Greater => join_decimals(higher, higher),
        };

        if next_invalid_num > self.upper {
            return None;
        }

        self.head = next_invalid_num + 1;
        Some(next_invalid_num)
    }
}

fn join_decimals(higher: usize, lower: usize) -> usize {
    let mut higher_str = higher.to_string();
    let lower_str = lower.to_string();

    higher_str.push_str(&lower_str);

    higher_str.chars().fold(0_usize, |acc, char| {
        acc * 10 + (char.to_digit(10).unwrap() as usize)
    })
}

fn split_decimal(number_str: &str) -> (usize, usize) {
    let len = number_str.len();

    if !len.is_multiple_of(2) {
        panic!()
    }

    let higher = number_str.chars().take(len / 2).fold(0_usize, |acc, char| {
        acc * 10 + char.to_digit(10).unwrap() as usize
    });

    let lower = number_str.chars().skip(len / 2).fold(0_usize, |acc, char| {
        acc * 10 + char.to_digit(10).unwrap() as usize
    });

    (higher, lower)
}

#[cfg(test)]
mod tests {
    use crate::{InvalidIds, split_decimal};

    #[test]
    fn decimal_digits() {
        assert_eq!(split_decimal("10"), (1, 0));
        assert_eq!(split_decimal("1001"), (10, 1));
    }

    #[test]
    fn test_invalid_iter() {
        assert_eq!(
            InvalidIds {
                head: 11,
                upper: 22,
            }
            .collect::<Vec<_>>(),
            vec![11, 22]
        );

        assert_eq!(
            InvalidIds {
                head: 95,
                upper: 115,
            }
            .collect::<Vec<_>>(),
            vec![99]
        );

        assert_eq!(
            InvalidIds {
                head: 1188511880,
                upper: 1188511890,
            }
            .collect::<Vec<_>>(),
            vec![1188511885]
        );
    }
}
