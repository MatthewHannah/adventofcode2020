use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Seat {
    Empty,
    Occupied,
    Floor,
}

fn parse_line(line: &str) -> Vec<Seat> {
    let line = line.trim();
    line.chars()
        .map(|c| match c {
            'L' => Seat::Empty,
            '.' => Seat::Floor,
            '#' => Seat::Occupied,
            x   => panic!("weird character {:?}", x)
        })
        .collect()
}

struct State {
    seats : Vec<Vec<Seat>>,
}

impl State {
    fn new(lines: &str) -> Self {
        let seats = lines.lines()
                         .map(|s| parse_line(s))
                         .collect();
        Self { seats }
    }

    fn run_step(&mut self) {
        let mut new_seats = self.seats.clone();
        for i in 0..self.seats.len() {
            for j in 0..self.seats.len() {
                new_seats[i][j] = self.get_new_seat(i, j);
            }
        }
        self.seats = new_seats;
    }

    fn run_step_new(&mut self) {
        let mut new_seats = self.seats.clone();
        for i in 0..self.seats.len() {
            for j in 0..self.seats.len() {
                new_seats[i][j] = self.get_new_seat_mod(i, j);
            }
        }
        self.seats = new_seats;
    }

    fn run_until_stable(&mut self) {
        loop {
            let old_seats = self.seats.clone();
            self.run_step();
            let old_flat = old_seats.iter().flat_map(|r| r.iter());
            let new_flat = self.seats.iter().flat_map(|r| r.iter());
            if old_flat.eq(new_flat) { 
                break;
            }
        }
    }

    fn run_until_stable_new(&mut self) {
        loop {
            let old_seats = self.seats.clone();
            self.run_step_new();
            let old_flat = old_seats.iter().flat_map(|r| r.iter());
            let new_flat = self.seats.iter().flat_map(|r| r.iter());
            if old_flat.eq(new_flat) { 
                break;
            }
        }
    }

    fn find_result(&mut self) -> usize {
        self.run_until_stable();
        self.seats.iter()
                  .flat_map(|r| r.iter())
                  .filter(|s| if let Seat::Occupied = s { true } else { false })
                  .count()
    }

    fn find_result_new(&mut self) -> usize {
        self.run_until_stable_new();
        self.seats.iter()
                  .flat_map(|r| r.iter())
                  .filter(|s| if let Seat::Occupied = s { true } else { false })
                  .count()
    }

    fn get_adjacent(&self, i: usize, j: usize) -> Vec<Seat> {
        let mut seats = vec![];
        for i_off in -1..=1 {
            for j_off in -1..=1 {
                if i_off == j_off && j_off == 0 {
                    continue;
                }
                if (i as i64 + i_off < 0) || (j as i64 + j_off < 0) {
                    continue;
                }
                let other_i = (i as i64 + i_off) as usize;
                let other_j = (j as i64 + j_off) as usize;
                if other_i < self.seats.len() && other_j < self.seats[i].len() {
                    seats.push(self.seats[other_i][other_j]);
                }
            }
        }
        seats
    }

    fn get_far_adjacent(&self, i: usize, j: usize) -> Vec<Seat> {
        let mut seats = vec![];
        let directions = vec![(-1,-1), (-1,0), (-1,1), (0,-1), (0,1), (1,-1), (1,0), (1,1)];
        for direction in directions {
            let mut curr_i = (i as i64) + direction.0;
            let mut curr_j = (j as i64) + direction.1;
            let seat_len = self.seats.len() as i64;
            while curr_i >= 0 && curr_i < seat_len &&
                  curr_j >= 0 && curr_j < seat_len {
                
                match self.seats[curr_i as usize][curr_j as usize] {
                    Seat::Floor => (),
                    Seat::Empty => {
                        seats.push(Seat::Empty);
                        break;
                    },
                    Seat::Occupied => {
                        seats.push(Seat::Occupied);
                        break;
                    },
                };
                curr_i = curr_i + direction.0;
                curr_j = curr_j + direction.1;
            }
        }
        seats
    }

    fn get_new_seat(&mut self, i: usize, j: usize) -> Seat {
        match self.seats[i][j] {
            Seat::Empty => {
                let occupied = self.get_adjacent(i, j).iter().filter(|s| if let Seat::Occupied = s { true } else { false }).count();
                if occupied == 0 {
                    Seat::Occupied
                } else {
                    Seat::Empty
                }
            },
            Seat::Occupied => {
                let occupied = self.get_adjacent(i, j).iter().filter(|s| if let Seat::Occupied = s { true } else { false }).count();
                if occupied >= 4 {
                    Seat::Empty
                } else {
                    Seat::Occupied
                }
            }
            Seat::Floor => Seat::Floor,
        }
    }

    fn get_new_seat_mod(&mut self, i: usize, j: usize) -> Seat {
        match self.seats[i][j] {
            Seat::Empty => {
                let occupied = self.get_far_adjacent(i, j).iter().filter(|s| if let Seat::Occupied = s { true } else { false }).count();
                if occupied == 0 {
                    Seat::Occupied
                } else {
                    Seat::Empty
                }
            },
            Seat::Occupied => {
                let occupied = self.get_far_adjacent(i, j).iter().filter(|s| if let Seat::Occupied = s { true } else { false }).count();
                if occupied >= 5 {
                    Seat::Empty
                } else {
                    Seat::Occupied
                }
            }
            Seat::Floor => Seat::Floor,
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.seats.iter() {
            let mut row : String =
                row.iter()
                   .map( |s| match s {
                       Seat::Empty => 'L',
                       Seat::Floor => '.',
                       Seat::Occupied => '#',
                   })
                   .collect();
            row.push('\n');
            write!(f, "{}", row)?;
        }
        Ok(())
    }

}


fn main() {
    let test_input = "\
    L.LL.LL.LL
    LLLLLLL.LL
    L.L.L..L..
    LLLL.LL.LL
    L.LL.LL.LL
    L.LLLLL.LL
    ..L.L.....
    LLLLLLLLLL
    L.LLLLLL.L
    L.LLLLL.LL";

    let mut test_state = State::new(test_input);
    print!("{}", test_state);
    for i in 0..8 {
        test_state.run_step_new();
        println!("{}", i);
        print!("{}", test_state);
    }

    let mut test_state = State::new(test_input);
    println!("result for test is {}", test_state.find_result());

    let real_input = include_str!("day11.txt");

    let mut real_state = State::new(&real_input);
    println!("result for real is {}", real_state.find_result());

    let mut test_state = State::new(test_input);
    println!("result for test new is {}", test_state.find_result_new());
    let mut real_state = State::new(&real_input);
    println!("result for real is {}", real_state.find_result_new());


}