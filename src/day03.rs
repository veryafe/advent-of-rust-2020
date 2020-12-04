
fn solve(dx: usize, dy: usize) -> u64 {
  let input_raw = include_str!("input03.txt");

  let map: Vec<&str> = input_raw
                .lines()
                .collect();

  let width = map[0].len();
  let height = map.len();

  let mut x: usize = 0;
  let mut y: usize = 0;
  let mut trees = 0;

  while y < height {
    if map[y].chars().nth(x % width).unwrap() == '#' {
      trees += 1;
    }
    x += dx;
    y += dy;
  }

  trees
}

pub fn part1() {
	let answer = solve(3, 1);
  println!("Day 03 Part1: {}", answer);
}

pub fn part2() {
  let mut answer = 1;
  answer *= solve(1, 1);
  answer *= solve(3, 1);
  answer *= solve(5, 1);
  answer *= solve(7, 1);
  answer *= solve(1, 2);
  println!("Day 03 Part2: {}", answer);
}