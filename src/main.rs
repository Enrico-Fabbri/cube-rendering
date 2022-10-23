mod common;
mod world;

fn main() {
    env_logger::init();

    println!("Hello StoneHearth 2!");

    let state = pollster::block_on(common::state::State::new());
    state.run();
}
