use core::panic;
use std::collections::HashMap;
use std::fs;

fn get_line(
    grid: &HashMap<(u64, u64), char>,
    row: Option<u64>,
    col: Option<u64>,
) -> Option<String> {
    if row.is_some() && col.is_some() {
        panic!("Should not happen!");
    } else if row.is_some() {
        if !grid.contains_key(&(row.unwrap(), 0)) {
            return None;
        }
        let max_col = get_max(grid, true);
        let mut out = String::new();
        for c in 0..max_col + 1 {
            out.push(*grid.get(&(row.unwrap(), c)).unwrap());
        }

        return Some(out);
    } else {
        if !grid.contains_key(&(0, col.unwrap())) {
            return None;
        }
        let max_row = get_max(grid, false);
        let mut out = String::new();
        for r in 0..max_row + 1 {
            out.push(*grid.get(&(r, col.unwrap())).unwrap());
        }

        return Some(out);
    }
}

fn find_diff(me: &Vec<String>, other: &Vec<String>) -> u64 {
    let mut result = 0;

    for i in 0..me.len() {
        let me_line = &me[i];
        let other_line = &other[i];

        if me_line.len() == other_line.len() {
            for (j, c) in me_line.chars().enumerate() {
                if c != other_line.chars().collect::<Vec<_>>()[j] {
                    result += 1;
                }
            }
        }
    }

    result
}

fn scan(
    start: u64,
    is_left_to_right: bool,
    is_vertical: bool,
    grid: &HashMap<(u64, u64), char>,
    is_p2: bool,
) -> Option<u64> {
    if start < 1 {
        return None;
    }
    let mut self_lines: Vec<String> = Vec::new();
    let mut other_lines: Vec<String> = Vec::new();

    for i in 0..start {
        let mine_idx = if is_left_to_right {
            start.checked_sub(i + 1)
        } else {
            start.checked_add(i)
        };
        let other_idx = if is_left_to_right {
            start.checked_add(i)
        } else {
            start.checked_sub(i + 1)
        };

        if mine_idx.is_none() || other_idx.is_none() {
            break;
        }

        let mine = get_line(
            grid,
            if is_vertical {
                None
            } else {
                Some(mine_idx.unwrap())
            },
            if is_vertical {
                Some(mine_idx.unwrap())
            } else {
                None
            },
        );
        let other = get_line(
            grid,
            if is_vertical {
                None
            } else {
                Some(other_idx.unwrap())
            },
            if is_vertical {
                Some(other_idx.unwrap())
            } else {
                None
            },
        );

        if mine.is_none() || other.is_none() {
            break;
        }

        self_lines.push(mine.unwrap());
        other_lines.push(other.unwrap());
    }

    if !is_p2 {
        if self_lines.len() > 0 && other_lines.len() > 0 && self_lines.eq(&other_lines) {
            Some(start)
        } else {
            None
        }
    } else {
        if self_lines.len() > 0
            && other_lines.len() > 0
            && find_diff(&self_lines, &other_lines) == 1
        {
            Some(start)
        } else {
            None
        }
    }
}

fn get_max(grid: &HashMap<(u64, u64), char>, is_horizontal: bool) -> u64 {
    grid.iter()
        .map(|a| if is_horizontal { a.0 .1 } else { a.0 .0 })
        .max()
        .unwrap()
}

fn scan_grid(grid: &HashMap<(u64, u64), char>, is_p2: bool) -> (u64, bool) {
    let width = get_max(grid, true) + 1;
    let height = get_max(grid, false) + 1;

    //1. hori left to right
    for i in 1..width {
        let scan = scan(i, true, true, grid, is_p2);
        if scan.is_some() {
            return (scan.unwrap(), true);
        }
    }

    //2. hori right to left
    for i in width - 1..0 {
        let scan = scan(i, false, true, grid, is_p2);
        if scan.is_some() {
            return (scan.unwrap(), true);
        }
    }

    //3. vert up to down
    for i in 1..height {
        let scan = scan(i, true, false, grid, is_p2);
        if scan.is_some() {
            return (scan.unwrap(), false);
        }
    }

    //4. vert down to up
    for i in height - 1..0 {
        let scan = scan(i, false, false, grid, is_p2);
        if scan.is_some() {
            return (scan.unwrap(), false);
        }
    }

    println!("Nothing found!!!");
    (0, true)
}

fn load_map(lines: &Vec<&str>, grid: &mut HashMap<(u64, u64), char>) {
    let mut i = 0;
    let mut j = 0;
    for l in lines {
        if l.len() < 2 {
            continue;
        }
        for c in l.trim().chars() {
            grid.insert((i, j), c);
            j += 1;
        }

        i += 1;
        j = 0;
    }
}

fn calc_result(file: &str, is_p2: bool) -> u64 {
    let mut result = 0;

    let mut cur_lines = Vec::new();
    let mut cur_grid = HashMap::new();

    let s = fs::read_to_string(file).expect("");
    let lines = s.split('\n').collect::<Vec<_>>();

    for l in lines {
        if l.len() >= 2 {
            //valid line
            cur_lines.push(l.trim());
        } else {
            //calc, then clear for next round
            //println!("{:?}", cur_lines);
            load_map(&cur_lines, &mut cur_grid);
            let cur_result = scan_grid(&mut cur_grid, is_p2);

            if cur_result.1 {
                //col scan
                result += cur_result.0;
            } else {
                result += cur_result.0 * 100;
            }

            cur_lines.clear();
            cur_grid.clear();
        }
    }
    result
}

fn p2(file: &str) -> u64 {
    calc_result(file, true)
}

fn p1(file: &str) -> u64 {
    calc_result(file, false)
}

fn main() {
    let file = "data/day13.txt";
    println!("p1: {}", p1(file));
    println!("p2: {}", p2(file));
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_p1() {
        let file = "data/day13_ex3.txt";

        assert_eq!(p1(file), 405);
    }

    #[test]
    fn test_p2() {
        let file = "data/day13_ex3.txt";

        assert_eq!(p2(file), 400);
    }

    #[test]
    fn test_scan_grid() {
        let file = "data/day13_ex.txt";
        let s = fs::read_to_string(file).expect("");
        let lines = s.split('\n').collect::<Vec<_>>();

        let mut grid = HashMap::new();
        load_map(&lines, &mut grid);

        let result = scan_grid(&grid, false);
        assert_eq!(result.1, true);
        assert_eq!(result.0, 5);

        let file = "data/day13_ex2.txt";
        let s = fs::read_to_string(file).expect("");
        let lines = s.split('\n').collect::<Vec<_>>();

        let mut grid = HashMap::new();
        load_map(&lines, &mut grid);

        let result = scan_grid(&grid, false);
        assert_eq!(result.1, false);
        assert_eq!(result.0, 4);
    }

    #[test]
    fn test_grid_ex() {
        let file = "data/day13_ex2.txt";
        let s = fs::read_to_string(file).expect("");
        let lines = s.split('\n').collect::<Vec<_>>();

        let mut grid = HashMap::new();
        load_map(&lines, &mut grid);

        let result = scan(7, true, false, &grid, false);
        assert_eq!(result.is_none(), true);
    }

    #[test]
    fn test_day13_4() {
        let file = "data/day13_ex4.txt";
        assert_eq!(p1(file), 709);
        assert_eq!(p2(file), 1400);
    }

    #[test]
    fn test_day13_5() {
        let file = "data/day13_ex5.txt";
        assert_eq!(p1(file), 12);
    }

    #[test]
    fn test_get_max() {
        let file = "data/day13_ex.txt";
        let s = fs::read_to_string(file).expect("");
        let lines = s.split('\n').collect::<Vec<_>>();

        let mut grid = HashMap::new();

        load_map(&lines, &mut grid);

        let result = get_max(&grid, true);
        assert_eq!(result, 8);
        let result = get_max(&grid, false);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_load_map() {
        let file = "data/day13_ex.txt";
        let s = fs::read_to_string(file).expect("");
        let lines = s.split('\n').collect::<Vec<_>>();

        let mut grid = HashMap::new();

        load_map(&lines, &mut grid);

        assert_eq!(grid.len(), 63);
    }

    #[test]
    fn test_get_line() {
        let mut grid = HashMap::new();

        grid.insert((0, 0), '.');
        grid.insert((0, 1), '#');
        grid.insert((1, 0), '#');
        grid.insert((1, 1), '.');

        let row = get_line(&grid, Some(0), None).unwrap();
        assert_eq!(row, ".#");

        let col = get_line(&grid, None, Some(1)).unwrap();
        assert_eq!(col, "#.");

        assert!(get_line(&grid, Some(3), None).is_none());

        assert!(get_line(&grid, None, Some(3)).is_none());
    }

    #[test]
    fn test_scan_horizontal() {
        let mut grid = HashMap::new();

        grid.insert((0, 0), '.');
        grid.insert((0, 1), '.');
        grid.insert((0, 2), '#');
        grid.insert((1, 0), '#');
        grid.insert((1, 1), '#');
        grid.insert((1, 2), '.');
        grid.insert((2, 0), '#');
        grid.insert((2, 1), '#');
        grid.insert((2, 2), '.');

        let result = scan(1, true, true, &grid, false);

        assert!(result.is_some());

        let mut grid = HashMap::new();

        grid.insert((0, 0), '.');
        grid.insert((0, 1), '#');
        grid.insert((0, 2), '#');
        grid.insert((1, 0), '#');
        grid.insert((1, 1), '.');
        grid.insert((1, 2), '.');
        grid.insert((2, 0), '#');
        grid.insert((2, 1), '#');
        grid.insert((2, 2), '#');

        let result = scan(2, false, true, &grid, false);

        assert!(result.is_some());

        let mut grid = HashMap::new();

        grid.insert((0, 0), '.');
        grid.insert((0, 1), '#');
        grid.insert((0, 2), '#');
        grid.insert((0, 3), '.');
        grid.insert((0, 4), '#');
        grid.insert((1, 0), '#');
        grid.insert((1, 1), '#');
        grid.insert((1, 2), '#');
        grid.insert((1, 3), '#');
        grid.insert((1, 4), '#');
        grid.insert((2, 0), '.');
        grid.insert((2, 1), '.');
        grid.insert((2, 2), '.');
        grid.insert((2, 3), '.');
        grid.insert((2, 4), '#');
        grid.insert((3, 0), '#');
        grid.insert((3, 1), '.');
        grid.insert((3, 2), '.');
        grid.insert((3, 3), '#');
        grid.insert((3, 4), '#');
        grid.insert((4, 0), '.');
        grid.insert((4, 1), '.');
        grid.insert((4, 2), '.');
        grid.insert((4, 3), '.');
        grid.insert((4, 4), '#');

        let result = scan(2, true, true, &grid, false);

        assert!(result.is_some());

        let mut grid = HashMap::new();

        grid.insert((0, 0), '.');
        grid.insert((0, 1), '#');
        grid.insert((0, 2), '.');
        grid.insert((0, 3), '.');
        grid.insert((0, 4), '#');
        grid.insert((1, 0), '#');
        grid.insert((1, 1), '#');
        grid.insert((1, 2), '#');
        grid.insert((1, 3), '#');
        grid.insert((1, 4), '#');
        grid.insert((2, 0), '.');
        grid.insert((2, 1), '#');
        grid.insert((2, 2), '.');
        grid.insert((2, 3), '.');
        grid.insert((2, 4), '#');
        grid.insert((3, 0), '#');
        grid.insert((3, 1), '#');
        grid.insert((3, 2), '#');
        grid.insert((3, 3), '#');
        grid.insert((3, 4), '#');
        grid.insert((4, 0), '.');
        grid.insert((4, 1), '#');
        grid.insert((4, 2), '.');
        grid.insert((4, 3), '.');
        grid.insert((4, 4), '#');

        let result = scan(3, false, true, &grid, false);

        assert!(result.is_some());

        //vert
        let mut grid = HashMap::new();

        grid.insert((0, 0), '#');
        grid.insert((0, 1), '#');
        grid.insert((0, 2), '#');
        grid.insert((1, 0), '#');
        grid.insert((1, 1), '#');
        grid.insert((1, 2), '#');
        grid.insert((2, 0), '.');
        grid.insert((2, 1), '#');
        grid.insert((2, 2), '.');

        let result = scan(1, true, false, &grid, false);

        assert!(result.is_some());

        let mut grid = HashMap::new();

        grid.insert((0, 0), '.');
        grid.insert((0, 1), '#');
        grid.insert((0, 2), '.');
        grid.insert((1, 0), '#');
        grid.insert((1, 1), '#');
        grid.insert((1, 2), '#');
        grid.insert((2, 0), '#');
        grid.insert((2, 1), '#');
        grid.insert((2, 2), '#');

        let result = scan(2, false, false, &grid, false);

        assert!(result.is_some());

        let mut grid = HashMap::new();

        grid.insert((0, 0), '#');
        grid.insert((0, 1), '.');
        grid.insert((0, 2), '#');
        grid.insert((0, 3), '.');
        grid.insert((0, 4), '#');
        grid.insert((1, 0), '#');
        grid.insert((1, 1), '#');
        grid.insert((1, 2), '#');
        grid.insert((1, 3), '#');
        grid.insert((1, 4), '#');
        grid.insert((2, 0), '#');
        grid.insert((2, 1), '#');
        grid.insert((2, 2), '#');
        grid.insert((2, 3), '#');
        grid.insert((2, 4), '#');
        grid.insert((3, 0), '#');
        grid.insert((3, 1), '.');
        grid.insert((3, 2), '#');
        grid.insert((3, 3), '.');
        grid.insert((3, 4), '#');
        grid.insert((4, 0), '.');
        grid.insert((4, 1), '#');
        grid.insert((4, 2), '.');
        grid.insert((4, 3), '.');
        grid.insert((4, 4), '#');

        let result = scan(2, true, false, &grid, false);

        assert!(result.is_some());

        let mut grid = HashMap::new();

        grid.insert((0, 0), '.');
        grid.insert((0, 1), '#');
        grid.insert((0, 2), '.');
        grid.insert((0, 3), '.');
        grid.insert((0, 4), '#');
        grid.insert((1, 0), '.');
        grid.insert((1, 1), '#');
        grid.insert((1, 2), '.');
        grid.insert((1, 3), '.');
        grid.insert((1, 4), '#');
        grid.insert((2, 0), '#');
        grid.insert((2, 1), '#');
        grid.insert((2, 2), '#');
        grid.insert((2, 3), '#');
        grid.insert((2, 4), '#');
        grid.insert((3, 0), '#');
        grid.insert((3, 1), '#');
        grid.insert((3, 2), '#');
        grid.insert((3, 3), '#');
        grid.insert((3, 4), '#');
        grid.insert((4, 0), '.');
        grid.insert((4, 1), '#');
        grid.insert((4, 2), '.');
        grid.insert((4, 3), '.');
        grid.insert((4, 4), '#');

        let result = scan(3, false, false, &grid, false);

        assert!(result.is_some());
    }
}
