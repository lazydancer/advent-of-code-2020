use std::ops::RangeInclusive;

struct PasswordPolicy {
    byte: u8,
    range: RangeInclusive<usize>,
}

impl PasswordPolicy {
    fn is_valid(&self, password: &str) -> bool {
        self.range.contains(
            &password
                .as_bytes()
                .iter()
                .copied()
                .filter(|&b| b == self.byte)
                .count(),
        )
    }
}

fn parse_line(s: &str) -> (PasswordPolicy, String){
    todo!();
}


fn main() {
    let count = include_str!("../input.txt")
        .lines()
        .map(parse_line)
        .filter(|(policy, password)| policy.is_valid(password))
        .count();
    println!("{} passwrods are valid", count);
}
