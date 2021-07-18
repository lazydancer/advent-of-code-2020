use std::collections::HashMap;

fn parse_passport(s: &str) -> HashMap<&str, &str> {
    let mut passport = HashMap::new();

    for prop in s.split_whitespace() {
        let mut p = prop.split(':');

        passport.insert(p.next().unwrap(), p.next().unwrap());
    }

    passport
}


fn valid_passport(pass: &HashMap<&str, &str>) -> bool {
    let required = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    required.iter().all(|r| pass.contains_key(r))
}

fn valid_passport_part_2(pass: &HashMap<&str, &str>) -> bool {
    let required = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    if !required.iter().all(|r| pass.contains_key(r)) {
        return false
    }

    if let Ok(x) = pass["byr"].parse::<i32>() {
        if x < 1920 || x > 2002 {
            return false
        }
    } else {
        return false
    }

    if let Ok(x) = pass["iyr"].parse::<i32>() {
        if x < 2010 || x > 2020 {
            return false
        }
    } else {
        return false
    }

    if let Ok(x) = pass["eyr"].parse::<i32>() {
        if x < 2020 || x > 2030 {
            return false
        }
    } else {
        return false
    }

    let len = pass["hgt"].len();
    if let Ok(x) = pass["hgt"][..len-2].parse::<i32>() {
        let unit = &pass["hgt"][len-2..];

        match unit {
            "cm" => {
                if x < 150 || x > 193 {
                    return false
                }
            },
            "in" => {
                if x < 59 || x > 76 {
                    return false
                }
            },
            _ => {
                return false
            },
        }

    } else {
        return false
    }


    if pass["hcl"].len() != 7 {
        return false
    }

    if let Some(x) = pass["hcl"].chars().nth(0) {
        if x != '#' {
            return false
        }
    }

    for c in pass["hcl"][1..].chars() {
        match c {
            '0' |'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9'|'a'|'b'|'c'|'d'|'e'|'f' => {},
            _ => return false,
        }
    }

    if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&pass["ecl"]) {
        return false
    }


    if pass["pid"].len() != 9 {
        return false
    }
    if let Ok(x) = pass["pid"].parse::<i32>() {
        
    } else {
        return false
    }

    return true
}

fn main() {
    let passports: Vec<HashMap<&str, &str>> = include_str!("../input.txt").split("\n\n").map(parse_passport).collect();

    let valid: Vec<HashMap<&str, &str>> = passports.clone().into_iter().filter(valid_passport).collect();

    println!("part 1: {:?}", valid.len()); // 204

    let valid: Vec<HashMap<&str, &str>> = passports.clone().into_iter().filter(valid_passport_part_2).collect();

    println!("part 2: {:?}", valid.len()); // 179

}
