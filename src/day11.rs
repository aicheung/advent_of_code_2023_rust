use std::{
    cmp::{max, min},
    fs::*,
};

use num::abs;

fn expand_universe(file: &str, file_expanded: &str) -> (Vec<u64>, Vec<u64>) {
    let result = read_to_string(file).unwrap();
    let lines = result.split_whitespace().collect::<Vec<_>>();

    let length: u64 = lines[0].len() as u64;
    let mut i: u64 = 0;
    let mut j: u64 = 0;
    let mut empty_rows: Vec<u64> = Vec::new();
    let mut empty_cols: Vec<u64> = Vec::new();
    let mut filled_cols: Vec<u64> = Vec::new();
    let mut galaxies: Vec<(u64, u64)> = Vec::new();
    for l in lines {
        let mut empty = true;
        for c in l.chars() {
            if c == '#' {
                empty = false;
                filled_cols.push(j);
                galaxies.push((i, j));
            }
            j += 1;
        }

        if empty {
            empty_rows.push(i);
        }
        i += 1;
        j = 0;
    }

    for j in 0..length {
        if !filled_cols.contains(&j) {
            empty_cols.push(j);
        }
    }

    //print
    let mut output = String::new();

    for i in 0..length {
        if empty_rows.contains(&i) {
            let target_len = length + empty_cols.len() as u64;
            for _j in 0..target_len {
                output.push('.');
            }
            output.push('\n');
            for _j in 0..target_len {
                output.push('.');
            }
        } else {
            for j in 0..length {
                if empty_cols.contains(&j) {
                    //write twice
                    output.push_str("..");
                } else if galaxies.contains(&(i, j)) {
                    output.push('#');
                } else {
                    //normal space
                    output.push('.');
                }
            }
        }
        if i != length - 1 {
            output.push('\n');
        }
    }
    write(file_expanded, output).expect("Cannot write expanded");
    (empty_rows, empty_cols)
}

fn load_galaxies(file: &str) -> Vec<(u64, u64)> {
    let result = read_to_string(file).unwrap();
    let lines = result.split_whitespace().collect::<Vec<_>>();

    let mut i: u64 = 0;
    let mut j: u64 = 0;
    let mut galaxies: Vec<(u64, u64)> = Vec::new();
    for l in lines {
        for c in l.chars() {
            if c == '#' {
                galaxies.push((i, j));
            }
            j += 1;
        }

        i += 1;
        j = 0;
    }

    galaxies
}

fn get_dist(me: &(u64, u64), other: &(u64, u64)) -> u64 {
    abs(other.0 as i64 - me.0 as i64) as u64 + abs(other.1 as i64 - me.1 as i64) as u64
}

fn get_dist_p2(
    me: &(u64, u64),
    other: &(u64, u64),
    empty_rows: &Vec<u64>,
    empty_cols: &Vec<u64>,
    scale: u64,
) -> u64 {
    let from_row = min(me.0, other.0);
    let to_row = max(me.0, other.0);
    let from_col = min(me.1, other.1);
    let to_col = max(me.1, other.1);

    let empty_row_count = empty_rows
        .iter()
        .filter(|r| **r > from_row && **r < to_row)
        .count() as u64;
    let empty_col_count = empty_cols
        .iter()
        .filter(|c| **c > from_col && **c < to_col)
        .count() as u64;

    let vert_dist =
        empty_row_count * scale + abs(other.0 as i64 - me.0 as i64) as u64 - empty_row_count;
    let hori_dist =
        empty_col_count * scale + abs(other.1 as i64 - me.1 as i64) as u64 - empty_col_count;

    vert_dist + hori_dist
}

fn p1(galaxies: &Vec<(u64, u64)>) -> u64 {
    let mut result = 0;
    let r = Vec::new();
    let c = Vec::new();

    for g in galaxies {
        result += get_dist_to_others(g, galaxies, false, &r, &c, 1);
    }
    result / 2
}

fn get_dist_to_others(
    me: &(u64, u64),
    galaxies: &Vec<(u64, u64)>,
    is_p2: bool,
    empty_rows: &Vec<u64>,
    empty_cols: &Vec<u64>,
    scale: u64,
) -> u64 {
    galaxies
        .iter()
        .map(|g| {
            if is_p2 {
                get_dist_p2(me, g, empty_rows, empty_cols, scale)
            } else {
                get_dist(me, g)
            }
        })
        .sum()
}

fn p2(galaxies: &Vec<(u64, u64)>, empty_rows: &Vec<u64>, empty_cols: &Vec<u64>, scale: u64) -> u64 {
    let mut result = 0;

    for g in galaxies {
        result += get_dist_to_others(g, galaxies, true, empty_rows, empty_cols, scale);
    }
    result / 2
}

fn main() {
    let file = "data/day11.txt";
    let file_expanded = "data/day11_expanded.txt";
    let empty_data = expand_universe(file, file_expanded);
    let gx: Vec<(u64, u64)> = load_galaxies(file_expanded);

    println!("p1: {}", p1(&gx));

    //p2
    /*
       1. load orig galaxies
       2. find orig empty rows, cols (already in empty_data)
       3. accomodate empty rows cols in dist calculation
    */

    let gx: Vec<(u64, u64)> = load_galaxies(file);
    println!("p2: {}", p2(&gx, &empty_data.0, &empty_data.1, 1000000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand() {
        let file = "data/day11_ex.txt";
        let file_expanded = "data/day11_ex_expanded.txt";
        let empties = expand_universe(file, file_expanded);

        let result = read_to_string(file_expanded).unwrap();
        let lines = result.split_whitespace().collect::<Vec<_>>();
        assert_eq!(lines.len(), 12);
        assert_eq!(lines[0].len(), 13);

        assert_eq!(empties.0.len(), 2);
        assert_eq!(empties.0[0], 3);
        assert_eq!(empties.0[1], 7);
        assert_eq!(empties.1.len(), 3);
        assert_eq!(empties.1[0], 2);
        assert_eq!(empties.1[1], 5);
        assert_eq!(empties.1[2], 8);
    }

    #[test]
    fn test_load_galaxies() {
        let file = "data/day11_ex_expanded.txt";
        let result: Vec<(u64, u64)> = load_galaxies(file);

        assert_eq!(result.len(), 9);
        assert!(result.contains(&(0, 4)));
        assert!(result.contains(&(1, 9)));
        assert!(result.contains(&(2, 0)));
        assert!(result.contains(&(5, 8)));
        assert!(result.contains(&(6, 1)));
        assert!(result.contains(&(7, 12)));
        assert!(result.contains(&(10, 9)));
        assert!(result.contains(&(11, 0)));
        assert!(result.contains(&(11, 5)));
    }

    #[test]
    fn test_p2() {
        let file = "data/day11_ex.txt";
        let file_expanded = "data/day11_ex_expanded.txt";
        let empties = expand_universe(file, file_expanded);
        let gx: Vec<(u64, u64)> = load_galaxies(file);

        let scale = 10;
        let result = p2(&gx, &empties.0, &empties.1, scale);
        assert_eq!(result, 1030);
        let scale = 100;
        let result = p2(&gx, &empties.0, &empties.1, scale);
        assert_eq!(result, 8410);
    }

    #[test]
    fn test_p1() {
        let file = "data/day11_ex_expanded.txt";
        let result: Vec<(u64, u64)> = load_galaxies(file);

        let p1 = p1(&result);
        assert_eq!(p1, 374);
    }

    #[test]
    fn test_dist_to_others() {
        let r = Vec::new();
        let gx = vec![(0, 0), (0, 1), (1, 1)];
        assert_eq!(get_dist_to_others(&(0, 0), &gx, false, &r, &r, 1), 3);
    }

    #[test]
    fn test_get_dist_p2() {
        //empty case, same as p1
        let empty_rows = Vec::new();
        let empty_cols = Vec::new();

        let scale = 1;

        let me = (0, 0);
        let other = (2, 1);
        assert_eq!(
            get_dist_p2(&me, &other, &empty_rows, &empty_cols, scale),
            get_dist(&me, &other)
        );

        //p2 case
        let scale = 2;
        let empty_rows = vec![1];
        let empty_cols = vec![2];
        assert_eq!(get_dist_p2(&me, &other, &empty_rows, &empty_cols, scale), 4);
        let scale = 10;
        let empty_rows = vec![1];
        let empty_cols = vec![2];
        assert_eq!(
            get_dist_p2(&me, &other, &empty_rows, &empty_cols, scale),
            12
        );

        //horizontal
        let me = (0, 0);
        let other = (1, 2);
        let scale = 2;
        let empty_rows = vec![];
        let empty_cols = vec![1];
        assert_eq!(get_dist_p2(&me, &other, &empty_rows, &empty_cols, scale), 4);
        let scale = 10;
        let empty_rows = vec![];
        let empty_cols = vec![1];
        assert_eq!(
            get_dist_p2(&me, &other, &empty_rows, &empty_cols, scale),
            12
        );
    }

    #[test]
    fn test_dist() {
        let me = (0, 1);
        let other = (1, 1);
        assert_eq!(get_dist(&me, &other), 1);

        let me = (1, 1);
        let other = (0, 1);
        assert_eq!(get_dist(&me, &other), 1);

        let me = (0, 1);
        let other = (0, 3);
        assert_eq!(get_dist(&me, &other), 2);

        let me = (0, 3);
        let other = (0, 1);
        assert_eq!(get_dist(&me, &other), 2);

        let me = (0, 1);
        let other = (1, 2);
        assert_eq!(get_dist(&me, &other), 2);

        let me = (0, 1);
        let other = (1, 0);
        assert_eq!(get_dist(&me, &other), 2);

        let me = (1, 1);
        let other = (0, 0);
        assert_eq!(get_dist(&me, &other), 2);

        let me = (1, 1);
        let other = (0, 2);
        assert_eq!(get_dist(&me, &other), 2);

        let nine = (11, 5);
        let five = (6, 1);
        assert_eq!(get_dist(&nine, &five), 9);
        assert_eq!(get_dist(&five, &nine), 9);
        assert_eq!(get_dist(&five, &five), 0);
    }

    #[test]
    fn test_range() {
        let length = 10;
        let empty_cols = (0..length + 1).collect::<Vec<_>>();
        assert!(empty_cols.contains(&10));
    }
}
