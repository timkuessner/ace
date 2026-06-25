use std::collections::HashMap;
use crate::agents::person::Person;

pub struct World {
    pub tick: u32,
    pub persons: HashMap<u32, Person>,
}

impl World {
    pub fn new() -> Self {
        World {
            tick: 0,
            persons: HashMap::new(),
        }
    }

    pub fn init(&mut self) {
        println!("\nInitializing world...");
        for i in 0..10 {
            self.persons.insert(i, Person::new(i, 20 + (i % 40)));
        }
    }

    pub fn run(&mut self, ticks: u32) {
        println!("\nRunning world...");

        for _ in 0..ticks {
            self.step();
        }
    }

    fn step(&mut self) {
        self.tick += 1;

        for person in self.persons.values_mut() {
            person.step();
        }
    }

    pub fn report(&self) {
        println!("\nWorld Report:");
        println!("World Report - Tick: {}", self.tick);
        for person in self.persons.values() {
            person.report();
        }
    }
}
