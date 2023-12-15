use std::fs::read_to_string;

fn get_hash(input: &str) -> u64 {
    let mut out = 0;

    for c in input.chars() {
        let ascii = c as u64;
        out += ascii;
        out *= 17;
        out %= 256;
    }

    out
}

fn p1(line: String) -> u64 {
    let mut out = 0;
    for l in line.split(',').into_iter() {
        out += get_hash(l);
    }
    out
}

fn calc_power(boxes: &Vec<Vec<(String, u64)>>) -> u64 {
    let mut out = 0;
    for (i, bx) in boxes.iter().enumerate() {
        let box_pow = i as u64 + 1;
        for (j, lens) in bx.iter().enumerate() {
            let lens_pow = j as u64 + 1;
            out += box_pow * lens_pow * lens.1;
        }
    }
    out
}

fn p2(line: String) -> u64 {
    let mut boxes: Vec<Vec<(String, u64)>> = Vec::new();

    for _i in 0..256 {
        //init
        boxes.push(Vec::new());
    }

    for op in line.split(',').into_iter() {
        let is_remove = op.contains('-');
        let label = &op[0..op.len() - if is_remove { 1 } else { 2 }];
        let box_no = get_hash(label);
        let lens = boxes[box_no as usize]
            .iter_mut()
            .position(|l| l.0.eq(label));
        let bx = &mut boxes[box_no as usize];
        if is_remove {
            if lens.is_some() {
                bx.remove(lens.unwrap());
            }
        } else {
            let power = op.chars().last().unwrap().to_digit(10).unwrap() as u64;
            if lens.is_some() {
                //replace lens
                bx[lens.unwrap()].1 = power;
            } else {
                bx.push((label.to_string(), power));
            }
        }
    }

    calc_power(&boxes)
}

fn main() {
    let file = "data/day15.txt";
    let str = read_to_string(file).expect("cannot open");
    println!("p1: {}", p1(str.trim().to_string()));
    println!("p2: {}", p2(str.trim().to_string()));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_hash() {
        let input = "HASH";
        let hash = get_hash(input);

        assert_eq!(hash, 52);

        let rn = "rn";
        let cm = "cm";
        let cmm = "cm-";

        assert_eq!(get_hash(rn), get_hash(cm));
        assert_eq!(get_hash(rn), 0);
        assert_eq!(&cmm[0..cmm.len() - 1], cm);
        assert_eq!(get_hash(&cmm[0..cmm.len() - 1]), 0);
    }

    #[test]
    fn test_calc_power() {
        let mut boxes: Vec<Vec<(String, u64)>> = Vec::new();
        for _i in 0..255 {
            //init
            boxes.push(Vec::new());
        }
        boxes[0].push((String::from("rn"), 1));
        boxes[0].push((String::from("cm"), 2));
        boxes[3].push((String::from("ot"), 7));
        boxes[3].push((String::from("ab"), 5));
        boxes[3].push((String::from("pc"), 6));

        assert_eq!(calc_power(&boxes), 145);
    }

    #[test]
    fn test_p1() {
        let input = String::from("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
        let hash = p1(input);

        assert_eq!(hash, 1320);
    }

    #[test]
    fn test_p2() {
        let input = String::from("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
        let power = p2(input);

        assert_eq!(power, 145);
    }
}
