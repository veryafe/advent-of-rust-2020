pub fn get_numbers() -> Vec<usize> {
  let input_raw = include_str!("input09.txt");

  input_raw.lines()
          .map(|v| { v.parse().unwrap() })
          .collect()
}

pub fn part1() {
  let numbers = get_numbers();
  
  for i in 25..numbers.len() {
    let n = numbers[i];
    let mut found = false;

    for j in i-25..i {
      for k in j+1..i {
        if numbers[j] + numbers[k] == n {
          found = true;
          break
        }
      }
    }

    if !found {
      println!("Day 09 Part1: {}", n);
      break;
    }
  }
}

pub fn part2() {
  let numbers = get_numbers();
  let part1 = 1309761972;

  for i in 0..numbers.len() {
    let mut sum = numbers[i];
    let mut min = sum;
    let mut max = sum;

    for j in i+1..numbers.len() {
      sum += numbers[j];
      min = std::cmp::min(min, numbers[j]);
      max = std::cmp::max(max, numbers[j]);

      if sum == part1 {
        println!("Day 09 Part2: {}", min+max);
      } else if sum > part1 {
        break;
      }
    }
  }
}