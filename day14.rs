use std::collections::HashMap;

#[derive(Debug)]
struct MemAction {
    address: u64,
    data: u64,
}

impl From<&str> for MemAction {
    fn from(input: &str) -> Self {
        let mut halves = input.split(']');
        let before = halves.next().expect("first half").trim();
        let after = halves.next().expect("second half").trim();
        let address = before[4..].parse::<u64>().expect("valid address");
        let data = 
            after.split('=')
                 .skip(1)
                 .next()
                 .expect("data")
                 .trim()
                 .parse::<u64>()
                 .expect("valid data");

        Self { address, data }
    }
}

#[derive(Debug)]
struct MemState<'a> {
    mem: HashMap<u64, u64>,
    ones_mask: u64,
    zeros_mask: u64,
    floating: Vec<u64>,
    program: &'a str,
}

#[derive(Copy, Clone)]
enum Version {
    V1,
    V2,
}

impl<'a> MemState<'a> {
    fn read_mask(input: &str) -> (u64, u64, Vec<u64>) {
        let mask = input.split('=').skip(1).next().expect("mask").trim();
        let mut ones_mask = 0;
        let mut zeros_mask = 0;
        let mut floating = vec![];
        for (i,c) in mask.char_indices() {
            let bit = 35-i;
            match c {
                'X' => floating.push(bit as u64),
                '1' => ones_mask |= 1 << bit,
                '0' => zeros_mask |= 1 << bit,
                _   => panic!("unexpected character")
            }
        }
        (ones_mask, zeros_mask, floating)
    }

    fn run_action_v1(&mut self, action: MemAction) {
        let mod_data = (action.data | self.ones_mask) & !(self.zeros_mask);
        self.mem.insert(action.address, mod_data);
    }

    fn run_action_v2(&mut self, action: MemAction) {
        let base_addr = action.address | self.ones_mask;
        let mut addresses = vec![base_addr];
        for bit in self.floating.iter() {
            let mask = 1 << bit;
            let mut oned_addr = addresses.clone();
            for addr in addresses.iter_mut() {
                let orig = *addr;
                *addr = orig & !mask;
                oned_addr.push(orig | mask);
            }
            addresses.append(&mut oned_addr);
        }

        for address in addresses {
            self.mem.insert(address, action.data);
        }
    }

    fn run_line(&mut self, line: &str, version: Version) {
        let line = line.trim();
        match &line[0..2]{
            "ma" => {
                let (ones_mask, zeros_mask, floating) = Self::read_mask(line);
                self.ones_mask = ones_mask;
                self.zeros_mask = zeros_mask;
                self.floating = floating;
            },
            "me" => {
                let action = MemAction::from(line);
                match version {
                    Version::V1 => self.run_action_v1(action),
                    Version::V2 => self.run_action_v2(action),
                }
            },
            x => panic!("unrecognized command {}", x),
        }
    }

    fn run(&mut self, version: Version) {
        for line in self.program.lines() {
            self.run_line(line, version);
        }
    }

    fn sum_memory(&self) -> u64 {
        self.mem.values().sum()
    }
}

impl<'a> From<&'a str> for MemState<'a> {
    fn from(input: &'a str) -> Self {
        Self {
            mem: HashMap::new(),
            ones_mask: 0,
            zeros_mask: 0,
            program: input,
            floating: vec![],
        }
    }
}

fn main() {
    let test_input = "\
    mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
    mem[8] = 11
    mem[7] = 101
    mem[8] = 0";

    let mut test = MemState::from(test_input);
    test.run(Version::V1);
    println!("test result is {}", test.sum_memory());

    let real_input = include_str!("day14.txt");
    let mut real = MemState::from(real_input);
    real.run(Version::V1);
    println!("real result is {}", real.sum_memory());

    let test2_input = "\
    mask = 000000000000000000000000000000X1001X
    mem[42] = 100
    mask = 00000000000000000000000000000000X0XX
    mem[26] = 1";

    let mut test = MemState::from(test2_input);
    test.run(Version::V2);
    println!("test2 result is {}", test.sum_memory());

    let mut real = MemState::from(real_input);
    real.run(Version::V2);
    println!("real2 result is {}", real.sum_memory());
}