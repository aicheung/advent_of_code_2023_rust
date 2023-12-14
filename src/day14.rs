use std::{collections::HashMap, fs, time::Instant};

const ROCK: char = 'O';
const EMPTY: char = '.';
const CUBE: char = '#';

enum Direction {
    NORTH, WEST, SOUTH, EAST
}

fn move_rock(dir: &Direction, row: u64, col: u64, grid: &mut HashMap<(u64, u64), char>) -> bool {
    let target_row: Option<u64>; 
    let target_col: Option<u64>;

    match dir {
        Direction::NORTH => {
            target_row = row.checked_sub(1);
            target_col  = Some(col); 
        },
        Direction::WEST => {
            target_row = Some(row);
            target_col  = col.checked_sub(1);
        },
        Direction::SOUTH => {
            target_row = Some(row + 1);
            target_col = Some(col);
        },
        Direction::EAST => {
            target_row = Some(row);
            target_col = Some(col + 1);
        }
    }

    if target_row.is_none() || target_col.is_none() || !grid.contains_key(&(target_row.unwrap(), target_col.unwrap())) {
        return false;
    }
    let dest = (target_row.unwrap(), target_col.unwrap());
    let cur = (row, col);

    if !grid.contains_key(&cur) {
        return false;
    }
    let cur_cell = *grid.get(&cur).unwrap();
    let target_cell = *grid.get(&dest).unwrap();

    if cur_cell != ROCK || target_cell == CUBE || target_cell == ROCK {
        return false; //blocked
    }

    grid.insert(dest, ROCK);
    grid.insert(cur, EMPTY);
    true
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

fn get_max(grid: &HashMap<(u64, u64), char>, is_horizontal: bool) -> u64 {
    grid.iter()
        .map(|a| if is_horizontal { a.0 .1 } else { a.0 .0 })
        .max()
        .unwrap()
}

fn move_round_to_dir(dir: &Direction, grid: &mut HashMap<(u64, u64), char>) -> bool {
    let mut any_moved = false;

    let width = get_max(grid, true) + 1;
    let height = get_max(grid, false) + 1;

    for i in 0 .. height {
        for j in 0 .. width {
            let result = move_rock(&dir, i, j, grid);
            if result {
                any_moved = true;
            }
        }
    }

    any_moved
}

fn tally_rocks(grid: &HashMap<(u64, u64), char>) -> u64 {
    let mut total = 0;
    let height = get_max(grid, false) + 1;
    let width = get_max(grid, true) + 1;

    for i in 0 .. height {
        let cur_score = height - i;
        for j in 0 .. width {
            let cell = *grid.get(&(i, j)).unwrap();
            if cell == ROCK {
                total += cur_score;
            }
        }
    }

    total
}

fn p1(file: &str) -> u64 {
    let s = fs::read_to_string(file).expect("");
    let lines = s.split('\n').collect::<Vec<_>>();

    let mut grid = HashMap::new();
    load_map(&lines, &mut grid);

    move_to_dir(&Direction::NORTH, &mut grid);
    tally_rocks(&grid)
}

fn move_to_dir(dir: &Direction, grid: &mut HashMap<(u64, u64), char>) {
    let mut can_move = true;
    while can_move {
        can_move = move_round_to_dir(&dir, grid);
    }
}

fn get_grid_snapshot(grid: &HashMap<(u64, u64), char>) -> String {
    let mut out = String::new();
    let height = get_max(grid, false) + 1;
    let width = get_max(grid, true) + 1;

    for i in 0 .. height {
        for j in 0 .. width {
            let cell = *grid.get(&(i, j)).unwrap();
            out.push(cell);
        }
    }
    out
}

fn p2(file: &str) -> u64 {
    let s = fs::read_to_string(file).expect("");
    let lines = s.split('\n').collect::<Vec<_>>();

    let mut grid = HashMap::new();
    load_map(&lines, &mut grid);

    const CYCLES: u64 = 300;
    let mut cycle_found = false;
    let mut cycle_found_begin = 0;
    let mut cycle_found_last = 0;
    let mut snapshots = HashMap::new();
    let mut cycle_scores = HashMap::new();
    for _i in 0 .. CYCLES {
        move_to_dir(&Direction::NORTH, &mut grid);
        move_to_dir(&Direction::WEST, &mut grid);
        move_to_dir(&Direction::SOUTH, &mut grid);
        move_to_dir(&Direction::EAST, &mut grid);

        let score = tally_rocks(&grid);
        cycle_scores.insert(_i, score);

        let snapshot:  String = get_grid_snapshot(&grid);
        snapshots.entry(snapshot.clone()).and_modify(|c| {
            println!("CYCLE FOUND: {}", *c);
            cycle_found = true;
            cycle_found_begin = *c;
            cycle_found_last = _i;
            *c = _i;
        }).or_insert(_i);

        if cycle_found {
            break;
        }
    }

    println!("cycle: {}, {}", cycle_found_begin, cycle_found_last);
    let target_cycle = (1_000_000_000 - cycle_found_begin) % (cycle_found_last - cycle_found_begin);
    *cycle_scores.get(&(cycle_found_begin + target_cycle - 1)).unwrap()
    
}

fn main() {
    let file = "data/day14.txt";
    let now = Instant::now();

    let result = p1(file);

    let elapsed_time = now.elapsed();
    println!("p1: {}, time: {}", result, elapsed_time.as_millis());

    let now = Instant::now();

    let result = p2(file);

    let elapsed_time = now.elapsed();
    println!("p2: {}, time: {}", result, elapsed_time.as_millis());
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, fs};

    use super::*;

    #[test]
    fn test_load_grid() {
        let file = "data/day14_ex.txt";
        let s = fs::read_to_string(file).expect("");
        let lines = s.split('\n').collect::<Vec<_>>();

        let mut grid = HashMap::new();
        load_map(&lines, &mut grid);
        assert_eq!(grid.len(), 100);
    }

    #[test]
    fn test_tally_rocks() {
        let file = "data/day14_ex_page2.txt";
        let s = fs::read_to_string(file).expect("");
        let lines = s.split('\n').collect::<Vec<_>>();

        let mut grid = HashMap::new();
        load_map(&lines, &mut grid);
        let result = tally_rocks(&grid);
        assert_eq!(result, 136);

    }

    #[test]
    fn test_p1() {
        let file = "data/day14_ex_page.txt";
        let result = p1(file);

        assert_eq!(result, 136);
    }

    #[test]
    fn test_p2() {
        let file = "data/day14_ex_page.txt";
        let result = p2(file);
        println!("{}", result);
    }

    #[test]
    fn test_move_round() {
        let file = "data/day14_ex_page.txt";
        let s = fs::read_to_string(file).expect("");
        let lines = s.split('\n').collect::<Vec<_>>();

        let mut grid = HashMap::new();
        load_map(&lines, &mut grid);

        let mut moved_grid = grid.clone();
        let move_result = move_round_to_dir(&Direction::NORTH, &mut moved_grid);

        assert_eq!(move_result, true);
        assert_ne!(grid, moved_grid);
        assert_eq!(*moved_grid.get(&(0,2)).unwrap(), ROCK);
        assert_eq!(*moved_grid.get(&(8,2)).unwrap(), ROCK);
        assert_eq!(*moved_grid.get(&(9,2)).unwrap(), EMPTY);
    }

    #[test]
    fn test_move() {
        let file = "data/day14_ex2.txt";
        let s = fs::read_to_string(file).expect("");
        let lines = s.split('\n').collect::<Vec<_>>();

        let mut grid = HashMap::new();
        load_map(&lines, &mut grid);

        let result = move_rock(&Direction::NORTH, 1,1, &mut grid);
        assert_eq!(result, true);

        assert_eq!(*grid.get(&(0,1)).unwrap(), ROCK);
        assert_eq!(*grid.get(&(1,1)).unwrap(), EMPTY);

        //p2 south
        let file = "data/day14_ex2.txt";
        let s = fs::read_to_string(file).expect("");
        let lines = s.split('\n').collect::<Vec<_>>();

        let mut grid = HashMap::new();
        load_map(&lines, &mut grid);

        let result = move_rock(&Direction::SOUTH, 1,1, &mut grid);
        assert_eq!(result, true);

        assert_eq!(*grid.get(&(2,1)).unwrap(), ROCK);
        assert_eq!(*grid.get(&(1,1)).unwrap(), EMPTY);

        //p2 west
        let file = "data/day14_ex2.txt";
        let s = fs::read_to_string(file).expect("");
        let lines = s.split('\n').collect::<Vec<_>>();

        let mut grid = HashMap::new();
        load_map(&lines, &mut grid);

        let result = move_rock(&Direction::WEST, 1,1, &mut grid);
        assert_eq!(result, true);

        assert_eq!(*grid.get(&(1,0)).unwrap(), ROCK);
        assert_eq!(*grid.get(&(1,1)).unwrap(), EMPTY);

        //p2 east
        let file = "data/day14_ex2.txt";
        let s = fs::read_to_string(file).expect("");
        let lines = s.split('\n').collect::<Vec<_>>();

        let mut grid = HashMap::new();
        load_map(&lines, &mut grid);

        let result = move_rock(&Direction::EAST, 1,1, &mut grid);
        assert_eq!(result, true);

        assert_eq!(*grid.get(&(1,2)).unwrap(), ROCK);
        assert_eq!(*grid.get(&(1,1)).unwrap(), EMPTY);

        let file = "data/day14_ex3.txt";
        let s = fs::read_to_string(file).expect("");
        let lines = s.split('\n').collect::<Vec<_>>();

        let mut grid = HashMap::new();
        load_map(&lines, &mut grid);

        let result = move_rock(&Direction::NORTH,1,1, &mut grid);
        assert_eq!(result, false);

        assert_eq!(*grid.get(&(0,1)).unwrap(), CUBE);
        assert_eq!(*grid.get(&(1,1)).unwrap(), ROCK);

        let file = "data/day14_ex4.txt";
        let s = fs::read_to_string(file).expect("");
        let lines = s.split('\n').collect::<Vec<_>>();

        let mut grid = HashMap::new();
        load_map(&lines, &mut grid);

        let result = move_rock(&Direction::NORTH,1,1, &mut grid);
        assert_eq!(result, false);

        assert_eq!(*grid.get(&(0,1)).unwrap(), ROCK);
        assert_eq!(*grid.get(&(1,1)).unwrap(), ROCK);

        //cant move to -1
        let result = move_rock(&Direction::NORTH,0,1, &mut grid);
        assert_eq!(result, false);

        //cant move empty
        let result = move_rock(&Direction::NORTH,1,0, &mut grid);
        assert_eq!(result, false);
    }

}