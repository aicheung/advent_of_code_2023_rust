use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};


fn load_grid(file: &str, grid: &mut HashMap<(u64, u64), char>) {
    let s = fs::read_to_string(file).expect("cannot open");
    let lines = s.split_whitespace().collect();
    load_map(&lines, grid);
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

fn get_neighbour(loc: (u64, u64), row_offset: i64, col_offset: i64) -> Option<(u64, u64)> {
    let row = loc.0.checked_add_signed(row_offset);
    let col = loc.1.checked_add_signed(col_offset);

    if row.is_some() && col.is_some() {
        return Some((row.expect(""), col.expect("")));
    }
    None
}

fn bfs(start: (u64, u64), grid: &HashMap<(u64, u64), char>, steps: u64) -> HashSet<(u64, u64)> {
    let mut out = HashSet::new();
    let mut to_visit = VecDeque::new();
    let mut seen = HashSet::new();
    to_visit.push_back((0, start));

    while !to_visit.is_empty() {
        let cur = to_visit.pop_front().unwrap();
        let loc = cur.1;
        let cur_steps = cur.0;

        if !grid.contains_key(&loc)
            || grid.get(&loc).expect("cannot get").eq(&'#')
            || seen.contains(&loc)
        {
            continue;
        }
        seen.insert(loc);

        if cur_steps <= steps && cur_steps % 2 == steps % 2 {
            //found
            out.insert(loc);
            //continue;
        }

        let up: Option<(u64, u64)> = get_neighbour(loc, -1, 0);
        let down: Option<(u64, u64)> = get_neighbour(loc, 1, 0);
        let left: Option<(u64, u64)> = get_neighbour(loc, 0, -1);
        let right: Option<(u64, u64)> = get_neighbour(loc, 0, 1);

        let contains = |l| grid.contains_key(&l);
        if up.is_some_and(contains) {
            to_visit.push_back((cur_steps + 1, up.expect("")));
        }
        if down.is_some_and(contains) {
            to_visit.push_back((cur_steps + 1, down.expect("")));
        }
        if left.is_some_and(contains) {
            to_visit.push_back((cur_steps + 1, left.expect("")));
        }
        if right.is_some_and(contains) {
            to_visit.push_back((cur_steps + 1, right.expect("")));
        }
    }

    out
}

fn find_plots(row: u64, col: u64, steps: u64, grid: &HashMap<(u64, u64), char>) -> u64 {
    bfs((row, col), grid, steps).len() as u64
}

fn expand_grid(
    grid: &HashMap<(u64, u64), char>,
    new_grid: &mut HashMap<(u64, u64), char>,
    multiplier: u64,
) {
    let size = grid.iter().map(|a| (*a.0).0).max().unwrap() + 1;

    for i in 0..multiplier * size {
        for j in 0..multiplier * size {
            let new_coord = (i, j);
            let orig_coord = (i % size, j % size);
            new_grid.insert(new_coord, *grid.get(&orig_coord).unwrap());
        }
    }
}

fn p1(file: &str) -> u64 {
    let mut grid = HashMap::new();

    load_grid(file, &mut grid);
    let start = grid.iter().find(|q| *q.1 == 'S');
    let start_loc = start.expect("cannot start").0;

    find_plots(start_loc.0, start_loc.1, 64, &grid)
}

fn find_x(x0: i128, y0: i128, x1: i128, y1: i128, x2: i128, y2: i128, x: i128) -> i128 {
    // Calculate the Lagrange basis polynomials
    let l0 = ((x - x1) * (x - x2)) / ((x0 - x1) * (x0 - x2));
    let l1 = ((x - x0) * (x - x2)) / ((x1 - x0) * (x1 - x2));
    let l2 = ((x - x0) * (x - x1)) / ((x2 - x0) * (x2 - x1));

    // Calculate the value of y
    let y = y0 * l0 + y1 * l1 + y2 * l2;
    y
}

fn p2(file: &str) -> i128 {
    let mut grid = HashMap::new();

    load_grid(file, &mut grid);

    let size_y = grid.iter().map(|a| (*a.0).0).max().unwrap() + 1;
    let size_x = grid.iter().map(|a| (*a.0).1).max().unwrap() + 1;
    let steps = 26501365;
    println!("size: {} * {}", size_x, size_y);

    let mut new_grid = HashMap::new();
    expand_grid(&grid, &mut new_grid, 9);

    let new_size = new_grid.iter().map(|a| (*a.0).0).max().unwrap() + 1;
    let new_start = new_size / 2;
    assert_eq!(*(&new_grid.get(&(new_start, new_start))).unwrap(), 'S');

    let n = 65;
    let n0 = find_plots(new_start, new_start, n, &new_grid);
    let n1 = find_plots(new_start, new_start, n + 131, &new_grid);
    let n2 = find_plots(new_start, new_start, n + 131 * 2, &new_grid);
    let x = (steps - 65) / size_x;

    println!("{} {} {} {}", n0, n1, n2, x);

    find_x(0, n0 as i128, 1, n1 as i128, 2, n2 as i128, x as i128)
}

fn main() {
    let file = "data/day21.txt";
    println!("p1: {}", p1(file));
    println!("p2: {}", p2(file));
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_div() {
        let three: u64 = 3;
        let two: u64 = 2;
        assert_eq!(three / two, 1);
    }

    #[test]
    fn test_load_grid() {
        let file = "data/day21_ex.txt";

        let mut grid = HashMap::new();

        load_grid(file, &mut grid);

        assert_eq!(grid.len(), 11 * 11);
        assert_eq!(*grid.get(&(0, 0)).unwrap(), '.');
        assert_eq!(*grid.get(&(5, 5)).unwrap(), 'S');
        assert_eq!(*grid.get(&(2, 1)).unwrap(), '#');
    }

    #[test]
    fn test_get_neighbour() {
        let now = (1, 1);
        let up = get_neighbour(now, -1, 0);
        assert!(up.is_some());
        assert_eq!(up.unwrap(), (0, 1));

        let down = get_neighbour(now, 1, 0);
        assert!(down.is_some());
        assert_eq!(down.unwrap(), (2, 1));

        let left = get_neighbour(now, 0, -1);
        assert!(left.is_some());
        assert_eq!(left.unwrap(), (1, 0));

        let right = get_neighbour(now, 0, 1);
        assert!(right.is_some());
        assert_eq!(right.unwrap(), (1, 2));
    }

    #[test]
    fn test_bfs() {
        let file = "data/day21_ex.txt";

        let mut grid = HashMap::new();

        load_grid(file, &mut grid);

        let result = bfs((5, 5), &grid, 1);

        assert_eq!(result.len(), 2);

        let result = bfs((5, 5), &grid, 2);
        assert_eq!(result.len(), 4);

        let result = bfs((5, 5), &grid, 3);

        assert_eq!(result.len(), 6);

        let result = bfs((5, 5), &grid, 6);

        assert_eq!(result.len(), 16);
    }
}
