use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Clone)]
struct House {
    x: i32,
    y: i32,
}

struct Navigator {
    map: HashMap<House, bool>,
    current_house: House,
    visited_houses: u32,
}

impl Navigator {
    fn new() -> Navigator {
        let current_house = House { x: 0, y: 0 };
        let mut navigator = Navigator { map: HashMap::new(), current_house: current_house.clone(), visited_houses: 0 };
        navigator.visit_house(current_house);

        navigator
    }

    fn next_house(&mut self, direction: char) {
        let new_house: Option<House> = match direction {
            '>' => Some(House { x: self.current_house.x + 1, y: self.current_house.y }),
            '<' => Some(House { x: self.current_house.x - 1, y: self.current_house.y }),
            '^' => Some(House { x: self.current_house.x, y: self.current_house.y + 1 }),
            'v' => Some(House { x: self.current_house.x, y: self.current_house.y - 1 }),
            _ => None,
        };

        match new_house {
            Some(new_house) => self.visit_house(new_house),
            None => (),
        }
    }

    fn visit_house(&mut self, house: House) {
        self.current_house = house;
        match self.map.insert(self.current_house.clone(), true) {
            Some(_) => (),
            None => self.visited_houses += 1,
        }
    }
}

fn main() {
    let mut directions = String::new();
    io::stdin().read_to_string(&mut directions).unwrap();

    let mut navigator = Navigator::new();

    for direction in directions.chars() {
        navigator.next_house(direction)
    }

    println!("Houses visited: {}", navigator.visited_houses);
}
