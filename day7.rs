use std::collections::HashMap;

#[derive(Debug, Default)]
struct Bag {
    incoming : Vec<(usize, usize)>,
    outgoing : Vec<(usize, usize)>,
}

#[derive(Debug, Default)]
struct Bags {
    bags_map : HashMap<String, usize>,
    bags : Vec<Bag>
}

impl Bags {
    fn get_bag_idx(&mut self, label: &str) -> usize {
        if self.bags_map.contains_key(label) {
            *self.bags_map.get(label).unwrap()
        } else {
            let idx = self.bags.len();
            self.bags.push(Bag::default());
            self.bags_map.insert(String::from(label), idx);
            idx
        }
    }

    fn add_to_bag(&mut self, outer: &str, inner : &str, count : usize) {
        let outer_idx = self.get_bag_idx(outer);
        let inner_idx = self.get_bag_idx(inner);

        self.bags[outer_idx].outgoing.push((inner_idx, count));
        self.bags[inner_idx].incoming.push((outer_idx, count));
    }
}

fn process_line(line: &str, bags: &mut Bags) {
    let mut contain_split = line.split("contain");
    let first_half = contain_split.next().unwrap();
    let second_half = contain_split.next().unwrap();

    let owner_str = first_half.split("bags").next().unwrap().trim();

    let children = second_half.split(",");
    for child in children {
        let child = child.trim();
        if child == "no other bags." {
            continue
        }
        let mut child = child.split(" ");
        let num : usize = child.next().unwrap().parse().unwrap();
        let child_string = {
            let sheen = child.next().unwrap();
            let color = child.next().unwrap();
            let mut s = String::from(sheen);
            s.push_str(" ");
            s.push_str(color);
            s
        };
        let child_str = child_string.trim();

        bags.add_to_bag(owner_str, child_str, num);
    }
}

fn find_containable_recursive(
    idx: usize,
    bags: &mut Bags,
    acc: Vec<usize>,
) -> Vec<usize> {
    let mut containing_bags : Vec<usize> =
        bags.bags[idx].incoming.iter()
                               .map(|(bag_idx, _c)| *bag_idx)
                               .collect();
    let mut acc = acc;
    for &bag_idx in containing_bags.iter() {
        acc = find_containable_recursive(bag_idx, bags, acc)
    }
    acc.append(&mut containing_bags);
    acc
}

fn find_containable_count(label: &str, bags: &mut Bags) -> usize {
    let bag_idx = bags.get_bag_idx(label);
    let mut all_containers = find_containable_recursive(bag_idx, bags, Vec::new());
    all_containers.sort();
    all_containers.dedup();
    all_containers.len()
}

fn reduce_contained_recursive(
    idx: usize,
    bags: &mut Bags,
    acc : usize
) -> usize {
    let contained_bags = bags.bags[idx].outgoing.clone();
    let mut acc = acc;
    for (bag_idx, num) in contained_bags {
        acc += num * reduce_contained_recursive(bag_idx, bags, 0);
    }
    acc + 1
}

fn reduce_contained(label: &str, bags: &mut Bags) -> usize {
    let bag_idx = bags.get_bag_idx(label);
    reduce_contained_recursive(bag_idx, bags, 0) - 1
}

fn main() {
    let test_input = "\
    light red bags contain 1 bright white bag, 2 muted yellow bags.
    dark orange bags contain 3 bright white bags, 4 muted yellow bags.
    bright white bags contain 1 shiny gold bag.
    muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
    shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
    dark olive bags contain 3 faded blue bags, 4 dotted black bags.
    vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
    faded blue bags contain no other bags.
    dotted black bags contain no other bags.";
    let mut bags = Bags::default();
    for line in test_input.lines() {
        process_line(line, &mut bags);
    }

    println!("count is {}", find_containable_count("shiny gold", &mut bags));

    let test_input = "\
    shiny gold bags contain 2 dark red bags.
    dark red bags contain 2 dark orange bags.
    dark orange bags contain 2 dark yellow bags.
    dark yellow bags contain 2 dark green bags.
    dark green bags contain 2 dark blue bags.
    dark blue bags contain 2 dark violet bags.
    dark violet bags contain no other bags.";

    let mut bags = Bags::default();
    for line in test_input.lines() {
        process_line(line, &mut bags);
    }
    println!("test output sum count is {}", reduce_contained("shiny gold", &mut bags));

    let real_input = include_str!("day7.txt");
    let mut bags = Bags::default();
    for line in real_input.lines() {
        process_line(line, &mut bags);
    }
    println!("real count is {}", find_containable_count("shiny gold", &mut bags));
    println!("real sum count is {}", reduce_contained("shiny gold", &mut bags));
}