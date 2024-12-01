use std::{collections::HashMap, io::Read};

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let read = std::fs::File::open(&filename).unwrap();
    let mut buf_reader = std::io::BufReader::new(read);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content).unwrap();

    let part_1 = aoc_2024_day01_part1(&content);
    let part_2 = aoc_2024_day01_part2(&content);

    println!("part 1: {}", part_1);
    println!("part 2: {}", part_2);
}

fn aoc_2024_day01_part1 (content: &str) -> u128 {
    let mut col_a: Vec<i32> = Vec::new();
    let mut col_b: Vec<i32> = Vec::new();

    content.lines().for_each(|oline| {
        let line = oline.trim();
        if !line.is_empty() {
            let mut iter = line.split_whitespace();
            col_a.push(iter.next().unwrap().parse().expect("Failed to parse input for A"));
            col_b.push(iter.next().unwrap().parse().expect("Failed to parse input for B"));
        }
    });

    col_a.sort();
    col_b.sort();

    let mut total: u128 = 0;
    col_a.iter().zip(col_b.iter()).for_each(|(a, b)| {
        let diff = (a - b).abs();
        total += diff as u128;
    });

    total
}

fn aoc_2024_day01_part2 (content: &str) -> u128 {
    let mut simil_scores_a: HashMap<i32, u128> = HashMap::new();
    let mut simil_scores_b: HashMap<i32, u128> = HashMap::new();

    content.lines().for_each(|oline| {
        let line = oline.trim();
        if !line.is_empty() {
            let mut iter = line.split_whitespace();
            let a = iter.next().unwrap().parse().expect("Failed to parse input for A");
            let b = iter.next().unwrap().parse().expect("Failed to parse input for B");

            let a_v = simil_scores_a.get(&a).unwrap_or(&0);
            let b_v = simil_scores_b.get(&b).unwrap_or(&0);

            simil_scores_a.insert(a, a_v + 1);
            simil_scores_b.insert(b, b_v + 1);
        }
    });

    let mut total = 0;
    for (num, occurrencies) in simil_scores_a {
        if simil_scores_b.contains_key(&num) {
            total += num as u128 * occurrencies * simil_scores_b[&num];
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use crate::{aoc_2024_day01_part1, aoc_2024_day01_part2};
    const AOC_20204_DAY_TEST_INPUT: &str = r#"
3   4
4   3
2   5
1   3
3   9
3   3"#;
    #[test]
    fn test_aoc_2024_day01_part1 () {
        let total = aoc_2024_day01_part1(AOC_20204_DAY_TEST_INPUT);

        assert_eq!(total, 11)
    }

    #[test]
    fn test_aoc_2024_day01_part2 () {
        let total = aoc_2024_day01_part2(AOC_20204_DAY_TEST_INPUT);

        assert_eq!(total, 31)
    }
}