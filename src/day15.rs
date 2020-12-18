use std::collections::HashMap;

fn get_after(iterations:isize) -> isize {
  let mut numbers:HashMap<isize, (isize, isize)> = HashMap::new();

  numbers.insert(0, (-1, 0));
  numbers.insert(8, (-1, 1));
  numbers.insert(15, (-1, 2));
  numbers.insert(2, (-1, 3));
  numbers.insert(12, (-1, 4));
  numbers.insert(1, (-1, 5));
  numbers.insert(4, (-1, 6));

  let mut last = 4;
  for t in 7..iterations {
    let (a, b) = numbers.get(&last).unwrap();
    if *a == -1 {
      let (_za, zb) = *numbers.get(&0).unwrap();
      numbers.insert(0, (zb, t));

      last = 0;
    } else {
      let diff = b - a;

      if numbers.contains_key(&diff) {
        let (_za, zb) = *numbers.get(&diff).unwrap();
        numbers.insert(diff, (zb, t));
      } else {
        numbers.insert(diff, (-1, t));
      }

      last = diff;
    }
  }

  last
}

pub fn part1() {
  println!("Day 15 Part1: {}", get_after(2020));
}

pub fn part2() {
  println!("Day 15 Part2: {}", get_after(30000000));
}
