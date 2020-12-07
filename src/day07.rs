use std::collections::HashMap;

pub fn get_rules() -> HashMap<String, Vec<(usize, String)>> {
  let input_raw = include_str!("input07.txt");

  let lines: Vec<String> = input_raw.lines()
                                      .map(|line| line.replace(".", ""))
                                      .map(|line| line.replace("bags", "bag"))
                                      .collect();
  
  let mut rules: HashMap<String, Vec<(usize, String)>> = HashMap::new();

  for line in lines {
    let parts: Vec<&str> = line.split("contain").collect();
    let (from, to) = (parts[0], parts[1]);
    let to_parts: Vec<(usize, String)> = to
                                  .split(",")
                                  .filter(|s| !s.contains("no other"))
                                  .map(|s| s.trim().to_string())
                                  .map(|s| (s.chars().next().unwrap().to_digit(10).unwrap() as usize, s[1..].trim().to_string()))
                                  .collect();
    rules.insert(from.trim().to_string(), to_parts);
  }

  rules
}

pub fn can_contain(bag:&String, rules:&HashMap<String, Vec<(usize, String)>>) -> bool { 
  let children = rules.get(bag).unwrap();

  for child in children {
    let (_num, to) = child;
    if to == &String::from("shiny gold bag") {
      return true
    }

    if can_contain(to, rules) {
      return true
    }
  }

  false
}

pub fn get_number_of_bags(bag:&String, rules:&HashMap<String, Vec<(usize, String)>>, multiplier:usize) -> usize { 
  let children = rules.get(bag).unwrap();

  let mut answer = 0;

  for child in children {
    let (num, to) = child;
    answer += num * multiplier;
    answer += get_number_of_bags(to, rules, multiplier * num);
  }

  answer
}

pub fn part1() {
  let rules = get_rules();

  let mut answer = 0;

  for rule in &rules {
    let (from, _to) = rule;
    if can_contain(from, &rules) {
      answer += 1;
    }
  }

  println!("Day 07 Part1: {}", answer);
}

pub fn part2() {
  let rules = get_rules();

  let answer = get_number_of_bags(&String::from("shiny gold bag"), &rules, 1);

  println!("Day 07 Part2: {}", answer);
}