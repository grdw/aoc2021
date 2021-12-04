use std::fs;

fn main() {
    let bingo_numbers_string = fs::read_to_string("numbers")
                                  .unwrap_or("".to_string());

    let bingo_cards_string = fs::read_to_string("bingo_cards")
                                .unwrap_or("".to_string());

    let bingo_cards: Vec<&str> = bingo_cards_string
        .split("\n\n")
        .collect();

    let mut bingo_numbers: Vec<u16> = bingo_numbers_string
        .split(",")
        .map(|n| n.parse::<u16>().unwrap_or(0))
        .collect();

    // Last digit is a 0 and should be removed
    bingo_numbers.pop();

    //println!("{:?}", bingo_numbers);
    println!("{:?}", bingo_cards[0]);
}

#[derive(Debug)]
struct Square(u8, bool);

impl Square {
    pub fn mark(&mut self) {
        self.1 = true
    }
}

struct BingoCard {
    points: Vec<Vec<Square>>
}

impl BingoCard {
    fn from_str(card: &str) -> BingoCard {
        let points: Vec<Vec<Square>> = card
            .split("\n")
            .map(|row| {
                row.split(" ").map(|n| {
                    Square(n.parse::<u8>().unwrap(), false)
                }).collect()
            })
            .collect();

        BingoCard { points: points }
    }

    pub fn cross(&mut self, digit: u8) {
        for row in &mut self.points {
            for square in row {
                if square.0 == digit {
                    square.mark();
                    break;
                }
            }
        }
    }

    pub fn is_bingo(&self) -> bool {
        let mut is_bingo = false;
        let mut len = self.points.len();

        // Horizontal bingo
        for row in &self.points {
            if row.iter().all(|n| n.1) {
                is_bingo = true;
                break;
            }
        }

        // Vertical bingo
        while len > 0 {
            if self.points.iter().all(|row| row[len - 1].1) {
                is_bingo = true;
                break;
            }
            len -= 1
        }

        is_bingo
    }
}

#[test]
fn test_bingo_card_initialize() {
    let bingo_card = BingoCard::from_str(
        "59 98 84 27 56\n\
        17 35 18 64 34\n\
        62 16 74 26 55\n\
        21 99 1 19 93\n\
        65 68 53 24 73"
    );

    assert_eq!(bingo_card.points.len(), 5);
    assert_eq!(bingo_card.points[0].len(), 5);
}

#[test]
fn test_bingo_card_cross() {
    let mut bingo_card = BingoCard::from_str(
        "59 98 84 27 56\n\
        17 35 18 64 34\n\
        62 16 74 26 55\n\
        21 99 1 19 93\n\
        65 68 53 24 73"
    );

    bingo_card.cross(59);
    bingo_card.cross(5);

    assert!(bingo_card.points[0][0].1)
}

#[test]
fn test_bingo_card_is_bingo_horizontal() {
    let mut bingo_card = BingoCard::from_str(
        "59 98 84 27 56\n\
        17 35 18 64 34\n\
        62 16 74 26 55\n\
        21 99 1 19 93\n\
        65 68 53 24 73"
    );

    bingo_card.cross(59);
    bingo_card.cross(98);
    bingo_card.cross(84);
    bingo_card.cross(27);
    assert!(!bingo_card.is_bingo());

     bingo_card.cross(56);

    assert!(bingo_card.is_bingo())
}

#[test]
fn test_bingo_card_is_bingo_vertical() {
    let mut bingo_card = BingoCard::from_str(
        "59 98 84 27 56\n\
        17 35 18 64 34\n\
        62 16 74 26 55\n\
        21 99 1 19 93\n\
        65 68 53 24 73"
    );

    bingo_card.cross(59);
    bingo_card.cross(17);
    bingo_card.cross(62);
    bingo_card.cross(21);
    assert!(!bingo_card.is_bingo());

     bingo_card.cross(65);

    assert!(bingo_card.is_bingo())
}
