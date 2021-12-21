struct Player {
    points: u64,
    position: u64
}

impl Player {
    fn new(position: u64) -> Player {
        Player { position: position - 1, points: 0 }
    }

    // assume the playing board as a vector:
    // {1,2,3,4,5,6,7,8,9,10}
    // [0,1,2,3,4,5,6,7,8,9]
    fn roll(&mut self, r1: u64, r2: u64, r3: u64) {
        self.position += r1 + r2 + r3;
        let n = (self.position % 10) + 1;

        self.points += n;
    }
}

#[test]
fn test_player_1() {
    let mut player = Player::new(7);
    player.roll(1, 2, 3);

    assert_eq!(player.points, 3);

    player.roll(7, 8, 9);
    assert_eq!(player.points, 10);
}

#[test]
fn test_player_2() {
    let mut player = Player::new(4);
    player.roll(1, 2, 3);

    assert_eq!(player.points, 10);

    player.roll(7, 8, 9);
    assert_eq!(player.points, 14);

    player.roll(13, 14, 15);
    assert_eq!(player.points, 20);
}

struct Dice(u64, u64);

impl Dice {
    fn new(max: u64) -> Dice {
        Dice(0, max)
    }

    fn roll(&mut self) -> u64 {
        self.0 += 1;

        if self.0 > 100 {
            self.0 = 1;
        }

        self.0
    }
}

#[test]
fn test_dice() {
    let mut dice = Dice::new(100);
    assert_eq!(dice.roll(), 1);
}

#[test]
fn test_dice_100() {
    let mut dice = Dice(100, 100);
    assert_eq!(dice.roll(), 1);
}

pub struct Game {
    dice: Dice,
    players: Vec<Player>,
    rolls: u64
}

impl Game {
    pub fn new(pos1: u64, pos2: u64) -> Game {
        Game {
            dice: Dice::new(100),
            players: vec![Player::new(pos1), Player::new(pos2)],
            rolls: 0
        }
    }

    pub fn play(&mut self) {
        while !self.players.iter().any(|p| p.points >= 1000) {
            let dicerolls: Vec<u64> = (0..3)
                .map(|_| self.dice.roll())
                .collect();

            let len = self.players.len();
            let who = self.rolls as usize % len;
            let player = &mut self.players[who];

            player.roll(dicerolls[0], dicerolls[1], dicerolls[2]);

            self.rolls += 3;
        }
    }

    pub fn part_1(&self) -> u64 {
        let points: Vec<u64> = self.players
            .iter()
            .map(|p| p.points)
            .filter(|&p| p < 1000)
            .collect();

        self.rolls * points[0]
    }
}

#[test]
fn test_game() {
    let player_one = Player::new(7);
    let player_two = Player::new(2);
    let dice = Dice::new(100);
    let mut game = Game {
        dice: dice,
        players: vec![player_one, player_two],
        rolls: 0
    };
    game.play();

    let points: Vec<u64> = game.players
        .iter()
        .map(|p| p.points)
        .collect();

    assert_eq!(game.rolls, 861);
    assert_eq!(points, vec![1008, 788]);
}

#[test]
fn test_part_1() {
    let player_one = Player::new(4);
    let player_two = Player::new(8);
    let dice = Dice::new(100);
    let mut game = Game {
        dice: dice,
        players: vec![player_one, player_two],
        rolls: 0
    };
    game.play();

    assert_eq!(game.rolls, 993);
    assert_eq!(game.part_1(), 739785);
}
