use std::collections::HashMap;

pub fn get_map() -> Vec<(String, isize)> {
  let input_raw = include_str!("input12.txt");

  let map:Vec<(String, isize)> = input_raw.lines()
          .map(|s| (s[0..1].to_string(), s[1..].parse().unwrap()))
          .collect();

  map
}

fn move_in_direction(i:&String, a:isize, p:(isize, isize)) -> (isize, isize) {
  let mut moves:HashMap<String, (isize, isize)> = HashMap::new();
  moves.insert("N".to_string(), (0, -1));
  moves.insert("S".to_string(), (0, 1));
  moves.insert("E".to_string(), (1, 0));
  moves.insert("W".to_string(), (-1, 0));

  if moves.contains_key(i) {
    let (dx, dy) = moves.get(i).unwrap();
    return (p.0 + dx * a, p.1 + dy * a);
  }

  p
}

fn rotate(i:&String, a:isize, p:(isize, isize)) -> (isize, isize) {
  if i == "L" || i == "R" {
    if a == 180 {
      return (-p.0, -p.1);
    }
    if (i == "L" && a == 90) || (i == "R" && a == 270) {
      return (p.1, -p.0)
    }
    if (i == "R" && a == 90) || (i == "L" && a == 270) {
      return (-p.1, p.0)
    }
  }

  p
}

fn move_forward(i:&String, a:isize, p:(isize, isize), d:(isize, isize)) -> (isize, isize) {
  if i == "F" {
    return (p.0 + d.0 * a, p.1 + d.1 * a);
  }

  p
}

pub fn part1() {
  let map = get_map();
  let mut d:(isize, isize) = (1, 0);
  let mut sp:(isize, isize) = (0, 0);

  for (i, a) in map {
    sp = move_forward(&i, a, sp, d);
    sp = move_in_direction(&i, a, sp);
    d = rotate(&i, a, d);
  }

  println!("Day 12 Part1: {}", sp.0.abs() + sp.1.abs());
}

pub fn part2() {
  let map = get_map();
  let mut wp:(isize, isize) = (10, -1);
  let mut sp:(isize, isize) = (0, 0);

  for (i, a) in map {
    sp = move_forward(&i, a, sp, wp);
    wp = move_in_direction(&i, a, wp);
    wp = rotate(&i, a, wp);
  }

  println!("Day 12 Part2: {}", sp.0.abs() + sp.1.abs());
}
