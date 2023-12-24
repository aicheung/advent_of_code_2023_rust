use std::{collections::HashSet, fs};

use z3::{Config, Context, ast::{self, Ast, Real}, Solver, SatResult};

#[derive(Clone)]
struct Line {
    id: i128,
    x: i128,
    y: i128,
    z: i128,
    x_slope: i128,
    y_slope: i128,
    z_slope: i128,
}

impl Line {
    fn parse(line: &str) -> Line {
        let split = line
            .trim()
            .replace(',', "")
            .replace('@', "")
            .split_whitespace()
            .map(|i| i128::from_str_radix(i, 10).unwrap())
            .collect::<Vec<_>>();

        Line {
            id: 0,
            x: split[0],
            y: split[1],
            z: split[2],
            x_slope: split[3],
            y_slope: split[4],
            z_slope: split[5],
        }
    }

    fn find_t(x: f64, x0: f64, slope: f64) -> f64 {
        //x = s * t + x0
        //t = (x - x0) / s

        (x - x0) / slope
    }

    fn intersects(&self, other: &Line, begin: i128, limit: i128) -> bool {
        if other.id == self.id {
            return false;
        }

        let p1 = (self.x, self.y);
        let p2 = (p1.0 + self.x_slope , p1.1 + self.y_slope);

        let p3 = (other.x, other.y);
        let p4 = (p3.0 + other.x_slope, p3.1 + other.y_slope);

        let denominator = (p1.0 - p2.0) * (p3.1 - p4.1) - (p1.1 - p2.1) * (p3.0 - p4.0);

        if denominator == 0 {
            return false;
        }

        let px = ((p1.0 * p2.1 - p1.1 * p2.0) * (p3.0 - p4.0)
            - (p1.0 - p2.0) * (p3.0 * p4.1 - p3.1 * p4.0))
            / denominator;
        let py = ((p1.0 * p2.1 - p1.1 * p2.0) * (p3.1 - p4.1)
            - (p1.1 - p2.1) * (p3.0 * p4.1 - p3.1 * p4.0))
            / denominator;
        let t = Line::find_t(px as f64, self.x as f64, self.x_slope as f64);
        //let ty = Line::find_t(py as f64, self.y as f64, self.y_slope as f64);
        let t2 = Line::find_t(px as f64, other.x as f64, other.x_slope as f64);
        //let ty2 = Line::find_t(py as f64, other.y as f64, other.y_slope as f64);
        //println!("{} {}, t {} {} {} {}", px, py, t, ty, t2, ty2);

        (t >= 0.0 && t2 >= 0.0)
            //&& t == ty
            && px >= begin 
            && px <= limit
            && py >= begin 
            && py <= limit 
    }
}

fn get_match_count(lines: Vec<Line>, begin: i128, limit: i128) -> u64 {
    let mut matched = HashSet::new();
    for l in lines.iter() {
        for l2 in lines.iter() {
            if l2.id > l.id && l.intersects(l2, begin, limit) {
                let k = (l.id.clone(), l2.id.clone());
                matched.insert(k);
            }
        }
    }

    matched.len() as u64
}

fn load(file: &str) -> Vec<Line> {
    let mut lines = Vec::new();
    let mut i = 0;
    for line in fs::read_to_string(file)
        .expect("")
        .replace('\r', "")
        .split('\n')
    {
        if !line.contains('@') {
            continue;
        }
        let mut l = Line::parse(line);
        l.id = i;
        lines.push(l);
        i += 1;
    }
    lines
}

fn var_name(prefix: &str, id: i64) -> String {
    let mut s = String::from(prefix);
    s.push_str(id.to_string().as_str());
    s
}

fn add_to_solver(lines: Vec<Line>, solver: &mut Solver, ctx: &Context, y_slope_rock: &Real,y0_rock: &Real, x_slope_rock: &Real, x0_rock: &Real,z_slope_rock: &Real, z0_rock:&Real  ) {
    

    for l in lines {
        let x0 = l.x as i64;
        let y0 = l.y as i64;
        let z0 = l.z as i64;
        let x_slope = l.x_slope as i64;
        let y_slope = l.y_slope as i64;
        let z_slope = l.z_slope as i64;
        let id = l.id as i64;

        let zero = ast::Real::from_int(&ast::Int::from_i64(&ctx, 0));
        let x = ast::Real::new_const(&ctx, var_name("x", id));
        let y = ast::Real::new_const(&ctx, var_name("y", id));
        let z = ast::Real::new_const(&ctx, var_name("z", id));
        let t = ast::Real::new_const(&ctx, var_name("t", id));
        let x_slope = ast::Real::from_int(&ast::Int::from_i64(&ctx, x_slope));
        let y_slope = ast::Real::from_int(&ast::Int::from_i64(&ctx, y_slope));
        let z_slope = ast::Real::from_int(&ast::Int::from_i64(&ctx, z_slope));
        let x0 = ast::Real::from_int(&ast::Int::from_i64(&ctx, x0));
        let y0 = ast::Real::from_int(&ast::Int::from_i64(&ctx, y0));
        let z0 = ast::Real::from_int(&ast::Int::from_i64(&ctx, z0));

        solver.assert(&x._eq(&ast::Real::add(&ctx, &[&x0, &ast::Real::mul(&ctx, &[&t, &x_slope])])));
        solver.assert(&y._eq(&ast::Real::add(&ctx, &[&y0, &ast::Real::mul(&ctx, &[&t, &y_slope])])));
        solver.assert(&z._eq(&ast::Real::add(&ctx, &[&z0, &ast::Real::mul(&ctx, &[&t, &z_slope])])));

        let y_rock = ast::Real::new_const(&ctx, var_name("y_rock", id));
        let x_rock = ast::Real::new_const(&ctx, var_name("x_rock", id));
        let z_rock = ast::Real::new_const(&ctx, var_name("z_rock", id));
        let t_rock = ast::Real::new_const(&ctx, var_name("t_rock", id));

        solver.assert(&x_rock._eq(&ast::Real::add(&ctx, &[&x0_rock, &ast::Real::mul(&ctx, &[&t_rock, &x_slope_rock])])));
        solver.assert(&y_rock._eq(&ast::Real::add(&ctx, &[&y0_rock, &ast::Real::mul(&ctx, &[&t_rock, &y_slope_rock])])));
        solver.assert(&z_rock._eq(&ast::Real::add(&ctx, &[&z0_rock, &ast::Real::mul(&ctx, &[&t_rock, &z_slope_rock])])));
        solver.assert(&x_rock._eq(&x));
        solver.assert(&y_rock._eq(&y));
        solver.assert(&z_rock._eq(&z));
        solver.assert(&t_rock._eq(&t));

        solver.assert(&t_rock.gt(&zero));
    }
    
    

}

fn p2(file: &str) -> u64 {
    let cfg = Config::new();
        let ctx = Context::new(&cfg);
        let mut solver = Solver::new(&ctx);

        let lines = load(file)[0..4].to_vec();        
        let y_slope_rock = ast::Real::new_const(&ctx, "y_slope_rock");
        let y0_rock = ast::Real::new_const(&ctx, "y0_rock");
        let x_slope_rock = ast::Real::new_const(&ctx, "x_slope_rock");
        let x0_rock = ast::Real::new_const(&ctx, "x0_rock");
        let z_slope_rock = ast::Real::new_const(&ctx, "z_slope_rock");
        let z0_rock = ast::Real::new_const(&ctx, "z0_rock");
        add_to_solver(lines, &mut solver, &ctx, &y_slope_rock, &y0_rock, &x_slope_rock, &x0_rock, &z_slope_rock, &z0_rock);

        assert_eq!(solver.check(), SatResult::Sat);
        let model = solver.get_model().unwrap();
        println!("{:?}", model);

        let rock_x0 = model.eval(&x0_rock, true).unwrap().as_real().unwrap();
        let rock_y0 = model.eval(&y0_rock, true).unwrap().as_real().unwrap();
        let rock_z0 = model.eval(&z0_rock, true).unwrap().as_real().unwrap();
        println!("{:?} {:?} {:?}", rock_x0, rock_y0, rock_z0);
        rock_x0.0 as u64 + rock_y0.0 as u64 + rock_z0.0 as u64
}

fn p1(file: &str) -> u64 {
    let lines = load(file);

    assert_eq!(lines.len(), 300);
    get_match_count(lines, 200000000000000, 400000000000000)
}

fn main() {
    let file = "data/day24.txt";
    println!("p1: {}", p1(file));
    println!("p2: {}", p2(file));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p2() {

        let file = "data/day24_ex.txt";
        let result = p2(file);
        assert_eq!(result, 47);
    }

    #[test]
    fn test_ex() {
        let file = "data/day24_ex.txt";

        let lines = load(file);

        let result = get_match_count(lines, 7, 27);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_parse() {
        let l = "18, 19, 22 @ -1, -1, -2";

        let r = Line::parse(l);

        assert_eq!(r.x, 18);
        assert_eq!(r.y, 19);
        assert_eq!(r.z, 22);
        assert_eq!(r.x_slope, -1);
        assert_eq!(r.y_slope, -1);
        assert_eq!(r.z_slope, -2);

        let l = "439854842455119, 383935112515580, 293031876578902 @ -218, -316, -253";
        let r = Line::parse(l);

        assert_eq!(r.x, 439854842455119);
        assert_eq!(r.y, 383935112515580);
        assert_eq!(r.z, 293031876578902);
        assert_eq!(r.x_slope, -218);
        assert_eq!(r.y_slope, -316);
        assert_eq!(r.z_slope, -253);
    }

    #[test]
    fn test_intersect() {
        let l = "19, 13, 30 @ -2, 1, -2";
        let l2 = "18, 19, 22 @ -1, -1, -2";

        let r = Line::parse(l);
        let r2 = Line::parse(l2);

        assert!(r.intersects(&r2, 7, 27));

        let l = "19, 13, 30 @ -2, 1, -2";
        let l2 = "20, 25, 34 @ -2, -2, -4";

        let r = Line::parse(l);
        let r2 = Line::parse(l2);

        assert!(r.intersects(&r2, 7, 27));

        let l = "19, 13, 30 @ -2, 1, -2";
        let l2 = "12, 31, 28 @ -1, -2, -1";

        let r = Line::parse(l);
        let r2 = Line::parse(l2);

        assert!(!r.intersects(&r2, 7, 27));

        let l = "19, 13, 30 @ -2, 1, -2";
        let l2 = "20, 19, 15 @ 1, -5, -3";

        let r = Line::parse(l);
        let r2 = Line::parse(l2);

        assert!(!r.intersects(&r2, 7, 27));

        let l = "18, 19, 22 @ -1, -1, -2";
        let l2 = "20, 25, 34 @ -2, -2, -4";

        let r = Line::parse(l);
        let r2 = Line::parse(l2);

        assert!(!r.intersects(&r2, 7, 27));
        let l = "20, 25, 34 @ -2, -2, -4";
        let l2 = "12, 31, 28 @ -1, -2, -1";

        let r = Line::parse(l);
        let r2 = Line::parse(l2);

        assert!(!r.intersects(&r2, 7, 27));
    }
}
