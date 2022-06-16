use std::io;
use std::io::Write;
use rand::thread_rng;
use rand::seq::SliceRandom;

struct Card {
    rank: i8,
    suit: i8,
    flip: bool
}

struct State {
    balance: i32,
    deck: Vec<Card>,
    dealer_hand: Vec<Card>,
    player_hand: Vec<Card>
}

fn main() {
    println!("\nWelcome to blackjack!\n");
    let mut balance: i32 = 0;

    loop {
        let mut state = State {
            balance,
            deck: Vec::new(),
            dealer_hand: Vec::new(),
            player_hand: Vec::new()
        };

        initialize_deck(&mut state.deck);

        deal(&mut state.player_hand, &mut state.deck, 2);
        deal(&mut state.dealer_hand, &mut state.deck, 2);

        state.dealer_hand.last_mut().unwrap().flip = false;

        print!("Bet amount? ");
        io::stdout().flush().unwrap();

        let mut bet: String = String::new();
        io::stdin().read_line(&mut bet).unwrap();

        let bet: i32 = bet.trim().parse::<i32>().unwrap();

        for _ in 0..27 {
            println!();
        }

        loop {
            let table_str: String = String::new();
            println!("{}", build_table_str(table_str, &state));

            let prompt_str: String = String::from(">");
            print!("{}", build_prompt_str(prompt_str, &state));
            io::stdout().flush().unwrap();

            let mut input: String = String::new();
            io::stdin().read_line(&mut input).unwrap();

            match input.chars().next().unwrap() {
                'h' => {
                    deal(&mut state.player_hand, &mut state.deck, 1);
                    if get_hand_value(&state.player_hand) > 21 { break };
                },

                's' => {
                    break;
                },

                'q' => {
                    println!("\n");
                    return;
                },

                _ => {}
            }
        }

        state.dealer_hand.last_mut().unwrap().flip = true;

        if get_hand_value(&state.player_hand) <= 21 {
            while get_hand_value(&state.dealer_hand) < 17 {
                deal(&mut state.dealer_hand, &mut state.deck, 1);
                if get_hand_value(&state.player_hand) > 21 { break };
            }
        }

        let table_str: String = String::new();
        println!("{}", build_table_str(table_str, &state));

        let player_value: i8 = get_hand_value(&state.player_hand);
        let dealer_value: i8 = get_hand_value(&state.dealer_hand);

        if player_value > 21 {
            state.balance -= bet;

            let prompt_str: String = String::from("You bust! Deal?");
            print!("{}", build_prompt_str(prompt_str, &state));

        } else if dealer_value > 21 {
            state.balance += bet;

            let prompt_str: String = String::from("Dealer bust! Deal?");
            print!("{}", build_prompt_str(prompt_str, &state));

        } else if player_value < dealer_value {
            state.balance -= bet;

            let prompt_str: String = String::from("You lose! Deal?");
            print!("{}", build_prompt_str(prompt_str, &state));

        } else if player_value > dealer_value {
            state.balance += bet;

            let prompt_str: String = String::from("You win! Deal?");
            print!("{}", build_prompt_str(prompt_str, &state));

        } else if player_value == dealer_value {
            let prompt_str: String = String::from("You draw! Deal?");
            print!("{}", build_prompt_str(prompt_str, &state));
        }

        io::stdout().flush().unwrap();

        let mut response: String = String::new();
        io::stdin().read_line(&mut response).unwrap();

        match response.to_lowercase().chars().next().unwrap() {
            'y' => {
                println!("\x1b[29F\x1b[0J");
                balance = state.balance;
            },

            _ => {
                println!("\n");
                return;
            }
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

    deck.shuffle(&mut thread_rng());
}

fn deal(hand: &mut Vec<Card>, deck: &mut Vec<Card>, num_cards: i8) {
    for _ in 0..num_cards {
        let card_option: Option<Card> = deck.pop();

        match card_option {
            Some(mut card) => {
                card.flip = true;
                hand.push(card);
            },

            None => return
        }
    }
}

fn get_hand_value(hand: &Vec<Card>) -> i8 {
    let mut value: i8 = 0;
    let mut num_aces: i8 = 0;

    for card in hand {
        if card.rank == 0 {
            num_aces += 1;

        } else if card.rank > 9 {
            value += 10;

        } else {
            value += card.rank + 1;
        }
    }

    for _ in 0..num_aces {
        value += if value + 11 < 21 { 11 } else { 1 };
    }

    value
}

fn build_prompt_str(mut string: String, state: &State) -> String {
    let prompt: String = string.clone();
    string = String::from("");

    let balance_len: i8 = state.balance.to_string().len() as i8;
    let prompt_len: i8 = prompt.len() as i8;

    let space_len: i8 = get_widest_row(&state) - (16 + balance_len + prompt_len);
    let dash_len: i8 = get_widest_row(&state) - (16 + balance_len + prompt_len);

    let balance_str: &str = &get_dash_str(balance_len)[..];
    let prompt_str: &str = &get_dash_str(prompt_len)[..];

    let space_str: &str = &get_space_str(space_len)[..];
    let dash_str: &str = &get_dash_str(dash_len)[..];

    string += &format!("┌──────────{}─┬─{}{}─┐\n", balance_str, prompt_str, dash_str)[..];
    string += &format!("│ Balance: {} │ {}{} │\n", state.balance, prompt, space_str)[..];
    string += &format!("└──────────{}─┴─{}{}─┘", balance_str, prompt_str, dash_str)[..];

    string += &format!("\x1b[1F\x1b[{}C", (15 + balance_len + prompt_len))[..];

    string
}

fn build_table_str(mut string: String, state: &State) -> String {
    let width: i8 = get_widest_row(&state);

    let dealer_value: i8 = get_hand_value(&state.dealer_hand);
    let player_value: i8 = get_hand_value(&state.player_hand);

    string += "\x1b[26F\x1b[0J";

    add_header(&mut string, width, "Dealer's cards:");
    add_header(&mut string, width, &format!("Value = {}", dealer_value)[..]);
    add_cards(&mut string, width, &state.dealer_hand);

    string += "\n";

    add_header(&mut string, width, "Player's cards:");
    add_header(&mut string, width, &format!("Value = {}", player_value)[..]);
    add_cards(&mut string, width, &state.player_hand);

    string
}

fn add_header(string: &mut String, width: i8, header: &str) {
    string.push_str(&get_space_str((width - header.len() as i8) / 2)[..]);
    string.push_str(header);
    string.push_str("\n");
}

fn add_cards(string: &mut String, width: i8, hand: &Vec<Card>) {
    let cards_width: i8 = (hand.len() as i8 * 14) - 1;

    for row in 0..9 {
        string.push_str(&get_space_str((width - cards_width) / 2)[..]);

        for card in hand {
            string.push_str(&get_card_row(card, row)[..]);
            string.push_str(" ");
        }

        string.push_str("\n");
    }
}

fn get_card_row(card: &Card, row: usize) -> String {
    const RANKS: [&str; 13] = ["A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K"];
    const SUITS: [&str; 4] = ["♥", "♦", "♠", "♣"];

    const NEW_STRING: String = String::new();

    let mut card_str: [String; 9] = [NEW_STRING; 9];
    let mut face_str: [String; 7] = [NEW_STRING; 7];

    let rank: &str = RANKS[card.rank as usize];
    let suit: &str = SUITS[card.suit as usize];

    let color: &str = if card.suit < 2 { "\x1b[1;31m" } else { "\x1b[1;90m" };
    let reset: &str = "\x1b[0m";

    let top_rank: String = if card.rank == 9 { rank.to_string() } else { format!("{} ", rank) };
    let bottom_rank: String = if card.rank == 9 { rank.to_string() } else { format!(" {}", rank) };

    face_str[0] = format!("│ {}{}      {}{} │", color, top_rank, suit, reset);
    face_str[1] = format!("│ {}         {} │", color, reset);
    face_str[2] = format!("│ {}         {} │", color, reset);
    face_str[3] = format!("│ {}    {}    {} │", color, suit, reset);
    face_str[4] = format!("│ {}         {} │", color, reset);
    face_str[5] = format!("│ {}         {} │", color, reset);
    face_str[6] = format!("│ {}{}      {}{} │", color, suit, bottom_rank, reset);

    card_str[0] = String::from("┌───────────┐");

    for i in 1..8 {
        if card.flip == true {
            card_str[i] = face_str[i - 1].clone();

        } else {
            card_str[i] = String::from("│ ░░░░░░░░░ │");
        }
    }

    card_str[8] = String::from("└───────────┘");

    card_str[row].clone()
}

fn get_space_str(num: i8) -> String {
    let mut string: String = String::new();

    for _ in 0..num {
        string.push_str(" ");
    }

    string
}

fn get_dash_str(num: i8) -> String {
    let mut string: String = String::new();

    for _ in 0..num {
        string.push_str("─");
    }

    string
}

fn get_widest_row(state: &State) -> i8 {
    let dealer_cards: i8 = state.dealer_hand.len() as i8;
    let player_cards: i8 = state.player_hand.len() as i8;

    (if player_cards > dealer_cards { player_cards } else { dealer_cards } * 14) - 1
}
