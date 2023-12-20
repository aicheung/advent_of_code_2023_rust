use std::{
    collections::{HashMap, VecDeque},
    fs,
};


#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Debug)]
enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcast,
}

#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Debug)]
enum Pulse {
    H,
    L,
}

struct MessageQueue {
    low_count: u64,
    high_count: u64,
    members: HashMap<String, Module>,
    messages: VecDeque<(String, String, Pulse)>,
}

impl MessageQueue {
    fn associate_conjunction(&mut self) {
        let mut associations: HashMap<String, HashMap<String, Pulse>> = HashMap::new();
        for (_k, m) in self.members.iter() {
            if m.module_type == ModuleType::Conjunction {
                //find inputs 
                let inputs: HashMap<String, Pulse> = self.members.iter()
                .filter(|k| k.1.targets.contains(&m.name)).map(|k| (k.0.clone(), Pulse::L)).collect();
                associations.insert(m.name.clone(), inputs);
            }
        }

        for (_k, m) in self.members.iter_mut() {
            if m.module_type == ModuleType::Conjunction {
                let a = associations.get(&m.name);
                if a.is_some() {
                    m.conjunction_memory.extend(a.expect("").iter().map(|a| ((*a.0).clone(), *a.1)));
                }
            }
        }
    }

    fn enqueue(&mut self, source: String, target: String, pulse: Pulse) {
        match pulse {
            Pulse::H => self.high_count += 1,
            Pulse::L => self.low_count += 1,
        };

        let module = self.members.get(&target);
        if module.is_some() {
            //module might not exist
            let m = module.expect("Cannot get module");
            self.messages
                .push_back((source.clone(), m.name.clone(), pulse));
        }
    }
    fn process(&mut self) {
        self.process_until(None);
    }

    fn process_until(&mut self, target_message: Option<(String, String, Pulse)>) -> bool {
        let has_target = target_message.is_some();
        while !self.messages.is_empty() {
            let message = self.messages.pop_front().expect("Cannot get message");
            
            if has_target && message.eq(target_message.as_ref().unwrap()) {
                return true;
            }
            
            let module = self.members.get_mut(&message.1);
            if module.is_some() {
                //module might not exist
                let m: &mut Module = module.expect("Cannot get module");
                let new_messages = m.receive(message.0.clone(), message.2);
                for nm in new_messages {
                    self.enqueue(nm.0, nm.1, nm.2);
                }
            }
        }
        false
    }
}

struct Module {
    name: String,
    module_type: ModuleType,
    flip_flop_on: bool,
    conjunction_memory: HashMap<String, Pulse>,
    targets: Vec<String>,
}

impl Module {
    fn receive(&mut self, source: String, pulse: Pulse) -> Vec<(String, String, Pulse)> {
        match self.module_type {
            ModuleType::FlipFlop => return self.flip_flop_receive(pulse),
            ModuleType::Conjunction => return self.conjunction_receive(source, pulse),
            ModuleType::Broadcast => return self.broadcast_receive(pulse),
        }
    }

    fn send(&mut self, pulse: Pulse) -> Vec<(String, String, Pulse)> {
        let mut out = Vec::new();
        for t in self.targets.iter() {
            out.push((self.name.clone(), t.to_string(), pulse));
        }
        out
    }

    fn flip_flop_receive(&mut self, pulse: Pulse) -> Vec<(String, String, Pulse)> {
        if pulse == Pulse::L {
            match self.flip_flop_on {
                false => {
                    //If it was off, it turns on and sends a high pulse.
                    self.flip_flop_on = true;
                    return self.send(Pulse::H);
                }
                true => {
                    //If it was on, it turns off and sends a low pulse.
                    self.flip_flop_on = false;
                    return self.send(Pulse::L);
                }
            }
        }
        vec![]
    }

    fn update_conjunction_records(&mut self, source: String, pulse: Pulse) {
        self.conjunction_memory.insert(source, pulse);
    }

    fn is_all_conjunction_memory_high(&self) -> bool {
        self.conjunction_memory.iter().all(|c| *c.1 == Pulse::H)
    }

    fn conjunction_receive(
        &mut self,
        source: String,
        pulse: Pulse,
    ) -> Vec<(String, String, Pulse)> {
        self.update_conjunction_records(source, pulse);
        if self.is_all_conjunction_memory_high() {
            return self.send(Pulse::L);
        } else {
            return self.send(Pulse::H);
        }
    }

    fn broadcast_receive(&mut self, pulse: Pulse) -> Vec<(String, String, Pulse)> {
        return self.send(pulse);
    }
}

fn find_rx_sources(mq: &MessageQueue) -> (Vec<String>, String) {
    let final_con = mq.members.iter().find(|m| m.1.targets.contains(&"rx".to_string())).unwrap();
    (final_con.1.conjunction_memory.iter().map(|k| k.0.clone()).collect(), final_con.0.clone())
}

fn create_module(line: &str) -> Module {
    let module_type: ModuleType;
    let name: String;
    let split = line.split("->").map(|s| s.trim()).collect::<Vec<_>>();

    if line.contains("broadcaster") {
        module_type = ModuleType::Broadcast;
        name = String::from("broadcaster");
    } else {
        match &line[0..1] {
            "%" => module_type = ModuleType::FlipFlop,
            _ => module_type = ModuleType::Conjunction,
        }
        name = String::from(&split[0][1..]);
    }
    let targets: Vec<String> = split[1]
        .split(',')
        .map(|s| String::from(s.trim()))
        .collect();
    let conjunction_memory: HashMap<String, Pulse> = HashMap::new();

    let module = Module {
        name,
        module_type,
        flip_flop_on: false,
        conjunction_memory,
        targets,
    };
    module
}

fn build_mq(file: &str) -> MessageQueue {
    let mut mq = MessageQueue {
        low_count: 0,
        high_count: 0,
        members: HashMap::new(),
        messages: VecDeque::new(),
    };

    for line in fs::read_to_string(file).expect("Cannot open file").lines() {
        if line.len() <= 2 {
            continue;
        }
        let module = create_module(line);
        mq.members.insert(module.name.clone(), module);
    }
    mq.associate_conjunction();
    mq
}

//p2: press button until a message is encountered
fn press_until(mq: &mut MessageQueue, message: (String, String, Pulse)) -> u64 {
    let mut count = 0;
    loop {
        mq.enqueue("button".to_string(), "broadcaster".to_string(), Pulse::L);
        count += 1;
        let found = mq.process_until(Some(message.clone()));
        if found {
            break;
        }
    }

    count
}

fn p1(file: &str) -> u64 {
    let mut mq = build_mq(file);
    for _i in 0..1000 {
        mq.enqueue("button".to_string(), "broadcaster".to_string(), Pulse::L);
        mq.process();
    }

    mq.high_count * mq.low_count
}

fn lcm(walks: &HashMap<String, u64>) -> u64 {
    let nums: Vec<u64> = walks.iter().map(|w| *w.1).collect();
    let mut lcm: u64 = nums[0] as u64;
    for n in nums {
        lcm = num::integer::lcm(lcm, n as u64);
    }
    lcm
}

fn p2(file: &str) -> u64 {
    let mq = build_mq(file);
    let (targets, con) = find_rx_sources(&mq);

    let mut button_presses = HashMap::new();

    for t in targets {
        let mut new_mq = build_mq(file); //reset each time we run
        let count = press_until(&mut new_mq, (t.clone(), con.clone(), Pulse::H));

        button_presses.insert(t, count);
    }
    println!("{:?}", button_presses);
    lcm(&button_presses)
}

fn main() {
    let file = "data/day20.txt";
    println!("p1: {}", p1(file));
    println!("p2: {}", p2(file));
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_p1() {
        let file = "data/day20_ex.txt";
        let result: u64 = p1(file);
        assert_eq!(result, 32000000);

        let file = "data/day20_ex2.txt";
        let result: u64 = p1(file);
        assert_eq!(result, 11687500);
    }

    #[test]
    fn test_run_until() {
        let file = "data/day20.txt";

        let  mut mq = build_mq(file);
        let result = press_until(&mut mq, ("rr".to_string(), "hb".to_string(), Pulse::H));
        assert!(result > 0);
        println!("{}", result);
    }

    #[test]
    fn test_get_rx_sources() {
        //get all flipflops required for sending low to rx
        let file = "data/day20.txt";

        let mut mq = MessageQueue {
            low_count: 0,
            high_count: 0,
            members: HashMap::new(),
            messages: VecDeque::new(),
        };
    
        for line in fs::read_to_string(file).expect("Cannot open file").lines() {
            if line.len() <= 2 {
                continue;
            }
            let module = create_module(line);
            mq.members.insert(module.name.clone(), module);
        }
        mq.associate_conjunction();

        let result = find_rx_sources(&mq);
        assert_eq!(result.0.len(), 4);
        assert_eq!(result.1.len(), 2);
    }

    #[test]
    fn test_mq_associate_conjunction() {
        let mut mq = MessageQueue {
            low_count: 0,
            high_count: 0,
            members: HashMap::new(),
            messages: VecDeque::new()
        };

        let a = create_module("%a -> inv, con");
        let b = create_module("%b -> con");
        let c = create_module("&con -> output");
        mq.members.insert(a.name.clone(), a);
        mq.members.insert(b.name.clone(), b);
        mq.members.insert(c.name.clone(), c);

        mq.associate_conjunction();
        let c = mq.members.get(&"con".to_string()).unwrap();
        assert_eq!(c.conjunction_memory.len(), 2);
        assert!(c.conjunction_memory.iter().map(|c| *c.1).all(|p| p == Pulse::L));
        assert!(c.conjunction_memory.iter().map(|c| c.0.clone()).any(|p| p == "a"));
        assert!(c.conjunction_memory.iter().map(|c| c.0.clone()).any(|p| p == "b"));
    
    }

    #[test]
    fn test_create_module() {
        let bc = "broadcaster -> a, b, c";
        let m = create_module(bc);
        assert_eq!(m.name, "broadcaster".to_string());
        assert_eq!(m.module_type, ModuleType::Broadcast);
        assert_eq!(m.targets.len(), 3);
        assert_eq!(m.targets[0], "a".to_string());
        assert_eq!(m.targets[1], "b".to_string());
        assert_eq!(m.targets[2], "c".to_string());

        
        let bc = "%a -> b";
        let m = create_module(bc);
        assert_eq!(m.name, "a".to_string());
        assert_eq!(m.module_type, ModuleType::FlipFlop);
        assert_eq!(m.targets.len(), 1);
        assert_eq!(m.targets[0], "b".to_string());

        let bc = "&inv -> a";
        let m = create_module(bc);
        assert_eq!(m.name, "inv".to_string());
        assert_eq!(m.module_type, ModuleType::Conjunction);
        assert_eq!(m.targets.len(), 1);
        assert_eq!(m.targets[0], "a".to_string());
    }

    #[test]
    fn test_broadcast() {
        let mut m = create_module("broadcaster -> a, b, c");

        let result = m.receive("button".to_string(), Pulse::L);
        assert!(!result.is_empty());
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].0, "broadcaster".to_string());
        assert_eq!(result[0].1, "a".to_string());
        assert_eq!(result[0].2, Pulse::L);
        assert_eq!(result[1].0, "broadcaster".to_string());
        assert_eq!(result[1].1, "b".to_string());
        assert_eq!(result[1].2, Pulse::L);
        assert_eq!(result[2].0, "broadcaster".to_string());
        assert_eq!(result[2].1, "c".to_string());
        assert_eq!(result[2].2, Pulse::L);
    }

    #[test]
    fn test_conjunction() {
        let mut con = create_module("&con -> output");
        //test only
        con.conjunction_memory.insert("a".to_string(), Pulse::L);
        con.conjunction_memory.insert("b".to_string(), Pulse::L);

        let result = con.receive("a".to_string(), Pulse::L);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, "con".to_string());
        assert_eq!(result[0].1, "output".to_string());
        assert_eq!(result[0].2, Pulse::H);
        assert_eq!(*con.conjunction_memory.get(&"a".to_string()).unwrap(), Pulse::L);
        assert_eq!(*con.conjunction_memory.get(&"b".to_string()).unwrap(), Pulse::L);

        
        let result = con.receive("a".to_string(), Pulse::H);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, "con".to_string());
        assert_eq!(result[0].1, "output".to_string());
        assert_eq!(result[0].2, Pulse::H);
        assert_eq!(*con.conjunction_memory.get(&"a".to_string()).unwrap(), Pulse::H);
        assert_eq!(*con.conjunction_memory.get(&"b".to_string()).unwrap(), Pulse::L);

        
        let result = con.receive("b".to_string(), Pulse::H);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, "con".to_string());
        assert_eq!(result[0].1, "output".to_string());
        assert_eq!(result[0].2, Pulse::L);
        assert_eq!(*con.conjunction_memory.get(&"a".to_string()).unwrap(), Pulse::H);
        assert_eq!(*con.conjunction_memory.get(&"b".to_string()).unwrap(), Pulse::H);

        let result = con.receive("a".to_string(), Pulse::H);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, "con".to_string());
        assert_eq!(result[0].1, "output".to_string());
        assert_eq!(result[0].2, Pulse::L);
        assert_eq!(*con.conjunction_memory.get(&"a".to_string()).unwrap(), Pulse::H);
        assert_eq!(*con.conjunction_memory.get(&"b".to_string()).unwrap(), Pulse::H);

        let result = con.receive("a".to_string(), Pulse::L);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, "con".to_string());
        assert_eq!(result[0].1, "output".to_string());
        assert_eq!(result[0].2, Pulse::H);
        assert_eq!(*con.conjunction_memory.get(&"a".to_string()).unwrap(), Pulse::L);
        assert_eq!(*con.conjunction_memory.get(&"b".to_string()).unwrap(), Pulse::H);
    }

    #[test]
    fn test_flip_flop() {
        let mut m = Module {
            name: "ff".to_string(),
            module_type: ModuleType::FlipFlop,
            flip_flop_on: false,
            conjunction_memory: HashMap::new(),
            targets: vec!["bb".to_string(), "cc".to_string()],
        };

        let result = m.receive("a".to_string(), Pulse::H);
        assert!(result.is_empty());
        assert_eq!(m.flip_flop_on, false);

        let result = m.receive("a".to_string(), Pulse::L);
        assert!(!result.is_empty());
        assert_eq!(m.flip_flop_on, true);
        assert_eq!(result[0].0, "ff".to_string());
        assert_eq!(result[0].1, "bb".to_string());
        assert_eq!(result[0].2, Pulse::H);
        assert_eq!(result[1].0, "ff".to_string());
        assert_eq!(result[1].1, "cc".to_string());
        assert_eq!(result[1].2, Pulse::H);

        let result = m.receive("a".to_string(), Pulse::H);
        assert!(result.is_empty());
        assert_eq!(m.flip_flop_on, true);

        let result = m.receive("a".to_string(), Pulse::L);
        assert!(!result.is_empty());
        assert_eq!(m.flip_flop_on, false);
        assert_eq!(result[0].0, "ff".to_string());
        assert_eq!(result[0].1, "bb".to_string());
        assert_eq!(result[0].2, Pulse::L);
        assert_eq!(result[1].0, "ff".to_string());
        assert_eq!(result[1].1, "cc".to_string());
        assert_eq!(result[1].2, Pulse::L);

        let result = m.receive("a".to_string(), Pulse::H);
        assert!(result.is_empty());
        assert_eq!(m.flip_flop_on, false);
    }
}
