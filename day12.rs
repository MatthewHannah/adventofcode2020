#[derive(Debug, Copy, Clone)]
enum Action {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

impl Action {
    fn from_line(line: &str) -> Option<Action> {
        let line = line.trim();
        let action = line.get(0..1)?;
        let num = line.get(1..)?.parse::<i32>().ok()?;
        match action {
            "N" => Some(Action::North(num)),
            "S" => Some(Action::South(num)),
            "E" => Some(Action::East(num)),
            "W" => Some(Action::West(num)),
            "L" => Some(Action::Left(num)),
            "R" => Some(Action::Right(num)),
            "F" => Some(Action::Forward(num)),
            _   => None,
        }
    }

    fn from_instructions(lines: &str) -> Option<Vec<Action>> {
        lines.lines()
             .map(|l| Action::from_line(l))
             .collect()
    }
}

#[derive(Debug)]
struct StatePart1 {
    x : f64,
    y : f64,
    angle : f64
}

impl StatePart1 {
    fn new() -> Self {
        Self { x: 0.0, y: 0.0, angle: 0.0 }
    }

    fn take_action(&mut self, action : Action) {
        match action {
            Action::North(v) => self.y += v as f64,
            Action::South(v) => self.y -= v as f64,
            Action::East(v) => self.x += v as f64,
            Action::West(v) => self.x -= v as f64,
            Action::Left(a) => self.angle += a as f64,
            Action::Right(a) => self.angle -= a as f64,
            Action::Forward(v) => {
                let (sin, cos) = self.angle.to_radians().sin_cos();
                self.x += (v as f64) * cos;
                self.y += (v as f64) * sin;
            }
        };
    }

    fn from_instructions(instructions: &[Action]) -> Self {
        let mut state = Self::new();
        for &action in instructions {
            //println!("state: {:?}", state);
            //println!("next action: {:?}", action);
            state.take_action(action);
        }
        //println!("final state: {:?}", state);
        state
    }

    fn distance(&self) -> f64 {
        self.x.abs() + self.y.abs()
    }
}

#[derive(Debug)]
struct StatePart2 {
    way_x : f64,
    way_y : f64,
    ship_x : f64,
    ship_y : f64,
}

impl StatePart2 {
    fn new() -> Self {
        Self { way_x: 10.0, way_y: 1.0, ship_x: 0.0, ship_y: 0.0 }
    }

    fn take_action(&mut self, action : Action) {
        match action {
            Action::North(v) => self.way_y += v as f64,
            Action::South(v) => self.way_y -= v as f64,
            Action::East(v) => self.way_x += v as f64,
            Action::West(v) => self.way_x -= v as f64,
            Action::Left(a) => {
                let (sin, cos) = (a as f64).to_radians().sin_cos();
                let new_way_x = self.way_x * cos - self.way_y * sin;
                let new_way_y = self.way_x * sin + self.way_y * cos;
                self.way_x = new_way_x;
                self.way_y = new_way_y;
            },
            Action::Right(a) => {
                let (sin, cos) = (-a as f64).to_radians().sin_cos();
                let new_way_x = self.way_x * cos - self.way_y * sin;
                let new_way_y = self.way_x * sin + self.way_y * cos;
                self.way_x = new_way_x;
                self.way_y = new_way_y;
            },
            Action::Forward(v) => {
                self.ship_x += (v as f64) * self.way_x;
                self.ship_y += (v as f64) * self.way_y;
            },
        };
    }

    fn from_instructions(instructions: &[Action]) -> Self {
        let mut state = Self::new();
        for &action in instructions {
            //println!("state: {:?}", state);
            //println!("next action: {:?}", action);
            state.take_action(action);
        }
        //println!("final state: {:?}", state);
        state
    }

    fn distance(&self) -> f64 {
        self.ship_x.abs() + self.ship_y.abs()
    }
}

fn main() {
    let test_input = "\
    F10
    N3
    F7
    R90
    F11";

    println!("test result {}", StatePart1::from_instructions(&Action::from_instructions(&test_input).expect("valid instructions")).distance());

    let real_input = include_str!("day12.txt");
    println!("real result {}", StatePart1::from_instructions(&Action::from_instructions(&real_input).expect("valid instructions")).distance());

    println!("test round two {}", StatePart2::from_instructions(&Action::from_instructions(&test_input).expect("valid instructions")).distance());
    println!("real round two {}", StatePart2::from_instructions(&Action::from_instructions(&real_input).expect("valid instructions")).distance());


}