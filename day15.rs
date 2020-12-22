use std::collections::HashMap;

#[derive(Debug)]
enum Spoken {
    New(u64),
    Existing(u64, u64),
}

#[derive(Debug)]
struct State {
    spoken: HashMap<u64, Spoken>,
    prev: u64,
    turn: u64
}

impl State {
    fn new(starting: &[u64]) -> Self {
        let mut spoken = HashMap::new();
        let mut turn = 1;
        for &num in starting {
            spoken.insert(num, Spoken::New(turn));
            turn += 1;
        }
        Self {
            spoken: spoken,
            prev: *starting.last().unwrap(),
            turn: turn
        }
    }

    fn update_entry(&mut self, num: u64, turn: u64) {
        self.spoken.entry(num)
                   .and_modify(|e| {
                       *e = match e {
                           &mut Spoken::New(a) => Spoken::Existing(a,turn),
                           &mut Spoken::Existing(_,b) => Spoken::Existing(b,turn),
                       };
                   })
                   .or_insert(Spoken::New(turn));
    }

    fn step(&mut self) {
        let new_spoken = 
            if let Some(s) = self.spoken.get(&self.prev) {
                match s {
                    Spoken::New(_) => 0,
                    Spoken::Existing(a,b) => b - a,
                }
            } else {
                panic!("everything should always be in the map")
            };
        self.update_entry(new_spoken, self.turn);
        self.prev = new_spoken;
        //println!("{:?}",self);
        self.turn += 1;
    }

    fn step_until_turn(&mut self, num: u64) {
        for _ in self.turn..=num {
            self.step()
        }
    }
}

fn main() {
    let inputs = 
        vec![
            vec![1,3,2],
            vec![2,1,3],
            vec![1,2,3],
            vec![2,3,1],
            vec![3,2,1],
            vec![3,1,2],
            vec![5,1,9,18,13,8,0]
        ];
    
    for input in inputs {
        let mut test = State::new(&input);
        test.step_until_turn(2020);
        println!("2020 result of {:?} is {}", input, test.prev);
    }

    let rd2 = 
        vec![
            vec![0,3,6],
            vec![1,3,2],
            vec![2,1,3],
            vec![1,2,3],
            vec![2,3,1],
            vec![3,2,1],
            vec![3,1,2],
            vec![5,1,9,18,13,8,0],
        ];

    for input in rd2 {
        let mut test = State::new(&input);
        test.step_until_turn(30000000);
        println!("30000000 result of {:?} is {}", input, test.prev);
    }
}