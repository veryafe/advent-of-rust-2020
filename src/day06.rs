use std::collections::HashSet;
use std::collections::HashMap;

pub fn get_groups() -> Vec<Vec<String>> {
  let input_raw = include_str!("input06.txt");

  let mut lines: Vec<&str> = input_raw.lines()
                                      .collect();
  lines.push("");

  let mut a: Vec<Vec<String>> = [].to_vec();
  let mut answers: Vec<String> = [].to_vec();

  for line in lines {
    if line.is_empty() {
      a.push(answers);
      answers = [].to_vec();
      continue;
    }

    answers.push(line.to_string());
  }

  a
}

pub fn part1() {
  let mut answer = 0;
  let groups = get_groups();
  for group in groups {
    let mut questions = HashSet::new();
    for a in &group {
      for c in a.chars() {
        questions.insert(c);
      }
    }

    answer += questions.len();
  }

  println!("Day 06 Part1: {}", answer);
}

pub fn part2() {
  let mut answer = 0;
  let groups = get_groups();
  for group in groups {
    let mut counts:HashMap<char, usize> = HashMap::new();

    for a in &group {
      for c in a.chars() {
        let counter = counts.entry(c).or_insert(0);
        *counter += 1;
      }
    }

    for v in counts.values() {
      if *v == group.len() {
        answer += 1;
      }
    }
  }

  println!("Day 06 Part2: {}", answer);
}