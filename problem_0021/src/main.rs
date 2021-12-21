mod game_thousand;

use game_thousand::Game;

fn main() {
    let mut game = Game::new(5, 9);
    game.play();

    println!("Part 1: {}", game.part_1());
}
