use std::{cmp::max, fs};
struct Round {
    blue: i32,
    green: i32,
    red: i32,
}
struct Game {
    id: i32,
    rounds: Vec<Round>,
}

fn parse_line(line: &str) -> Game {
    let splited: Vec<&str> = line.split(':').collect();
    let id: i32 = splited[0][5..].parse().expect("Cannot parse ID");
    let mut result = Game {
        id,
        rounds: Vec::new(),
    };
    for r in splited[1].split(';') {
        let mut red: i32 = 0;
        let mut green: i32 = 0;
        let mut blue: i32 = 0;

        //split colors, and they are not ordered!!!
        for c in r.split(',') {
            let count_color: Vec<&str> = c.trim().split(' ').collect();
            match count_color[1] {
                "red" => red += count_color[0].parse::<i32>().expect("Cannot parse red."),
                "green" => green += count_color[0].parse::<i32>().expect("Cannot parse green."),
                "blue" => blue += count_color[0].parse::<i32>().expect("Cannot parse blue."),
                &_ => println!("Not Possible!"),
            }
        }
        let round = Round { red, green, blue };
        result.rounds.push(round);
    }

    result
}

fn game_possible(game: &Game) -> bool {
    let red_limit = 12;
    let green_limit = 13;
    let blue_limit = 14;

    for g in &game.rounds {
        if g.red > red_limit || g.blue > blue_limit || g.green > green_limit {
            return false;
        }
    }

    true
}

fn game_power(game: &Game) -> i32 {
    let mut red_min = 0;
    let mut blue_min = 0;
    let mut green_min = 0;
    for r in &game.rounds {
        red_min = max(red_min, r.red);
        blue_min = max(blue_min, r.blue);
        green_min = max(green_min, r.green);
    }

    red_min * blue_min * green_min
}

fn main() {
    let data = fs::read_to_string("data/day2.txt").expect("Cannot read file.");

    let mut p1_result = 0;
    let mut p2_result = 0;

    for line in data.split('\n') {
        if !line.contains("Game") {
            println!("WARN: invalid line {}", line);
            continue;
        }
        let game = parse_line(line);
        if game_possible(&game) {
            p1_result += game.id;
        }
        p2_result += game_power(&game);
    }

    println!("p1: {}", p1_result);
    println!("p2: {}", p2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let result = parse_line(line);
        assert_eq!(result.id, 1);
        assert_eq!(result.rounds.len(), 3);
        assert_eq!(result.rounds[0].blue, 3);
        assert_eq!(result.rounds[0].red, 4);
        assert_eq!(result.rounds[0].green, 0);
        assert_eq!(result.rounds[1].blue, 6);
        assert_eq!(result.rounds[1].red, 1);
        assert_eq!(result.rounds[1].green, 2);
        assert_eq!(result.rounds[2].blue, 0);
        assert_eq!(result.rounds[2].red, 0);
        assert_eq!(result.rounds[2].green, 2);
        let line = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        let result = parse_line(line);
        assert_eq!(result.id, 2);
        assert_eq!(result.rounds.len(), 3);
        assert_eq!(result.rounds[0].blue, 1);
        assert_eq!(result.rounds[0].red, 0);
        assert_eq!(result.rounds[0].green, 2);
        assert_eq!(result.rounds[1].blue, 4);
        assert_eq!(result.rounds[1].red, 1);
        assert_eq!(result.rounds[1].green, 3);
        assert_eq!(result.rounds[2].blue, 1);
        assert_eq!(result.rounds[2].red, 0);
        assert_eq!(result.rounds[2].green, 1);
        let line = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let result = parse_line(line);
        assert_eq!(result.id, 3);
        assert_eq!(result.rounds.len(), 3);
        assert_eq!(result.rounds[0].blue, 6);
        assert_eq!(result.rounds[0].red, 20);
        assert_eq!(result.rounds[0].green, 8);
        assert_eq!(result.rounds[1].blue, 5);
        assert_eq!(result.rounds[1].red, 4);
        assert_eq!(result.rounds[1].green, 13);
        assert_eq!(result.rounds[2].blue, 0);
        assert_eq!(result.rounds[2].red, 1);
        assert_eq!(result.rounds[2].green, 5);
        let line = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
        let result = parse_line(line);
        assert_eq!(result.id, 4);
        assert_eq!(result.rounds.len(), 3);
        assert_eq!(result.rounds[0].blue, 6);
        assert_eq!(result.rounds[0].red, 3);
        assert_eq!(result.rounds[0].green, 1);
        assert_eq!(result.rounds[1].blue, 0);
        assert_eq!(result.rounds[1].red, 6);
        assert_eq!(result.rounds[1].green, 3);
        assert_eq!(result.rounds[2].blue, 15);
        assert_eq!(result.rounds[2].red, 14);
        assert_eq!(result.rounds[2].green, 3);
        let line = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = parse_line(line);
        assert_eq!(result.id, 5);
        assert_eq!(result.rounds.len(), 2);
        assert_eq!(result.rounds[0].blue, 1);
        assert_eq!(result.rounds[0].red, 6);
        assert_eq!(result.rounds[0].green, 3);
        assert_eq!(result.rounds[1].blue, 2);
        assert_eq!(result.rounds[1].red, 1);
        assert_eq!(result.rounds[1].green, 2);
    }

    #[test]
    fn test_game_possible() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let result = parse_line(line);
        assert!(game_possible(&result));
        let line = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        let result = parse_line(line);
        assert!(game_possible(&result));
        let line = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let result = parse_line(line);
        assert!(!game_possible(&result));
        let line = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
        let result = parse_line(line);
        assert!(!game_possible(&result));
        let line = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = parse_line(line);
        assert!(game_possible(&result));
    }

    #[test]
    fn test_game_power() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let result = parse_line(line);
        assert_eq!(game_power(&result), 48);
        let line = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        let result = parse_line(line);
        assert_eq!(game_power(&result), 12);
        let line = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let result = parse_line(line);
        assert_eq!(game_power(&result), 1560);
        let line = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
        let result = parse_line(line);
        assert_eq!(game_power(&result), 630);
        let line = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = parse_line(line);
        assert_eq!(game_power(&result), 36);
    }
}
