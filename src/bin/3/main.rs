const INPUT: &str = include_str!("./input.txt");

fn main() {
    let result: usize = INPUT
        .lines()
        .map(parse_bank)
        .map(|iter| iter.collect::<Box<[_]>>())
        // .inspect(|bank| {
        //     dbg!(&bank);
        // })
        .map(|bank| find_max_possible_joltage(&bank, 12))
        .sum();

    println!("result is: {result}")
}

fn parse_bank(input: &str) -> impl Iterator<Item = u8> + Clone {
    input.chars().map(|char| char.to_digit(10).unwrap() as u8)
}

fn find_max_possible_joltage(bank: &[u8], count: u8) -> usize {
    let len = bank.len();

    if usize::from(count) > len {
        panic!()
    }

    (0..count)
        .rev()
        .fold((0_usize, 0), |(start_index, acc), pow| {
            let end_index = len - usize::from(pow) - 1;
            let (index, max_digit) = bank
                .iter()
                .enumerate()
                .skip(start_index)
                .take_while(|(index, _)| index <= &end_index)
                .max_by(
                    |(index_a, digit_a), (index_b, digit_b)| match digit_a.cmp(digit_b) {
                        std::cmp::Ordering::Equal => index_b.cmp(index_a),
                        _ => digit_a.cmp(digit_b),
                    },
                )
                .unwrap();

            (
                index + 1,
                acc + (usize::from(*max_digit)) * 10_usize.pow(u32::from(pow)),
            )
        })
        .1
}

#[cfg(test)]
mod tests {
    use crate::find_max_possible_joltage;

    #[test]
    fn test_max_joltage() {
        assert_eq!(find_max_possible_joltage(&[1, 2], 2), 12);
        assert_eq!(find_max_possible_joltage(&[3, 1, 2], 2), 32);
        assert_eq!(find_max_possible_joltage(&[3, 1, 2], 3), 312);
        assert_eq!(find_max_possible_joltage(&[3, 1, 9, 2], 3), 392);
        assert_eq!(find_max_possible_joltage(&[3, 9, 1, 2], 3), 912);
        assert_eq!(
            find_max_possible_joltage(&[9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1], 12),
            987654321111
        );
    }
}
