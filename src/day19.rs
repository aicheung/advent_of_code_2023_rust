use std::{
    collections::{BinaryHeap, HashMap},
    fs,
};

#[derive(PartialEq, Eq, Debug)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Part {
    fn apply(&self, workflows: &HashMap<String, Workflow>) -> WorkflowResult {
        let mut cur_wf = workflows.get(&String::from("in")).unwrap();
        let out;
        loop {
            let (result, next) = cur_wf.apply(self);

            if result == WorkflowResult::Accept || result == WorkflowResult::Reject {
                out = result;
                break;
            }

            cur_wf = workflows.get(&next.unwrap()).unwrap();
        }

        out
    }

    fn sum(&self, workflows: &HashMap<String, Workflow>) -> u64 {
        let result = self.apply(workflows);
        if result == WorkflowResult::Accept {
            return self.x + self.m + self.a + self.s;
        }
        0
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
enum Comparator {
    LessThan,
    MoreThan,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
enum Rating {
    X,
    M,
    A,
    S,
}

#[derive(PartialEq, Eq, Debug)]
enum WorkflowResult {
    Accept,
    Reject,
    NextStep,
    NextWorkflow,
}

struct Rule {
    rating: Option<Rating>,
    comparator: Option<Comparator>,
    value: Option<u64>,
    is_accept: bool,
    is_reject: bool,
    next_workflow: Option<String>,
}

impl Rule {
    fn apply(&self, part: &Part) -> WorkflowResult {
        if self.rating.is_none() || self.comparator.is_none() || self.value.is_none() {
            if self.is_accept {
                //always accept
                return WorkflowResult::Accept;
            }
            if self.is_reject {
                return WorkflowResult::Reject;
            }
            return WorkflowResult::NextWorkflow;
        } else {
            let target = match self.rating.as_ref().unwrap() {
                Rating::X => part.x,
                Rating::M => part.m,
                Rating::A => part.a,
                Rating::S => part.s,
            };
            let val = self.value.unwrap();
            let comp_result = match self.comparator.as_ref().unwrap() {
                Comparator::LessThan => target < val,
                Comparator::MoreThan => target > val,
            };
            if !comp_result {
                return WorkflowResult::NextStep;
            } else {
                if self.is_accept {
                    //always accept
                    return WorkflowResult::Accept;
                }
                if self.is_reject {
                    return WorkflowResult::Reject;
                }
                return WorkflowResult::NextWorkflow;
            }
        }
    }
}

#[derive(PartialEq, Eq, Debug, PartialOrd, Ord, Clone)]
struct Criterion {
    rating: Rating,
    comparator: Comparator,
    value: u64,
}

struct Workflow {
    name: String,
    steps: Vec<Rule>,
}

impl Workflow {
    fn apply(&self, part: &Part) -> (WorkflowResult, Option<String>) {
        for s in self.steps.iter() {
            let result = s.apply(part);
            if result == WorkflowResult::Accept || result == WorkflowResult::Reject {
                return (result, None);
            }
            if result == WorkflowResult::NextStep {
                continue;
            }
            return (
                WorkflowResult::NextWorkflow,
                Some(s.next_workflow.as_ref().unwrap().clone()),
            );
        }
        println!("WARNING: SHOULD NOT HAPPEN!!!");
        (WorkflowResult::Reject, None)
    }
}

fn load_workflow(line: &str) -> Workflow {
    let sp = line.trim().split('{').collect::<Vec<_>>();
    let name = sp[0];
    let steps_line = sp[1].replace('}', "");
    let mut steps = Vec::new();
    for s in steps_line.split(',') {
        let is_accept = s.contains("A");
        let is_reject = s.contains("R");
        let wf_only = !s.contains(":");
        let wf_name = if wf_only {
            s
        } else {
            s.split(':').last().unwrap()
        };
        let mut comparator: Option<Comparator> = None;
        let mut rating: Option<Rating> = None;
        let mut value: Option<u64> = None;

        let next_workflow = if is_accept || is_reject {
            None
        } else {
            Some(wf_name.to_string())
        };

        if !wf_only {
            let rule_str = s.split(':').nth(0).unwrap();
            let val_str = &rule_str[2..];
            let rating_str = &rule_str[0..1];
            let cmp_str = &rule_str[1..2];

            value = Some(u64::from_str_radix(val_str, 10).unwrap());
            comparator = match cmp_str {
                ">" => Some(Comparator::MoreThan),
                _ => Some(Comparator::LessThan),
            };
            rating = Some(match rating_str {
                "x" => Rating::X,
                "m" => Rating::M,
                "a" => Rating::A,
                _ => Rating::S,
            });
        }

        let step = Rule {
            rating: rating,
            comparator: comparator,
            value: value,
            is_accept: is_accept,
            is_reject: is_reject,
            next_workflow: next_workflow,
        };
        steps.push(step);
    }

    Workflow {
        name: name.to_string(),
        steps,
    }
}

fn load_part(line: &str) -> Part {
    let mut x = 0;
    let mut m = 0;
    let mut a = 0;
    let mut s = 0;
    for v in line.trim().replace('{', "").replace('}', "").split(',') {
        let val_str = &v[2..];
        match &v[0..1] {
            "x" => x = u64::from_str_radix(val_str, 10).unwrap(),
            "m" => m = u64::from_str_radix(val_str, 10).unwrap(),
            "a" => a = u64::from_str_radix(val_str, 10).unwrap(),
            _ => s = u64::from_str_radix(val_str, 10).unwrap(),
        }
    }
    Part { x, m, a, s }
}

fn p1(file: &str) -> u64 {
    let file_str = fs::read_to_string(file).expect("cannot read");

    let mut parts = Vec::new();
    let mut workflows = HashMap::new();
    for line in file_str.split('\n') {
        if line.len() < 2 {
            continue;
        }
        if &line[0..1] == "{" {
            //part
            parts.push(load_part(line.trim()));
        } else {
            let wf = load_workflow(line.trim());
            workflows.insert(wf.name.to_string(), wf);
        }
    }
    parts.iter().map(|p| p.sum(&workflows)).sum()
}

fn find_criteria_list(workflows: &HashMap<String, Workflow>) -> Vec<Vec<Criterion>> {
    let mut out = Vec::new();
    let mut to_visit = BinaryHeap::new();
    to_visit.push(("in", vec![]));

    while !to_visit.is_empty() {
        let (wf, mut criteria) = to_visit.pop().unwrap();

        //get wf, then traverse and push criteria in each step, until reaching A or R
        let workflow = workflows.get(wf).unwrap();

        for s in workflow.steps.iter() {
            if s.rating.is_none() {
                if s.is_reject {
                    break;
                }
                if s.is_accept {
                    //last step with no filtering
                    let success = criteria.clone();
                    out.push(success);
                } else if s.next_workflow.is_some() {
                    //next wf with no filtering
                    to_visit.push((s.next_workflow.as_ref().unwrap().as_str(), criteria.clone()));
                }
            } else {
                let c = Criterion {
                    rating: s.rating.as_ref().unwrap().clone(),
                    comparator: s.comparator.as_ref().unwrap().clone(),
                    value: s.value.unwrap(),
                };
                let c2 = Criterion {
                    rating: c.rating,
                    comparator: if c.comparator == Comparator::LessThan {
                        Comparator::MoreThan
                    } else {
                        Comparator::LessThan
                    },
                    value: if c.comparator == Comparator::LessThan {
                        c.value - 1
                    } else {
                        c.value + 1
                    },
                };
                if s.is_accept {
                    let mut success_stack = criteria.clone();
                    success_stack.push(c);
                    out.push(success_stack);
                } else if s.is_reject {
                    // if rej, then ignore the criteria and push complimentary to criterion for next step
                } else if s.next_workflow.is_some() {
                    //2 cases, push push criteria and wf & push complimentatry criteria and next
                    let mut next_wf_criteria = criteria.clone();
                    next_wf_criteria.push(c);
                    to_visit.push((s.next_workflow.as_ref().unwrap().as_str(), next_wf_criteria));
                }

                criteria.push(c2);
            }
        }
    }

    out
}

fn update_by_criteria(lower_bound: &mut i64, upper_bound: &mut i64, criterion: &Criterion) {
    match criterion.comparator {
        Comparator::MoreThan => {
            let new_lower_bound = (criterion.value + 1) as i64;
            if *lower_bound < new_lower_bound {
                *lower_bound = new_lower_bound;
            }
        }
        Comparator::LessThan => {
            let new_upper_bound = (criterion.value - 1) as i64;
            if *upper_bound > new_upper_bound {
                *upper_bound = new_upper_bound;
            }
        }
    }
}

fn find_combinations(criteria_lists: &Vec<Vec<Criterion>>) -> u64 {
    let mut out = 0;
    for list in criteria_lists {
        // min and max ranges for x,m,a,s
        let mut cur_criteria: (i64, i64, i64, i64, i64, i64, i64, i64) =
            (1, 4000, 1, 4000, 1, 4000, 1, 4000);
        for c in list {
            let lower_bound: &mut i64;
            let upper_bound: &mut i64;
            match c.rating {
                Rating::X => {
                    lower_bound = &mut cur_criteria.0;
                    upper_bound = &mut cur_criteria.1;
                }
                Rating::M => {
                    lower_bound = &mut cur_criteria.2;
                    upper_bound = &mut cur_criteria.3;
                }
                Rating::A => {
                    lower_bound = &mut cur_criteria.4;
                    upper_bound = &mut cur_criteria.5;
                }
                Rating::S => {
                    lower_bound = &mut cur_criteria.6;
                    upper_bound = &mut cur_criteria.7;
                }
            }
            update_by_criteria(lower_bound, upper_bound, c);
        }

        let combinations = (cur_criteria.1 - cur_criteria.0 + 1)
            * (cur_criteria.3 - cur_criteria.2 + 1)
            * (cur_criteria.5 - cur_criteria.4 + 1)
            * (cur_criteria.7 - cur_criteria.6 + 1);
        out += combinations as u64;
    }
    out
}

fn p2(file: &str) -> u64 {
    let file_str = fs::read_to_string(file).expect("cannot read");

    let mut workflows = HashMap::new();
    for line in file_str.split('\n') {
        if line.len() < 2 {
            continue;
        }
        if &line[0..1] == "{" {
            //part
            //parts.push(load_part(line.trim()));
        } else {
            let wf = load_workflow(line.trim());
            workflows.insert(wf.name.to_string(), wf);
        }
    }
    let criteria_lists: Vec<Vec<Criterion>> = find_criteria_list(&workflows);
    find_combinations(&criteria_lists)
}
fn main() {
    let file = "data/day19.txt";
    println!("{}", p1(file));
    println!("{}", p2(file));
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, fs};

    use crate::{
        find_criteria_list, load_part, load_workflow, p1, p2, update_by_criteria, Comparator,
        Criterion, Part, Rating, Rule, Workflow, WorkflowResult,
    };

    #[test]
    fn test_step() {
        let part = Part {
            x: 787,
            m: 2655,
            a: 1222,
            s: 2876,
        };
        let step = Rule {
            rating: Some(Rating::S),
            comparator: Some(Comparator::LessThan),
            value: Some(1351),
            is_accept: false,
            is_reject: false,
            next_workflow: Some(String::from("px")),
        };
        let result = step.apply(&part);
        assert_eq!(result, WorkflowResult::NextStep);

        let step = Rule {
            rating: None,
            comparator: None,
            value: None,
            is_accept: false,
            is_reject: false,
            next_workflow: Some(String::from("qqz")),
        };
        let result = step.apply(&part);
        assert_eq!(result, WorkflowResult::NextWorkflow);

        let part = Part {
            x: 0,
            m: 2655,
            a: 1222,
            s: 0,
        };
        let step = Rule {
            rating: Some(Rating::S),
            comparator: Some(Comparator::LessThan),
            value: Some(1351),
            is_accept: false,
            is_reject: false,
            next_workflow: Some(String::from("px")),
        };
        let result = step.apply(&part);
        assert_eq!(result, WorkflowResult::NextWorkflow);
    }

    #[test]
    fn test_workflow() {
        /*
        {x=787,m=2655,a=1222,s=2876}: in -> qqz -> qs -> lnx -> A
        {x=1679,m=44,a=2067,s=496}: in -> px -> rfg -> gd -> R
        {x=2036,m=264,a=79,s=2244}: in -> qqz -> hdj -> pv -> A
        {x=2461,m=1339,a=466,s=291}: in -> px -> qkq -> crn -> R
        {x=2127,m=1623,a=2188,s=1013}: in -> px -> rfg -> A
         */
        let part = Part {
            x: 787,
            m: 2655,
            a: 1222,
            s: 2876,
        };
        let part2 = Part {
            x: 1679,
            m: 44,
            a: 2067,
            s: 496,
        };
        let part3 = Part {
            x: 2036,
            m: 264,
            a: 79,
            s: 2244,
        };
        let part4 = Part {
            x: 2461,
            m: 1339,
            a: 466,
            s: 291,
        };
        let part5 = Part {
            x: 2127,
            m: 1623,
            a: 2188,
            s: 1013,
        };

        let steps = vec![
            Rule {
                rating: Some(Rating::S),
                comparator: Some(Comparator::LessThan),
                value: Some(1351),
                is_accept: false,
                is_reject: false,
                next_workflow: Some(String::from("px")),
            },
            Rule {
                rating: None,
                comparator: None,
                value: None,
                is_accept: false,
                is_reject: false,
                next_workflow: Some(String::from("qqz")),
            },
        ];
        let wf = Workflow {
            name: "in".to_string(),
            steps: steps,
        };
        let (result, next) = wf.apply(&part);
        assert_eq!(result, WorkflowResult::NextWorkflow);
        assert!(next.is_some());
        assert_eq!(next.unwrap(), "qqz".to_string());

        let (result, next) = wf.apply(&part2);
        assert_eq!(result, WorkflowResult::NextWorkflow);
        assert!(next.is_some());
        assert_eq!(next.unwrap(), "px".to_string());

        let (result, next) = wf.apply(&part3);
        assert_eq!(result, WorkflowResult::NextWorkflow);
        assert!(next.is_some());
        assert_eq!(next.unwrap(), "qqz".to_string());

        let (result, next) = wf.apply(&part4);
        assert_eq!(result, WorkflowResult::NextWorkflow);
        assert!(next.is_some());
        assert_eq!(next.unwrap(), "px".to_string());

        let (result, next) = wf.apply(&part5);
        assert_eq!(result, WorkflowResult::NextWorkflow);
        assert!(next.is_some());
        assert_eq!(next.unwrap(), "px".to_string());

        let lnx_steps = vec![
            Rule {
                rating: Some(Rating::M),
                comparator: Some(Comparator::MoreThan),
                value: Some(1548),
                is_accept: true,
                is_reject: false,
                next_workflow: None,
            },
            Rule {
                rating: None,
                comparator: None,
                value: None,
                is_accept: true,
                is_reject: false,
                next_workflow: None,
            },
        ];
        let lnx = Workflow {
            name: "lnx".to_string(),
            steps: lnx_steps,
        };

        let lnx_result = lnx.apply(&part);
        assert_eq!(lnx_result.0, WorkflowResult::Accept);

        let gd_steps = vec![
            Rule {
                rating: Some(Rating::A),
                comparator: Some(Comparator::MoreThan),
                value: Some(3333),
                is_accept: false,
                is_reject: true,
                next_workflow: None,
            },
            Rule {
                rating: None,
                comparator: None,
                value: None,
                is_accept: false,
                is_reject: true,
                next_workflow: None,
            },
        ];
        let gd = Workflow {
            name: "gd".to_string(),
            steps: gd_steps,
        };

        let gd_result = gd.apply(&part2);
        assert_eq!(gd_result.0, WorkflowResult::Reject);

        let pv = Workflow {
            name: "pv".to_string(),
            steps: vec![
                Rule {
                    rating: Some(Rating::A),
                    comparator: Some(Comparator::MoreThan),
                    value: Some(1716),
                    is_accept: false,
                    is_reject: true,
                    next_workflow: None,
                },
                Rule {
                    rating: None,
                    comparator: None,
                    value: None,
                    is_accept: true,
                    is_reject: false,
                    next_workflow: None,
                },
            ],
        };
        assert_eq!(pv.apply(&part3).0, WorkflowResult::Accept);

        let crn = Workflow {
            name: "crn".to_string(),
            steps: vec![
                Rule {
                    rating: Some(Rating::X),
                    comparator: Some(Comparator::MoreThan),
                    value: Some(2662),
                    is_accept: true,
                    is_reject: false,
                    next_workflow: None,
                },
                Rule {
                    rating: None,
                    comparator: None,
                    value: None,
                    is_accept: false,
                    is_reject: true,
                    next_workflow: None,
                },
            ],
        };
        assert_eq!(crn.apply(&part4).0, WorkflowResult::Reject);

        let rfg = Workflow {
            name: "rfg".to_string(),
            steps: vec![
                Rule {
                    rating: Some(Rating::S),
                    comparator: Some(Comparator::LessThan),
                    value: Some(537),
                    is_accept: false,
                    is_reject: false,
                    next_workflow: Some(String::from("gd")),
                },
                Rule {
                    rating: Some(Rating::X),
                    comparator: Some(Comparator::MoreThan),
                    value: Some(2440),
                    is_accept: false,
                    is_reject: true,
                    next_workflow: None,
                },
                Rule {
                    rating: None,
                    comparator: None,
                    value: None,
                    is_accept: true,
                    is_reject: false,
                    next_workflow: None,
                },
            ],
        };
        assert_eq!(rfg.apply(&part5).0, WorkflowResult::Accept);
    }

    #[test]
    fn test_part_workflows() {
        let qqz = Workflow {
            name: "qqz".to_string(),
            steps: vec![
                Rule {
                    rating: Some(Rating::S),
                    comparator: Some(Comparator::MoreThan),
                    value: Some(2770),
                    is_accept: false,
                    is_reject: false,
                    next_workflow: Some(String::from("qs")),
                },
                Rule {
                    rating: Some(Rating::M),
                    comparator: Some(Comparator::LessThan),
                    value: Some(1801),
                    is_accept: false,
                    is_reject: false,
                    next_workflow: Some(String::from("hdj")),
                },
                Rule {
                    rating: None,
                    comparator: None,
                    value: None,
                    is_accept: false,
                    is_reject: true,
                    next_workflow: None,
                },
            ],
        };

        let lnx = Workflow {
            name: "lnx".to_string(),
            steps: vec![
                Rule {
                    rating: Some(Rating::M),
                    comparator: Some(Comparator::MoreThan),
                    value: Some(1548),
                    is_accept: true,
                    is_reject: false,
                    next_workflow: None,
                },
                Rule {
                    rating: None,
                    comparator: None,
                    value: None,
                    is_accept: true,
                    is_reject: false,
                    next_workflow: None,
                },
            ],
        };

        let qs = Workflow {
            name: "qs".to_string(),
            steps: vec![
                Rule {
                    rating: Some(Rating::S),
                    comparator: Some(Comparator::MoreThan),
                    value: Some(3448),
                    is_accept: true,
                    is_reject: false,
                    next_workflow: None,
                },
                Rule {
                    rating: None,
                    comparator: None,
                    value: None,
                    is_accept: false,
                    is_reject: false,
                    next_workflow: Some(String::from("lnx")),
                },
            ],
        };

        let in_wf = Workflow {
            name: "in".to_string(),
            steps: vec![
                Rule {
                    rating: Some(Rating::S),
                    comparator: Some(Comparator::LessThan),
                    value: Some(1351),
                    is_accept: false,
                    is_reject: false,
                    next_workflow: Some(String::from("px")),
                },
                Rule {
                    rating: None,
                    comparator: None,
                    value: None,
                    is_accept: false,
                    is_reject: false,
                    next_workflow: Some(String::from("qqz")),
                },
            ],
        };

        let mut map = HashMap::new();
        map.insert(in_wf.name.clone(), in_wf);
        map.insert(qs.name.clone(), qs);
        map.insert(lnx.name.clone(), lnx);
        map.insert(qqz.name.clone(), qqz);

        let part = Part {
            x: 787,
            m: 2655,
            a: 1222,
            s: 2876,
        };

        let result = part.apply(&map);

        assert_eq!(result, WorkflowResult::Accept);
    }

    #[test]
    fn test_update_by_criteria() {
        let c = Criterion {
            rating: Rating::X,
            comparator: Comparator::MoreThan,
            value: 10,
        };

        let cl = Criterion {
            rating: Rating::X,
            comparator: Comparator::LessThan,
            value: 3500,
        };

        let mut lower = 1;
        let mut upper = 4000;
        update_by_criteria(&mut lower, &mut upper, &c);
        update_by_criteria(&mut lower, &mut upper, &cl);
        assert_eq!(lower, 11);
        assert_eq!(upper, 3499);

        let c = Criterion {
            rating: Rating::X,
            comparator: Comparator::MoreThan,
            value: 9,
        };

        let cl = Criterion {
            rating: Rating::X,
            comparator: Comparator::LessThan,
            value: 3501,
        };
        update_by_criteria(&mut lower, &mut upper, &c);
        update_by_criteria(&mut lower, &mut upper, &cl);
        assert_eq!(lower, 11);
        assert_eq!(upper, 3499);

        let c = Criterion {
            rating: Rating::X,
            comparator: Comparator::MoreThan,
            value: 99,
        };

        let cl = Criterion {
            rating: Rating::X,
            comparator: Comparator::LessThan,
            value: 3000,
        };
        update_by_criteria(&mut lower, &mut upper, &c);
        update_by_criteria(&mut lower, &mut upper, &cl);
        assert_eq!(lower, 100);
        assert_eq!(upper, 2999);
    }

    #[test]
    fn test_find_criteria_list() {
        let file = "data/day19_ex.txt";
        let file_str = fs::read_to_string(file).expect("cannot read");

        let mut workflows = HashMap::new();
        for line in file_str.split('\n') {
            if line.len() < 2 {
                continue;
            }
            if &line[0..1] == "{" {
                //part
                //parts.push(load_part(line.trim()));
            } else {
                let wf = load_workflow(line.trim());
                workflows.insert(wf.name.to_string(), wf);
            }
        }
        let criteria_lists: Vec<Vec<Criterion>> = find_criteria_list(&workflows);

        assert!(!criteria_lists.is_empty());

        let item = criteria_lists.iter().find(|list| {
            list.len() == 3
                && list[0].value == 1351
                && list[1].value == 2005
                && list[2].value == 2090
        });
        assert!(item.is_some());

        let item = criteria_lists
            .iter()
            .find(|list| list.len() > 2 && list[0].value == 1350 && list[1].value == 2770);
        assert!(item.is_some());

        let item = criteria_lists
            .iter()
            .find(|list| list.len() > 2 && list[0].value == 1350 && list[1].value == 2771);
        assert!(item.is_some());
    }

    #[test]
    fn test_load_workflow() {
        let line = "px{a<2006:qkq,m>2090:A,rfg}";

        let wf = load_workflow(line);
        assert_eq!(wf.name, String::from("px"));
        assert_eq!(wf.steps.len(), 3);
        let f = &wf.steps[0];
        assert_eq!(f.rating.as_ref().unwrap().eq(&Rating::A), true);
        assert_eq!(f.value.unwrap(), 2006);
        assert_eq!(f.comparator.as_ref().unwrap(), &Comparator::LessThan);
        assert_eq!(f.next_workflow.as_ref().unwrap(), &String::from("qkq"));
        assert_eq!(f.is_accept, false);
        assert_eq!(f.is_reject, false);
        let f = &wf.steps[1];
        assert_eq!(f.rating.as_ref().unwrap().eq(&Rating::M), true);
        assert_eq!(f.value.unwrap(), 2090);
        assert_eq!(f.comparator.as_ref().unwrap(), &Comparator::MoreThan);
        assert_eq!(f.is_accept, true);
        assert_eq!(f.is_reject, false);
        assert_eq!(f.next_workflow.is_none(), true);
        let f = &wf.steps[2];
        assert_eq!(f.rating.is_none(), true);
        assert_eq!(f.value.is_none(), true);
        assert_eq!(f.comparator.is_none(), true);
        assert_eq!(f.is_accept, false);
        assert_eq!(f.is_reject, false);
        assert_eq!(f.next_workflow.as_ref().unwrap(), &String::from("rfg"));

        let line = "pv{a>1716:R,A}";

        let wf = load_workflow(line);
        assert_eq!(wf.name, String::from("pv"));
        assert_eq!(wf.steps.len(), 2);
        let f = &wf.steps[0];
        assert_eq!(f.rating.as_ref().unwrap().eq(&Rating::A), true);
        assert_eq!(f.value.unwrap(), 1716);
        assert_eq!(f.comparator.as_ref().unwrap(), &Comparator::MoreThan);
        assert_eq!(f.next_workflow.is_none(), true);
        assert_eq!(f.is_reject, true);
        assert_eq!(f.is_accept, false);
        let f = &wf.steps[1];
        assert_eq!(f.rating.is_none(), true);
        assert_eq!(f.value.is_none(), true);
        assert_eq!(f.comparator.is_none(), true);
        assert_eq!(f.is_accept, true);
        assert_eq!(f.is_reject, false);
        assert_eq!(f.next_workflow.is_none(), true);
    }

    #[test]
    fn test_load_part() {
        let p = "{x=787,m=2655,a=1222,s=2876}";

        let result: Part = load_part(p);
        assert_eq!(result.x, 787);
        assert_eq!(result.m, 2655);
        assert_eq!(result.a, 1222);
        assert_eq!(result.s, 2876);
    }

    #[test]
    fn test_p1() {
        let file = "data/day19_ex.txt";
        assert_eq!(p1(file), 19114);
    }

    #[test]
    fn test_p2() {
        let file = "data/day19_ex.txt";
        assert_eq!(p2(file), 167409079868000);
    }
}
