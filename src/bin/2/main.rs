use std::{cmp::Ordering, usize};

const INPUT: &str = include_str!("./input.txt");

type CustomRange = (usize, usize);

fn main() {
    let parsed = parse_input(INPUT);

    let total: usize = parsed
        .into_iter()
        .map(|(lower, higher)| {
            FunnyInvalidIds {
                head: lower,
                upper: higher,
            }
            .inspect(|funny| {
                dbg!(funny);
            })
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

struct FunnyInvalidIds {
    head: usize,
    upper: usize,
}

impl Iterator for FunnyInvalidIds {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.head < 11 {
            self.head = 11
        }

        let head_str = self.head.to_string();

        let multipliers = get_multipliers(head_str.len());

        // dbg!(self.head);

        let next_invalid_num = multipliers
            .inspect(|multiplier| {
                // dbg!(multiplier);
            })
            .map(|multiplier| {
                let mut digits = divide_decimal_digits(&head_str, multiplier);

                let first = digits.next().unwrap();
                let first_cmp =
                    digits.fold(Ordering::Equal, |acc, curr| match (acc, first.cmp(&curr)) {
                        (Ordering::Equal, Ordering::Equal) => Ordering::Equal,
                        (Ordering::Equal, Ordering::Less) => Ordering::Less,
                        (Ordering::Equal, Ordering::Greater) => Ordering::Greater,

                        (Ordering::Less, Ordering::Equal) => Ordering::Less,
                        (Ordering::Less, Ordering::Less) => Ordering::Less,
                        (Ordering::Less, Ordering::Greater) => Ordering::Less,

                        (Ordering::Greater, Ordering::Equal) => Ordering::Greater,
                        (Ordering::Greater, Ordering::Less) => Ordering::Greater,
                        (Ordering::Greater, Ordering::Greater) => Ordering::Greater,
                    });

                // dbg!(&first_cmp);

                match first_cmp {
                    Ordering::Equal => self.head,
                    Ordering::Greater => {
                        join_decimal_digits(std::iter::repeat_n(first, head_str.len() / multiplier))
                    }
                    Ordering::Less => join_decimal_digits(std::iter::repeat_n(
                        first + 1,
                        head_str.len() / multiplier,
                    )),
                }
            })
            .min()
            .unwrap();

        // dbg!(next_invalid_num);

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

fn divide_decimal_digits(number_str: &str, by: usize) -> impl Iterator<Item = usize> {
    if !number_str.len().is_multiple_of(by) {
        panic!()
    }

    number_str.as_bytes().chunks(by).map(|chunk| {
        chunk.iter().fold(0_usize, |acc, byte| {
            acc * 10 + ((*byte as char).to_digit(10).unwrap() as usize)
        })
    })
}

fn join_decimal_digits(input: impl Iterator<Item = usize>) -> usize {
    let string = input.fold(String::new(), |mut string, num| {
        string.push_str(&num.to_string());
        string
    });

    string.parse().unwrap()
}

fn get_multipliers(num: usize) -> impl Iterator<Item = usize> {
    (1..num).filter(move |multiplier| num.is_multiple_of(*multiplier))
}

#[cfg(test)]
mod tests {
    use crate::{
        FunnyInvalidIds, InvalidIds, divide_decimal_digits, get_multipliers, join_decimal_digits,
        split_decimal,
    };

    #[test]
    fn decimal_digits() {
        assert_eq!(split_decimal("10"), (1, 0));
        assert_eq!(split_decimal("1001"), (10, 1));
    }

    #[test]
    fn test_divide_decimal_digits() {
        assert_eq!(
            divide_decimal_digits("21", 1).collect::<Vec<_>>(),
            vec![2, 1]
        );
        assert_eq!(divide_decimal_digits("21", 2).collect::<Vec<_>>(), vec![21]);
        assert_eq!(
            divide_decimal_digits("555555", 3).collect::<Vec<_>>(),
            vec![555, 555]
        );
    }

    #[test]
    fn test_multipliers() {
        assert_eq!(get_multipliers(5).collect::<Vec<_>>(), vec![1]);
        assert_eq!(get_multipliers(6).collect::<Vec<_>>(), vec![1, 2, 3]);
    }

    #[test]
    fn test_join_decimal_digits() {
        assert_eq!(join_decimal_digits([1, 2, 3].into_iter()), 123);
        assert_eq!(join_decimal_digits([3, 2, 3, 9].into_iter()), 3239);
    }

    #[test]
    fn test_funny_invalid_iter() {
        let res = FunnyInvalidIds {
            head: 300000,
            upper: 999999999,
        }
        .take(50)
        .collect::<Vec<_>>();
        dbg!(res);
        panic!()
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
