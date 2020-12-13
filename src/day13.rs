fn egcd(a: i128, b: i128) -> (i128, i128, i128) {
  if a == 0 {
    (b, 0, 1)
  } else {
    let (g, x, y) = egcd(b%a, a);
    (g, y - (b/a) *x, x)
  }
}

fn mod_inv(x: i128, n: i128) -> i128 {
  let (g, x, _) = egcd(x, n);
  if g == 1 {
    return (x % n + n) % n;
  }
  panic!("No Inverse!");
}

fn chinese_remainder(mod_res: &Vec<(i128, i128)>) -> i128 {
  let mut prod = 1;
  for (m, _) in mod_res {
    prod *= m;
  }

  let mut n = 0;

  for (m, r) in mod_res {
    let p = prod / m;
    n += r * mod_inv(p, *m) * p
  }

  n % prod
}

fn get_data() -> (i128, Vec<i128>) {
  let input_raw = include_str!("input13.txt");

  let mut lines = input_raw.lines();

  let depart = lines.next().unwrap().parse().unwrap();

  let busses:Vec<i128> = lines.next().unwrap()
          .split(',')
          .map(|s| s.parse().unwrap_or(0))
          .collect();

  (depart, busses)
}


pub fn part1() {
  let (depart, busses) = get_data();

  let mut wait = i128::MAX;
  let mut answer = 0;

  for bus in busses {
    if bus == 0 {
      continue;
    }

    let d = depart / bus;
    let next = bus * (d+1);

    if next-depart < wait {
      wait = next-depart;
      answer = wait * bus;
    }
  }

  println!("Day 13 Part1: {}", answer);
}

pub fn part2() {
  let (_, busses) = get_data();
  let mut mod_rem: Vec<(i128, i128)> = Vec::new();
 
  for (i, bus) in busses.iter().enumerate() {
    if *bus == 0 {
      continue;
    }

    mod_rem.push((*bus, (*bus - i as i128) % *bus));
  }

  let answer = chinese_remainder(&mod_rem);
  println!("Day 13 Part2: {}", answer);
}
