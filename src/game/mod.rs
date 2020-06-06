extern crate rand;
use rand::Rng;

pub struct Game {
    pub init: String,
    pub player1: String,
    pub player2: String,
    pub next: String,
    pub tricks: u32,
    pub turns: u32
}

impl Game {
    pub fn new(init: &String) -> Game {
        let length = init.len();

        Game {
            init: init.clone(),
            player1: init[..length/2].to_string(),
            player2: init[length/2..].to_string(),
            next: "".to_string(),
            tricks: 0,
            turns: 0
        }
    }

    pub fn random() -> Game {
        let cards: String = String::from("------------------------------------JJJJQQQQKKKKAAAA");
        let mut bytes = cards.into_bytes();
        let mut rng = rand::thread_rng();

        for i in (1..(bytes.len())).rev() {
            let temp;
            let pos = rng.gen_range(0,i);
            temp = bytes[i];
            bytes[i] = bytes[pos];
            bytes[pos] = temp;
        };

        let cards = String::from_utf8(bytes).unwrap();

        Game::new(&cards)
    }

    pub fn play(&mut self) {
        play(self);
    } 

    pub fn mutate(&mut self) -> Game {
        mutate(&self)
    }

    // pub fn mutate(hand: &Game) -> Game {
        // let mut bytes = hand.init.clone().into_bytes();
        // let mut rng = rand::thread_rng();
        // let temp: u8;
        // let mut marker: usize;
        // let mut marker2: usize;

        // loop {
            // marker = rng.gen_range(0,52);
            // if bytes[marker] != 45 {
                // continue;
            // }
            // temp = bytes[marker];

            // loop {
                // marker2 = rng.gen_range(0,52);
                // if bytes[marker2] == 45 {
                    // continue;
                // }
                // bytes[marker] = bytes[marker2];
                // bytes[marker2] = temp;
                // break;
            // }
            // break;
        // }

        // Game::new(&String::from_utf8(bytes).unwrap())
    // }
}

pub fn mutate(hand: &Game) -> Game {
    let mut bytes: Vec<u8> = hand.init.clone().into_bytes();
    let mut rng = rand::thread_rng();

    let mut marker: usize;

    loop {
        marker = rng.gen_range(0, 51);
        if bytes[marker] != bytes[marker + 1] {
            break;
        }
    }

    bytes.swap(marker, marker + 1);
    let cards = String::from_utf8(bytes);
    let cards = match cards {
        Ok(s) => s,
        _ => "Error".to_string()
    };
    Game::new(&cards)
}

// Implementation from https://github.com/matthewmayer/beggarmypython/blob/master/beggarmypython/__init__.py
fn play(hands: &mut Game) {

    let mut stack: String = "".to_string();
    let mut player = 1;
    let mut battle: bool;
    let mut debt: u32;

    while hands.player1.len() != 0 && hands.player2.len() != 0 {

        battle = false;
        debt = 1;

        while debt > 0 {
            hands.turns += 1;
            if player == 1 {
                if hands.player1.len() == 0 {
                    break;
                }
                hands.next = hands.player1[..1].to_string();
                hands.player1 = hands.player1[1..].to_string();
            } else {
                if hands.player2.len() == 0 {
                    break;
                }
                hands.next = hands.player2[..1].to_string();
                hands.player2 = hands.player2[1..].to_string();
            }
            // TODO: Catch this error: Running out of cards

            stack += &hands.next;

            if hands.next == "-" {
                if battle {
                    debt -= 1;
                } else {
                    player *= -1;
                }
            } else {
                battle = true;
                debt = penalty_value(&hands.next);
                player *= -1;
            }

        }
        hands.tricks += 1;

        if player == 1 {
            hands.player2 += &stack;
            stack = "".to_string();
        } else {
            hands.player1 += &stack;
            stack = "".to_string();
        }

        player *= -1;
    }
}

pub fn penalty_value(card: &String) -> u32 {
    match &card[..] {
        "J" => 1,
        "Q" => 2,
        "K" => 3,
        "A" => 4,
        _ => 0 
    }
}
