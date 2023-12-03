use std::fs;

struct Gear {
    adj_num_points: Vec<(usize, usize)>,
    adj_numbers: Vec<i32>,
}

fn fill_grid(data: String, grid: &mut Vec<Vec<char>>) {
    for r in data.split('\n') {
        if r.len() < 2 {
            continue; //empty
        }
        let mut row = Vec::new();
        for c in r.chars() {
            row.push(c);
        }
        grid.push(row);
    }
}

fn is_symbol(c: &char) -> bool {
    !"1234567890.".contains([*c])
}

fn get_adj_indexes(i: usize, j: usize, grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();
    let start_i = if i >= 1 { i - 1 } else { i };
    let start_j = if j >= 1 { j - 1 } else { j };
    let end_i = if i < grid.len() - 1 { i + 1 } else { i };
    let end_j = if j < grid[0].len() - 1 { j + 1 } else { j };

    for r in start_i..end_i + 1 {
        for c in start_j..end_j + 1 {
            if r == i && c == j {
                continue;
            }
            result.push((r, c));
        }
    }
    result
}

fn get_adj_symbols(i: usize, j: usize, grid: &Vec<Vec<char>>) -> Vec<char> {
    let mut result: Vec<char> = Vec::new();
    get_adj_indexes(i, j, &grid)
        .iter()
        .for_each(|a| result.push(grid[a.0][a.1]));
    result
}

fn has_adj_number(i: usize, j: usize, grid: &Vec<Vec<char>>) -> bool {
    let adj_symbols: Vec<char> = get_adj_symbols(i, j, &grid);
    let contains_digit = adj_symbols.iter().any(|&c| c.is_digit(10));
    contains_digit
}

fn erase_adj_number(
    i: usize,
    j: usize,
    grid: &Vec<Vec<char>>,
    tainted_numbers: &mut Vec<(usize, usize)>,
) {
    get_adj_indexes(i, j, &grid)
        .iter()
        .for_each(|a| tainted_numbers.push(*a));
}

fn is_tained_num(
    i: usize,
    num_st_end: (usize, usize),
    tainted_numbers: &Vec<(usize, usize)>,
) -> bool {
    for j in num_st_end.0..num_st_end.1 + 1 {
        let cell = (i, j);
        if tainted_numbers.contains(&cell) {
            return true;
        }
    }
    false
}

fn get_full_num(i: usize, j: usize, grid: &Vec<Vec<char>>) -> (usize, usize) {
    //first digit is j so loop until end/non-digit
    let mut end = j;
    for c in j..grid[i].len() {
        if grid[i][c].is_digit(10) {
            end = c;
        } else {
            break;
        }
    }

    (j, end)
}

fn parse_num(i: usize, num_st_end: (usize, usize), grid: &Vec<Vec<char>>) -> i32 {
    let mut num_chars = Vec::new();
    for c in num_st_end.0..num_st_end.1 + 1 {
        num_chars.push(grid[i][c]);
    }
    let num_str: String = num_chars.into_iter().collect();
    //println!("{:?}", num_st_end);
    num_str.parse::<i32>().expect(&num_str)
}

fn is_first_digit(i: usize, j: usize, grid: &Vec<Vec<char>>) -> bool {
    if j == 0 {
        return true;
    }
    !grid[i][j - 1].is_digit(10)
}

fn find_gear(i: usize, num_st_end: (usize, usize), gears: &mut Vec<Gear>) -> Option<&mut Gear> {
    for g in gears.iter_mut() {
        for c in num_st_end.0..num_st_end.1 + 1 {
            let pt = (i, c);
            if g.adj_num_points.contains(&pt) {
                return Some(g);
            }
        }
    }
    None
}

fn is_gear(c: char) -> bool {
    c == '*'
}

fn add_gear(i: usize, j: usize, grid: &Vec<Vec<char>>, gears: &mut Vec<Gear>) {
    let mut gear = Gear {
        adj_num_points: Vec::new(),
        adj_numbers: Vec::new(),
    };

    get_adj_indexes(i, j, &grid).iter().for_each(|p| {
        if grid[p.0][p.1].is_digit(10) {
            gear.adj_num_points.push(*p);
        }
    });

    gears.push(gear);
}

fn main() {
    let data = fs::read_to_string("data/day3.txt").expect("Cannot read file.");

    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut tainted_numbers: Vec<(usize, usize)> = Vec::new();
    let mut gears: Vec<Gear> = Vec::new();
    let mut p1_result = 0;
    let mut p2_result = 0;

    fill_grid(data, &mut grid);

    assert_eq!(grid.len(), 140);
    assert_eq!(grid[0].len(), 140);
    assert_eq!(grid[139].len(), 140);

    //println!("{:?}", grid);

    for (i, row) in grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if is_symbol(col) && has_adj_number(i, j, &grid) {
                erase_adj_number(i, j, &grid, &mut tainted_numbers);
                //p2
                if is_gear(*col) {
                    //is gear, add all adj number
                    add_gear(i, j, &grid, &mut gears);
                }
            }
        }
    }

    for (i, row) in grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if col.is_digit(10) && is_first_digit(i, j, &grid) {
                let num_st_end: (usize, usize) = get_full_num(i, j, &grid);
                if is_tained_num(i, num_st_end, &tainted_numbers) {
                    //target
                    p1_result += parse_num(i, num_st_end, &grid);
                }

                let gear = find_gear(i, num_st_end, &mut gears);
                if gear.is_some() {
                    let g = gear.unwrap();
                    g.adj_numbers.push(parse_num(i, num_st_end, &grid));
                }
            }
        }
    }

    //p2
    for g in gears {
        if g.adj_numbers.len() == 2 {
            p2_result += g.adj_numbers[0] * g.adj_numbers[1];
        }
    }

    println!("p1: {}", p1_result);
    println!("p2: {}", p2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_symbol() {
        let sym = '\"';
        assert!(is_symbol(&sym) == true);
        let sym = '-';
        assert!(is_symbol(&sym) == true);
        let sym = '@';
        assert!(is_symbol(&sym) == true);
        let sym = '#';
        assert!(is_symbol(&sym) == true);
        let sym = '$';
        assert!(is_symbol(&sym) == true);
        let sym = '%';
        assert!(is_symbol(&sym) == true);
        let sym = '&';
        assert!(is_symbol(&sym) == true);
        let sym = '*';
        assert!(is_symbol(&sym) == true);
        let sym = '-';
        assert!(is_symbol(&sym) == true);
    }

    #[test]
    fn test_has_adj_number() {
        let data = String::from(
            "1..\n\
            .$.\n\
            ...",
        );
        let mut grid: Vec<Vec<char>> = Vec::new();

        fill_grid(data, &mut grid);
        assert_eq!(has_adj_number(1, 1, &grid), true);

        let data = String::from(
            ".1.\n\
            .$.\n\
            ...",
        );
        let mut grid: Vec<Vec<char>> = Vec::new();

        fill_grid(data, &mut grid);
        assert_eq!(has_adj_number(1, 1, &grid), true);

        let data = String::from(
            "..1\n\
            .$.\n\
            ...",
        );
        let mut grid: Vec<Vec<char>> = Vec::new();

        fill_grid(data, &mut grid);
        assert_eq!(has_adj_number(1, 1, &grid), true);
        let data = String::from(
            "...\n\
            1$.\n\
            ...",
        );
        let mut grid: Vec<Vec<char>> = Vec::new();

        fill_grid(data, &mut grid);
        assert_eq!(has_adj_number(1, 1, &grid), true);
        let data = String::from(
            "...\n\
            .$1\n\
            ...",
        );
        let mut grid: Vec<Vec<char>> = Vec::new();

        fill_grid(data, &mut grid);
        assert_eq!(has_adj_number(1, 1, &grid), true);
        let data = String::from(
            "...\n\
            .$.\n\
            1..",
        );
        let mut grid: Vec<Vec<char>> = Vec::new();

        fill_grid(data, &mut grid);
        assert_eq!(has_adj_number(1, 1, &grid), true);
        let data = String::from(
            "...\n\
            .$.\n\
            .1.",
        );
        let mut grid: Vec<Vec<char>> = Vec::new();

        fill_grid(data, &mut grid);
        assert_eq!(has_adj_number(1, 1, &grid), true);
        let data = String::from(
            "...\n\
            .$.\n\
            ..1",
        );
        let mut grid: Vec<Vec<char>> = Vec::new();

        fill_grid(data, &mut grid);
        assert_eq!(has_adj_number(1, 1, &grid), true);

        let data = String::from(
            "...\n\
            .$.\n\
            ...",
        );
        let mut grid: Vec<Vec<char>> = Vec::new();

        fill_grid(data, &mut grid);
        assert_eq!(has_adj_number(1, 1, &grid), false);
    }

    #[test]
    fn test_get_adj_symbols() {
        let data = String::from(
            "1..\n\
            .$.\n\
            ...",
        );
        let mut grid: Vec<Vec<char>> = Vec::new();
        fill_grid(data, &mut grid);
        let result = get_adj_symbols(1, 1, &grid);
        assert_eq!(result[0], '1');
        assert_eq!(result.len(), 8);
    }

    #[test]
    fn test_erase_adj_number() {
        let mut tainted_numbers = Vec::new();
        let data = String::from(
            "1..\n\
            .$.\n\
            ...",
        );
        let mut grid: Vec<Vec<char>> = Vec::new();
        fill_grid(data, &mut grid);
        erase_adj_number(1, 1, &grid, &mut tainted_numbers);

        assert_eq!(tainted_numbers.len(), 8);
        assert_eq!(tainted_numbers[0], (0, 0));
    }

    #[test]
    fn test_tainted_num() {
        let tainted_numbers: Vec<(usize, usize)> = vec![(1, 3)];

        assert!(is_tained_num(1, (1, 3), &tainted_numbers));

        let tainted_numbers: Vec<(usize, usize)> = vec![(1, 3)];

        assert!(!is_tained_num(0, (1, 3), &tainted_numbers));
    }

    #[test]
    fn test_is_first_digit() {
        let data = String::from("100..");
        let mut grid: Vec<Vec<char>> = Vec::new();
        fill_grid(data, &mut grid);

        assert_eq!(is_first_digit(0, 0, &grid), true);
        assert_eq!(is_first_digit(0, 1, &grid), false);

        let data = String::from(".100..");
        let mut grid: Vec<Vec<char>> = Vec::new();
        fill_grid(data, &mut grid);

        assert_eq!(is_first_digit(0, 1, &grid), true);
        assert_eq!(is_first_digit(0, 2, &grid), false);
        assert_eq!(is_first_digit(0, 3, &grid), false);
    }

    #[test]
    fn test_full_num() {
        let data = String::from("100..");
        let mut grid: Vec<Vec<char>> = Vec::new();
        fill_grid(data, &mut grid);

        assert_eq!(get_full_num(0, 0, &grid), (0, 2));

        let data = String::from("...100");
        let mut grid: Vec<Vec<char>> = Vec::new();
        fill_grid(data, &mut grid);

        assert_eq!(get_full_num(0, 3, &grid), (3, 5));
    }

    #[test]
    fn test_parse_int() {
        let data = String::from("100..");
        let mut grid: Vec<Vec<char>> = Vec::new();
        fill_grid(data, &mut grid);

        assert_eq!(parse_num(0, (0, 2), &grid), 100);
    }

    #[test]
    fn test_find_gear() {
        let g = Gear {
            adj_num_points: vec![(0, 0), (2, 1)],
            adj_numbers: Vec::new(),
        };

        let mut gears: Vec<Gear> = vec![g];
        let result = find_gear(0, (0, 1), &mut gears);
        assert!(result.is_some() == true);

        let result = find_gear(2, (1, 2), &mut gears);
        assert!(result.is_some() == true);

        let result = find_gear(3, (0, 2), &mut gears);
        assert!(result.is_some() == false);
    }

    #[test]
    fn test_is_gear() {
        assert!(is_gear('*') == true);
        assert!(is_gear('$') == false);
    }

    #[test]
    fn test_add_gear() {
        let data = String::from(
            "1..\n\
            .$.\n\
            ...",
        );
        let mut grid: Vec<Vec<char>> = Vec::new();
        let mut gears: Vec<Gear> = Vec::new();
        fill_grid(data, &mut grid);

        add_gear(1, 1, &grid, &mut gears);

        assert!(gears.len() == 1);
        assert!(gears[0].adj_num_points[0] == (0, 0));
    }
}
