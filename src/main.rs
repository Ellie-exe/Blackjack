use std::io;
use rand::thread_rng;
use rand::seq::SliceRandom;

struct Card {
    rank: i8,
    suit: i8
}

fn main() {
    let mut deck: Vec<Card> = Vec::new();

    for suit in 0..4 {
        for rank in 0..13 {
            let card = Card {
                rank,
                suit
            };

            deck.push(card);
        }
    }

    deck.shuffle(&mut thread_rng());
}
