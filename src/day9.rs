use std::fs::read_to_string;

fn is_all_zero(seq: &Vec<i64>) -> bool {
    seq.iter().all(|a| *a == 0)
}

fn find_seq_diff(seq: &Vec<i64>) -> Vec<i64> {
    let mut result = Vec::new();
    for i in 1..seq.len() {
        result.push(seq[i] - seq[i - 1]);
    }
    result
}

fn derive(seq: &Vec<i64>, p1: bool) -> (Vec<Vec<i64>>, Vec<i64>) {
    let mut degrees = Vec::new();
    let mut diffs: Vec<i64> = Vec::new();
    let mut diff = seq.clone();
    while !is_all_zero(&diff) {
        let diff_result = find_seq_diff(&diff);
        diff = diff_result.clone();
        if !is_all_zero(&diff_result) {
            degrees.push(diff_result);
        }
    }

    //fill 0
    for _i in 0..degrees.len() {
        diffs.push(0);
    }

    let mut prev = 0;
    for (i, d) in diffs.iter_mut().enumerate().rev() {
        if i == degrees.len() - 1 {
            // last one
            *d = *degrees[i].last().unwrap();
        } else {
            //last val of seq + prev diff
            *d = if p1 {
                *degrees[i].last().unwrap() + prev
            } else {
                degrees[i][0] - prev
            };
        }
        prev = *d;
    }
    (degrees, diffs)
}

fn find_next_seq_val(seq: &Vec<i64>, p1: bool) -> i64 {
    let derivatives = derive(&seq, p1);
    if p1 {
        seq.iter().last().unwrap() + derivatives.1[0]
    } else {
        seq[0] - derivatives.1[0]
    }
}

fn p1(sequences: &Vec<Vec<i64>>) -> i64 {
    sequences.iter().map(|s| find_next_seq_val(s, true)).sum()
}

fn p2(sequences: &Vec<Vec<i64>>) -> i64 {
    sequences.iter().map(|s| find_next_seq_val(s, false)).sum()
}

fn load_seqs(file: &str) -> Vec<Vec<i64>> {
    let mut seqs = Vec::new();

    let lines = read_to_string(file).expect("Cannot read file");

    for l in lines.split('\n').collect::<Vec<_>>() {
        if l.len() < 2 {
            continue;
        }
        seqs.push(
            l.trim()
                .split_whitespace()
                .collect::<Vec<_>>()
                .iter()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<i64>>(),
        );
    }

    seqs
}

fn main() {
    let file = "data/day9.txt";
    let seqs = load_seqs(file);

    println!("p1: {}", p1(&seqs));
    println!("p2: {}", p2(&seqs));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_is_all_zero() {
        let s = vec![0, 0, 0, 0, 0];
        assert!(is_all_zero(&s));
        let s = vec![1, 2, 3, 4, 5];
        assert!(!is_all_zero(&s));
    }

    #[test]
    fn test_find_seq_diff() {
        let seq = vec![0, 3, 6, 9, 12, 15];
        assert_eq!(find_seq_diff(&seq), vec![3, 3, 3, 3, 3]);
    }

    #[test]
    fn test_derive() {
        let seq = vec![0, 3, 6, 9, 12, 15];
        let derivatives: (Vec<Vec<i64>>, Vec<i64>) = derive(&seq, true);
        assert_eq!(derivatives.0.len(), 1);
        assert_eq!(derivatives.0[0][0], 3);
        assert_eq!(derivatives.1.len(), 1);
        assert_eq!(derivatives.1[0], 3);

        let seq = vec![10, 13, 16, 21, 30, 45];
        let derivatives: (Vec<Vec<i64>>, Vec<i64>) = derive(&seq, true);
        assert_eq!(derivatives.0.len(), 3);
        assert_eq!(derivatives.0[0][0], 3);
        assert_eq!(derivatives.0[1][0], 0);
        assert_eq!(derivatives.0[2][0], 2);
        assert_eq!(derivatives.1.len(), 3);
        assert_eq!(derivatives.1[0], 23);
        assert_eq!(derivatives.1[1], 8);
        assert_eq!(derivatives.1[2], 2);
    }

    #[test]
    fn test_find_next_seq_val() {
        let seq = vec![0, 3, 6, 9, 12, 15];
        assert_eq!(find_next_seq_val(&seq, true), 18);
        let seq = vec![1, 3, 6, 10, 15, 21];
        assert_eq!(find_next_seq_val(&seq, true), 28);
        let seq = vec![10, 13, 16, 21, 30, 45];
        assert_eq!(find_next_seq_val(&seq, true), 68);
    }

    #[test]
    fn test_p1() {
        let seqs = vec![
            vec![0, 3, 6, 9, 12, 15],
            vec![1, 3, 6, 10, 15, 21],
            vec![10, 13, 16, 21, 30, 45],
        ];
        assert_eq!(p1(&seqs), 114);
    }

    #[test]
    fn test_p2() {
        let seqs = vec![
            vec![0, 3, 6, 9, 12, 15],
            vec![1, 3, 6, 10, 15, 21],
            vec![10, 13, 16, 21, 30, 45],
        ];
        assert_eq!(p2(&seqs), 2);
    }

    #[test]
    fn test_load_seqs() {
        let file = "data/day9_ex.txt";

        let result = load_seqs(file);

        assert_eq!(result.len(), 3);
        assert_eq!(result[0][0], 0);
        assert_eq!(result[1][0], 1);
        assert_eq!(result[2][0], 10);
        assert_eq!(result[0][5], 15);
        assert_eq!(result[1][5], 21);
        assert_eq!(result[2][5], 45);
    }
}
