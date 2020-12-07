pub fn convert_number(s: &str) -> usize {
  let n = s.replace("F", "0")
           .replace("B", "1")
           .replace("L", "0")
           .replace("R", "1");
  
  usize::from_str_radix(n.as_str(), 2).unwrap()
}

pub fn get_numbers() -> Vec<usize> {
  let input_raw = include_str!("input05.txt");

  input_raw.lines()
           .map(|s| convert_number(s))
           .collect()
}

pub fn part1() {
  let numbers = get_numbers();
  let max_number = numbers.iter().max().unwrap();

  println!("Day 05 Part1: {}", max_number);
}

pub fn part2() {
  let numbers = get_numbers();
  let min_number = numbers.iter().min().unwrap();
  let max_number = numbers.iter().max().unwrap();

  for i in *min_number..*max_number {
    if !numbers.contains(&i) {
        println!("Day 05 Part2: {}", i);
        break;
    }
  }
}