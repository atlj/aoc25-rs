use std::ops::RangeInclusive;

const INPUT: &str = include_str!("./input.txt");

fn main() {
    part1();
    part2();
}

fn part2() {
    let (ranges_str, _) = INPUT.split_once("\n\n").unwrap();

    let ranges = parse_ranges(ranges_str);

    let final_ranges: Vec<RangeInclusive<usize>> =
        ranges.fold(Vec::new(), |splitted_ranges, range| {
            let mut result = Vec::with_capacity(splitted_ranges.len());
            let mut head = Some(range);

            for processed_range in splitted_ranges.iter() {
                match head {
                    Some(unwrapped_head) => {
                        let untangled_ranges = untangle_ranges(processed_range, &unwrapped_head);

                        result.push(untangled_ranges.0);
                        head = untangled_ranges.1;
                    }
                    None => {
                        let last = result.last_mut().unwrap();

                        let untangled_ranges = untangle_ranges(processed_range, last);

                        *last = untangled_ranges.0;
                        head = untangled_ranges.1;
                    }
                }
            }

            if let Some(head) = head {
                result.push(head)
            }

            result
        });

    let result: usize = final_ranges.into_iter().map(|range| range.count()).sum();
    dbg!(result);
}

fn part1() {
    let (ranges_str, ids_str) = INPUT.split_once("\n\n").unwrap();

    let ranges: Vec<_> = parse_ranges(ranges_str).collect();

    let ids = parse_ids(ids_str);

    let result = ids
        .filter(|id| ranges.iter().any(|range| range.contains(id)))
        .count();

    dbg!(result);
}

fn parse_ranges(input: &str) -> impl Iterator<Item = RangeInclusive<usize>> {
    input.lines().map(|line| {
        let (a, b) = line.split_once("-").unwrap();
        let a_int: usize = a.parse().unwrap();
        let b_int: usize = b.parse().unwrap();
        a_int..=b_int
    })
}

fn parse_ids(input: &str) -> impl Iterator<Item = usize> {
    input.lines().map(|line| line.parse().unwrap())
}

fn untangle_ranges<T>(
    a: &RangeInclusive<T>,
    b: &RangeInclusive<T>,
) -> (RangeInclusive<T>, Option<RangeInclusive<T>>)
where
    T: std::cmp::Ord
        + std::ops::Sub<Output = T>
        + std::ops::Add<Output = T>
        + Copy
        + From<u8>
        + std::fmt::Debug,
{
    let a_start = a.start();
    let b_start = b.start();

    let a_end = a.end();
    let b_end = b.end();

    match (a_start.cmp(b_start), a_end.cmp(b_end)) {
        // Same range
        (std::cmp::Ordering::Equal, std::cmp::Ordering::Equal) => (a.clone(), None),

        // Start is equal
        (std::cmp::Ordering::Equal, std::cmp::Ordering::Greater) => (a.clone(), None),
        (std::cmp::Ordering::Equal, std::cmp::Ordering::Less) => (b.clone(), None),

        // End is equal
        (std::cmp::Ordering::Less, std::cmp::Ordering::Equal) => (a.clone(), None),
        (std::cmp::Ordering::Greater, std::cmp::Ordering::Equal) => (b.clone(), None),

        // One contains the other
        (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => (a.clone(), None),
        (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => (b.clone(), None),

        // They might be separate
        (std::cmp::Ordering::Less, std::cmp::Ordering::Less)
        | (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => match () {
            // They can be joined
            _ if a_start == b_end || (b_start < a_start && b_end > a_start) => {
                (*b_start..=*a_end, None)
            }
            _ if b_start == a_end || (a_start < b_start && a_end > b_start) => {
                (*a_start..=*b_end, None)
            }

            _ if a_start > b_start && a_start < b_end => {
                (*b_start..=*a_start, Some(*b_end..=*a_end))
            }
            _ if b_start > a_start && b_start < a_end => {
                (*a_start..=*b_start, Some(*a_end..=*b_end))
            }

            // Truly separate
            _ if a_start > b_end || b_start > a_end => (a.clone(), Some(b.clone())),
            _ => {
                dbg!(a, b);
                todo!()
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::untangle_ranges;

    #[test]
    fn untangle() {
        // Correct
        assert_eq!(untangle_ranges(&(0..=5), &(0..=5)), (0..=5, None),);

        // Now do the rest. If there is a second item in vec, it becomes Some(item)

        assert_eq!(untangle_ranges(&(0..=5), &(0..=2)), (0..=5, None));
        assert_eq!(untangle_ranges(&(0..=5), &(3..=5)), (0..=5, None));

        assert_eq!(untangle_ranges(&(3..=5), &(0..=5)), (0..=5, None));

        assert_eq!(untangle_ranges(&(0..=5), &(5..=10)), (0..=10, None));
        assert_eq!(untangle_ranges(&(0..=5), &(6..=10)), (0..=5, Some(6..=10)));
        assert_eq!(untangle_ranges(&(0..=5), &(7..=10)), (0..=5, Some(7..=10)));

        assert_eq!(untangle_ranges(&(5..=10), &(0..=7)), (0..=10, None));

        assert_eq!(untangle_ranges(&(3..=5), &(0..=10)), (0..=10, None));
        assert_eq!(untangle_ranges(&(16..=20), &(12..=18)), (12..=20, None));
        assert_eq!(untangle_ranges(&(10..=14), &(12..=18)), (10..=18, None));
        assert_eq!(untangle_ranges(&(12..=18), &(10..=14)), (10..=18, None));
    }
}
