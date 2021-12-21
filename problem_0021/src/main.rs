fn main() {
    println!("Hello, world!");
}

struct Player {
    points: u64,
    position: u64
}

impl Player {
    fn roll(&mut self, r1: u64, r2: u64, r3: u64) {
        self.position += r1 + r2 + r3;
        self.points += self.position % 10;
    }
}

#[test]
fn test_player() {
    let mut player = Player { points: 0, position: 7 };
    player.roll(1, 2, 3);

    assert_eq!(player.position, 13);
    assert_eq!(player.points, 3);

    player.roll(7, 8, 9);
    assert_eq!(player.position, 37);
    assert_eq!(player.points, 10);
}

struct Game {
    players: Vec<Player>,
    rolls: u64
}

impl Game {
    fn play(&mut self) {
        while !self.players.iter().any(|p| p.points >= 1000) {
            let t = self.rolls * 3;
            let dicerolls: Vec<u64> = (t + 1..=t + 3)
                .map(|n| n)
                .collect();

            let len = self.players.len();
            let player = &mut self.players[
                self.rolls as usize % len
            ];

            player.roll(dicerolls[0], dicerolls[1], dicerolls[2]);

            self.rolls += 1;
        }
    }
}

#[test]
fn test_game() {
    let player_one = Player { points: 0, position: 7 };
    let player_two = Player { points: 0, position: 2 };
    let mut game = Game {
        players: vec![player_one, player_two],
        rolls: 0
    };
    game.play();

    let points: Vec<u64> = game.players
        .iter()
        .map(|p| p.points)
        .collect();

    assert_eq!(game.rolls, 287);
    assert_eq!(points, vec![1008, 498]);
}

