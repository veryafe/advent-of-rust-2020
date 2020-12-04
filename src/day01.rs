use itertools::Itertools;

fn solve(n: usize) -> u32 {
  let input_raw = include_str!("input01.txt");

  let numbers: Vec<u32> = input_raw
                .lines()
                .map(|v| { v.parse().unwrap() })
                .collect();

  for v in numbers.iter().combinations(n) {
    let sum: u32 = v.iter().copied().sum();
    if sum == 2020 {
      let product: u32 = v.iter().copied().product();
      
      return product
    }
  }

  panic!("No solution found!")
}

pub fn part1() {
	let answer = solve(2);
  println!("Day 01 Part1: {}", answer);
}

pub fn part2() {
  let answer = solve(3);
  println!("Day 01 Part2: {}", answer);
}