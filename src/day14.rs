use regex::Regex;
use std::collections::HashMap;

fn get_numbers_in_line(line: &str) -> Vec<usize> {
  lazy_static! {
      static ref RE: Regex = Regex::new(r"(\d+)").unwrap();
  }

  let mut numbers:Vec<usize> = Vec::new();
  for caps in RE.captures_iter(line) {
    numbers.push(caps.get(1).unwrap().as_str().parse().unwrap());
  }

  numbers
}

fn get_masked_number(n: usize, mask: &str) -> usize {
  let value = format!("{:036b}", n);

  let s = mask.chars().zip(value.chars()).map(|(mc, c)| {
    match mc {
      'X' => c,
      _ => mc
    }
  }).collect::<String>();

  usize::from_str_radix(&s, 2).unwrap()
}

fn get_map() -> Vec<&'static str> {
  let input_raw = include_str!("input14.txt");

  input_raw.lines().collect()
}

fn get_addresses(n: &str, mask: &str, res: Vec<char>, addr: &mut Vec<usize>) {
  let l = res.len();

  if l == 36 {
    addr.push(usize::from_str_radix(&res.iter().collect::<String>(), 2).unwrap());
    return;
  }

  let c = mask.chars().nth(l).unwrap();
  let nc = n.chars().nth(l).unwrap();
  match c {
    'X' => {
      let mut next = res.clone();
      next.push('0');
      get_addresses(n, mask, next, addr);
      let mut next = res.clone();
      next.push('1');
      get_addresses(n, mask, next, addr);
    },
    '1' => {
      let mut next = res.clone();
      next.push('1');      
      get_addresses(n, mask, next, addr);
    },
    '0' => {
      let mut next = res.clone();
      next.push(nc);
      get_addresses(n, mask, next, addr);
    },
    _ => {
      panic!();
    }
  }
}

pub fn part1() {
  let lines = get_map();
  let mut mem:HashMap<usize, usize> = HashMap::new();
  let mut mask = String::new();

  for line in lines {
    if line.starts_with("mask") {
      mask = line[7..].to_string();
    } else {
      let numbers = get_numbers_in_line(line);
      mem.insert(numbers[0], get_masked_number(numbers[1], &mask));
    }
  }

  println!("Day 14 Part1: {}", mem.values().sum::<usize>());
}

pub fn part2() {
  let lines = get_map();
  let mut mem:HashMap<usize, usize> = HashMap::new();
  let mut mask = String::new();

  for line in lines {
    if line.starts_with("mask") {
      mask = line[7..].to_string();
    } else {
      let numbers = get_numbers_in_line(line);
      let mut addr = vec![];
      get_addresses(format!("{:036b}", numbers[0]).as_str(), &mask, vec![], &mut addr);

      for a in addr {
        mem.insert(a, numbers[1]);
      }
    }
  }

  println!("Day 14 Part2: {}", mem.values().sum::<usize>());
}

