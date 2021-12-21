type Player = (u64, u64);

fn add(player: &mut Player, r1: u64, r2: u64, r3: u64) {
    player.1 += r1 + r2 + r3;
    let n = (player.1 % 10) + 1;

    player.0 += n;
}

#[test]
fn test_player_1() {
    let mut player = (0, 6);
    add(&mut player, 1, 2, 3);

    assert_eq!(player.0, 3);

    add(&mut player, 7, 8, 9);
    assert_eq!(player.0, 10);
}

fn tryout(start: usize) {
    let mut cycle = 0;
    let mut previous_positions = vec![0; 10];
    let mut positions = vec![0; 10];
    positions[start] = 1;

    println!("START: {:?}", positions);
    loop {
        println!("{:?} {:?} {}", positions, previous_positions, cycle);
        let mut rolls = vec![];
        for i in 0..positions.len() {
            if positions[i] != previous_positions[i] {
                rolls.push(i);
                previous_positions[i] = positions[i];
            }
        }

        println!("{:?} {}", rolls, cycle);
        for r in rolls {
            positions[(r + 1) % 10] += 1;
            positions[(r + 2) % 10] += 1;
            positions[(r + 3) % 10] += 1;
        }

        cycle += 1;

        if cycle > 2 {
            break;
        }
    }
}

#[test]
fn test_tryout() {
    tryout(5);
}
