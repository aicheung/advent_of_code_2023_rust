
use rayon::prelude::*;
use std::{collections::{HashSet, HashMap}, fs};
fn match_records(record: &str, row: &str) -> bool {
    let spring_sets: Vec<u64> = record
        .split(',')
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    let spring_only = row.replace(".", " ");
    let sets: Vec<&str> = spring_only.split_whitespace().collect();

    if spring_sets.len() != sets.len() {
        return false;
    }

    for (i, spring_count) in spring_sets.iter().enumerate() {
        if sets[i].len() as u64 != *spring_count {
            return false;
        }
    }

    true
}

fn early_pruning(record: &str, spring_set: &str) -> bool {
    let pass = true;

    if spring_set.len() > 0 {
        let spring_set_counts: Vec<u64> = spring_set
            .split(',')
            .map(|s| s.parse::<u64>().unwrap())
            .collect();

        let sum: u64 = spring_set_counts.iter().sum();
        let total_possible = record.chars().filter(|c| *c == '?' || *c == '#').count();
        if sum > total_possible as u64 {
            return false; //already impossible to fit all springs
        } 

        let first_unknown = record.find('?');
        if first_unknown.is_some() {
            let unknown_idx = first_unknown.unwrap();
            let (known, _) = record.split_at(unknown_idx);
            let spring_only = known.replace('.', " ");
            let groups = spring_only.split_whitespace().collect::<Vec<_>>();

            for (i, g) in groups.iter().enumerate() {
                //skip last group because it may be incomplete
                if i >= spring_set_counts.len() || (i == groups.len() - 1 && spring_set_counts[i] >= g.len() as u64) {
                    continue;
                }
                if spring_set_counts[i] != g.len() as u64 {
                    return false;
                }
            }
        }
    }

    pass
}

fn get_all_permutations(record: &str, spring_set: &str, wrong_cache: &mut HashSet<String>) -> HashSet<String> {
    let mut result = HashSet::new();

    if record.contains('?') && wrong_cache.contains(&record.replace('.', " ").replace('#', " ").trim_start().to_string()) {
        //already known it is wrong
        return result;
    }

    if !record.contains('?') || !early_pruning(record, spring_set) {
        if record.contains('?') {
            //this sub-branch is already wrong, no need to calc next time
            wrong_cache.insert(record.replace('.', " ").replace('#', " ").trim_start().to_string());
        }
        return result;
    }

    let spring = replace_one_question_mark(record.to_string(), true);
    let non_spring = replace_one_question_mark(record.to_string(), false);
    let spring_sets = get_all_permutations(&spring, spring_set, wrong_cache);
    let non_spring_sets = get_all_permutations(&non_spring, spring_set, wrong_cache);
    if !spring.contains('?') {
        result.insert(spring);
    }
    if !non_spring.contains('?') {
        result.insert(non_spring);
    }

    result.extend(spring_sets.iter().map(|s| s.clone()));
    result.extend(non_spring_sets.iter().map(|s| s.clone()));

    result
}

fn replace_one_question_mark(record: String, change_to_spring: bool) -> String {
    let mut result = String::new();
    let mut done = false;
    for c in record.chars() {
        if c == '?' && !done {
            result.push(if change_to_spring { '#' } else { '.' });

            done = true;
        } else {
            result.push(c);
        }
    }
    result
}

fn expand(src: &str, times: i32, sep: char, out: &mut String) {
    out.push_str(src);
    for _i in 0..times - 1 {
        out.push(sep);
        out.push_str(src);
    }
}

fn is_match(condition_records: &[char], at: usize, len: usize) -> bool {
    let edges_could_be_operational = *condition_records
        .get(at.checked_sub(1).unwrap_or(usize::MAX))
        .unwrap_or(&'.')
        != '#'
        && *condition_records.get(at + len).unwrap_or(&'.') != '#';

    let springs_could_be_damaged = condition_records
        .get(at..at + len)
        .map(|slice| slice.len() == len && slice.iter().all(|c| *c != '.'))
        .unwrap_or(false);

    edges_could_be_operational && springs_could_be_damaged
}

fn number_of_matches(
    cache: &mut HashMap<(usize, usize), usize>,
    condition_records: &[char],
    groups: &[usize],
    start: usize,
) -> usize {
    if let Some((group, remaining_groups)) = groups.split_first() {
        let mut ans = 0;
        let remaining_len: usize = remaining_groups.iter().sum();
        for at in start..(condition_records.len() - group - remaining_len + 1) {
            if is_match(condition_records, at, *group) {
                let next_at = at + *group + 1;
                if let Some(cached) = cache.get(&(remaining_len, next_at)) {
                    ans += cached
                } else {
                    let val = number_of_matches(
                        cache,
                        condition_records,
                        remaining_groups,
                        at + *group + 1,
                    );

                    cache.insert((remaining_len, next_at), val);
                    ans += val
                }
            }

            if condition_records[at] == '#' {
                break;
            }
        }
        ans
    } else {
        condition_records
            .get(start..)
            .map(|slice| !slice.iter().any(|c| *c == '#'))
            .unwrap_or(true) as usize
    }
}


fn dp(sub_record: &str, remaining_springs: &mut Vec<u64>) -> u64 {
    let records: Vec<char> = sub_record.chars().collect();
    let groups = remaining_springs.iter().map(|a| *a as usize).collect::<Vec<_>>();
    
    number_of_matches(&mut HashMap::new(), &records, &groups, 0) as u64
}


fn day12(file: &str, is_p2: bool) -> u64 {
    let lines = fs::read_to_string(file).expect("cannot read");
    let total = lines
        .split('\n')
        .collect::<Vec<_>>()
        .par_iter()
        .map(|l| {
            if l.len() < 2 {
                return 0;
            }
            let mut combinations = HashSet::new();
            let line_split = l.split_whitespace().collect::<Vec<_>>();
            let mut spring_sets = String::new();
            let mut record = String::new();
            if !is_p2 {
                spring_sets.push_str(line_split[1])
            } else {
                expand(line_split[1], 5, ',', &mut spring_sets)
            };
            if !is_p2 {
                record.push_str(line_split[0])
            } else {
                expand(line_split[0], 5, '?', &mut record)
            };
            if !is_p2 {
                let mut wrong_cache = HashSet::new();
                let permutations = get_all_permutations(&record, &spring_sets, &mut wrong_cache);
                for p in permutations {
                    if match_records(&spring_sets, p.as_str()) {
                        combinations.insert(p);
                    }
                }
            } else {
                let mut spring_set_counts: Vec<u64> = spring_sets
                    .split(',')
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect();
                return dp(&record, &mut spring_set_counts)
            }
            combinations.len() as u64
        })
        .sum();
    total
}

fn main() {
    let file = "data/day12.txt";
    println!("p1: {}", day12(file, false));
    println!("p2: {}", day12(file, true));
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dp() {
        let sub_record = "";
        let mut remaining = vec![];
        assert_eq!(dp(sub_record, &mut remaining), 1);

        let sub_record = ".";
        let mut remaining = vec![];
        assert_eq!(dp(sub_record, &mut remaining), 1);

        let sub_record = "#";
        let mut remaining = vec![];
        assert_eq!(dp(sub_record, &mut remaining), 0);
        let mut remaining = vec![1];
        assert_eq!(dp(sub_record, &mut remaining), 1);

        let sub_record = "..?..?..";
        let mut remaining = vec![1,1];
        assert_eq!(dp(sub_record, &mut remaining), 1);

        let sub_record = "??.";
        let mut remaining = vec![1];
        assert_eq!(dp(sub_record, &mut remaining), 2);
        let sub_record = "???.###";
        let mut remaining = vec![1,1,3];
        assert_eq!(dp(sub_record, &mut remaining), 1);

    }
}
