use std::{collections::HashMap, fs};

fn get_cards_for_line(line: &str) -> i32 {
    let colon_split: Vec<&str> = line.split(':').collect();
    let nums: Vec<&str> = colon_split[1].trim().split("|").collect();
    let winning_nums: Vec<i32> = nums[0]
        .trim()
        .split_whitespace()
        .map(|n| n.parse::<i32>().expect(""))
        .collect();
    let my_nums: Vec<i32> = nums[1]
        .trim()
        .split_whitespace()
        .map(|n| n.parse::<i32>().expect(""))
        .collect();
    let mut result: i32 = 0;

    for n in my_nums {
        if winning_nums.contains(&n) {
            result += 1;
        }
    }
    result
}

fn get_point_for_line(line: &str) -> i32 {
    let result = get_cards_for_line(line);
    if result == 0 {
        return 0;
    }
    let base: i32 = 2;
    base.pow((result - 1).try_into().unwrap())
}

fn get_repeat_counts(line_no: i32, repeat_counts: &HashMap<i32, i32>) -> i32 {
    if !repeat_counts.contains_key(&line_no) {
        return 1;
    }
    *repeat_counts.get(&line_no).expect("")
}

fn update_repeat_counts(line_no: i32, card_count: i32, repeat_counts: &mut HashMap<i32, i32>) {
    let first_card = line_no + 1;
    let last_card = line_no + card_count;
    for c in first_card..last_card + 1 {
        if !repeat_counts.contains_key(&c) {
            repeat_counts.insert(c, 2);
        } else {
            let cur_count = repeat_counts.get(&c).expect("");
            repeat_counts.insert(c, cur_count + 1);
        }
    }
}

fn fill_zero(last_no: i32, repeat_counts: &mut HashMap<i32, i32>) {
    for i in 1..last_no + 1 {
        if !repeat_counts.contains_key(&i) {
            repeat_counts.insert(i, 1);
        }
    }
}

fn p2(data: String) -> i32 {
    let mut repeat_counts: HashMap<i32, i32> = HashMap::new();
    repeat_counts.insert(1, 1);

    let mut line_no = 0;

    for line in data.split('\n') {
        if line.len() < 4 {
            continue; //empty
        }
        line_no += 1;
        let card_count = get_cards_for_line(line);
        for _count in 1..get_repeat_counts(line_no, &repeat_counts) + 1 {
            update_repeat_counts(line_no, card_count, &mut repeat_counts);
        }
    }

    fill_zero(line_no, &mut repeat_counts);
    repeat_counts.values().sum()
}

fn main() {
    let data = fs::read_to_string("data/day4.txt").expect("Cannot read file.");
    let mut p1_result: i32 = 0;

    for line in data.split('\n') {
        if line.len() < 4 {
            continue; //empty
        }
        p1_result += get_point_for_line(line);
    }

    println!("p1: {}", p1_result);
    println!("p2: {}", p2(data));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_point_for_line() {
        let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let result = get_point_for_line(line);
        assert_eq!(result, 8);
        let line = "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19";
        let result = get_point_for_line(line);
        assert_eq!(result, 2);
        let line = "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1";
        let result = get_point_for_line(line);
        assert_eq!(result, 2);
        let line = "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83";
        let result = get_point_for_line(line);
        assert_eq!(result, 1);
        let line = "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36";
        let result = get_point_for_line(line);
        assert_eq!(result, 0);
        let line = "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = get_point_for_line(line);
        assert_eq!(result, 0);
        let line = "Card  36:  4 60 58 47 12 77 94 89  1 82 |  8  5  2 45 10 89 64 30 95 60 20 61 66 74  7 31  4 83 62 36 25 40 33 87 93";
        let result = get_point_for_line(line);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_get_cards_for_line() {
        let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let result = get_cards_for_line(line);
        assert_eq!(result, 4);
        let line = "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19";
        let result = get_cards_for_line(line);
        assert_eq!(result, 2);
        let line = "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1";
        let result = get_cards_for_line(line);
        assert_eq!(result, 2);
        let line = "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83";
        let result = get_cards_for_line(line);
        assert_eq!(result, 1);
        let line = "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36";
        let result = get_cards_for_line(line);
        assert_eq!(result, 0);
        let line = "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = get_cards_for_line(line);
        assert_eq!(result, 0);
        let line = "Card  36:  4 60 58 47 12 77 94 89  1 82 |  8  5  2 45 10 89 64 30 95 60 20 61 66 74  7 31  4 83 62 36 25 40 33 87 93";
        let result = get_cards_for_line(line);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_get_card_counts() {
        let mut repeat_counts: HashMap<i32, i32> = HashMap::new();
        repeat_counts.insert(1, 3);
        assert_eq!(get_repeat_counts(1, &repeat_counts), 3);
        assert_eq!(get_repeat_counts(10, &repeat_counts), 1);
    }

    #[test]
    fn test_update_repeat_counts() {
        let mut repeat_counts: HashMap<i32, i32> = HashMap::new();
        update_repeat_counts(1, 3, &mut repeat_counts);
        assert_eq!(get_repeat_counts(2, &repeat_counts), 2);
        assert_eq!(get_repeat_counts(3, &repeat_counts), 2);
        assert_eq!(get_repeat_counts(4, &repeat_counts), 2);
        update_repeat_counts(2, 1, &mut repeat_counts);
        assert_eq!(get_repeat_counts(3, &repeat_counts), 3);
        assert_eq!(get_repeat_counts(4, &repeat_counts), 2);
    }

    #[test]
    fn test_p2() {
        let data = fs::read_to_string("data/day4_ex.txt").expect("Cannot read file.");
        let result = p2(data);
        assert_eq!(result, 30);
    }

    #[test]
    fn test_fill_zero() {
        let mut repeat_counts: HashMap<i32, i32> = HashMap::new();
        update_repeat_counts(1, 3, &mut repeat_counts);
        assert_eq!(get_repeat_counts(2, &repeat_counts), 2);
        assert_eq!(get_repeat_counts(3, &repeat_counts), 2);
        assert_eq!(get_repeat_counts(4, &repeat_counts), 2);
        fill_zero(6, &mut repeat_counts);
        assert!(repeat_counts.contains_key(&5));
        assert!(repeat_counts.contains_key(&6));
    }
}
