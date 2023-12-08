use std::collections::HashMap;
use std::fs;

fn load_path(file: &str) -> String {
    let lines = fs::read_to_string(file).expect("file not found");

    let split = lines.split('\n').collect::<Vec<_>>()[0].trim();
    String::from(split)
}

fn load_map(file: &str) -> HashMap<String, (String, String)> {
    let mut result = HashMap::new();
    let lines = fs::read_to_string(file).expect("file not found");
    for l in lines.split('\n').into_iter().skip(2) {
        if l.len() < 2 {
            continue;
        }

        let sp: Vec<&str> = l.split('=').collect();
        let label = sp[0].trim().to_string();
        let left = sp[1].trim()[1 .. 4].to_string();
        let right = sp[1].trim()[6 .. 9].to_string();
        result.insert(label, (left, right));

    }
    result
}

fn walk(map: &HashMap<String, (String, String)>, current: &String, remaining_steps: &String) -> (String, String) {
    let is_left = remaining_steps.chars().collect::<Vec<_>>()[0] == 'L';
    let result:String = if is_left {
        map.get(current).unwrap().0.clone()
    } else {
        map.get(current).unwrap().1.clone()
    };
    let steps_after_walk = remaining_steps[1..].to_string();
    (result, steps_after_walk)
}

fn run_round(map: &HashMap<String, (String, String)>, path: &String, start: &String) -> (String, u32) {
    let mut steps: u32 = 0;
    let mut remaining_steps = path.clone();
    let mut current: String = start.clone();
    while remaining_steps.len() > 0 {
        let walk_result = walk(map, &current, &remaining_steps);
        steps += 1;
        if walk_result.0.eq(&String::from("ZZZ")) {
            return (String::from("ZZZ"), steps);
        }
        remaining_steps = walk_result.1;
        current = walk_result.0;
    }
    (current, steps)
}

fn walk_from_to(map: &HashMap<String, (String, String)>, path: &String, from: &String, to: &String, is_p1: bool) -> u32 {
    let mut p1 = 0;
    let mut finish = false;
    let mut cur: String = from.clone();
    while !finish {
        let round_result: (String, u32) = run_round(&map, &path, &cur);
        if is_p1 && round_result.0.eq(to) {
            finish = true;
        } else if !is_p1 && round_result.0.chars().collect::<Vec<_>>()[2].eq(&'Z') {
            finish = true;
        }
        p1 += round_result.1;
        cur = round_result.0;
    }
    p1
}

fn find_all_a(map: &HashMap<String, (String, String)>) -> Vec<String> {
    map.keys().map(|s| s.clone())
    .filter(|s| s.chars().collect::<Vec<_>>()[2] == 'A')
    .collect::<Vec<_>>()
}

fn lcm(walks: &HashMap<String, u32>) -> u64 {
    let nums: Vec<u32> = walks.iter().map(|w| *w.1).collect();
    let mut lcm: u64 = nums[0] as u64;
    for n in nums {
        lcm = num::integer::lcm(lcm, n as u64);
    }
    lcm
}

fn main() {
    let file = "data/day8.txt";

    let map: HashMap<String, (String, String)> = load_map(file);
    let path = load_path(file);
    let p1 = walk_from_to(&map, &path, &"AAA".to_string(), &"ZZZ".to_string(), true);
    println!("p1: {}", p1);
    let mut walks: HashMap<String, u32> = HashMap::new();
    let all_steps_with_a: Vec<String> = find_all_a(&map);
    for a in all_steps_with_a {
        let walk_steps = walk_from_to(&map, &path, &a, &"!!!".to_string(), false);
        walks.insert(a.clone(), walk_steps);
    }
    let p2 = lcm(&walks);

    println!("p2: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_map() {
        let file = "data/day8_ex.txt";

        let map: HashMap<String, (String, String)> = load_map(file);
        assert_eq!(map.len(), 3);
        assert_eq!(*map.get("AAA").unwrap(), (String::from("BBB"), String::from("BBB")));
        assert_eq!(*map.get("BBB").unwrap(), (String::from("AAA"), String::from("ZZZ")));
        assert_eq!(*map.get("ZZZ").unwrap(), (String::from("ZZZ"), String::from("ZZZ")));
    }

    #[test]
    fn test_load_path() {
        let file = "data/day8_ex.txt";

        let path = load_path(file);
        assert_eq!(path, String::from("LLR"));
    }

    #[test]
    fn test_walk() {
        let file = "data/day8_ex.txt";

        let map: HashMap<String, (String, String)> = load_map(file);
        let path = "RL";
        let result = walk(&map, &"AAA".to_string(), &path.to_string());
        assert_eq!(result.0, "BBB".to_string());
        assert_eq!(result.1, "L".to_string());
        let path = "L";
        let result = walk(&map, &"BBB".to_string(), &path.to_string());
        assert_eq!(result.0, "AAA".to_string());
        assert_eq!(result.1, "".to_string());
        assert_eq!(result.1.len(), 0);
    }
}