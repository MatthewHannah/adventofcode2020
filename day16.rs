use std::ops::RangeInclusive;

#[derive(Debug)]
struct Rule {
    field: String,
    first: RangeInclusive<u32>,
    last: RangeInclusive<u32>,
}

impl From<&str> for Rule {
    fn from(input: &str) -> Rule {
        let mut input = input.split(':');
        let field = input.next().unwrap();
        let mut second_half = 
            input.next()
                 .unwrap()
                 .trim()
                 .split("or");
        let mut first_range = second_half.next().unwrap().split('-');
        let mut second_range = second_half.next().unwrap().split('-');
        let num1 = first_range.next().unwrap().trim().parse().expect("num");
        let num2 = first_range.next().unwrap().trim().parse().expect("num");
        let num3 = second_range.next().unwrap().trim().parse().expect("num");
        let num4 = second_range.next().unwrap().trim().parse().expect("num");

        Rule {
            field: String::from(field),
            first: num1..=num2,
            last: num3..=num4,
        }
    }
}

#[derive(Debug)]
struct Ticket {
    fields: Vec<u32>
}

impl From<&str> for Ticket {
    fn from(input: &str) -> Ticket {
        let fields = 
            input.trim()
                 .split(',')
                 .map(|s| s.parse::<u32>().unwrap())
                 .collect();
        Ticket { fields }
    }
}

impl Ticket {
    fn empty() -> Self {
        Ticket { fields: Vec::new() }
    }
}

#[derive(Debug)]
struct State{
    rules: Vec<Rule>,
    my_ticket: Ticket,
    nearby: Vec<Ticket>,
    field_order: Vec<String>
}

#[derive(Debug)]
enum ParseState {
    Rules,
    MyTicket,
    NearbyTickets,
}

impl From<&str> for State {
    fn from(input: &str) -> State {
        let mut rules = Vec::new();
        let mut my_ticket = Ticket::empty();
        let mut nearby = Vec::new();
        let mut curr = ParseState::Rules;
        for line in input.lines() {
            let line = line.trim();
            match curr {
                ParseState::Rules => {
                    match line {
                        "your ticket:" => {
                            curr = ParseState::MyTicket;
                        },
                        "" => {},
                        x => {
                            rules.push(Rule::from(x));
                        }
                    };
                },
                ParseState::MyTicket => {
                    match line {
                        "nearby tickets:" => {
                            curr = ParseState::NearbyTickets;
                        },
                        "" => {},
                        x => {
                            my_ticket = Ticket::from(x);
                        }
                    }
                },
                ParseState::NearbyTickets => {
                    nearby.push(Ticket::from(line));
                },
            }
        }

        State { rules, my_ticket, nearby, field_order: Vec::new() }
    }
}

impl State {
    fn find_sure_invalid_fields(&self) -> Vec<u32> {
        use std::iter;

        let all_ranges : Vec<RangeInclusive<u32>> = 
            self.rules.iter()
                      .flat_map(|r| {
                          iter::once(r.first.clone())
                               .chain(iter::once(r.last.clone()))
                      })
                      .collect();
        self.nearby.iter()
                   .flat_map(|t| t.fields.iter())
                   .filter(|f| {
                       all_ranges.iter()
                                 .all(|r| !r.contains(f))
                   })
                   .map(|x| *x)
                   .collect()
    }

    fn remove_invalid_tickets(&mut self) {
        use std::iter;

        let all_ranges : Vec<RangeInclusive<u32>> = 
            self.rules.iter()
                      .flat_map(|r| {
                          iter::once(r.first.clone())
                               .chain(iter::once(r.last.clone()))
                      })
                      .collect();
        
        let mut i = 0;
        while i != self.nearby.len() {
            let is_invalid = 
                self.nearby[i].fields.iter()
                                     .any(|f| {
                                         all_ranges.iter()
                                                   .all(|r| !r.contains(f))
                                     });
            if is_invalid {
                self.nearby.remove(i);
            } else {
                i += 1;
            }
        }
    }

    fn deduce_fields(&mut self) {
        let mut valid_positions : Vec<(String,Vec<usize>)> =
            self.rules
                .iter()
                .map(|r| {
                    let valid_pos =
                        (0..self.rules.len())
                            .filter(|&pos| {
                                self.check_field_assignment(r, pos)
                            })
                            .collect::<Vec<usize>>();
                    (r.field.clone(), valid_pos)
                })
                .collect();
        // sort should speed things up when there are not multiple options
        valid_positions.sort_by_key(|(_r, v)| v.len());
        let mut assignments : Vec<Option<String>> = vec![None; self.rules.len()];
        let result = Self::assign_fields_recursive(&valid_positions, &mut assignments);
        let assignments : Vec<String> = assignments.into_iter().flatten().collect();
        if result {
            if assignments.len() != self.rules.len() {
                panic!("fields could not be deduced, not all rules assigned");
            }
        } else {
            panic!("fields could not be deduced, result as false");
        }
        self.field_order = assignments;
        println!("field order is {:?}", self.field_order);
    }

    fn assign_fields_recursive(
        valid: &[(String,Vec<usize>)],
        assigned: &mut Vec<Option<String>>,
    ) -> bool {
        // assume assigned is correct
        if valid.len() == 0 {
            return true;
        }
        let (rule, guesses) = &valid[0];
        //println!("entering function for rule {}, valid are {:?}, assigned are {:?}", rule.field, valid, assigned);
        for &guess in guesses {
            //println!("trying guess pos {} for rule {}", guess, rule.field);
            let mut assigned_guess = assigned.clone();
            assigned_guess[guess] = Some(rule.to_string());
            let mut valid_guess = Vec::from(valid);
            valid_guess.remove(0);
            for (_rule, pos) in valid_guess.iter_mut() {
                pos.retain(|&p| p != guess);
            }
            if Self::assign_fields_recursive(&valid_guess, &mut assigned_guess) {
                *assigned = assigned_guess.clone();
                return true;
            }
        }

        panic!("recursion should end above");
    }

    fn check_field_assignment(
        &self,
        rule: &Rule,
        field_num: usize,
    ) -> bool {
        use std::iter;

        self.nearby.iter()
                   .chain(iter::once(&self.my_ticket))
                   .map(|t| t.fields[field_num])
                   .all(|f| {
                       rule.first.contains(&f) || rule.last.contains(&f)
                   })
    }

    fn calculate_part2(&mut self) -> u64 {
        self.remove_invalid_tickets();
        self.deduce_fields();

        self.field_order
            .iter()
            .enumerate()
            .filter(|(_i, r)| r.starts_with("departure"))
            .map(|(i,_r)| (self.my_ticket.fields[i] as u64))
            .product()
    }
}

fn main() {
    let test_input = "\
    class: 1-3 or 5-7
    row: 6-11 or 33-44
    seat: 13-40 or 45-50

    your ticket:
    7,1,14

    nearby tickets:
    7,3,47
    40,4,50
    55,2,20
    38,6,12";

    let test_case = State::from(test_input);
    println!("test case {}", test_case.find_sure_invalid_fields().into_iter().sum::<u32>());

    let real_input = include_str!("day16.txt");
    let real = State::from(real_input);
    println!("real case {}", real.find_sure_invalid_fields().into_iter().sum::<u32>());

    let mut test_case2 = State::from(test_input);
    test_case2.remove_invalid_tickets();
    println!("cleaned up test case {:?}", test_case2);

    let test_input3 = "\
    class: 0-1 or 4-19
    row: 0-5 or 8-19
    seat: 0-13 or 16-19
    
    your ticket:
    11,12,13
    
    nearby tickets:
    3,9,18
    15,1,5
    5,14,9";

    let mut test_case3 = State::from(test_input3);
    test_case3.remove_invalid_tickets();
    test_case3.deduce_fields();

    let mut real2 = State::from(real_input);
    println!("result for our test case is {}", real2.calculate_part2());
}