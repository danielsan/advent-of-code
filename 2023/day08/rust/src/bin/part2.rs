use std::env;
use std::fs;
use std::io;
// use std::collections::HashMap;
// use std::time::Instant;
use regex::Regex;
use std::rc::Rc;
use ahash::AHashMap as HashMap;

fn read_input_file(path: &str) -> io::Result<String> {
  fs::read_to_string(path)
}

#[derive(Debug)]
struct Node<'a> {
  L: Rc<&'a str>,
  R: Rc<&'a str>
}

impl Node<'_> {
  fn new<'a>(L: Rc<&'a str>, R: Rc<&'a str>) -> Node<'a> {
    Node { L, R }
  }
}

#[derive(Debug)]
struct CommandLineArgs {
  input_file: String
}

impl CommandLineArgs {
  fn new() -> CommandLineArgs {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let input_file = args.get(1).map_or("input.txt", |s| &s[..]).to_string();

    CommandLineArgs {
      input_file
    }
  }
}

fn count_hops(map: &HashMap<Rc<&str>, Node>, initial_key: &Rc<&str>, first_line: &String) -> u64 {
  let first_line_len = &first_line.len();
  let mut total = 0;
  let mut key = initial_key;
  let mut i = 0;

  loop {
    let direction = first_line.chars().nth(i).unwrap();
    let node = map.get(key).unwrap();
    let next_key = if direction == 'L' { &node.L } else { &node.R };

    total += 1;
    i = (i + 1) % first_line_len;

    if next_key.ends_with('Z') { break; }
    key = next_key;
    
  }

  total
}

fn main() -> std::io::Result<()> {
  let args = CommandLineArgs::new();
  dbg!(&args);
  let file_string = read_input_file(args.input_file.as_str());
  let binding = file_string.unwrap();
  let mut lines = binding.lines();
  let first_line = lines.next().unwrap().to_string();
  lines.next(); // removes empty line

  let mut map: HashMap<Rc<&str>, Node> = HashMap::new();
  let re = Regex::new(r"^(\w+)\s+=\s+\((\w+)\W+(\w+)").unwrap();
  let mut starting_nodes: Vec<Rc<&str>> = Vec::new();

  for line in lines {
    if line.is_empty() { break; }

    if let Some(caps) = re.captures(line) {
      let key = caps.get(1).unwrap().as_str();
      let l = caps.get(2).unwrap().as_str();
      let r = caps.get(3).unwrap().as_str();

      if key.ends_with('A') {
        starting_nodes.push(Rc::new(key));
      }

      map.insert(Rc::new(key), Node::new(Rc::new(l), Rc::new(r)));
    }
  }

  println!("Starting nodes: {:?}", starting_nodes);


  // iterate over starting nodes
  let all_counts: Vec<_> = starting_nodes.iter().map(|node_key| {
    let total = count_hops(&map, node_key, &first_line);
    println!("Node: {:?}, total: {:?}", &node_key, &total);

    total
  }).collect();

  let total_acm = all_counts.iter().fold(1, |acc, &x| lcm(acc, x));

  println!("Total ACM: {:?}", total_acm);
  Ok(())
}

// calculate greatest common divisor
fn gcd(a: u64, b: u64) -> u64 {
  if b == 0 { a } else { gcd(b, a % b) }
}

// calculate lowest common multiple
fn lcm(a: u64, b: u64) -> u64 {
  (a * b) / gcd(a, b)
}