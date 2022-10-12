mod state;
mod window;

fn main() {
    println!("Hello StoneHearth 2!");
    pollster::block_on(run())
}

async fn run() {
    env_logger::init();

    let window = window::StoneHearthWindow::new();
    let state = state::StoneHearthState::new(&window).await;

    window.run(state);
}
