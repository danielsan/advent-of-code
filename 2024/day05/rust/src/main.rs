use std::io::Read;
fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let read = std::fs::File::open(&filename).unwrap();
    let mut buf_reader = std::io::BufReader::new(read);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content).unwrap();

    let (part_1, part_2) = aoc_2024_day05_part1(&content);
    println!("part 1 new: {}", part_1);

    // let part_2 = aoc_2024_day05_part2(&content);
    println!("part 2 new: {}", part_2);   
}

use hashbrown::HashMap; // 0.15.2
use std::cmp::Ordering::{Equal, Greater, Less};
type Digit = u8;
fn aoc_2024_day05_part1(input:&str) -> (u32, u32) {
  let mut pair_map: HashMap<Digit, Vec<Digit>>=HashMap::with_capacity(10);
  let mut jump = false;
  let mut to_sort: Vec<Vec<Digit>> = Vec::with_capacity(20);

    input.lines().for_each (|line| {
        if !jump {
            if line.len() < 5 {
                jump = true;
                return;
            }

            let a:Digit = line[0..2].parse().unwrap();
            let b:Digit = line[3..5].parse().unwrap();

            match pair_map.get_mut(&a) {
                Some(v) => v.push(b),
                None => { pair_map.insert(a, vec![b]); },
            }
        } else {
            let row:Vec<Digit> = line.split(',')
              .map(|c|  c.parse().expect("update item should parse into a digit"))
              .collect();
            // println!("{:?}", &row);
            to_sort.push(row);
        }
    });
    // println!("pairs: {:?}\n", &pairs);
    // println!("pair_map: {:?}", &pair_map);
    
    for v in pair_map.values_mut() {
        v.sort();
    }
    
    // println!("pair_map: {:?}\n", &pair_map);

    let mut total_same = 0u32;
    let mut total_changed = 0u32;
    for mut line in to_sort {
        let original = line.clone();
        line.sort_by(|a, b| {
            let aa = match pair_map.get(a) {
                Some(v) => v,
                None => { return Greater }
            };

            if aa.contains(&b) { return Less }

            let bb = match pair_map.get(b) {
                Some(v) => v,
                None => { return Less }
            };

            if bb.contains(&a) { return Greater }

            // println!("Sorting CMP\n\taa: ({}) {:?} vs\n\tbb: ({}) {:?}", &a, &aa, &b, &bb);
            
            let len = if aa.len() > bb.len() { bb.len() } else { aa.len () };
            
            let mut res = Equal;
            for i in 0..len {
                res = aa[i].cmp(&bb[i]);
                if res != Equal { break };
            }

            res
        });

        let mid_value = line[line.len() / 2];
        if &line == &original {
            total_same += mid_value as u32;
        } else {
            total_changed += mid_value as u32;
        }
    }

    (total_same, total_changed)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str =
"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_aoc_2024_day05_part1() {
        assert_eq!(aoc_2024_day05_part1(TEST_INPUT), (143, 123));
    }
}