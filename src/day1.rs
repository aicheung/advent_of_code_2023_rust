use std::fs;

fn main() {
    let mut sum = 0;
    let mut line_first;
    let mut line_last;

    let data = fs::read_to_string("data/day1.txt").expect("Cannot read file.");

    //println!("{}", data);

    for line in data.split("\n") {
        line_first = -1;
        line_last = -1;
        for c in line.chars() {
            if c.is_digit(10) {
                let val: i32 = c.to_digit(10).expect("Cannot convert.").try_into().unwrap();
                //println!("conv {} {}", c, val);
                if line_first == -1 {
                    line_first = val;
                } else {
                    line_last = val;
                }
            }
        }
        if line_first == -1 {
            println!("no digit!");
            continue;
        }
        sum += line_first * 10
            + if line_last == -1 {
                line_first
            } else {
                line_last
            };
    }

    println!("p1: {}", sum);

    part2(&data);
}

fn has_valid_number(chars: &[char]) -> bool {
    let nums = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    // Convert the Vec<char> to a String
    let cur: String = chars.iter().collect();

    for &num in &nums {
        if cur.contains(num) {
            return true;
        }
    }
    false
}

fn convert_to_val(chars: &[char]) -> Option<i32> {
    let nums = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    // Convert the Vec<char> to a String
    let cur: String = chars.iter().collect();

    // Find the corresponding tuple in the nums array and return the value
    nums.iter()
        .find(|&&(word, _)| cur.contains(word))
        .map(|&(_, value)| value)
}

fn val_for_line(data: &str) -> (i32, i32) {
    let mut line_first = -1;
    let mut line_last = -1;
    let mut chars: Vec<char> = Vec::new();
    for c in data.chars() {
        if c.is_digit(10) {
            chars.clear();
            let val: i32 = c.to_digit(10).expect("Cannot convert.").try_into().unwrap();
            //println!("conv {} {}", c, val);
            if line_first == -1 {
                line_first = val;
            } else {
                line_last = val;
            }
        } else {
            //char
            chars.push(c);
            if has_valid_number(&chars) {
                let val = convert_to_val(&chars).expect("Cannot convert.");
                let last_char = chars.last().expect("").clone();
                chars.clear();
                chars.push(last_char);
                if line_first == -1 {
                    line_first = val;
                } else {
                    line_last = val;
                }
            }
        }
    }
    
    (line_first, line_last)
}

fn part2(data: &String) {
    let mut sum = 0;

    for line in data.split("\n") {
        let result = val_for_line(&line);
        let line_first:i32 = result.0;
        let line_last:i32 = result.1;
        if line_first == -1 {
            println!("no digit found: {}", line);
            continue;
        }
        if line_last == -1 {
            println!("WARN: no last digit (first digit {}): {}", line_first, line);
        }
        sum += line_first * 10
            + if line_last == -1 {
                line_first
            } else {
                line_last
            };
    }

    println!("p2: {}", sum);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_val_for_line() {
        let line = "63hbdkxljlq";
        let result = val_for_line(line);
        assert_eq!(result.0, 6);
        assert_eq!(result.1, 3);

        let line = "26sixpzpsixtwozqff";
        let result = val_for_line(line);
        assert_eq!(result.0, 2);
        assert_eq!(result.1, 2);

        let line = "9oneaaatwobbbthree1";
        let result = val_for_line(line);
        assert_eq!(result.0, 9);
        assert_eq!(result.1, 1);

        let line = "oneaaatwobbbthree";
        let result = val_for_line(line);
        assert_eq!(result.0, 1);
        assert_eq!(result.1, 3);

        let line = "aaaonebbb";
        let result = val_for_line(line);
        assert_eq!(result.0, 1);
        assert_eq!(result.1, -1);

        let line = "two";
        let result = val_for_line(line);
        assert_eq!(result.0, 2);
        assert_eq!(result.1, -1);

        let line = "q8bfhspkgmsevenninevdqmlzxznhmdlg";
        let result = val_for_line(line);
        assert_eq!(result.0, 8);
        assert_eq!(result.1, 9);

        let line = "a2sev8en";
        let result = val_for_line(line);
        assert_eq!(result.0, 2);
        assert_eq!(result.1, 8);

        let line = "two1nine";
        let result = val_for_line(line);
        assert_eq!(result.0, 2);
        assert_eq!(result.1, 9);
        let line = "eightwothree";
        let result = val_for_line(line);
        assert_eq!(result.0, 8);
        assert_eq!(result.1, 3);
        let line = "abcone2threexyz";
        let result = val_for_line(line);
        assert_eq!(result.0, 1);
        assert_eq!(result.1, 3);
        let line = "xtwone3four";
        let result = val_for_line(line);
        assert_eq!(result.0, 2);
        assert_eq!(result.1, 4);
        let line = "4nineeightseven2";
        let result = val_for_line(line);
        assert_eq!(result.0, 4);
        assert_eq!(result.1, 2);
        let line = "zoneight234";
        let result = val_for_line(line);
        assert_eq!(result.0, 1);
        assert_eq!(result.1, 4);
        let line = "7pqrstsixteen";
        let result = val_for_line(line);
        assert_eq!(result.0, 7);
        assert_eq!(result.1, 6);
        let line = "bfdvsdftwonevxcvv";
        let result = val_for_line(line);
        assert_eq!(result.0, 2);
        assert_eq!(result.1, 1);
        let line = "agveightwodfvdfv";
        let result = val_for_line(line);
        assert_eq!(result.0, 8);
        assert_eq!(result.1, 2);

    }
}
