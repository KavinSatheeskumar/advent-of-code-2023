use std::fs;

const PATTERNS: [(&str, &str); 10] = [
    ("0", "zero"),
    ("1", "one"),
    ("2", "two"),
    ("3", "three"),
    ("4", "four"),
    ("5", "five"),
    ("6", "six"),
    ("7", "seven"),
    ("8", "eight"),
    ("9", "nine")
];

fn check_start_numeric(s : &str, start : usize) -> i32 {
    let cropped_str = &s[start..];

    for idx in 0..(PATTERNS.len()) {
        let (s1, s2) = PATTERNS[idx];
        if cropped_str.starts_with(s1) || cropped_str.starts_with(s2) {
            return idx as i32;
        }
    }

    return -1;
}

fn main() {
    let contents: String = fs::read_to_string("./inp.in")
        .expect("Should have been able to read the file");

    let lines = contents.split('\n');

    let mut total = 0;
    let mut first_digit : i32 = 0;
    let mut last_digit : i32 = 0;

    for line in lines {
        for i in 0..line.len() {
            let val = check_start_numeric(line, i);
            if val != -1 {
                first_digit = val;
                break;
            }
        }

        for i in (0..line.len()).rev() {
            let val = check_start_numeric(line, i);
            if val != -1 {
                last_digit = val;
                break;
            }
        }

        total += format!("{}{}", first_digit, last_digit).parse::<i32>().expect("should be a number");
    }

    println!("With text:\n{total}");
}
