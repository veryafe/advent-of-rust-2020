pub fn get_instructions() -> Vec<(String, isize, bool)> {
  let input_raw = include_str!("input08.txt");

  input_raw.lines()
           .map(|s| (s[0..3].to_string(), s[4..].parse().unwrap(), false))
           .collect()
}

pub fn part1() {
  let mut lines = get_instructions();
  
  let mut acc:isize = 0;
  let mut i:isize = 0;

  loop {
    let ref mut line = lines[i as usize];

    if line.2 {
      break;
    }

    if line.0 == "acc" {
      acc += line.1;
      i += 1
    } else if line.0 == "jmp" {
      i += line.1;
    } else {
      i += 1
    }

    line.2 = true;
  }

  println!("Day 08 Part1: {}", acc);
}

pub fn part2() {
 
  for change in 0..get_instructions().len() {
    let mut lines = get_instructions();
    let mut acc:isize = 0;
    let mut i:isize = 0;
    let mut looping = false;

    loop {
      if i == lines.len() as isize {
        break;
      }
      let ref mut line = lines[i as usize];

      if line.2 {
        looping = true;
        break;
      }

      let mut cmd = line.0.to_string();
      if i as usize == change {
        if cmd == "nop" {
          cmd = "jmp".to_string();
        } else if cmd == "jmp" {
          cmd = "nop".to_string();
        }
      }

      if cmd == "acc" {
        acc += line.1;
        i += 1
      } else if cmd == "jmp" {
        i += line.1;
      } else {
        i += 1
      }

      line.2 = true;
    }

    if !looping {
      println!("Day 08 Part2: {}", acc);
    }
  }   
}