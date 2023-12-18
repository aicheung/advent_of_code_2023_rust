use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::read_to_string;
use std::hash::*;

enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

// Define a struct for a vertex with a name and a distance
#[derive(Eq, Clone)]
struct Vertex {
    name: String,
    pos: (i32, i32),
    distance: i32,
}


// Implement the Hash trait for Vertex
impl<'a> Hash for Vertex {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

// Implement the Ord trait for Vertex
// Use reverse ordering so that the smallest distance is the highest priority
impl<'a> Ord for Vertex {
    fn cmp(&self, other: &Vertex) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

// Implement the PartialOrd trait for Vertex
impl<'a> PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Vertex) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Implement the PartialEq trait for Vertex
impl<'a> PartialEq for Vertex {
    fn eq(&self, other: &Vertex) -> bool {
        self.name == other.name
    }
}

fn get_cell(row: i32, col: i32, grid: &HashMap<Vertex, Vec<(Vertex, i32)>>) -> &Vertex {
    grid.iter().find(|v| v.0.pos == (row, col)).unwrap().0
}

struct Search {
    dist: i32,
    vert: Vertex,
    row_dir: i32,
    col_dir: i32,
    cur_dir_steps: i32,
}

impl PartialEq for Search {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
            && self.vert == other.vert
            && self.row_dir == other.row_dir
            && self.col_dir == other.col_dir
            && self.cur_dir_steps == other.cur_dir_steps
    }
}

impl Eq for Search {}

impl Ord for Search {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for Search {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra<'a>(
    source: &'a Vertex,
    destination: &'a Vertex,
    adjacency_list: &'a HashMap<Vertex, Vec<(Vertex, i32)>>,
    is_p2: bool
) -> Option<i32> {
    let max_row = adjacency_list.iter().map(|v| v.0.pos.0).max().unwrap();
    let max_col = adjacency_list.iter().map(|v| v.0.pos.1).max().unwrap();
    let mut seen = HashSet::new();

    let mut to_visit: BinaryHeap<Search> = BinaryHeap::new();
    let init = Search {
        dist: 0,
        vert: source.clone(),
        row_dir: 0,
        col_dir: 0,
        cur_dir_steps: 0,
    };
    to_visit.push(init);
    // Loop until the binary heap is empty or the destination is visited
    while to_visit.len() > 0 {
        //to_visit.sort_by(|v, v2| v2.0.cmp(&v.0));
        let vertex = to_visit.pop().unwrap();

        // If the vertex is the destination, break the loop
        if vertex.vert.pos == destination.pos && (!is_p2 || vertex.cur_dir_steps >= 4){
            return Some(vertex.dist);
        }

        let seen_key = (
            vertex.vert.pos.0,
            vertex.vert.pos.1,
            vertex.row_dir,
            vertex.col_dir,
            vertex.cur_dir_steps,
        );
        if seen.contains(&seen_key) {
            continue;
        }

        seen.insert(seen_key);

        let max_straight_step = if !is_p2 {3} else {10};
        if vertex.cur_dir_steps < max_straight_step && (vertex.row_dir, vertex.col_dir) != (0, 0) {
            let next_row = vertex.vert.pos.0.checked_add(vertex.row_dir);
            let next_col = vertex.vert.pos.1.checked_add(vertex.col_dir);

            if next_row.is_some_and(|r| 0 <= r && r <= max_row)
                && next_col.is_some_and(|c| 0 <= c && c <= max_col)
            {
                let next = get_cell(next_row.unwrap(), next_col.unwrap(), adjacency_list);
                let next_dist = next.distance;
                let search = Search {
                    dist: vertex.dist + next_dist,
                    vert: next.clone(),
                    row_dir: vertex.row_dir,
                    col_dir: vertex.col_dir,
                    cur_dir_steps: vertex.cur_dir_steps + 1,
                };
                to_visit.push(search);
            }
        }

        if !is_p2 || (is_p2 && (vertex.cur_dir_steps >= 4 || (vertex.row_dir, vertex.col_dir) == (0, 0))) {
            for (row_dir, col_dir) in vec![(0, 1), (1, 0), (0, -1), (-1, 0)] {
                if (row_dir, col_dir) != (vertex.row_dir, vertex.col_dir)
                    && (row_dir, col_dir) != (-vertex.row_dir, -vertex.col_dir)
                {
                    let next_row = vertex.vert.pos.0.checked_add(row_dir);
                    let next_col = vertex.vert.pos.1.checked_add(col_dir);
    
                    if next_row.is_some_and(|r| 0 <= r && r <= max_row)
                        && next_col.is_some_and(|c| 0 <= c && c <= max_col)
                    {
                        let next = get_cell(next_row.unwrap(), next_col.unwrap(), adjacency_list);
                        let next_dist = next.distance;
                        let search = Search {
                            dist: vertex.dist + next_dist,
                            vert: next.clone(),
                            row_dir,
                            col_dir,
                            cur_dir_steps: 1,
                        };
                        to_visit.push(search);
                    }
                }
            }
        }
    }
    None
}

fn find_neighbour<'a>(v: &'a Vertex, vtx: &'a Vec<Vertex>, dir: Direction) -> Option<Vertex> {
    let target_col: Option<i32>;
    let target_row: Option<i32>;

    match dir {
        Direction::WEST => {
            target_row = Some(v.pos.0);
            target_col = v.pos.1.checked_sub(1);
        }
        Direction::EAST => {
            target_row = Some(v.pos.0);
            target_col = v.pos.1.checked_add(1);
        }
        Direction::NORTH => {
            target_row = v.pos.0.checked_sub(1);
            target_col = Some(v.pos.1);
        }
        Direction::SOUTH => {
            target_row = v.pos.0.checked_add(1);
            target_col = Some(v.pos.1);
        }
    }

    if target_row.is_none() || target_col.is_none() {
        return None;
    }

    let target = vtx
        .iter()
        .find(|v| v.pos.0 == target_row.unwrap() && v.pos.1 == target_col.unwrap());
    if target.is_some() {
        Some(target.unwrap().clone())
    } else {
        None
    }
}

fn load_grid(file: &str) -> HashMap<Vertex, Vec<(Vertex, i32)>> {
    let str = read_to_string(file).expect("cannot open");
    let mut vtx: Vec<Vertex> = Vec::new();
    let mut grid = HashMap::new();

    let mut i = 0;
    for l in str.split_whitespace() {
        let mut j = 0;
        for c in l.chars() {
            let v = Vertex {
                name: format!("{}:{}", i, j),
                pos: (i, j),
                distance: c.to_digit(10).unwrap() as i32,
            };
            vtx.push(v);
            j += 1;
        }
        i += 1;
    }

    let vertexes = &vtx;
    for v in vertexes.iter() {
        let east: Option<Vertex> = find_neighbour(&v, vertexes, Direction::EAST);
        let west: Option<Vertex> = find_neighbour(&v, vertexes, Direction::WEST);
        let north: Option<Vertex> = find_neighbour(&v, vertexes, Direction::NORTH);
        let south: Option<Vertex> = find_neighbour(&v, vertexes, Direction::SOUTH);

        let mut adj_list = Vec::new();
        if east.is_some() {
            let e = east.unwrap();
            let dist = e.distance;
            adj_list.push((e, dist));
        }
        if west.is_some() {
            let w = west.unwrap();
            let dist = w.distance;
            adj_list.push((w, dist));
        }
        if north.is_some() {
            let n = north.unwrap();
            let dist = n.distance;
            adj_list.push((n, dist));
        }
        if south.is_some() {
            let s = south.unwrap();
            let dist = s.distance;
            adj_list.push((s, dist));
        }
        grid.insert(v.clone(), adj_list);
    }

    grid
}

fn main() {
    let file = "data/day17.txt";

    let grid: HashMap<Vertex, Vec<(Vertex, i32)>> = load_grid(file);
    let max_row = grid.iter().map(|v| v.0.pos.0).max().unwrap();
    let max_col = grid.iter().map(|v| v.0.pos.1).max().unwrap();
    let topleft = grid.iter().find(|(v, _adj)| v.pos.0 == 0 && v.pos.1 == 0);
    let tgt = grid.iter().find(|(v, _adj)| v.pos.0 == max_row && v.pos.1 == max_col);
    let result = dijkstra(&topleft.unwrap().0, &tgt.unwrap().0, &grid, false);
    let r = result.unwrap();
    println!("p1: {}", r);
    let result = dijkstra(&topleft.unwrap().0, &tgt.unwrap().0, &grid, true);
    let r = result.unwrap();
    println!("p2: {}", r);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_load() {
        let file = "data/day17_ex.txt";

        let grid: HashMap<Vertex, Vec<(Vertex, i32)>> = load_grid(file);

        assert_eq!(grid.len(), 13 * 13);
        let topleft = grid.iter().find(|(v, _adj)| v.pos.0 == 0 && v.pos.1 == 0);
        assert!(topleft.is_some());
        assert!(topleft.unwrap().1.len() == 2);
        let oneone = grid.iter().find(|(v, _adj)| v.pos.0 == 12 && v.pos.1 == 12);
        assert!(oneone.is_some());
        assert!(oneone.unwrap().1.len() == 4);
    }

    #[test]
    fn path_find() {
        //should find something with raw dijkstra
        let file = "data/day17_ex.txt";

        let grid: HashMap<Vertex, Vec<(Vertex, i32)>> = load_grid(file);
        let topleft = grid.iter().find(|(v, _adj)| v.pos.0 == 0 && v.pos.1 == 0);
        assert!(topleft.is_some());
        let tgt = grid.iter().find(|(v, _adj)| v.pos.0 == 12 && v.pos.1 == 12);
        //let tgt = grid.iter().find(|(v, adj)| v.pos.0 == 0 && v.pos.1 == 2);
        assert!(tgt.is_some());
        let result = dijkstra(&topleft.unwrap().0, &tgt.unwrap().0, &grid, false);
        assert!(result.is_some());
        let r = result.unwrap();
        assert_eq!(r, 102);
        let result = dijkstra(&topleft.unwrap().0, &tgt.unwrap().0, &grid, true);
        let r = result.unwrap();
        assert_eq!(r, 94);
    }
}
