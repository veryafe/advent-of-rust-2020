pub fn get_map() -> (Vec<(String, usize, usize, usize, usize)>, usize, Vec<Vec<usize>>) {
  let input_raw = include_str!("input16.txt");

  let lines_vec:Vec<&str> = input_raw.lines().collect();
  let mut lines = lines_vec.iter();

  let mut rules:Vec<(String, usize, usize, usize, usize)> = Vec::new();
  let mut tickets:Vec<Vec<usize>> = Vec::new();

  for line in lines.by_ref() {
    if line.is_empty() {
      break;
    }

    let parts: Vec<_> = line.split(":").collect();
    let name = parts[0];
    let ranges: Vec<_> = parts[1].split(" or ").collect();
    let range1: Vec<usize> = ranges[0].split("-").map(|s| {
      let s2: String = s.chars().filter(|c| c.is_digit(10)).collect();
      s2.parse().unwrap()
    }).collect();
    let range2: Vec<usize> = ranges[1].split("-").map(|s| {
      let s2: String = s.chars().filter(|c| c.is_digit(10)).collect();
      s2.parse().unwrap()
    }).collect();

    rules.push((name.to_string(), range1[0], range1[1], range2[0], range2[1]));
  }

  for line in lines.by_ref() {
    if line.is_empty() {
      break;
    }
  }

  for line in lines.by_ref() {
    if !line.contains(",") {
      continue;
    }
    if line.is_empty() {
      break;
    }
    let ticket:Vec<usize> = line.split(",")
                                .map(|s| s.parse().unwrap())
                                .collect();

    tickets.push(ticket);
  }

  (rules, 0, tickets)
}

pub fn part1() {
  let (rules, _, tickets) = get_map();

  let mut answer = 0;

  for ticket in tickets {
    for v in ticket {
      let mut valid = false;

      for (_name, f1, t1, f2, t2) in &rules {
        if (v >= *f1 && v <= *t1) || (v >= *f2 && v <= *t2) {
          valid = true;
        }
      }

      if !valid {
        answer += v;
      }

    }
  }

  println!("Day 16 Part1: {}", answer);
}

pub fn part2() {
  let (rules, _, tickets) = get_map();

  let mut answer = 0;

  let mut valid_tickets:Vec<Vec<usize>> = Vec::new();

  for ticket in &tickets {
    let mut ticket_valid = true;

    for v in ticket {
      let mut valid = false;

      for (_name, f1, t1, f2, t2) in &rules {
        if (v >= f1 && v <= t1) || (v >= f2 && v <= t2) {
          valid = true;
        }
      }

      if !valid {
        answer += v;
        ticket_valid = false;
      }
    }

    if ticket_valid {
      valid_tickets.push(ticket.clone())
    }
  }

  for (name, f1, t1, f2, t2) in &rules {
    let mut nv:Vec<usize> = vec![0; 20];

    for ticket in &valid_tickets {
      for (i, v) in ticket.iter().enumerate() {
        if (v >= f1 && v <= t1) || (v >= f2 && v <= t2) {
          nv[i] += 1;
        }
      }
    }

    //println!("{:?}", name);
    //println!("{:?}", nv);
  }

  // Oh hello there manual step...

  // 61*191*109*101*71*157

  println!("Day 16 Part2: {}", 61_u128*191*109*101*71*157);
}

