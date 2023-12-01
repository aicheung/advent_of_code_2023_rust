use std::env;
use std::fs;

fn main() {
    let mut sum = 0;
    let mut lineFirst = -1;
    let mut lineLast = -1;

    let data = fs::read_to_string("data/day1.txt").expect("Cannot read file.");

    //println!("{}", data);

    for line in data.split("\n") {
        lineFirst = -1;
        lineLast = -1;
        for c in line.chars() {
            if c.is_digit(10) {
                let val: i32 = c.to_digit(10).expect("Cannot convert.").try_into().unwrap();
                //println!("conv {} {}", c, val);
                if lineFirst == -1 {
                    lineFirst = val;
                } else {
                    lineLast = val;
                }
            }
        }
        if lineFirst == -1 {
            println!("no digit!");
            continue;
        }
        sum += lineFirst * 10 + if lineLast == -1 {lineFirst} else {lineLast};
    }

    println!("{}", sum);
}