use regex::Regex;

fn get_first_and_last_digits(input: &str) -> u8 {
  let start_regex = Regex::new(r"(\d).*$").unwrap();
  let end_regex = Regex::new(r"^.*(\d)").unwrap();

  let regexps = vec![start_regex, end_regex];
  let mut digits = String::new();

  for re in regexps {
    match re.captures(input) {
      Some(cap) => {
        let digit = cap.get(1).unwrap().as_str();
        digits.push_str(digit);
      },
      None => { dbg!("No match"); }
    }
  }

  // convert to a u8 and return it
  digits.parse::<u8>().unwrap_or(0)
}

fn part1(input: &str) -> String {
  let mut results = vec![];
  
  for line in input.lines() {
    let digits = get_first_and_last_digits(line);
    results.push(digits);
  }

  // dbg!(&results);

  // sum the vector
  let mut sum: u32 = 0;
  for digit in results {
    sum += digit as u32;
  }

  sum.to_string()
}

fn main() {
  // get a file path from the command line arguments
  let input_file = std::env::args().nth(1).expect("please supply a file path");
  // read the file into a string
  let input = std::fs::read_to_string(input_file).expect("failed to read file");
  dbg!(&input);
  // let input = include_str!("./test-input-day01.txt");  
  let output = part1(&input);
  println!("{}", output);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part1() {
    assert_eq!(part1("abc1def\na1b2c\n1abc2"), "35".to_string());
  }
}