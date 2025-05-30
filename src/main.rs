mod game;
mod midi;

#[tokio::main]
async fn main() {
    game::game_main::start();
}
