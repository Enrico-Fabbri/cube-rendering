mod common;
mod world;

fn main() {
    println!("Hello StoneHearth 2!");
    pollster::block_on(run())
}

async fn run() {
    env_logger::init();

    let window = common::window::StoneHearthWindow::new();
    let state = common::state::StoneHearthState::new(&window).await;

    window.run(state);
}
