fn parse_line(line:&str) -> (u64, u64, char, &str) {
  let v: Vec<&str> = line.split(|c| c == ' ' || c == '-' || c== ':')
      .filter(|s| !s.is_empty())
      .collect();

  (v[0].parse().unwrap(), v[1].parse().unwrap(), v[2].chars().next().unwrap(), v[3])
}

pub fn part1() {
  let mut answer = 0;
  for line in include_str!("input02.txt").lines() {
    let (f, t, c, pw) = parse_line(line);
    
    let o:u64 = pw.matches(c).count() as u64;
    if o >= f && o <= t {
      answer += 1;
    }
  }

  println!("Day 02 Part1: {}", answer);
}

pub fn part2() {
  let mut answer = 0;
  for line in include_str!("input02.txt").lines() {
    let (f, t, c, pw) = parse_line(line);

    let mut o:u64 = 0;
    if pw.chars().nth((f-1) as usize).unwrap() == c {
      o += 1
    }
    if pw.chars().nth((t-1) as usize).unwrap() == c {
      o += 1
    }

    if o == 1 {
      answer += 1;
    }
  }

  println!("Day 02 Part2: {}", answer);
}