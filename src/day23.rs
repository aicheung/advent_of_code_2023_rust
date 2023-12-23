use std::cmp::max;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

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

fn can_walk(cell: char, on_slope: bool, dir: Direction) -> bool {
    if !on_slope {
        return true;
    }

    if dir == Direction::WEST && cell == '>' {
        return false;
    }

    if dir == Direction::EAST && cell == '<' {
        return false;
    }

    if dir == Direction::SOUTH {
        return true; // south always downslope? can't find upslope in example and data
    }

    if dir == Direction::NORTH && cell == 'v' {
        return false;
    }
    true
}

fn bfs(
    start: (u64, u64),
    end: (u64, u64),
    grid: &HashMap<(u64, u64), char>,
    is_p1: bool,
    points: &Vec<(u64, u64)>,
) -> HashSet<(i32, Vec<(u64, u64)>)> {
    let mut out = HashSet::new();
    let mut to_visit = VecDeque::new();
    //let mut seen = HashSet::new();
    to_visit.push_back((0, start, false, Direction::SOUTH, HashSet::new(), vec![]));

    while !to_visit.is_empty() {
        let cur = to_visit.pop_front().unwrap();
        let loc = cur.1;
        let cur_steps = cur.0;
        let mut on_slope = cur.2;
        let cur_dir = cur.3;
        let mut seen = cur.4;
        let mut path = cur.5;

        if !grid.contains_key(&loc)
            || grid.get(&loc).expect("cannot get").eq(&'#')
            || seen.contains(&loc)
        {
            continue;
        }

        let cur_tile = *grid.get(&loc).expect("cannot get");
        if is_p1 && on_slope && !can_walk(cur_tile, on_slope, cur_dir) {
            continue;
        }
        seen.insert(loc);
        path.push(loc);
        //valid tile, toggle on_slope if stepped on slope
        if cur_tile != '.' {
            on_slope = !on_slope;
        }

        if loc == end {
            //found
            out.insert((cur_steps, path));
            if !is_p1 {
                break; // need to find the shortest path
            }
            //break;
            continue;
        }

        if !is_p1 && points.contains(&loc) && loc != start && loc != end {
            continue;
        }

        let up: Option<(u64, u64)> = get_neighbour(loc, -1, 0);
        let down: Option<(u64, u64)> = get_neighbour(loc, 1, 0);
        let left: Option<(u64, u64)> = get_neighbour(loc, 0, -1);
        let right: Option<(u64, u64)> = get_neighbour(loc, 0, 1);

        let contains = |l| {
            !seen.contains(&l)
                && grid.contains_key(&l)
                && !grid.get(&l).expect("cannot get").eq(&'#')
        };
        if up.is_some_and(contains) {
            to_visit.push_back((
                cur_steps + 1,
                up.expect(""),
                on_slope,
                Direction::NORTH,
                seen.clone(),
                path.clone(),
            ));
        }
        if down.is_some_and(contains) {
            to_visit.push_back((
                cur_steps + 1,
                down.expect(""),
                on_slope,
                Direction::SOUTH,
                seen.clone(),
                path.clone(),
            ));
        }
        if left.is_some_and(contains) {
            to_visit.push_back((
                cur_steps + 1,
                left.expect(""),
                on_slope,
                Direction::WEST,
                seen.clone(),
                path.clone(),
            ));
        }
        if right.is_some_and(contains) {
            to_visit.push_back((
                cur_steps + 1,
                right.expect(""),
                on_slope,
                Direction::EAST,
                seen.clone(),
                path.clone(),
            ));
        }
    }

    out
}

fn find_longest_path(file: &str, is_p1: bool) -> u64 {
    let mut grid = HashMap::new();
    load_grid(file, &mut grid);

    let result = bfs((0, 1), (140, 139), &grid, is_p1, &vec![]);

    let max = result.iter().map(|r| r.0).max().unwrap();
    max as u64
}

fn p1(file: &str) -> u64 {
    find_longest_path(file, true)
}

fn find_intersections(grid: &HashMap<(u64, u64), char>, points: &mut Vec<(u64, u64)>, size: u64) {
    for r in 0..size {
        for c in 0..size {
            let loc = (r, c);
            if grid.get(&loc).unwrap().ne(&'.') {
                continue;
            }
            let mut total_non_wall = 0;
            let up: Option<(u64, u64)> = get_neighbour(loc, -1, 0);
            let down: Option<(u64, u64)> = get_neighbour(loc, 1, 0);
            let left: Option<(u64, u64)> = get_neighbour(loc, 0, -1);
            let right: Option<(u64, u64)> = get_neighbour(loc, 0, 1);

            for pt in vec![up, down, left, right] {
                if pt.is_some() {
                    let point = pt.unwrap();
                    if grid.contains_key(&point) && grid.get(&point).unwrap().ne(&'#') {
                        total_non_wall += 1;
                    }
                }
            }

            if total_non_wall >= 3 {
                points.push(loc.clone());
            }
        }
    }
}

fn build_graph(
    grid: &HashMap<(u64, u64), char>,
    points: &Vec<(u64, u64)>,
    graph: &mut HashMap<(u64, u64), HashSet<((u64, u64), u64)>>,
) {
    //load to graph first
    for p in points {
        graph.insert(*p, HashSet::new());
    }
    for p in points {
        let mut s = vec![(0, *p)];
        let mut seen = HashSet::new();

        while !s.is_empty() {
            let point = s.pop().unwrap();
            let loc = point.1;
            if point.0 != 0 && points.contains(&loc) {
                graph.get_mut(p).unwrap().insert((loc.clone(), point.0));
                continue;
            }

            let up: Option<(u64, u64)> = get_neighbour(loc, -1, 0);
            let down: Option<(u64, u64)> = get_neighbour(loc, 1, 0);
            let left: Option<(u64, u64)> = get_neighbour(loc, 0, -1);
            let right: Option<(u64, u64)> = get_neighbour(loc, 0, 1);

            for next in vec![up, down, left, right] {
                if next.is_some() && grid.contains_key(&next.unwrap()) {
                    let next_loc = next.unwrap();
                    let c = grid.get(&next_loc).unwrap();
                    if *c != '#' && !seen.contains(&next_loc) {
                        s.push((point.0 + 1, next_loc));
                        seen.insert(next_loc);
                    }
                }
            }
        }
    }
}

fn bfs_p2(
    start: (u64, u64),
    end: (u64, u64),
    graph: &HashMap<(u64, u64), HashSet<((u64, u64), u64)>>,
) -> u64 {
    let mut out: Vec<(u64, HashSet<(u64, u64)>)> = Vec::new();
    let mut m = 0;
    let mut to_visit = VecDeque::new();
    to_visit.push_back((start, 0, HashSet::new()));

    while !to_visit.is_empty() {
        let cur = to_visit.pop_front().unwrap();
        let loc = cur.0;
        let steps = cur.1;
        let mut seen = cur.2;

        if loc == end {
            out.push((steps, /*seen.clone()*/ HashSet::new()));
            m = max(m, steps);
            continue;
        }

        if seen.contains(&loc) {
            continue;
        }

        seen.insert(loc);
        for neighbour in graph.get(&loc).unwrap() {
            if !seen.contains(&neighbour.0) {
                to_visit.push_back((neighbour.0, steps + neighbour.1, seen.clone()));
            }
        }
    }
    println!("{:?} {}", m, out.len());
    m
}

fn p2(file: &str) -> u64 {
    let mut grid = HashMap::new();
    load_grid(file, &mut grid);
    let size = grid.iter().map(|a| a.0 .0).max().unwrap() + 1;

    let mut points = vec![(0, 1), (size - 1, size - 2)];
    find_intersections(&grid, &mut points, size);
    let mut graph: HashMap<(u64, u64), HashSet<((u64, u64), u64)>> = HashMap::new();

    build_graph(&grid, &points, &mut graph);

    //let result = dfs((0,1), (size - 1, size - 2), &graph, &mut HashSet::new());
    bfs_p2((0, 1), (size - 1, size - 2), &graph)
}

fn main() {
    let file = "data/day23.txt";
    println!("p1: {}", p1(file));
    println!("p2: {}", p2(file));
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_p2() {
        assert_eq!(p2("data/day23_ex.txt"), 154);
    }

    #[test]
    fn test_build_graph() {
        let file = "data/day23_ex.txt";
        let mut grid = HashMap::new();
        load_grid(file, &mut grid);

        let size = grid.iter().map(|a| a.0 .0).max().unwrap() + 1;

        let mut points = vec![(0, 1), (size - 1, size - 2)];
        find_intersections(&grid, &mut points, size);
        let mut graph: HashMap<(u64, u64), HashSet<((u64, u64), u64)>> = HashMap::new();

        build_graph(&grid, &points, &mut graph);

        let node: Vec<(u64, u64)> = graph.get(&(5, 3)).unwrap().iter().map(|n| n.0).collect();
        //println!("{:?}", node);
        //assert_eq!(node.len(), 3);
        assert!(node.contains(&(0, 1)));
        assert!(node.contains(&(13, 5)));
        assert!(node.contains(&(3, 11)));

        let node: Vec<(u64, u64)> = graph.get(&(0, 1)).unwrap().iter().map(|n| n.0).collect();

        //assert_eq!(node.len(), 1);
        assert!(node.contains(&(5, 3)));

        let node: Vec<(u64, u64)> = graph.get(&(22, 21)).unwrap().iter().map(|n| n.0).collect();

        //assert_eq!(node.len(), 1);
        assert!(node.contains(&(19, 19)));

        let node: Vec<(u64, u64)> = graph.get(&(19, 19)).unwrap().iter().map(|n| n.0).collect();

        //assert_eq!(node.len(), 3);
        assert!(node.contains(&(22, 21)));
        assert!(node.contains(&(19, 13)));
        assert!(node.contains(&(11, 21)));

        let node: Vec<(u64, u64)> = graph.get(&(11, 21)).unwrap().iter().map(|n| n.0).collect();

        //assert_eq!(node.len(), 3);
        assert!(node.contains(&(19, 19)));
        assert!(node.contains(&(13, 13)));
        assert!(node.contains(&(3, 11)));
    }

    #[test]
    fn test_bfs_p2() {
        let file = "data/day23_ex.txt";
        let mut grid = HashMap::new();
        load_grid(file, &mut grid);

        let src = (0, 1);
        let dst = (5, 3);
        let points = vec![
            (0, 1),
            (22, 21),
            (3, 11),
            (5, 3),
            (11, 21),
            (13, 5),
            (13, 13),
            (19, 13),
            (19, 19),
        ];
        let result = bfs(src, dst, &grid, false, &points);
        let result_rev = bfs(dst, src, &grid, false, &points);

        assert_eq!(result.len(), 1);
        assert_eq!(result_rev.len(), 1);
        //println!("{:?}", result);

        let src = (19, 19);
        let dst = (22, 21);
        let result = bfs(src, dst, &grid, false, &points);
        let result_rev = bfs(dst, src, &grid, false, &points);

        assert_eq!(result.len(), 1);
        assert_eq!(result_rev.len(), 1);

        let src = (5, 3);
        let dst = (13, 5);
        //multi path, must return the non-intersection one
        let result = bfs(src, dst, &grid, false, &points);
        let result_rev = bfs(dst, src, &grid, false, &points);

        assert_eq!(result.len(), 1);
        assert_eq!(result_rev.len(), 1);
        //println!("{:?}", result);

        //will go through one intersection, so none
        let src = (13, 5);
        let dst = (11, 21);
        let result = bfs(src, dst, &grid, false, &points);
        let result_rev = bfs(dst, src, &grid, false, &points);
        assert_eq!(result.len(), 0);
        assert_eq!(result_rev.len(), 0);
    }

    #[test]
    fn test_find_intersections() {
        let file = "data/day23_ex.txt";
        let mut grid = HashMap::new();
        load_grid(file, &mut grid);
        let size = grid.iter().map(|a| a.0 .0).max().unwrap() + 1;

        let mut points = vec![(0, 1), (size - 1, size - 2)];
        find_intersections(&grid, &mut points, size);

        assert!(points.len() > 2);
        println!("{:?}", points);
    }

    #[test]
    fn test_load() {
        let file = "data/day23_ex.txt";
        let mut grid = HashMap::new();
        load_grid(file, &mut grid);

        assert_eq!(grid.len(), 23 * 23);
    }

    #[test]
    fn test_pathfind() {
        let file = "data/day23_ex.txt";
        let mut grid = HashMap::new();
        load_grid(file, &mut grid);

        let result = bfs((0, 1), (22, 21), &grid, true, &vec![]);

        assert_ne!(result.len(), 0);
        for r in result.iter() {
            if r.0 > 94 {
                //why?
                println!("{} : {:?}", r.0, r.1);
            }
        }
        let max = result.iter().map(|r| r.0).max().unwrap();
        assert_eq!(max, 94);

        // p2
        //let result = bfs((0,1), (22,21), &grid, false, vec![]);
        //let max = result.iter().map(|r| r.0).max().unwrap();
        //assert_eq!(max, 154);
    }
}
