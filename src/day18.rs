use std::{
    collections::{BinaryHeap, HashMap},
    fs,
};

use num::abs;

#[derive(Clone, PartialEq, Eq, Debug)]
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}
#[derive(Clone)]
struct Step {
    dir: Direction,
    length: i32,
}
#[derive(Clone)]
struct Cell {}

fn load(file: &str) -> Vec<Step> {
    let s = fs::read_to_string(file).expect("");
    let mut out = Vec::new();
    for l in s.split('\n') {
        if l.len() < 2 {
            continue;
        }
        let line = l.trim().split_whitespace().collect::<Vec<_>>();
        let dir = match line[0] {
            "R" => Direction::EAST,
            "L" => Direction::WEST,
            "U" => Direction::NORTH,
            _ => Direction::SOUTH,
        };
        let s = Step {
            dir,
            length: line[1].parse::<i32>().unwrap(),
        };
        out.push(s);
    }
    out
}

fn get_cells_to_dig(cur: &(i32, i32), step: &Step) -> Vec<(i32, i32)> {
    let mut out = Vec::new();
    match step.dir {
        Direction::EAST => {
            let start = cur.1 + 1;
            let end = start + step.length;
            for i in start..end {
                out.push((cur.0, i));
            }
        }
        Direction::WEST => {
            let start = cur.1 - step.length;
            let end = cur.1;
            for i in (start..end).rev() {
                out.push((cur.0, i));
            }
        }
        Direction::NORTH => {
            let start = cur.0 - step.length;
            let end = cur.0;
            for i in (start..end).rev() {
                out.push((i, cur.1));
            }
        }
        Direction::SOUTH => {
            let start = cur.0 + 1;
            let end = start + step.length;
            for i in start..end {
                out.push((i, cur.1));
            }
        }
    }
    out
}

fn get_cells_to_dig_p2(cur: &(i32, i32), step: &Step) -> Vec<(i32, i32)> {
    let mut out = Vec::new();
    match step.dir {
        Direction::EAST => {
            out.push((cur.0, cur.1 + step.length));
        }
        Direction::WEST => {
            out.push((cur.0, cur.1 - step.length));
        }
        Direction::NORTH => {
            out.push((cur.0 - step.length, cur.1));
        }
        Direction::SOUTH => {
            out.push((cur.0 + step.length, cur.1));
        }
    }
    out
}

fn dig(steps: &Vec<Step>, grid: &mut HashMap<(i32, i32), Cell>) -> Vec<(i32, i32)> {
    let mut out = Vec::new();
    let start = (0, 0);
    out.push(start);
    let start_cell = Cell {};
    grid.insert(start, start_cell);

    let mut cur = start;
    for s in steps {
        let cells_to_dig: Vec<(i32, i32)> = get_cells_to_dig(&cur, s);

        cur = cells_to_dig.last().unwrap().clone();
        if !out.contains(&cur) {
            out.push(cur);
        }
        for c in cells_to_dig {
            let cell = Cell {};
            grid.insert(c, cell);
        }
    }

    out
}

fn find_fill_start(grid: &HashMap<(i32, i32), Cell>) -> Option<(i32, i32)> {
    let min_row = grid.keys().map(|k| k.0).min().unwrap();
    let max_row = grid.keys().map(|k| k.0).max().unwrap();
    let min_col = grid.keys().map(|k| k.1).min().unwrap();
    let max_col = grid.keys().map(|k| k.1).max().unwrap();

    for r in min_row..max_row + 1 {
        for c in min_col..max_col + 1 {
            if grid.contains_key(&(r, c)) {
                //wall
                continue;
            }

            //point in poly - horizontal
            let rt_start = min_col - 1;
            let mut wall_count_hori = 0;
            for pc in rt_start..c {
                if grid.contains_key(&(r, pc)) {
                    wall_count_hori += 1;
                }
            }

            //check vert as well
            let rt_start = min_row - 1;
            let mut wall_count_vert = 0;
            for pr in rt_start..r {
                if grid.contains_key(&(pr, c)) {
                    wall_count_vert += 1;
                }
            }

            if wall_count_hori % 2 == 1 && wall_count_vert % 2 == 1 {
                return Some((r, c));
            }
        }
    }
    None
}

fn fill_pit(start: &(i32, i32), fill: &mut Vec<(i32, i32)>, grid: &HashMap<(i32, i32), Cell>) {
    let mut to_visit = BinaryHeap::new();
    to_visit.push(start.clone());

    while !to_visit.is_empty() {
        let cur = to_visit.pop().unwrap();
        if grid.contains_key(&cur) {
            continue; // hit wall
        }
        if fill.contains(&cur) {
            continue; //already filled
        }

        fill.push(cur);
        to_visit.push((cur.0, cur.1 - 1));
        to_visit.push((cur.0, cur.1 + 1));
        to_visit.push((cur.0 + 1, cur.1));
        to_visit.push((cur.0 - 1, cur.1));
    }
}

fn shoelace(pts: &Vec<(i32, i32)>) -> u64 {
    let mut out: i64 = 0;

    for (i, p) in pts.iter().enumerate() {
        let j = if i == pts.len() - 1 { 0 } else { i + 1 };
        let next = pts[j];

        let xn = p.1;
        let yn = p.0;
        let xm = next.1;
        let ym = next.0;

        out += (xn as i64 * ym as i64 - xm as i64 * yn as i64) as i64;
    }
    abs(out / 2).try_into().unwrap()
}

fn p1(file: &str) -> i32 {
    let steps: Vec<Step> = load(file);
    let mut grid = HashMap::new();
    dig(&steps, &mut grid);

    let fill_start: Option<(i32, i32)> = find_fill_start(&grid);
    let mut fill: Vec<(i32, i32)> = Vec::new();
    fill_pit(&fill_start.unwrap(), &mut fill, &grid);

    grid.len() as i32 + fill.len() as i32
}

fn pick(i: i64, b: i64) -> u64 {
    (i + b / 2 - 1).try_into().unwrap()
}

fn dig_p2(steps: &Vec<Step>) -> (Vec<(i32, i32)>, u64) {
    let mut out = Vec::new();
    let mut total_length = 0;
    let start = (0, 0);
    out.push(start);
    total_length += 1;

    let mut cur = start;
    for s in steps {
        let cells_to_dig: Vec<(i32, i32)> = get_cells_to_dig_p2(&cur, s);

        cur = cells_to_dig.last().unwrap().clone();
        if !out.contains(&cur) {
            out.push(cur);
        }
        total_length += s.length;
    }

    (out, total_length.try_into().unwrap())
}

fn load_p2(file: &str) -> Vec<Step> {
    let s = fs::read_to_string(file).expect("");
    let mut out = Vec::new();
    for l in s.split('\n') {
        if l.len() < 2 {
            continue;
        }
        let line = l.trim().split_whitespace().collect::<Vec<_>>();

        let hex = &line[2][2..7];
        let length = i32::from_str_radix(hex, 16).unwrap();
        let dir_str = &line[2][7..8];

        let dir = match dir_str {
            "0" => Direction::EAST,
            "2" => Direction::WEST,
            "3" => Direction::NORTH,
            _ => Direction::SOUTH,
        };
        let s = Step { dir, length };
        out.push(s);
    }

    out
}

fn p2(file: &str) -> u64 {
    let steps = load_p2(file);
    let (points, b) = dig_p2(&steps);
    let sl = shoelace(&points);
    let p = pick(sl.try_into().unwrap(), b.try_into().unwrap());

    println!("Shoelace: {}, points: {}, pick: {}", sl, b, p);
    let t = p + 2;
    t
}

fn main() {
    let file = "data/day18.txt";
    println!("p1: {}", p1(file));
    println!("p2: {}", p2(file));
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_load() {
        let file = "data/day18_ex.txt";

        let steps: Vec<Step> = load(file);

        assert_eq!(steps.len(), 14);
    }

    #[test]
    fn test_load_p2() {
        let file = "data/day18_ex.txt";

        let steps: Vec<Step> = load_p2(file);

        assert_eq!(steps.len(), 14);
        assert_eq!(steps[0].dir, Direction::EAST);
        assert_eq!(steps[0].length, 461937);
        assert_eq!(steps[1].dir, Direction::SOUTH);
        assert_eq!(steps[1].length, 56407);
        assert_eq!(steps[2].dir, Direction::EAST);
        assert_eq!(steps[2].length, 356671);
        assert_eq!(steps[13].dir, Direction::NORTH);
        assert_eq!(steps[13].length, 500254);
    }

    #[test]
    fn test_p1() {
        let file = "data/day18_ex.txt";
        assert_eq!(p1(file), 62);
    }

    #[test]
    fn test_fill() {
        let file = "data/day18_ex.txt";

        let steps: Vec<Step> = load(file);
        let mut grid = HashMap::new();
        dig(&steps, &mut grid);

        let fill_start: Option<(i32, i32)> = find_fill_start(&grid);
        let mut fill: Vec<(i32, i32)> = Vec::new();
        fill_pit(&fill_start.unwrap(), &mut fill, &grid);
        assert_eq!(fill.len(), 24);
    }

    #[test]
    fn test_p2() {
        let file = "data/day18_ex.txt";
        assert_eq!(p2(file), 952408144115);
    }

    #[test]
    fn test_dig() {
        let file = "data/day18_ex.txt";

        let steps: Vec<Step> = load(file);
        let mut grid = HashMap::new();
        dig(&steps, &mut grid);

        assert_eq!(grid.len(), 38);
    }

    #[test]
    fn test_find_fill_start() {
        let file = "data/day18_ex.txt";

        let steps: Vec<Step> = load(file);
        let mut grid = HashMap::new();
        dig(&steps, &mut grid);

        let fill_start: Option<(i32, i32)> = find_fill_start(&grid);
        assert!(fill_start.is_some());
        let d = fill_start.unwrap();
        assert_ne!(d, (3, 1));
        assert_ne!(d, (4, 1));
        assert_ne!(d, (6, 5));
        assert!(!grid.contains_key(&d));
    }

    #[test]
    fn test_shoelace() {
        let points = vec![(-2, -2), (4, 0), (-1, 3), (-1, 1)];

        let result = shoelace(&points);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_pick() {
        let i = 7;
        let b = 8;
        let result = pick(i, b);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_p2_ex() {
        let file = "data/day18_ex.txt";

        let steps: Vec<Step> = load(file);
        let mut grid = HashMap::new();

        let points: Vec<(i32, i32)> = dig(&steps, &mut grid);
        let sl = shoelace(&points);
        let b = grid.len();
        let p = pick(sl.try_into().unwrap(), b.try_into().unwrap());

        println!("Shoelace: {}, points: {}, pick: {}", sl, b, p);
        let t = p + 2;
        println!("out: {}", t);
        assert_eq!(t, 62);
    }

    #[test]
    fn test_cell_to_dig() {
        let cur = (0, 0);
        let step = Step {
            dir: Direction::EAST,
            length: 6,
        };

        let cells: Vec<(i32, i32)> = get_cells_to_dig(&cur, &step);
        assert_eq!(cells.len(), 6);
        assert!(cells.contains(&(0, 1)));
        assert!(cells.contains(&(0, 2)));
        assert!(cells.contains(&(0, 3)));
        assert!(cells.contains(&(0, 4)));
        assert!(cells.contains(&(0, 5)));
        assert!(cells.contains(&(0, 6)));

        let cur = (0, 0);
        let step = Step {
            dir: Direction::WEST,
            length: 3,
        };

        let cells: Vec<(i32, i32)> = get_cells_to_dig(&cur, &step);
        assert_eq!(cells.len(), 3);
        assert!(cells.contains(&(0, -1)));
        assert!(cells.contains(&(0, -2)));
        assert!(cells.contains(&(0, -3)));

        let cur = (0, 0);
        let step = Step {
            dir: Direction::SOUTH,
            length: 4,
        };

        let cells: Vec<(i32, i32)> = get_cells_to_dig(&cur, &step);
        assert_eq!(cells.len(), 4);
        assert!(cells.contains(&(1, 0)));
        assert!(cells.contains(&(2, 0)));
        assert!(cells.contains(&(3, 0)));
        assert!(cells.contains(&(4, 0)));

        let cur = (0, 0);
        let step = Step {
            dir: Direction::NORTH,
            length: 7,
        };

        let cells: Vec<(i32, i32)> = get_cells_to_dig(&cur, &step);
        assert_eq!(cells.len(), 7);
        assert!(cells.contains(&(-1, 0)));
        assert!(cells.contains(&(-2, 0)));
        assert!(cells.contains(&(-3, 0)));
        assert!(cells.contains(&(-4, 0)));
        assert!(cells.contains(&(-5, 0)));
        assert!(cells.contains(&(-6, 0)));
        assert!(cells.contains(&(-7, 0)));
    }

    #[test]
    fn test_cell_to_dig_p2() {
        let cur = (0, 0);
        let step = Step {
            dir: Direction::EAST,
            length: 6,
        };

        let cells: Vec<(i32, i32)> = get_cells_to_dig_p2(&cur, &step);
        assert_eq!(cells.len(), 1);
        assert!(cells.contains(&(0, 6)));

        let cur = (0, 0);
        let step = Step {
            dir: Direction::WEST,
            length: 3,
        };

        let cells: Vec<(i32, i32)> = get_cells_to_dig_p2(&cur, &step);
        assert_eq!(cells.len(), 1);
        assert!(cells.contains(&(0, -3)));

        let cur = (0, 0);
        let step = Step {
            dir: Direction::SOUTH,
            length: 4,
        };

        let cells: Vec<(i32, i32)> = get_cells_to_dig_p2(&cur, &step);
        assert_eq!(cells.len(), 1);
        assert!(cells.contains(&(4, 0)));

        let cur = (0, 0);
        let step = Step {
            dir: Direction::NORTH,
            length: 7,
        };

        let cells: Vec<(i32, i32)> = get_cells_to_dig_p2(&cur, &step);
        assert_eq!(cells.len(), 1);
        assert!(cells.contains(&(-7, 0)));
    }
}
