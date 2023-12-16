use std::{collections::{HashMap, HashSet}, fs, thread};


#[derive(PartialEq, Clone, Copy, Eq, Hash)]
enum Direction {
    NORTH,
    WEST,
    SOUTH,
    EAST,
}

struct Beam {
    pos: (u64, u64),
    dir: Direction,
    walked: HashSet<(u64, u64, Direction)>,
}

impl Beam {
    fn get_target_cell(&self) -> (Option<u64>, Option<u64>) {
        let row = self.pos.0;
        let col = self.pos.1;
        let target_row: Option<u64>;
        let target_col: Option<u64>;

        //determine by dir
        match self.dir {
            Direction::NORTH => {
                target_row = row.checked_sub(1);
                target_col = Some(col);
            }
            Direction::SOUTH => {
                target_row = row.checked_add(1);
                target_col = Some(col);
            }
            Direction::EAST => {
                target_row = Some(row);
                target_col = col.checked_add(1);
            }
            Direction::WEST => {
                target_row = Some(row);
                target_col = col.checked_sub(1);
            }
        }
        (target_row, target_col)
    }

    fn walk_to_next(&mut self, grid: &HashMap<(u64, u64), char>, global_walked: &mut HashSet<(u64, u64, Direction)>) {
        let (target_row, target_col) = self.get_target_cell();
        self.walk(target_row, target_col, grid, global_walked);
    }

    fn walk(&mut self, row: Option<u64>, col: Option<u64>, grid: &HashMap<(u64, u64), char>, global_walked: &mut HashSet<(u64, u64, Direction)>) {
        if row.is_none() || col.is_none() {
            return;
        }
        self.pos = (row.unwrap(), col.unwrap());

        if !grid.contains_key(&self.pos) {
            return;
        }

        let walk_key = (self.pos.0, self.pos.1, self.dir);

        if self.walked.contains(&walk_key) || global_walked.contains(&walk_key) {
            return;
        }

        self.walked.insert(walk_key);
        global_walked.insert(walk_key);

        let cell = *grid.get(&self.pos).unwrap();
        match cell {
            '.' => {
                self.walk_to_next(grid, global_walked);
            }
            '/' => {
                self.dir = match self.dir {
                    Direction::EAST => Direction::NORTH,
                    Direction::WEST => Direction::SOUTH,
                    Direction::NORTH => Direction::EAST,
                    Direction::SOUTH => Direction::WEST,
                };
                self.walk_to_next(grid, global_walked);
            }
            '\\' => {
                self.dir = match self.dir {
                    Direction::EAST => Direction::SOUTH,
                    Direction::WEST => Direction::NORTH,
                    Direction::NORTH => Direction::WEST,
                    Direction::SOUTH => Direction::EAST,
                };
                self.walk_to_next(grid, global_walked);
            }
            '-' => {
                if self.dir == Direction::EAST || self.dir == Direction::WEST {
                    self.walk_to_next(grid, global_walked);
                } else {
                    let mut sub_beam_east = Beam {
                        pos: self.pos,
                        dir: Direction::EAST,
                        walked: self.walked.clone(),
                    };
                    sub_beam_east.walk_to_next(grid, global_walked);

                    let mut sub_beam_west = Beam {
                        pos: self.pos,
                        dir: Direction::WEST,
                        walked: self.walked.clone(),
                    };
                    sub_beam_west.walk_to_next(grid, global_walked);

                    self.walked.extend(&mut sub_beam_east.walked.iter());
                    self.walked.extend(&mut sub_beam_west.walked.iter());
                }
            }
            '|' => {
                if self.dir == Direction::SOUTH || self.dir == Direction::NORTH {
                    self.walk_to_next(grid, global_walked);
                } else {
                    let mut sub_beam_south = Beam {
                        pos: self.pos,
                        dir: Direction::SOUTH,
                        walked: self.walked.clone(),
                    };
                    sub_beam_south.walk_to_next(grid, global_walked);

                    let mut sub_beam_north = Beam {
                        pos: self.pos,
                        dir: Direction::NORTH,
                        walked: self.walked.clone(),
                    };
                    sub_beam_north.walk_to_next(grid, global_walked);

                    self.walked.extend(&mut sub_beam_south.walked.iter());
                    self.walked.extend(&mut sub_beam_north.walked.iter());
                }
            }
            _ => {}
        }
    }
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

fn start_beam_walk(grid:&HashMap<(u64, u64), char>, pos: (u64, u64), dir: Direction) -> u64 {
    let mut global_walked: HashSet<_> = HashSet::new();
    let mut beam = Beam {
        pos,
        dir,
        walked: HashSet::new()
    };
    beam.walk(Some(beam.pos.0), Some(beam.pos.1), &grid, &mut global_walked);
    assert!(!beam.walked.is_empty());

    let set = beam.walked.iter().map(|w| (w.0, w.1)).collect::<HashSet<_>>();
    println!("{}", set.len());
    set.len() as u64
}

fn p1(file: &str) -> u64 {
    let s = fs::read_to_string(file).unwrap();
    let mut grid = HashMap::new();

    load_map(&s.split_whitespace().collect::<Vec<_>>(), &mut grid);
    start_beam_walk(&grid, (0,0), Direction::EAST)
}

fn get_max(grid: &HashMap<(u64, u64), char>, is_horizontal: bool) -> u64 {
    grid.iter()
        .map(|a| if is_horizontal { a.0 .1 } else { a.0 .0 })
        .max()
        .unwrap()
}

fn p2(file: &str) -> u64 {
    let s = fs::read_to_string(file).unwrap();
    let mut grid = HashMap::new();

    load_map(&s.split_whitespace().collect::<Vec<_>>(), &mut grid);
    let width = get_max(&grid, true) + 1;
    let height = get_max(&grid, false) + 1;

    let mut start_pts: Vec<_> = Vec::new();
    for i in 0 .. width {
        //up & down
        start_pts.push((0, i, Direction::SOUTH));
        start_pts.push((height - 1, i, Direction::NORTH));
    }
    for j in 0 .. height {
        start_pts.push((j, 0, Direction::EAST));
        start_pts.push((j, width - 1, Direction::WEST));
    }

    start_pts.iter().map(|d| {
        start_beam_walk(&grid, (d.0, d.1), d.2)
    }).max().unwrap()
}

const STACK_SIZE: usize = 80 * 1024 * 1024;
fn run() {
    let file = "data/day16.txt";
    println!("p1: {}", p1(file));
    println!("p2: {}", p2(file));
}

fn main() {
    // Spawn thread with explicit stack size
    let child = thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(run)
        .unwrap();

    // Wait for thread to join
    child.join().unwrap();
}

#[cfg(test)]
mod tests {

    use std::{collections::HashMap, fs};

    use super::*;

    #[test]
    fn test_load_grid() {
        let file = "data/day16_ex.txt";
        let s = fs::read_to_string(file).unwrap();
        let mut grid = HashMap::new();

        load_map(&s.split_whitespace().collect::<Vec<_>>(), &mut grid);

        assert_eq!(grid.len(), 100);
    }

    #[test]
    fn test_walk() {
        let file = "data/day16_ex.txt";
        let s = fs::read_to_string(file).unwrap();
        let mut grid = HashMap::new();

        load_map(&s.split_whitespace().collect::<Vec<_>>(), &mut grid);
        let mut global_walked = HashSet::new();
        let mut beam = Beam {
            pos: (0,0),
            dir: Direction::EAST,
            walked: HashSet::new()
        };
        beam.walk(Some(0), Some(0), &grid, &mut global_walked);
        println!("{}", beam.walked.len());
        assert!(!beam.walked.is_empty());

        let set = beam.walked.iter().map(|w| (w.0, w.1)).collect::<HashSet<_>>();
        for i in 0 .. 10 {
            let mut line = String::new();
            for j in 0 .. 10 {
                if set.contains(&(i, j)) {
                    line.push('#');
                } else {
                    line.push('.');
                }
            }
            println!("{}", line);
        }
        println!("{}", set.len());
        assert_eq!(set.len(), 46);
    }

    #[test]
    fn test_p2() {
        let file = "data/day16_ex.txt";
        let result = p2(file);
        assert_eq!(result, 51);
    }

    #[test]
    fn test_walk_p2() {
        let file = "data/day16_ex.txt";
        let s = fs::read_to_string(file).unwrap();
        let mut grid = HashMap::new();

        load_map(&s.split_whitespace().collect::<Vec<_>>(), &mut grid);
        let result = start_beam_walk(&grid, (0,3), Direction::SOUTH);
        assert_eq!(result, 51);
    }
}
