pub struct World {
    pub tick: u32,
}

impl World {
    pub fn new() -> Self {
        World {
            tick: 0,
        }
    }

    pub fn init(&mut self) {
        println!("\nInitializing world...");
    }

    pub fn run(&mut self, ticks: u32) {
        println!("\nRunning world...");
        for _ in 0..ticks {
            self.tick += 1;
            println!("Tick: {}", self.tick);
        }
    }

    pub fn report(&self) {
        println!("\nWorld Report:");
    }
}
