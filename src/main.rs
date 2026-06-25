use ace::world::World;

fn main() {
    let tick: u32 = 100;
    let mut world = World::new();
    world.init();
    world.report();
    world.run(tick);
    world.report();
}
