use rand::thread_rng;
use rand::seq::SliceRandom;

struct Card {
    rank: i8,
    suit: i8,
    flip: bool
}

struct State {
    deck: Vec<Card>,
    dealer_hand: Vec<Card>,
    player_hand: Vec<Card>
}

fn main() {
    let mut state = State {
        deck: Vec::new(),
        dealer_hand: Vec::new(),
        player_hand: Vec::new()
    };

    initialize_deck(&mut state.deck);
    state.deck.shuffle(&mut thread_rng());

    deal(&mut state);
    print(&state);
}

fn initialize_deck(deck: &mut Vec<Card>) {
    for suit in 0..4 {
        for rank in 0..13 {
            let card = Card {
                rank,
                suit,
                flip: false
            };

            deck.push(card);
        }
    }
}

fn deal(state: &mut State) {
    let mut flip = true;

    for _i in 0..2 {
        let player_card = state.deck.pop();
        let dealer_card = state.deck.pop();

        match player_card {
            Some(mut card) => {
                card.flip = true;
                state.player_hand.push(card);
            },

            None => return
        }

        match dealer_card {
            Some(mut card) => {
                card.flip = flip;
                state.dealer_hand.push(card);
            },

            None => return
        }

        flip = false;
    }
}

fn card_str(card: &Card, row: usize) -> String {
    let ranks = ["A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K"];
    let suits = ["♥", "♦", "♠", "♣"];

    let card_back = "░░░░░░░░░".to_string();

    let rank = ranks[card.rank as usize];
    let suit = suits[card.suit as usize];

    let red = "\x1b[1;31m";
    let black = "\x1b[1;90m";

    let color = if card.suit < 2 { red } else { black };
    let reset = "\x1b[0m";

    let top_rank = if card.rank == 9 { rank.to_string() } else { format!("{rank} ") };
    let bottom_rank = if card.rank == 9 { rank.to_string() } else { format!(" {rank}") };

    let top_row;
    let blank_row;
    let middle_row;
    let bottom_row;

    if card.flip == true {
        top_row = format!("{color}{top_rank}      {suit}{reset}");
        blank_row = format!("{color}         {reset}");
        middle_row = format!("{color}    {suit}    {reset}");
        bottom_row = format!("{color}{suit}      {bottom_rank}{reset}");

    } else {
        top_row = card_back.clone();
        blank_row = card_back.clone();
        middle_row = card_back.clone();
        bottom_row = card_back.clone();
    }

    let card_front = [
        format!("┌───────────┐"),
        format!("│ {top_row} │"),
        format!("│ {blank_row} │"),
        format!("│ {blank_row} │"),
        format!("│ {middle_row} │"),
        format!("│ {blank_row} │"),
        format!("│ {blank_row} │"),
        format!("│ {bottom_row} │"),
        format!("└───────────┘")
    ];

    return card_front[row].clone();
}

fn print(state: &State) {
    for i in 0..9 {
        println!("{} {}", card_str(&state.dealer_hand[0], i), card_str(&state.dealer_hand[1], i));
    }

    println!();

    for i in 0..9 {
        println!("{} {}", card_str(&state.player_hand[0], i), card_str(&state.player_hand[1], i));
    }
}
