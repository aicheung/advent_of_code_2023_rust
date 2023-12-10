use std::{
    cmp::max,
    collections::HashMap,
    fs, thread,
};

const GRID_MAX: u64 = 139;

fn is_cell_exist(cell: &(u64, u64)) -> bool {
    max(GRID_MAX, max(cell.0, cell.1)) <= GRID_MAX
}

fn add_connection_if_exist(
    row: u64,
    col: u64,
    target_row: Option<u64>,
    target_col: Option<u64>,
    map: &mut HashMap<(u64, u64), Vec<(u64, u64)>>,
) {
    map.entry((row, col)).or_insert(Vec::new());

    if target_row.is_some() && target_col.is_some() {
        let target = (target_row.unwrap(), target_col.unwrap());
        if is_cell_exist(&target) {
            map.entry((row, col)).and_modify(|a| a.push(target));
        }
    }
}

fn add_connections(
    row: u64,
    col: u64,
    symbol: char,
    map: &mut HashMap<(u64, u64), Vec<(u64, u64)>>,
) {
    let cur_row = Some(row);
    let cur_col = Some(col);
    let up_row = row.checked_sub(1);
    let down_row = Some(row + 1);
    let left_col = col.checked_sub(1);
    let right_col = Some(col + 1);

    match symbol {
        '|' => {
            add_connection_if_exist(row, col, up_row, cur_col, map);
            add_connection_if_exist(row, col, down_row, cur_col, map);
        }
        '-' => {
            add_connection_if_exist(row, col, cur_row, left_col, map);
            add_connection_if_exist(row, col, cur_row, right_col, map);
        }
        'L' => {
            add_connection_if_exist(row, col, up_row, cur_col, map);
            add_connection_if_exist(row, col, cur_row, right_col, map);
        }
        'J' => {
            add_connection_if_exist(row, col, up_row, cur_col, map);
            add_connection_if_exist(row, col, cur_row, left_col, map);
        }
        '7' => {
            add_connection_if_exist(row, col, down_row, cur_col, map);
            add_connection_if_exist(row, col, cur_row, left_col, map);
        }
        'F' => {
            add_connection_if_exist(row, col, down_row, cur_col, map);
            add_connection_if_exist(row, col, cur_row, right_col, map);
        }
        _ => {
            // no connection but still add mappings?
            map.entry((row, col)).or_insert(Vec::new());
        }
    }
}

fn find_s_mappings(s: Option<(u64, u64)>, map: &mut HashMap<(u64, u64), Vec<(u64, u64)>>) {
    let mut neighbours = Vec::new();
    if s.is_some() {
        for k in map.keys() {
            if map.get(k).unwrap().contains(&s.unwrap()) {
                neighbours.push(*k);
            }
        }

        map.entry(s.unwrap())
            .and_modify(|m| m.append(&mut neighbours));
    }
}

fn load_map(file: &str, map: &mut HashMap<(u64, u64), Vec<(u64, u64)>>) -> Option<(u64, u64)> {
    let mut s_loc: Option<(u64, u64)> = None;
    let lines = fs::read_to_string(file).expect("file not found");

    let split = lines.split('\n').collect::<Vec<_>>();

    let mut row = 0;
    for line in split {
        if line.len() < 2 {
            continue;
        }
        let mut col = 0;
        for c in line.chars().collect::<Vec<_>>() {
            add_connections(row, col, c, map);
            if c == 'S' {
                s_loc = Some((row, col));
            }

            col += 1;
        }

        row += 1;
    }
    s_loc
}

fn get_extra_line_elem(prev: (u64, u64), cur: (u64, u64)) -> (u64, u64) {
    let row_diff: i64 = cur.0 as i64 - prev.0 as i64;
    let col_diff: i64 = cur.1 as i64 - prev.1 as i64;

    (
        (prev.0 as i64 * 2 + 1 + row_diff) as u64,
        (prev.1 as i64 * 2 + 1 + col_diff) as u64,
    )
}

fn traverse_from_s(
    s: (u64, u64),
    map: &HashMap<(u64, u64), Vec<(u64, u64)>>,
    dist_from_s: &mut HashMap<(u64, u64), u64>,
) -> Vec<(u64, u64)> {
    // move from one starting pt until reaching the end
    // then move from another, overriding if the current dist is smaller
    // stop if we reach midway?
    let mut points_for_p2 = Vec::new();
    let starting_pts = map.get(&s).unwrap();

    points_for_p2.push((s.0 * 2 + 1, s.1 * 2 + 1));

    for pt in starting_pts {
        let mut dist = 1;
        let mut cur = pt.clone();
        let mut prev = s;

        while cur != s {
            //check if we already walked, if so no need to walk anymore
            if dist_from_s.contains_key(&cur) && dist >= *dist_from_s.get(&cur).unwrap() {
                break;
            }

            points_for_p2.push(get_extra_line_elem(prev, cur));
            points_for_p2.push((cur.0 * 2 + 1, cur.1 * 2 + 1));

            dist_from_s.insert(cur, dist);

            let next_steps = map
                .get(&cur)
                .unwrap()
                .iter()
                .filter(|c| **c != prev)
                .collect::<Vec<_>>();

            if next_steps.is_empty() {
                //should not happen!!!
                panic!("DEAD END!!!!");
            }

            prev = cur;
            cur = *next_steps[0];
            dist += 1;
        }
    }
    points_for_p2
}

/*
1. build map
2. build S mappings
3. traverse from S to all reachable cells?
4. find max
*/
fn p1(file: &str) -> u64 {
    let mut map: HashMap<(u64, u64), Vec<(u64, u64)>> = HashMap::new();
    let s_loc = load_map(file, &mut map);
    find_s_mappings(s_loc, &mut map);
    let mut dist_from_s: HashMap<(u64, u64), u64> = HashMap::new();

    traverse_from_s(s_loc.unwrap(), &map, &mut dist_from_s);
    *dist_from_s.values().max().unwrap()
}

fn write_loop(file: &str, p2_file: &str, points: Vec<(u64, u64)>) {
    let lines = fs::read_to_string(file).expect("file not found");
    let mut output = String::new();

    let split = lines.split('\n').collect::<Vec<_>>();

    let mut expanded_map: HashMap<(u64, u64), char> = HashMap::new();

    let mut row = 0;
    for line in split {
        if line.len() < 2 {
            continue;
        }
        let mut col = 0;
        for c in line.chars().collect::<Vec<_>>() {
            let mapped_row = row * 2 + 1;
            let mapped_col = col * 2 + 1;

            expanded_map.insert((mapped_row, mapped_col), c);

            col += 1;
        }
        row += 1;
    }

    for p in points {
        expanded_map.insert(p, '#');
    }

    for r in 0..GRID_MAX * 2 + 2 {
        for c in 0..GRID_MAX * 2 + 2 {
            if expanded_map.contains_key(&(r, c)) {
                output.push(*expanded_map.get(&(r, c)).unwrap());
            } else {
                output.push(' ');
            }
        }
        output.push('\n');
    }

    fs::write(p2_file, output).expect("Cannot write p2");
}

fn flood_fill_cell(cell: (u64, u64), map: &mut HashMap<(u64, u64), char>) {
    if map.contains_key(&cell) {
        let cur = *map.get(&cell).unwrap();
        if cur.eq(&'#') || cur.eq(&'A') {
            return;
        }
    }

    map.insert(cell, '#');
    let up = cell.0.checked_sub(1);
    let down = if cell.0 >= GRID_MAX * 2 + 2 {
        None
    } else {
        Some(cell.0 + 1)
    };
    let left = cell.1.checked_sub(1);
    let right = if cell.1 >= GRID_MAX * 2 + 2 {
        None
    } else {
        Some(cell.1 + 1)
    };

    if up.is_some() {
        flood_fill_cell((up.unwrap(), cell.1), map);
    }
    if left.is_some() {
        flood_fill_cell((cell.0, left.unwrap()), map);
    }
    if right.is_some() {
        flood_fill_cell((cell.0, right.unwrap()), map);
    }
    if down.is_some() {
        flood_fill_cell((down.unwrap(), cell.1), map);
    }
}

fn flood_fill(file: &str) -> u64 {
    let lines = fs::read_to_string(file).expect("file not found");

    let split = lines.split('\n').collect::<Vec<_>>();

    let mut expanded_map: HashMap<(u64, u64), char> = HashMap::new();

    let mut row = 0;
    for line in split {
        if line.len() < 2 {
            continue;
        }
        let mut col = 0;
        for c in line.chars().collect::<Vec<_>>() {
            expanded_map.insert((row, col), c);

            col += 1;
        }
        row += 1;
    }

    flood_fill_cell((0, 0), &mut expanded_map);

    let mut output = String::new();
    for r in 0..GRID_MAX * 2 + 2 {
        for c in 0..GRID_MAX * 2 + 2 {
            if expanded_map.contains_key(&(r, c)) {
                output.push(*expanded_map.get(&(r, c)).unwrap());
            } else {
                output.push(' ');
            }
        }
        output.push('\n');
    }
    fs::write("data/day10_ff.txt", output).expect("cannot write flood result");
    expanded_map.values().filter(|c| **c != '#' && **c != ' ').count() as u64
}

fn p2(file: &str, p2_file: &str) -> u64 {
    let mut map: HashMap<(u64, u64), Vec<(u64, u64)>> = HashMap::new();
    let s_loc = load_map(file, &mut map);
    find_s_mappings(s_loc, &mut map);
    let mut dist_from_s: HashMap<(u64, u64), u64> = HashMap::new();

    let points = traverse_from_s(s_loc.unwrap(), &map, &mut dist_from_s);

    write_loop(file, p2_file, points);
    file.to_string().push_str(".ff");
    flood_fill(p2_file)
}

const STACK_SIZE: usize = 40 * 1024 * 1024;
fn run() {
    let file = "data/day10.txt";
    let file_p2 = "data/day10_p2.txt";
    println!("p1: {}", p1(file));
    println!("p2: {}", p2(file, file_p2));
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
    use super::*;

    #[test]
    fn test_cell_exist() {
        let c = (0, 0);
        assert!(is_cell_exist(&c));
        let c = (GRID_MAX, 0);
        assert!(is_cell_exist(&c));
        let c = (0, GRID_MAX);
        assert!(is_cell_exist(&c));
        let c = (GRID_MAX, GRID_MAX);
        assert!(is_cell_exist(&c));
        let c = (0, GRID_MAX + 1);
        assert!(!is_cell_exist(&c));
        let c = (GRID_MAX + 1, 0);
        assert!(!is_cell_exist(&c));
    }

    #[test]
    fn test_add_connection() {
        let mut map: HashMap<(u64, u64), Vec<(u64, u64)>> = HashMap::new();
        add_connection_if_exist(1, 1, Some(0), Some(1), &mut map);
        assert_eq!(map.len(), 1);
        assert_eq!(map.contains_key(&(1, 1)), true);
        assert_eq!(map.get(&(1, 1)).unwrap().len(), 1);

        let mut map: HashMap<(u64, u64), Vec<(u64, u64)>> = HashMap::new();
        add_connection_if_exist(0, 0, None, Some(0), &mut map);
        assert_eq!(map.len(), 1);
        assert_eq!(map.contains_key(&(0, 0)), true);
        assert_eq!(map.get(&(0, 0)).unwrap().len(), 0);

        let mut map: HashMap<(u64, u64), Vec<(u64, u64)>> = HashMap::new();
        add_connection_if_exist(GRID_MAX, GRID_MAX, Some(GRID_MAX), None, &mut map);
        assert_eq!(map.len(), 1);
        assert_eq!(map.contains_key(&(GRID_MAX, GRID_MAX)), true);
        assert_eq!(map.get(&(GRID_MAX, GRID_MAX)).unwrap().len(), 0);

        let mut map: HashMap<(u64, u64), Vec<(u64, u64)>> = HashMap::new();
        add_connection_if_exist(
            GRID_MAX,
            GRID_MAX,
            Some(GRID_MAX + 1),
            Some(GRID_MAX),
            &mut map,
        );
        assert_eq!(map.len(), 1);
        assert_eq!(map.contains_key(&(GRID_MAX, GRID_MAX)), true);
        assert_eq!(map.get(&(GRID_MAX, GRID_MAX)).unwrap().len(), 0);
    }

    #[test]
    fn test_add_connection_by_symbol() {
        let mut map: HashMap<(u64, u64), Vec<(u64, u64)>> = HashMap::new();
        add_connections(1, 1, '|', &mut map);
        assert_eq!(map.len(), 1);
        assert_eq!(map.contains_key(&(1, 1)), true);
        assert_eq!(map.get(&(1, 1)).unwrap().len(), 2);
        assert_eq!(map.get(&(1, 1)).unwrap().contains(&(0, 1)), true);
        assert_eq!(map.get(&(1, 1)).unwrap().contains(&(2, 1)), true);

        let mut map: HashMap<(u64, u64), Vec<(u64, u64)>> = HashMap::new();
        add_connections(1, 1, '-', &mut map);
        assert_eq!(map.len(), 1);
        assert_eq!(map.contains_key(&(1, 1)), true);
        assert_eq!(map.get(&(1, 1)).unwrap().len(), 2);
        assert_eq!(map.get(&(1, 1)).unwrap().contains(&(1, 0)), true);
        assert_eq!(map.get(&(1, 1)).unwrap().contains(&(1, 2)), true);

        let mut map: HashMap<(u64, u64), Vec<(u64, u64)>> = HashMap::new();
        add_connections(1, 1, 'L', &mut map);
        assert_eq!(map.len(), 1);
        assert_eq!(map.contains_key(&(1, 1)), true);
        assert_eq!(map.get(&(1, 1)).unwrap().len(), 2);
        assert_eq!(map.get(&(1, 1)).unwrap().contains(&(0, 1)), true);
        assert_eq!(map.get(&(1, 1)).unwrap().contains(&(1, 2)), true);

        let mut map: HashMap<(u64, u64), Vec<(u64, u64)>> = HashMap::new();
        add_connections(1, 1, 'J', &mut map);
        assert_eq!(map.len(), 1);
        assert_eq!(map.contains_key(&(1, 1)), true);
        assert_eq!(map.get(&(1, 1)).unwrap().len(), 2);
        assert_eq!(map.get(&(1, 1)).unwrap().contains(&(0, 1)), true);
        assert_eq!(map.get(&(1, 1)).unwrap().contains(&(1, 0)), true);

        let mut map: HashMap<(u64, u64), Vec<(u64, u64)>> = HashMap::new();
        add_connections(1, 1, '7', &mut map);
        assert_eq!(map.len(), 1);
        assert_eq!(map.contains_key(&(1, 1)), true);
        assert_eq!(map.get(&(1, 1)).unwrap().len(), 2);
        assert_eq!(map.get(&(1, 1)).unwrap().contains(&(2, 1)), true);
        assert_eq!(map.get(&(1, 1)).unwrap().contains(&(1, 0)), true);

        let mut map: HashMap<(u64, u64), Vec<(u64, u64)>> = HashMap::new();
        add_connections(1, 1, 'F', &mut map);
        assert_eq!(map.len(), 1);
        assert_eq!(map.contains_key(&(1, 1)), true);
        assert_eq!(map.get(&(1, 1)).unwrap().len(), 2);
        assert_eq!(map.get(&(1, 1)).unwrap().contains(&(2, 1)), true);
        assert_eq!(map.get(&(1, 1)).unwrap().contains(&(1, 2)), true);

        let mut map: HashMap<(u64, u64), Vec<(u64, u64)>> = HashMap::new();
        add_connections(1, 1, '.', &mut map);
        assert_eq!(map.len(), 1);
        assert_eq!(map.contains_key(&(1, 1)), true);
        assert_eq!(map.get(&(1, 1)).unwrap().len(), 0);

        let mut map: HashMap<(u64, u64), Vec<(u64, u64)>> = HashMap::new();
        add_connections(1, 1, 'S', &mut map);
        assert_eq!(map.len(), 1);
        assert_eq!(map.contains_key(&(1, 1)), true);
        assert_eq!(map.get(&(1, 1)).unwrap().len(), 0);
    }

    #[test]
    fn test_load_map() {
        let file = "data/day10_ex.txt";
        let mut map: HashMap<(u64, u64), Vec<(u64, u64)>> = HashMap::new();

        load_map(file, &mut map);

        assert_eq!(map.get(&(1, 1)).unwrap().len(), 0); //s
        assert_eq!(map.get(&(1, 2)).unwrap().len(), 2);
        assert_eq!(map.get(&(1, 3)).unwrap().len(), 2);
        assert_eq!(map.get(&(2, 1)).unwrap().len(), 2);
        assert_eq!(map.get(&(2, 2)).unwrap().len(), 0); //.
        assert_eq!(map.get(&(2, 3)).unwrap().len(), 2);
        assert_eq!(map.get(&(3, 1)).unwrap().len(), 2);
        assert_eq!(map.get(&(3, 2)).unwrap().len(), 2);
        assert_eq!(map.get(&(3, 3)).unwrap().len(), 2);
        assert_eq!(map.get(&(4, 1)).unwrap().len(), 0);
        assert_eq!(map.get(&(4, 2)).unwrap().len(), 0);
        assert_eq!(map.get(&(4, 3)).unwrap().len(), 0);
    }

    #[test]
    fn test_find_s() {
        let file = "data/day10_ex.txt";
        let mut map: HashMap<(u64, u64), Vec<(u64, u64)>> = HashMap::new();
        let s = load_map(file, &mut map);
        find_s_mappings(s, &mut map);

        assert_eq!(map.get(&(1, 1)).unwrap().len(), 2); //s
        assert_eq!(map.get(&(1, 1)).unwrap().contains(&(1, 2)), true);
        assert_eq!(map.get(&(1, 1)).unwrap().contains(&(2, 1)), true);
    }

    #[test]
    fn test_move_from_s() {
        let file = "data/day10_ex.txt";
        let mut map: HashMap<(u64, u64), Vec<(u64, u64)>> = HashMap::new();
        let s = load_map(file, &mut map);
        let mut dist_from_s: HashMap<(u64, u64), u64> = HashMap::new();
        find_s_mappings(s, &mut map);

        traverse_from_s(s.unwrap(), &map, &mut dist_from_s);
        assert_eq!(*dist_from_s.get(&(3, 3)).unwrap(), 4);
        assert_eq!(*dist_from_s.get(&(3, 2)).unwrap(), 3);
        assert_eq!(*dist_from_s.get(&(2, 3)).unwrap(), 3);
        assert_eq!(*dist_from_s.get(&(3, 1)).unwrap(), 2);
        assert_eq!(*dist_from_s.get(&(1, 3)).unwrap(), 2);
        assert_eq!(*dist_from_s.get(&(1, 2)).unwrap(), 1);
        assert_eq!(*dist_from_s.get(&(2, 1)).unwrap(), 1);
    }

    #[test]
    fn test_p1() {
        let file = "data/day10_ex.txt";
        assert_eq!(p1(file), 4);
    }

    #[test]
    fn test_mapped_extra() {
        let prev = (0, 0);
        let cur = (1, 0);
        assert_eq!(get_extra_line_elem(prev, cur), (2, 1));

        let prev = (0, 0);
        let cur = (0, 1);
        assert_eq!(get_extra_line_elem(prev, cur), (1, 2));

        let prev = (1, 1);
        let cur = (0, 1);
        assert_eq!(get_extra_line_elem(prev, cur), (2, 3));

        let prev = (1, 1);
        let cur = (1, 0);
        assert_eq!(get_extra_line_elem(prev, cur), (3, 2));
    }
}
