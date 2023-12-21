use std::collections::{HashMap, VecDeque};
use std::hash::{Hash, Hasher};
use num::Integer;
advent_of_code::solution!(20);


#[derive(Clone, Debug, Copy, Eq, PartialEq)]
enum Pulse {
    Low,
    High
}


#[derive(Clone, Debug, Copy, Eq, PartialEq, Hash)]
enum FlipFlopState {
    On,
    Off
}


trait Trigger {
    fn process_pulse(&mut self, pulse: Pulse, input: &str) -> Option<Pulse>;
}


// Flip-Flop Trigger


struct FlipFlop {
    name: String,
    call_count: u64,
    state: FlipFlopState
}

impl FlipFlop {
    fn new(name: &str) -> Self {
        return FlipFlop {
            name: name.to_string(),
            call_count: 0,
            state: FlipFlopState::Off
        }
    }

    fn switch(&mut self, pulse: Pulse) {
        match self.state {
            FlipFlopState::On => self.state = FlipFlopState::Off,
            FlipFlopState::Off => self.state = FlipFlopState::On
        }
    }
}


impl Into<Pulse> for FlipFlopState {
    fn into(self) -> Pulse {
        match self {
            FlipFlopState::On => Pulse::High,
            FlipFlopState::Off => Pulse::Low
        }
    }
}


impl Trigger for FlipFlop {
    fn process_pulse(&mut self, pulse: Pulse, name: &str) -> Option<Pulse> {
        self.call_count += 1;
        if pulse == Pulse::High {
            return None;
        }
        self.switch(pulse);
        Some(self.state.into())
    }
}


// Conjunction trigger


#[derive(Debug)]
struct Conjunction {
    name: String,
    call_count: u64,
    state: HashMap<String, Pulse>
}

impl Conjunction {
    fn new(name: &str, connections: &HashMap<&str, Vec<&str>>) -> Self {
        let state = connections
            .iter()
            .filter(|(k, v)| v.contains(&name))
            .map(|(&k, v)| k)
            .fold(
                HashMap::new(),
                |mut map, v| {
                    map.insert(v.to_string(), Pulse::Low); map
                }
            );
        Conjunction {
            name: name.to_string(),
            call_count: 0,
            state
        }
    }
}

impl Trigger for Conjunction {
    fn process_pulse(&mut self, pulse: Pulse, input: &str) -> Option<Pulse> {
        self.call_count += 1;
        self.state.insert(input.to_string(), pulse);
        let all_high = self.state.values().all(|&val| val == Pulse::High);
        let pulse_send = if all_high { Pulse::Low } else { Pulse::High };
        Some(pulse_send)
    }
}


// broadcast trigger


struct Broadcast {
    name: String,
    call_count: u64
}


impl Broadcast{
    fn new(name: &str) -> Self {
        Broadcast {
            name: name.to_string(),
            call_count: 0,
        }
    }
}


impl Trigger for Broadcast {
    fn process_pulse(&mut self, pulse: Pulse, input: &str) -> Option<Pulse> {
        self.call_count += 1;
        Some(pulse)
    }
}

struct Dummy;


impl Trigger for Dummy {
    fn process_pulse(&mut self, pulse: Pulse, input: &str) -> Option<Pulse> {
        None
    }
}


#[derive(Debug)]
struct GridParseError;


struct TriggerGrid <'a> {
    trigger_map: HashMap<&'a str, Box<dyn Trigger>>,
    connections: HashMap<&'a str, Vec<&'a str>>,
}


impl <'a> TriggerGrid <'a> {
    fn new(rules: Vec<&'a str>) -> Result<Self, GridParseError> {
        let mut trigger_map = HashMap::new();
        let mut connections = HashMap::new();
        let mut type_map = HashMap::new();

        for line in rules.iter() {
            let [trigger, dest]: [&str; 2] =
                <[&str; 2]>::try_from(line.split(" -> ").collect::<Vec<&str>>()).unwrap();
            let destinations = dest.split(", ").collect::<Vec<&str>>();
            let name = &trigger[1..trigger.len()];
            type_map.insert(name, trigger.chars().next().unwrap());
            connections.insert(name, destinations.clone());
        }

        for &name in connections.keys() {
            let trigger: Box<dyn Trigger> = match type_map[name] {
                '&' => Box::new(Conjunction::new(name, &connections)) as Box<dyn Trigger>,
                '%' => Box::new(FlipFlop::new(name)) as Box<dyn Trigger>,
                'b' => Box::new(Broadcast::new(name)) as Box<dyn Trigger>,
                _ => return Err(GridParseError)
            };
            trigger_map.insert(name, trigger);
        }

        Ok(TriggerGrid { trigger_map, connections })
    }

    fn run_one(&'a mut self, iterations: usize) -> i128 {
        let (mut high_res, mut low_res) = (0i128, 0i128);
        for _ in 0..iterations {
            let mut queue = VecDeque::from([("roadcaster", Pulse::Low, "push")]);
            while queue.len() > 0 {
                let (trigger_name, pulse, prev) = queue.pop_front().unwrap();
                match pulse {
                    Pulse::Low => low_res += 1,
                    Pulse::High => high_res += 1
                }
                let trigger = self.trigger_map
                    .entry(trigger_name)
                    .or_insert(Box::new(Dummy {}) as Box<dyn Trigger>);
                let out_pulse = trigger.process_pulse(pulse, prev);
                self.append_childs(&mut queue, out_pulse, trigger_name);
            }
        }
        high_res*low_res
    }

    fn append_childs(
        &mut self,
        queue: & mut VecDeque<(&'a str, Pulse, &'a str)>,
        out_pulse: Option<Pulse>,
        trigger_name: &'a str
    ) {
        if let Some(val) = out_pulse {
            if !self.connections.contains_key(trigger_name) {
                return;
            }
            for &target_name in &self.connections[trigger_name] {
                queue.push_back((target_name, val, trigger_name));
            }
        }
    }


    fn run_two(&mut self, iterations: usize, feed: &str) -> Option<HashMap<&str, i64>>{
        let mut feed_map = HashMap::new();

        for i in 1..iterations {
            let mut queue = VecDeque::from([("roadcaster", Pulse::Low, "push")]);

            while queue.len() > 0 {
                let (trigger_name, pulse, prev) = queue.pop_front().unwrap();
                if (trigger_name == feed) && (pulse == Pulse::High) {
                    feed_map.entry(prev).or_insert(i as i64);
                }
                if self.connections
                    .iter()
                    .filter(|(&k, v)| v.contains(&feed))
                    .map(|(&k, v)| k)
                    .all(|a| feed_map.contains_key(a))
                {
                    return Some(feed_map);
                }
                let trigger = self.trigger_map
                    .entry(trigger_name)
                    .or_insert(Box::new(Dummy {}) as Box<dyn Trigger>);
                let out_pulse = trigger.process_pulse(pulse, prev);
                self.append_childs(&mut queue, out_pulse, trigger_name);
            }
        }
        None
    }
}


pub fn part_one(input: &str) -> Option<i128> {
    let rules = input.lines().collect::<Vec<&str>>();
    let mut grid = TriggerGrid::new(rules).unwrap();
    Some(grid.run_one(1000))
}

pub fn part_two(input: &str) -> Option<i64> {
    let rules = input.lines().collect::<Vec<&str>>();
    let mut grid = TriggerGrid::new(rules).unwrap();
    let feed_map = grid.run_two(6000, "tj").unwrap();
    feed_map.values().map(|&a| a).reduce(|a, b| a.lcm(&b))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(818649769));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(246313604784977));
    }
}
