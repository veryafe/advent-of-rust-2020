
pub fn get_map() -> Vec<(usize, usize, usize)> {
  let input_raw = include_str!("input17.txt");

  let map:Vec<Vec<char>> = input_raw.lines().map(|s| s.chars().collect()).collect();
  let mut points:Vec<(usize, usize, usize)> = Vec::new();

  for y in 0..map.len() {
    for x in 0..map[0].len() {
      if map[y][x] == '#' {
        points.push((x as usize, y as usize, 0))
      }
    }
  }

  points
}

pub fn part1() {
  let points = get_map();

  let mut m = [[[false; 100]; 100]; 100];

  for (x, y, z) in points {
    m[x+50][y+50][z+50] = true;
  }

  for _ in 0..6 {
    let mut m2 = [[[false; 100]; 100]; 100];
    for mx in 1..100-1 {
      for my in 1..100-1 {
        for mz in 1..100-1 {
          let mut n = 0;
          for x in mx-1..=mx+1 {
            for y in my-1..=my+1 {
              for z in mz-1..=mz+1 {
                if (x, y, z) == (mx, my, mz) {
                  continue;
                }

                if m[x][y][z] {
                  n += 1;
                }
              }
            }
          }

          if m[mx][my][mz] {
            if n == 2 || n == 3 {
              m2[mx][my][mz] = true;
            }
          } else {
            if n == 3 {
              m2[mx][my][mz] = true;
            }
          }
          
        }
      }
    }

    m = m2;
  }
  
  let mut active = 0;
  for mx in 1..100-1 {
    for my in 1..100-1 {
      for mz in 1..100-1 {
        if m[mx][my][mz] {
          active += 1;
        }
      }
    }
  }

  println!("Day 17 Part1: {}", active);
}

pub fn part2() {
  let points = get_map();

  const S:usize = 35;

  let mut m = [[[[false; S]; S]; S]; S];

  for (x, y, z) in points {
    m[x+S/2][y+S/2][z+S/2][S/2] = true;
  }


  for _ in 0..6 {
    let mut m2 = [[[[false; S]; S]; S]; S];

    for mx in 1..S-1 {
      for my in 1..S-1 {
        for mz in 1..S-1 {
          for mw in 1..S-1 {
            let mut n = 0;
            for x in mx-1..=mx+1 {
              for y in my-1..=my+1 {
                for z in mz-1..=mz+1 {
                  for w in mw-1..=mw+1 {
                    if (x, y, z, w) == (mx, my, mz, mw) {
                      continue;
                    }

                    if m[x][y][z][w] {
                      n += 1;
                    }
                  }
                }
              }
            }

            if m[mx][my][mz][mw] {
              if n == 2 || n == 3 {
                m2[mx][my][mz][mw] = true;
              }
            } else {
              if n == 3 {
                m2[mx][my][mz][mw] = true;
              }
            }
          }
        }
      }
    }

    m = m2;
  }
  
  let mut active = 0;
  for mx in 0..S {
    for my in 0..S {
      for mz in 0..S {
        for mw in 0..S {
          if m[mx][my][mz][mw] {
            active += 1;
          }
        }
      }
    }
  }

  println!("Day 17 Part2: {}", active);
}