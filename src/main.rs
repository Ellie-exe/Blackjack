use std::io;
use std::io::Write;
use rand::thread_rng;
use rand::seq::SliceRandom;

struct Card {
    rank: i8,
    suit: i8,
    flip: bool
}

struct Hand {
    value: i8,
    ace_value: i8,
    cards: Vec<Card>
}

struct State {
    deck: Vec<Card>,
    bet: i32,

    dealer_hand: Hand,
    player_hand: Hand
}

fn main() {
    let mut state = State {
        deck: Vec::new(),
        bet: 0,

        dealer_hand: Hand {
            value: 0,
            ace_value: 1,
            cards: Vec::new()
        },

        player_hand: Hand {
            value: 0,
            ace_value: 1,
            cards: Vec::new()
        }
    };

    initialize_deck(&mut state.deck);
    state.deck.shuffle(&mut thread_rng());

    deal(&mut state, 0);
    deal(&mut state, 0);

    print!("\nBet> $");

    let mut bet = String::new();

    io::stdout()
        .flush()
        .expect("Failed to flush line");

    io::stdin()
        .read_line(&mut bet)
        .expect("Failed to read line");

    let bet: i32 = bet.trim().parse().expect("Please type a number!");
    state.bet = bet;

    loop {
        let mut input = String::new();

        print(&state);

        if is_natural(&state.player_hand) && !is_natural(&state.dealer_hand) {
            return;
        }

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.chars().next() {
            Some(c) => {
                match c {
                    'h' => {
                        deal(&mut state, 0);
                    },

                    's' => {
                        deal(&mut state, 1);
                    },

                    'a' => {
                        let hand = &mut state.player_hand;

                        hand.ace_value = if hand.ace_value == 1 { 11 } else { 1 };
                        hand.value = hand_value(hand);
                    },

                    'q' => {
                        println!();
                        return;
                    },

                    _ => continue
                }
            },

            None => continue
        }
    }
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

fn hand_value(hand: &mut Hand) -> i8 {
    let mut value = 0;

    for card in &hand.cards {
        value += if card.rank == 0 { hand.ace_value }
        else if card.rank > 10 { 10 }
        else { card.rank + 1 };
    }

    value
}

fn is_natural(hand: &Hand) -> bool {
    let is_ace_or_ten_1 = hand.cards[0].rank == 0 || hand.cards[0].rank == 9;
    let is_ace_or_ten_2 = hand.cards[1].rank == 0 || hand.cards[1].rank == 9;

    is_ace_or_ten_1 && is_ace_or_ten_2
}

fn deal(state: &mut State, play_type: i8) {
    let mut hand = &mut state.player_hand;

    for hand_type in play_type..2 {
        if play_type == 1 { hand = &mut state.dealer_hand; }
        let card_option = state.deck.pop();

        match card_option {
            Some(mut card) => {
                if hand_type == 0 {
                    card.flip = true;

                } else {
                    let last_card_option = hand.cards.last_mut();

                    match last_card_option {
                        Some(last_card) => last_card.flip = true,
                        None => card.flip = true
                    }
                }

                hand.cards.push(card);
                hand.value = hand_value(hand);
            },

            None => return
        }

        hand = &mut state.dealer_hand;
    }
}

fn card_str(card: &Card, row: usize) -> String {
    let ranks = ["A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K"];
    let suits = ["♥", "♦", "♠", "♣"];

    let card_back = String::from("░░░░░░░░░");

    let rank = ranks[card.rank as usize];
    let suit = suits[card.suit as usize];

    let red = "\x1b[1;31m";
    let black = "\x1b[1;90m";

    let color = if card.suit < 2 { red } else { black };
    let reset = "\x1b[0m";

    let top_rank = if card.rank == 9 { rank.to_string() } else { format!("{} ", rank) };
    let bottom_rank = if card.rank == 9 { rank.to_string() } else { format!(" {}", rank) };

    let mut top_row = card_back.clone();
    let mut blank_row = card_back.clone();
    let mut middle_row = card_back.clone();
    let mut bottom_row = card_back.clone();

    if card.flip == true {
        top_row = format!("{}{}      {}{}", color, top_rank, suit, reset);
        blank_row = format!("{}         {}", color, reset);
        middle_row = format!("{}    {}    {}", color, suit, reset);
        bottom_row = format!("{}{}      {}{}", color, suit, bottom_rank, reset);
    }

    let card_front = [
        format!("┌───────────┐"),
        format!("│ {} │", top_row),
        format!("│ {} │", blank_row),
        format!("│ {} │", blank_row),
        format!("│ {} │", middle_row),
        format!("│ {} │", blank_row),
        format!("│ {} │", blank_row),
        format!("│ {} │", bottom_row),
        format!("└───────────┘")
    ];

    card_front[row].clone()
}

fn space_str(num: i8) -> String {
    let mut string = String::new();

    for _i in 0..num {
        string += " ";
    }

    string
}

fn print(state: &State) {
    let mut num_cards = state.dealer_hand.cards.len();

    let mut num_spaces = (((num_cards * 14) - 16) / 2) as i8;
    println!("\n{}Dealer's cards:{}", space_str(num_spaces), space_str(num_spaces));

    num_spaces = (((num_cards * 14) - (8 + state.dealer_hand.value.to_string().len())) / 2) as i8;
    println!("{}Value = {}{}", space_str(num_spaces), state.dealer_hand.value, space_str(num_spaces));

    for row in 0..9 {
        for index in 0..num_cards {
            print!("{} ", card_str(&state.dealer_hand.cards[index], row));
        }

        println!();
    }

    num_cards = state.player_hand.cards.len();

    num_spaces = (((num_cards * 14) - 16) / 2) as i8;
    println!("\n{}Player's cards:{}", space_str(num_spaces), space_str(num_spaces));

    num_spaces = (((num_cards * 14) - (8 + state.player_hand.value.to_string().len())) / 2) as i8;
    println!("{}Value = {}{}", space_str(num_spaces), state.player_hand.value, space_str(num_spaces));

    for row in 0..9 {
        for index in 0..num_cards {
            print!("{} ", card_str(&state.player_hand.cards[index], row));
        }

        println!();
    }

    print!("\nInput> ");

    io::stdout()
        .flush()
        .expect("Failed to flush line");
}
