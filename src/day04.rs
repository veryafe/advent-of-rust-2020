use std::collections::HashMap;

fn get_valid_passports() -> Vec<HashMap<String, String>> {
  let input_raw = include_str!("input04.txt");

  let mut lines: Vec<&str> = input_raw
                .lines()
                .collect();
  lines.push("");

  let mut valid_passports:Vec<HashMap<String, String>> = vec![];

  let req_keys = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

  let mut p:HashMap<String, String> = HashMap::new();
  for line in lines {
      if line.is_empty() {

        let mut valid_passport = true;
        for key in req_keys.iter() {
          if !p.contains_key(&key.to_string()) {
            valid_passport = false;
          }
        }

        if valid_passport {
          valid_passports.push(p);
        }

        p = HashMap::new();

        continue;
      }

      let parts: Vec<&str> = line.split(' ')
                                 .collect();

      for part in parts {
        let keyvals: Vec<&str> = part.split(':')
                                     .collect();

        p.insert(String::from(keyvals[0]), String::from(keyvals[1]));
      }
  }

  valid_passports
}

pub fn part1() {
  let valid_passports = get_valid_passports();

  println!("Day 04 Part1: {}", valid_passports.len());
}

pub fn part2() {
  let passports = get_valid_passports();

  let mut valid_passports = 0;

  for passport in passports {
    //byr (Birth Year) - four digits; at least 1920 and at most 2002.
    let byr:u32 = passport.get("byr").unwrap().parse().unwrap();
    if byr < 1920 || byr > 2002 {
      continue;
    }

    //iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    let iyr:u32 = passport.get("iyr").unwrap().parse().unwrap();
    if iyr < 2010 || iyr > 2020 {
      continue;
    }

    //eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    let eyr:u32 = passport.get("eyr").unwrap().parse().unwrap();
    if eyr < 2020 || eyr > 2030 {
      continue;
    }

    //hgt (Height) - a number followed by either cm or in:
    //  If cm, the number must be at least 150 and at most 193.
    //  If in, the number must be at least 59 and at most 76.    
    let hgt = passport.get("hgt").unwrap();
    if hgt.contains("cm") {
      let height:u32 = hgt[0..hgt.len()-2].parse().unwrap();
      if height < 150 || height > 193 {
        continue;
      }
    } else if hgt.contains("in") {
      let height:u32 = hgt[0..hgt.len()-2].parse().unwrap();
      if height < 59 || height > 76 {
        continue;
      }
    } else {
      continue;
    }

    //hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    let mut hcl = passport.get("hcl").unwrap().chars();
    let hex_digits = ['0','1','2','3','4','5','6','7','8','9', 'a', 'b', 'c', 'd', 'e', 'f'];
    if hcl.next().unwrap() != '#' {
      continue;
    }
    let mut hcl_valid = true;
    for c in hcl {
      if !hex_digits.contains(&c) {
        hcl_valid = false;
      }
    }
    if !hcl_valid {
      continue;
    }

    //ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    let ecl = passport.get("ecl").unwrap();
    let eye_colors = ["amb","blu","brn","gry","grn","hzl","oth"];
    if !eye_colors.contains(&&ecl[..]) {
      continue;
    }

    //pid (Passport ID) - a nine-digit number, including leading zeroes.
    let pid = passport.get("pid").unwrap();
    if pid.len() != 9 {
      continue;
    }
    if !pid.parse::<u64>().is_ok() {
      continue;
    }

    //cid (Country ID) - ignored, missing or not.

    valid_passports += 1;
  }

  println!("Day 04 Part2: {}", valid_passports);
}