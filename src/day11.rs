pub fn get_map() -> Vec<Vec<char>> {
  let input_raw = include_str!("input11.txt");

  let map:Vec<Vec<char>> = input_raw.lines()
          .map(|s| s.trim().to_string())
          .map(|s| s.chars().collect())
          .collect();

  map
}

pub fn part1() {
  let mut map = get_map();
  let w = map[0].len();
  let h = map.len();
  let d:Vec<(isize, isize)> = vec![(-1, 0), (-1, -1), (0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1)];

  loop {
    let mut new_map = vec![vec!['.'; w]; h];
    let mut changed = 0;

    for row in 0..h {
      for col in 0..w {
        let old = map[row][col];
        let mut o = 0;
        for (dx, dy) in &d {
          let nx = col as isize + dx;
          let ny = row as isize + dy;

          if nx < 0 || ny < 0 || nx >= w as isize || ny >= h as isize {
            continue;
          }

          if map.get(ny as usize).unwrap().get(nx as usize).unwrap() == &'#' {
            o += 1;
          }
        }

        if old == 'L' && o == 0 {
          new_map[row][col] = '#';
          changed += 1;
        } else if old == '#' && o >= 4 {
          new_map[row][col] = 'L';
          changed += 1;
        } else {
          new_map[row][col] = old;
        }
      }
    }

    if changed == 0 {
      map = new_map;
      break;
    }

    map = new_map;
  }

  let mut answer = 0;
  for row in &map {
    for col in row {
      if col == &'#' {
        answer += 1;
      }
    }
  }

  println!("Day 11 Part1: {}", answer);
}

pub fn part2() {
  let mut map = get_map();
  let w = map[0].len();
  let h = map.len();
  let d:Vec<(isize, isize)> = vec![(-1, 0), (-1, -1), (0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1)];

  loop {
    let mut new_map = vec![vec!['.'; w]; h];
    let mut changed = 0;

    for row in 0..h {
      for col in 0..w {
        let old = map[row][col];
        let mut o = 0;
        for (dx, dy) in &d {
          let mut nx = col as isize;
          let mut ny = row as isize;

          loop {
            nx += dx;
            ny += dy;

            if nx < 0 || ny < 0 || nx >= w as isize || ny >= h as isize {
              break;
            }

            if map.get(ny as usize).unwrap().get(nx as usize).unwrap() == &'#' {
              o += 1;
              break;
            } else if map.get(ny as usize).unwrap().get(nx as usize).unwrap() == &'L' {
              break;
            }
          }
        }

        if old == 'L' && o == 0 {
          new_map[row][col] = '#';
          changed += 1;
        } else if old == '#' && o >= 5 {
          new_map[row][col] = 'L';
          changed += 1;
        } else {
          new_map[row][col] = old;
        }
      }
    }

    if changed == 0 {
      map = new_map;
      break;
    }

    map = new_map;
  }

  let mut answer = 0;
  for row in &map {
    for col in row {
      if col == &'#' {
        answer += 1;
      }
    }
  }

  println!("Day 11 Part2: {}", answer);
}
