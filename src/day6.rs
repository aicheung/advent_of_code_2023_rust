use std::fs;
struct Race {
    total_time: u64,
    hold_time: u64,
}

impl Race {
    fn get_travel_distance(&self) -> u64 {
        if self.hold_time == 0 || self.total_time == 0 {
            return 0;
        }

        let remaining = self.total_time - self.hold_time;

        remaining * self.hold_time
    }
}

struct Competition {
    races: Vec<Race>,
    cur_record: u64,
    winning_races: u64,
}

impl Competition {
    fn run_races(&mut self) {
        self.winning_races = 0;
        for r in &self.races {
            let race_record = r.get_travel_distance();
            if race_record > self.cur_record {
                self.winning_races += 1;
            }
        }
    }
}

fn create_comps(times: Vec<u64>, dists: Vec<u64>) -> Vec<Competition> {
    let mut competitions = Vec::new();
    for i in 0 .. times.len() {
        let mut races = Vec::new();
        for t in 0 .. times[i] + 1 {
            let race = Race {
                total_time: times[i],
                hold_time: t
            };
            races.push(race);
        }
        let comp = Competition {
            races,
            cur_record: dists[i],
            winning_races: 0
        };
        competitions.push(comp);
    }

    competitions
}

fn parse(data: &String) -> Vec<Competition> {
    let lines: Vec<&str> = data.lines().collect();
    let times: Vec<u64> = lines[0]
        .split(':')
        .last()
        .expect("No times")
        .trim()
        .split_whitespace()
        .into_iter()
        .map(|t| t.parse::<u64>().expect("cannot parse"))
        .collect();
    let dists: Vec<u64> = lines[1]
        .split(':')
        .last()
        .expect("No dists")
        .trim()
        .split_whitespace()
        .into_iter()
        .map(|t| t.parse::<u64>().expect("cannot parse"))
        .collect();

    assert!(times.len() == dists.len());

    create_comps(times, dists)
}

fn parse_p2(data: &String) -> Vec<Competition> {
    let lines: Vec<&str> = data.lines().collect();
    let times: Vec<u64> = lines[0]
        .split(':')
        .last()
        .expect("No times")
        .trim()
        .replace(' ', "")
        .split_whitespace()
        .into_iter()
        .map(|t| t.parse::<u64>().expect("cannot parse"))
        .collect();
    let dists: Vec<u64> = lines[1]
        .split(':')
        .last()
        .expect("No dists")
        .trim()
        .replace(' ', "")
        .split_whitespace()
        .into_iter()
        .map(|t| t.parse::<u64>().expect("cannot parse"))
        .collect();

    assert!(times.len() == dists.len());

    create_comps(times, dists)
}

fn main() {
    let data = fs::read_to_string("data/day6.txt").expect("Cannot read file.");
    let comps = parse(&data);
    let comps2 = parse_p2(&data);

    let mut p1_result = 1;
    for mut c in comps {
        c.run_races();
        p1_result *= c.winning_races;
    }

    println!("p1: {}", p1_result);

    let mut p2_result = 1;
    for mut c in comps2 {
        c.run_races();
        p2_result *= c.winning_races;
    }

    println!("p2: {}", p2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_race() {
        let r = Race {
            total_time: 7,
            hold_time: 0,
        };
        assert_eq!(r.get_travel_distance(), 0);
        let r = Race {
            total_time: 7,
            hold_time: 1,
        };
        assert_eq!(r.get_travel_distance(), 6);
        let r = Race {
            total_time: 7,
            hold_time: 2,
        };
        assert_eq!(r.get_travel_distance(), 10);
        let r = Race {
            total_time: 7,
            hold_time: 3,
        };
        assert_eq!(r.get_travel_distance(), 12);
        let r = Race {
            total_time: 7,
            hold_time: 4,
        };
        assert_eq!(r.get_travel_distance(), 12);
        let r = Race {
            total_time: 7,
            hold_time: 5,
        };
        assert_eq!(r.get_travel_distance(), 10);
        let r = Race {
            total_time: 7,
            hold_time: 6,
        };
        assert_eq!(r.get_travel_distance(), 6);
        let r = Race {
            total_time: 7,
            hold_time: 7,
        };
        assert_eq!(r.get_travel_distance(), 0);
    }

    #[test]
    fn test_competition() {
        let mut comp = Competition {
            races: vec![
                Race {
                    total_time: 7,
                    hold_time: 0,
                },
                Race {
                    total_time: 7,
                    hold_time: 1,
                },
                Race {
                    total_time: 7,
                    hold_time: 2,
                },
                Race {
                    total_time: 7,
                    hold_time: 3,
                },
                Race {
                    total_time: 7,
                    hold_time: 4,
                },
                Race {
                    total_time: 7,
                    hold_time: 5,
                },
                Race {
                    total_time: 7,
                    hold_time: 6,
                },
                Race {
                    total_time: 7,
                    hold_time: 7,
                },
            ],
            cur_record: 9,
            winning_races: 0,
        };
        comp.run_races();
        assert_eq!(comp.winning_races, 4);
    }

    #[test]
    fn test_parse() {
        let data: String = fs::read_to_string("data/day6_ex.txt").expect("Cannot read file.");
        let result = parse(&data);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].cur_record, 9);
        assert_eq!(result[0].races.len(), 8);
        assert_eq!(result[0].races[0].hold_time, 0);
        assert_eq!(result[0].races[0].total_time, 7);
        assert_eq!(result[0].races[7].hold_time, 7);
        assert_eq!(result[0].races[7].total_time, 7);
        assert_eq!(result[1].cur_record, 40);
        assert_eq!(result[2].cur_record, 200);
    }

    #[test]
    fn test_parse_p2() {
        let data: String = fs::read_to_string("data/day6_ex.txt").expect("Cannot read file.");
        let result = parse_p2(&data);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].cur_record, 940200);
        assert_eq!(result[0].races.len(),71531);
        assert_eq!(result[0].races[0].hold_time, 0);
        assert_eq!(result[0].races[0].total_time, 71530);
        assert_eq!(result[0].races[71530].hold_time, 71530);
        assert_eq!(result[0].races[71530].total_time, 71530);
    }
}
