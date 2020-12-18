#[derive(Debug, Copy, Clone)]
enum Operation {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl From<&str> for Operation {
    fn from(line: &str) -> Operation {
        let line = line.trim();
        let immediate = line[4..].parse().unwrap();
        match &line[0..3] {
            "acc" => Operation::Acc(immediate),
            "jmp" => Operation::Jmp(immediate),
            "nop" => Operation::Nop(immediate),
            _ => panic!("line not valid"),
        }
    }
}

fn parse_program(program: &str) -> Vec<Operation> {
    program.lines()
           .map(|line| Operation::from(line))
           .collect()
}

struct MachineState {
    acc : i32,
    pc : usize,
    program : Vec<Operation>,
    visited : Vec<usize>,
}

impl MachineState {
    fn new(program: Vec<Operation>) -> MachineState {
        MachineState { acc: 0, pc: 0, program: program, visited: vec![] }
    }

    fn run_step(mut self) -> Self {
        let op = self.program[self.pc];
        self.visited.push(self.pc);
        match op {
            Operation::Nop(_) => MachineState { 
                pc: (self.pc + 1),
                ..self
            },
            Operation::Acc(val) => MachineState {
                pc: (self.pc + 1),
                acc: (self.acc + val),
                ..self
            },
            Operation::Jmp(val) => MachineState {
                pc: (self.pc as i64 + val as i64) as usize,
                ..self
            }
        }
    }

    fn run_till_end(self) -> Result<i32, i32> {
        let mut state = self;
        loop {
            if state.visited.contains(&state.pc) {
                break Err(state.acc);
            } else if state.pc >= state.program.len() {
                break Ok(state.acc);
            }
            state = state.run_step();
        }
    }

    fn flip_instruction(&mut self, op : usize) {
        let original_instruction = self.program[op];
        self.program[op] = match original_instruction {
            Operation::Acc(x) => Operation::Acc(x),
            Operation::Jmp(x) => Operation::Nop(x),
            Operation::Nop(x) => Operation::Jmp(x),
        };
    }

    fn run_till_end_fixed(self) -> i32 {
        let mut found = false;
        let mut ret = 0;
        let program = self.program.clone();

        for op in 0..self.program.len() {
            let mut new_machine = MachineState::new(program.clone());
            new_machine.flip_instruction(op);
            if let Ok(acc) = new_machine.run_till_end() {
                ret = acc;
                found = true;
                break
            }
        }

        if found {
            ret
        } else {
            panic!("Still haven't found a single flippable instruction")
        }
    }
}

fn main() {
    let test_input = "\
    nop +0
    acc +1
    jmp +4
    acc +3
    jmp -3
    acc -99
    acc +1
    jmp -4
    acc +6";

    let test_program = parse_program(test_input);
    println!("{:?}", test_program);

    println!("val of test program is {}", (MachineState::new(test_program)).run_till_end().expect_err("Did not loop"));

    let real_input = include_str!("day8.txt");
    let real_program = parse_program(real_input);
    println!("val of real program is {}", (MachineState::new(real_program.clone())).run_till_end().expect_err("Did not loop"));

    println!("fixed val of real program is {}", (MachineState::new(real_program.clone())).run_till_end_fixed());
}