const INPUT: &str = include_str!("./day1.txt");

const LETTERS: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn a() {
    let sum = INPUT.lines().map(process_line).sum::<u32>();
    println!("Day 1a: {}", sum);
}

pub fn b() {
    let sum = INPUT.lines().map(process_line_b).sum::<u32>();
    println!("Day 1b: {}", sum);
}

fn process_line(line: &str) -> u32 {
    let first_digit = line.chars().find(char::is_ascii_digit).unwrap();
    let last_digit = line.chars().rev().find(char::is_ascii_digit).unwrap();
    let combined = format!("{}{}", first_digit, last_digit);
    let value = combined.parse::<u32>().unwrap();
    value
}

fn process_line_b(line: &str) -> u32 {
    fn find_digit(line: &str, iterator: impl Iterator<Item = usize>) -> u32 {
        for i in iterator {
            let line = &line[i..];
            let first_char = line.chars().next().unwrap();
            if first_char.is_ascii_digit() {
                return first_char.to_digit(10).unwrap();
            }

            for (j, letter) in LETTERS.iter().enumerate() {
                if line.starts_with(letter) {
                    return (j + 1) as u32;
                }
            }
        }

        unreachable!()
    }

    let first_digit = find_digit(line, (0..line.len()).into_iter());
    let last_digit = find_digit(line, (0..line.len()).into_iter().rev());
    let combined = format!("{}{}", first_digit, last_digit);
    let value = combined.parse::<u32>().unwrap();
    value
}
