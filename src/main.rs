mod game;

use game::Game;

fn main() {

    let mut total: usize = 0;

    let mut best: Game = Game::new(&String::from("J-------Q------AAA-----QQ-K----JA-----------KQ-K-JJK"));
    best.play();

    loop {
        total += 1;

        let mut hand = Game::random();

        hand.play();

        if hand.turns > best.turns {
            best = hand;

            println!("Turns: {}, Tricks: {}, Iter: {}", best.turns, best.tricks, total);
            println!("Hands: {}", best.init);

        }
    }

}
