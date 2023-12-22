use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    fs,
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash)]
struct Loc {
    x: i64,
    y: i64,
    z: i64,
}

impl Loc {
    fn from(x: i64, y: i64, z: i64) -> Loc {
        Loc { x, y, z }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Hash)]
struct Brick {
    id: i64,
    entry: String,
    start: Loc,
    end: Loc,
}

impl Brick {
    fn parse_from_file(file: &str) -> Vec<Brick> {
        let s = fs::read_to_string(file).expect("cannot read");
        let mut out = Vec::new();
        for (i, line) in s.split_whitespace().enumerate() {
            out.push(Brick::parse(line, i as i64));
        }
        out
    }

    fn parse(line: &str, id: i64) -> Brick {
        let sp = line.trim().split('~').collect::<Vec<_>>();

        let start_locs = sp[0]
            .split(',')
            .map(|l| i64::from_str_radix(l, 10).expect("cannot parse"))
            .collect::<Vec<_>>();
        let end_locs = sp[1]
            .split(',')
            .map(|l| i64::from_str_radix(l, 10).expect("cannot parse"))
            .collect::<Vec<_>>();

        Brick {
            id,
            entry: line.trim().to_string(),
            start: Loc::from(start_locs[0], start_locs[1], start_locs[2]),
            end: Loc::from(end_locs[0], end_locs[1], end_locs[2]),
        }
    }

    fn get_tiles(&self) -> Vec<Loc> {
        let mut out = Vec::new();

        let x_span = self.end.x - self.start.x;
        let y_span = self.end.y - self.start.y;
        let z_span = self.end.z - self.start.z;

        if z_span > 0 && x_span == 0 && y_span == 0 {
            //vert

            for z in self.start.z..self.end.z + 1 {
                out.push(Loc::from(self.start.x, self.start.y, z));
            }
        } else if z_span == 0 && x_span > 0 && y_span == 0 {
            //x brick
            for x in self.start.x..self.end.x + 1 {
                out.push(Loc::from(x, self.start.y, self.start.z));
            }
        } else if z_span == 0 && x_span == 0 && y_span > 0 {
            for y in self.start.y..self.end.y + 1 {
                out.push(Loc::from(self.start.x, y, self.start.z));
            }
        } else if x_span == 0 && y_span == 0 && z_span == 0 {
            //one tile
            out.push(Loc::from(self.start.x, self.start.y, self.start.z));
        } else {
            panic!("SHOULD NOT HAPPEN!!! {}, {}, {}", x_span, y_span, z_span);
        }

        out
    }

    fn get_bottom_z(&self) -> i64 {
        min(self.start.z, self.end.z)
    }

    fn remove_from_grid(&self, grid: &mut HashMap<Loc, Brick>) {
        let cur = grid
            .iter()
            .filter(|e| e.1.entry.eq(&self.entry))
            .map(|e| e.0.clone())
            .collect::<Vec<_>>();

        for c in cur {
            grid.remove(&c);
        }
    }

    fn refresh_grid(&self, grid: &mut HashMap<Loc, Brick>) {
        self.remove_from_grid(grid);

        let new_tiles = self.get_tiles();

        for t in new_tiles {
            grid.insert(t, self.clone());
        }
    }

    fn move_brick_down(&mut self, dist: i64, grid: &mut HashMap<Loc, Brick>) {
        self.start.z -= dist;
        self.end.z -= dist;

        self.refresh_grid(grid);
    }

    fn fall(&mut self, grid: &mut HashMap<Loc, Brick>) -> bool {
        //returns true if brick moved

        let mut horizontals = HashSet::new();

        for l in self.get_tiles() {
            horizontals.insert((l.x, l.y));
        }
        let bottom_z: i64 = self.get_bottom_z();

        let mut max_z = 0;
        for h in horizontals {
            //check each x,y and find max z below them
            let max_at_h = grid
                .iter()
                .filter(|l| l.0.x == h.0 && l.0.y == h.1 && l.0.z < bottom_z)
                .map(|l| l.0.z)
                .max();
            max_z = max(max_z, max_at_h.unwrap_or(0));
        }

        let z_diff = bottom_z - max_z - 1;
        assert!(z_diff >= 0);
        let moved = z_diff != 0;

        if moved {
            self.move_brick_down(z_diff, grid);
        }

        moved
    }

    fn simulate_disintegration(&self, grid: &HashMap<Loc, Brick>) -> u64 {
        let mut virtual_grid = grid.clone();
        self.remove_from_grid(&mut virtual_grid);
        //let bottom_z: i64 = self.get_bottom_z();
        let other_bricks = virtual_grid
            .iter()
            //.filter(|e| e.0.z >= bottom_z) //optimise
            .map(|e| e.1.clone())
            .collect::<HashSet<_>>();

        let mut total_moved = 0;

        let mut sorted: Vec<Brick> = other_bricks.iter().map(|b| b.clone()).collect();
        sorted.sort_by(|b, c| b.start.z.cmp(&c.start.z).then(b.id.cmp(&c.id)));

        for mut b in sorted.into_iter() {
            let moved = b.fall(&mut virtual_grid);
            if moved {
                total_moved += 1;
            }
        }
        total_moved
    }

    fn can_move(&self, grid: &HashMap<Loc, Brick>) -> bool {
        self.simulate_disintegration(grid) == 0
    }
}

fn p1(file: &str) -> u64 {
    let mut out = 0;

    let mut bricks = Brick::parse_from_file(file);
    bricks.sort_by(|a, b| a.start.z.cmp(&b.start.z).then(a.id.cmp(&b.id)));
    let mut grid = HashMap::new();
    for mut b in bricks.clone() {
        b.start.z += 100000; //adjust enough height for falling
        b.end.z += 100000;

        b.fall(&mut grid);
    }

    for b in bricks {
        if b.can_move(&grid) {
            out += 1;
        }
    }

    out
}

fn p2(file: &str) -> u64 {
    let mut out = 0;

    let mut bricks = Brick::parse_from_file(file);
    bricks.sort_by(|a, b| a.start.z.cmp(&b.start.z).then(a.id.cmp(&b.id)));
    let mut grid = HashMap::new();
    for mut b in bricks.clone() {
        b.start.z += 100000; //adjust enough height for falling
        b.end.z += 100000;

        b.fall(&mut grid);
    }

    for b in bricks {
        out += b.simulate_disintegration(&grid);
    }

    out
}

fn main() {
    let file = "data/day22.txt";
    // slow today, might take around 10 min to run
    println!("{}", p1(file));
    println!("{}", p2(file));
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_p2() {
        let file = "data/day22_ex.txt";

        let result = p2(file);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_p1() {
        let file = "data/day22_ex.txt";

        let result = p1(file);
        assert_eq!(result, 5);
        let file = "data/day22_ex2.txt";

        let result = p1(file);
        assert_eq!(result, 3);
        let file = "data/day22_ex3.txt";

        let result = p1(file);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_parse() {
        let line = "3,5,110~3,8,110";
        let bk = Brick::parse(line, 0);

        assert_eq!(bk.start.x, 3);
        assert_eq!(bk.start.y, 5);
        assert_eq!(bk.start.z, 110);
        assert_eq!(bk.end.x, 3);
        assert_eq!(bk.end.y, 8);
        assert_eq!(bk.end.z, 110);

        let line = "5,0,254~5,2,254";
        let bk = Brick::parse(line, 0);

        assert_eq!(bk.start.x, 5);
        assert_eq!(bk.start.y, 0);
        assert_eq!(bk.start.z, 254);
        assert_eq!(bk.end.x, 5);
        assert_eq!(bk.end.y, 2);
        assert_eq!(bk.end.z, 254);
    }

    #[test]
    fn test_get_tiles() {
        let line = "4,0,47~7,0,47";
        let bk = Brick::parse(line, 0);

        let t = bk.get_tiles();

        assert_eq!(t.len(), 4);
        assert_eq!(t[0], Loc::from(4, 0, 47));
        assert_eq!(t[1], Loc::from(5, 0, 47));
        assert_eq!(t[2], Loc::from(6, 0, 47));
        assert_eq!(t[3], Loc::from(7, 0, 47));

        let line = "5,0,254~5,2,254";
        let bk = Brick::parse(line, 0);
        let t = bk.get_tiles();

        assert_eq!(t.len(), 3);
        assert_eq!(t[0], Loc::from(5, 0, 254));
        assert_eq!(t[1], Loc::from(5, 1, 254));
        assert_eq!(t[2], Loc::from(5, 2, 254));

        let line = "2,4,118~2,4,121";
        let bk = Brick::parse(line, 0);
        let t = bk.get_tiles();

        assert_eq!(t.len(), 4);
        assert_eq!(t[0], Loc::from(2, 4, 118));
        assert_eq!(t[1], Loc::from(2, 4, 119));
        assert_eq!(t[2], Loc::from(2, 4, 120));
        assert_eq!(t[3], Loc::from(2, 4, 121));
    }

    #[test]
    fn test_parse_all() {
        let file = "data/day22.txt";
        let bricks = Brick::parse_from_file(file);
        assert_eq!(bricks.len(), 1218);

        let tiles = bricks.iter().map(|b| b.get_tiles()).collect::<Vec<_>>();
        assert!(!tiles.is_empty()); //can parse all
    }

    #[test]
    fn test_move_brick_down() {
        let mut grid = HashMap::new();
        let line = "5,0,254~5,2,254";
        let mut bk = Brick::parse(line, 0);
        bk.refresh_grid(&mut grid);

        bk.move_brick_down(3, &mut grid);

        assert_eq!(grid.len(), 3);
        assert_eq!(grid.contains_key(&Loc::from(5, 0, 251)), true);
        assert_eq!(grid.contains_key(&Loc::from(5, 1, 251)), true);
        assert_eq!(grid.contains_key(&Loc::from(5, 2, 251)), true);

        let mut grid = HashMap::new();
        let line = "2,1,36~2,1,38";
        let mut bk = Brick::parse(line, 0);
        bk.refresh_grid(&mut grid);

        bk.move_brick_down(3, &mut grid);

        assert_eq!(grid.len(), 3);
        assert_eq!(grid.contains_key(&Loc::from(2, 1, 33)), true);
        assert_eq!(grid.contains_key(&Loc::from(2, 1, 34)), true);
        assert_eq!(grid.contains_key(&Loc::from(2, 1, 35)), true);
    }

    #[test]
    fn test_refresh_grid() {
        let mut grid = HashMap::new();

        let line = "5,0,254~5,2,254";
        let mut bk = Brick::parse(line, 0);
        bk.refresh_grid(&mut grid);

        assert_eq!(grid.len(), 3);
        assert_eq!(grid.contains_key(&Loc::from(5, 0, 254)), true);
        assert_eq!(grid.contains_key(&Loc::from(5, 1, 254)), true);
        assert_eq!(grid.contains_key(&Loc::from(5, 2, 254)), true);

        bk.start.z -= 10;
        bk.end.z -= 10;
        bk.refresh_grid(&mut grid);

        assert_eq!(grid.len(), 3);
        assert_eq!(grid.contains_key(&Loc::from(5, 0, 254)), false);
        assert_eq!(grid.contains_key(&Loc::from(5, 1, 254)), false);
        assert_eq!(grid.contains_key(&Loc::from(5, 2, 254)), false);
        assert_eq!(grid.contains_key(&Loc::from(5, 0, 244)), true);
        assert_eq!(grid.contains_key(&Loc::from(5, 1, 244)), true);
        assert_eq!(grid.contains_key(&Loc::from(5, 2, 244)), true);

        let mut grid = HashMap::new();

        let line = "2,1,36~2,1,38";
        let mut bk = Brick::parse(line, 0);
        bk.refresh_grid(&mut grid);

        assert_eq!(grid.len(), 3);
        assert_eq!(grid.contains_key(&Loc::from(2, 1, 36)), true);
        assert_eq!(grid.contains_key(&Loc::from(2, 1, 37)), true);
        assert_eq!(grid.contains_key(&Loc::from(2, 1, 38)), true);

        bk.start.z -= 10;
        bk.end.z -= 10;
        bk.refresh_grid(&mut grid);

        assert_eq!(grid.len(), 3);
        assert_eq!(grid.contains_key(&Loc::from(2, 1, 36)), false);
        assert_eq!(grid.contains_key(&Loc::from(2, 1, 37)), false);
        assert_eq!(grid.contains_key(&Loc::from(2, 1, 38)), false);
        assert_eq!(grid.contains_key(&Loc::from(2, 1, 26)), true);
        assert_eq!(grid.contains_key(&Loc::from(2, 1, 27)), true);
        assert_eq!(grid.contains_key(&Loc::from(2, 1, 28)), true);
    }

    #[test]
    fn remove_from_grid() {
        let mut grid = HashMap::new();

        let line = "5,0,254~5,2,254";
        let bk = Brick::parse(line, 0);
        bk.refresh_grid(&mut grid);
        let line2 = "5,2,300~5,5,300";
        let bk2 = Brick::parse(line2, 0);
        bk2.refresh_grid(&mut grid);
        assert!(!grid.is_empty());

        bk.remove_from_grid(&mut grid);
        assert_eq!(grid.contains_key(&Loc::from(5, 0, 254)), false);
        assert_eq!(grid.contains_key(&Loc::from(5, 1, 254)), false);
        assert_eq!(grid.contains_key(&Loc::from(5, 2, 254)), false);
        assert_eq!(grid.contains_key(&Loc::from(5, 2, 300)), true);
        assert_eq!(grid.contains_key(&Loc::from(5, 3, 300)), true);
        assert_eq!(grid.contains_key(&Loc::from(5, 4, 300)), true);
        assert_eq!(grid.contains_key(&Loc::from(5, 5, 300)), true);
    }

    #[test]
    fn test_can_move() {
        let mut grid = HashMap::new();
        let line = "5,0,254~5,2,254";
        let mut bk = Brick::parse(line, 0);
        bk.fall(&mut grid);
        let line2 = "5,2,300~5,5,300";
        let mut bk2 = Brick::parse(line2, 0);
        bk2.fall(&mut grid);

        assert!(!bk.can_move(&grid));
        assert!(bk2.can_move(&grid));
    }

    #[test]
    fn test_fall() {
        let mut grid = HashMap::new();

        let line = "5,0,254~5,2,254";
        let mut bk = Brick::parse(line, 0);
        bk.refresh_grid(&mut grid);

        let fell = bk.fall(&mut grid);

        assert!(fell);
        assert_eq!(grid.len(), 3);
        assert_eq!(grid.contains_key(&Loc::from(5, 0, 1)), true);
        assert_eq!(grid.contains_key(&Loc::from(5, 1, 1)), true);
        assert_eq!(grid.contains_key(&Loc::from(5, 2, 1)), true);

        let line2 = "5,2,300~5,5,300";
        let mut bk2 = Brick::parse(line2, 0);
        let fell = bk2.fall(&mut grid);
        assert!(fell);
        assert_eq!(grid.len(), 7);
        assert_eq!(grid.contains_key(&Loc::from(5, 0, 1)), true);
        assert_eq!(grid.contains_key(&Loc::from(5, 1, 1)), true);
        assert_eq!(grid.contains_key(&Loc::from(5, 2, 1)), true);
        assert_eq!(grid.contains_key(&Loc::from(5, 2, 2)), true);
        assert_eq!(grid.contains_key(&Loc::from(5, 3, 2)), true);
        assert_eq!(grid.contains_key(&Loc::from(5, 4, 2)), true);
        assert_eq!(grid.contains_key(&Loc::from(5, 5, 2)), true);
    }
}
