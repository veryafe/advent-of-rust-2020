pub fn get_numbers() -> Vec<usize> {
  let input_raw = include_str!("input10.txt");

  let mut numbers:Vec<usize> = input_raw.lines()
          .map(|v| { v.parse().unwrap() })
          .collect();

  numbers.push(0);
  numbers.push(numbers.iter().max().unwrap()+3);

  numbers.sort();

  numbers
}

pub fn part1() {
  let numbers = get_numbers();

  let mut d1 = 0;
  let mut d3 = 0;
  let mut n = 0;
  for i in numbers {
    let d = i - n;
    if d == 1 {
      d1 += 1;
    } else if d == 3 {
      d3 += 1;
    }
    n = i;
  }

  println!("Day 10 Part1: {}", d1*d3);
}

pub fn part2() {
  let numbers = get_numbers();

  let mut ways = vec![0_usize; numbers.len()];

  ways[0] = 1;
  for i in 1..numbers.len() {
    ways[i] += ways[i-1];
    if i.checked_sub(2).is_some() && numbers[i]-numbers[i-2] <= 3 {
      ways[i] += ways[i-2]
    }
    if i.checked_sub(3).is_some() && numbers[i]-numbers[i-3] <= 3 {
      ways[i] += ways[i-3]
    }
  }
  
  println!("Day 10 Part2: {:?}", ways.last().unwrap());
}
