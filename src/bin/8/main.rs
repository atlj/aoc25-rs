use std::{collections::HashSet, str::FromStr};

const INPUT: &str = include_str!("./input.txt");

fn main() {
    let boxes: Vec<_> = INPUT.lines().flat_map(JunctionBox::from_str).collect();

    // let mut relations: BinaryHeap<Reverse<JunctionPair>> =
    //     BinaryHeap::with_capacity(boxes.len() * (boxes.len() - 1));
    //
    // boxes.iter().enumerate().for_each(|(index, junction_box)| {
    //     boxes.iter().skip(index + 1).for_each(|second_box| {
    //         relations.push(Reverse(JunctionPair(
    //             junction_box.clone(),
    //             second_box.clone(),
    //         )));
    //     });
    // });
    //
    // for relation in &relations {
    //     println!(
    //         "Distance: {}, Box A: {:?}, Box B: {:?}",
    //         relation.0.0.square_distance_from(&relation.0.1),
    //         relation.0.0,
    //         relation.0.1
    //     );
    // }

    let mut relations: Vec<JunctionPair> = Vec::with_capacity(boxes.len() * (boxes.len() - 1));

    boxes.iter().enumerate().for_each(|(index, junction_box)| {
        boxes.iter().skip(index + 1).for_each(|second_box| {
            relations.push(JunctionPair(junction_box.clone(), second_box.clone()));
        });
    });

    relations.sort();

    // for relation in &relations {
    //     println!(
    //         "Distance: {}, Box A: {:?}, Box B: {:?}",
    //         relation.0.square_distance_from(&relation.1),
    //         relation.0,
    //         relation.1
    //     );
    // }
    //
    let mut pool: Vec<HashSet<JunctionBox>> = boxes
        .into_iter()
        .map(|item| HashSet::from([item]))
        .collect();

    for relation in relations.into_iter().take(1000) {
        let a_index = pool
            .iter()
            .enumerate()
            .find_map(|(index, set)| {
                if set.contains(&relation.0) {
                    Some(index)
                } else {
                    None
                }
            })
            .unwrap();

        let b_index = pool
            .iter()
            .enumerate()
            .find_map(|(index, vec)| {
                if vec.contains(&relation.1) {
                    Some(index)
                } else {
                    None
                }
            })
            .unwrap();

        if a_index == b_index {
            continue;
        }

        let a_vec = std::mem::replace(pool.get_mut(a_index).unwrap(), HashSet::new());
        let b_vec = pool.get_mut(b_index).unwrap();

        b_vec.extend(a_vec);
    }

    pool.sort_by(|a, b| (a.len().cmp(&b.len())).reverse());

    let result: usize = pool.into_iter().map(|set| set.len()).take(3).product();

    dbg!(result);
}

fn get_boxes(input: &str) -> impl Iterator<Item = JunctionBox> {
    input.lines().flat_map(JunctionBox::from_str)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct JunctionPair(JunctionBox, JunctionBox);

impl PartialOrd for JunctionPair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for JunctionPair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0
            .square_distance_from(&self.1)
            .cmp(&other.0.square_distance_from(&other.1))
    }

    // fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    //     let distance_cmp = self
    //         .0
    //         .distance_squared_from(&self.1)
    //         .cmp(&other.0.distance_squared_from(&other.1));
    //
    //     if !matches!(distance_cmp, std::cmp::Ordering::Equal) {
    //         return distance_cmp;
    //     }
    //
    //     let x_cmp = self.0.x.cmp(&other.0.x);
    //
    //     if !matches!(x_cmp, std::cmp::Ordering::Equal) {
    //         return x_cmp;
    //     }
    //
    //     let y_cmp = self.0.y.cmp(&other.0.y);
    //
    //     if !matches!(y_cmp, std::cmp::Ordering::Equal) {
    //         return y_cmp;
    //     }
    //
    //     self.0.z.cmp(selfo)
    // }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct JunctionBox {
    x: usize,
    y: usize,
    z: usize,
}

impl JunctionBox {
    fn square_distance_from(&self, another: &JunctionBox) -> u64 {
        (self.x.abs_diff(another.x).pow(2) as u64)
            + (self.y.abs_diff(another.y).pow(2) as u64)
            + (self.z.abs_diff(another.z).pow(2) as u64)
    }
}

impl FromStr for JunctionBox {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(',');

        let result = match (iter.next(), iter.next(), iter.next()) {
            (Some(x), Some(y), Some(z)) => JunctionBox {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
                z: z.parse().unwrap(),
            },
            _ => panic!("Unexpected input, {s}"),
        };

        Ok(result)
    }
}
