use std::io;
use std::io::Write;
use rand::thread_rng;
use rand::seq::SliceRandom;
use std::cmp::Ordering;

struct Card {
    rank: i8,
    suit: i8,
    flip: bool
}

struct State {
    bet: i32,
    balance: i32,
    deck: Vec<Card>,
    dealer_hand: Vec<Card>,
    player_hand: Vec<Card>
}

macro_rules! print_table {
    ($a:expr, $b:expr) => {
        print_table($a, $b);
    };
}

fn main() {
    println!();

    println!("   ██████╗ ██╗      █████╗  █████╗ ██╗  ██╗     ██╗ █████╗  █████╗ ██╗  ██╗");
    println!("   ██╔══██╗██║     ██╔══██╗██╔══██╗██║ ██╔╝     ██║██╔══██╗██╔══██╗██║ ██╔╝");
    println!("   ██████╦╝██║     ███████║██║  ╚═╝█████═╝      ██║███████║██║  ╚═╝█████═╝ ");
    println!("   ██╔══██╗██║     ██╔══██║██║  ██╗██╔═██╗ ██╗  ██║██╔══██║██║  ██╗██╔═██╗ ");
    println!("   ██████╦╝███████╗██║  ██║╚█████╔╝██║ ╚██╗╚█████╔╝██║  ██║╚█████╔╝██║ ╚██╗");
    println!("   ╚═════╝ ╚══════╝╚═╝  ╚═╝ ╚════╝ ╚═╝  ╚═╝ ╚════╝ ╚═╝  ╚═╝ ╚════╝ ╚═╝  ╚═╝");

    println!("\n\n");

    let mut balance: i32 = 0;

    loop {
        let mut state = State {
            bet: 0,
            balance,
            deck: Vec::new(),
            dealer_hand: Vec::new(),
            player_hand: Vec::new()
        };

        initialize_deck(&mut state.deck);

        deal(&mut state.player_hand, &mut state.deck, 2);
        deal(&mut state.dealer_hand, &mut state.deck, 2);

        state.dealer_hand.last_mut().unwrap().flip = false;

        print_table!("Bet amount?", &state);
        let mut bet: String = String::new();
        io::stdin().read_line(&mut bet).unwrap();

        match bet.trim().parse() {
            Ok(value) => { state.bet = value; },
            Err(_) => { return; }
        }

        let natural: bool = check_for_naturals(&mut state);

        loop {
            if natural == true { break; }

            print_table!(">", &state);
            let mut input: String = String::new();
            io::stdin().read_line(&mut input).unwrap();

            let command: char;

            match input.to_lowercase().chars().next() {
                Some(c) => { command = c; },
                None => { continue; }
            }

            match command {
                'h' => {
                    deal(&mut state.player_hand, &mut state.deck, 1);
                    if get_hand_value(&state.player_hand) > 21 { break; }
                },

                'd' => {
                    match get_hand_value(&state.player_hand) {
                        9 | 10 | 11 => {
                            state.bet *= 2;

                            deal(&mut state.player_hand, &mut state.deck, 1);
                            if get_hand_value(&state.player_hand) > 21 { break; }
                        },

                        _ => { continue; }
                    }
                },

                's' => { break; },

                'q' => { println!("\n"); return; },

                _ => {}
            }
        }

        if natural == false { settle(&mut state); }

        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let res: char;

        match input.to_lowercase().chars().next() {
            Some(c) => { res = c; },
            None => { return; }
        }

        match res {
            'y' => {
                balance = state.balance;
            },

            _ => {
                println!("\x1b[25E");
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
        match deck.pop() {
            Some(mut card) => {
                card.flip = true;
                hand.push(card);
            },

            None => { return; }
        }
    }
}

fn check_for_naturals(state: &mut State) -> bool {
    if state.player_hand.len() > 2 && state.dealer_hand.len() > 2 { return false; }

    let player_value: i8 = get_hand_value(&state.player_hand);
    let dealer_value: i8 = get_hand_value(&state.dealer_hand);

    let player_natural: bool = state.player_hand.len() == 2 && player_value == 21;
    let dealer_natural: bool = state.dealer_hand.len() == 2 && match dealer_value {
        9 | 11 => {
            let card: &mut Card = &mut state.dealer_hand[1];

            match card.rank {
                9 | 11 => { card.flip = true; true },
                _ => { false }
            }
        },

        _ => { false }
    };

    let prompt: &str;

    match (player_natural, dealer_natural) {
        (true, false) => {
            prompt = "Win! You got a blackjack, deal again (Y/N)?";
            state.balance += state.bet + (state.bet / 2);
        },

        (false, true) => {
            prompt ="Lose! The dealer got a blackjack, deal again (Y/N)?";
            state.balance -= state.bet;
        },

        (true, true) => {
            prompt = "Draw! Both got a blackjack, deal again (Y/N)?";
        },

        _ => { return false; }
    }

    print_table!(prompt, &state);

    true
}

fn settle(state: &mut State) {
    state.dealer_hand.last_mut().unwrap().flip = true;

    let player_value: i8 = get_hand_value(&state.player_hand);
    let mut dealer_value: i8 = get_hand_value(&state.dealer_hand);

    if player_value <= 21 {
        while dealer_value < 17 {
            deal(&mut state.dealer_hand, &mut state.deck, 1);
            dealer_value = get_hand_value(&state.dealer_hand);

            if dealer_value > 21 { break; }
        }
    }

    let prompt: &str;

    if player_value > 21 {
        prompt = "Lose! You bust, deal again (Y/N)?";
        state.balance -= state.bet;

    } else if dealer_value > 21 {
        prompt = "Win! The dealer busts, deal again (Y/N)?";
        state.balance += state.bet;

    } else {
        match player_value.cmp(&dealer_value) {
            Ordering::Less => {
                prompt = "Lose! Deal again (Y/N)?";
                state.balance -= state.bet;
            },

            Ordering::Equal => {
                prompt = "Draw! Deal again (Y/N)?";
            },

            Ordering::Greater => {
                prompt = "Win! Deal again (Y/N)?";
                state.balance += state.bet;
            },
        }
    }

    print_table!(prompt, &state);
}

fn get_hand_value(hand: &Vec<Card>) -> i8 {
    let mut num_aces: i8 = 0;
    let mut value: i8 = 0;

    for card in hand {
        if card.flip == false { continue; }

        match card.rank {
            0 => { num_aces += 1; },
            10 | 11 | 12 => { value += 10; },
            _ => { value += card.rank + 1; }
        }
    }

    for _ in 0..num_aces {
        value += if value + 11 > 21 { 1 } else { 11 };
    }

    value
}

fn print_table(prompt: &str, state: &State) {
    let width: i8 = get_width(&state);

    let dealer_value: i8 = get_hand_value(&state.dealer_hand);
    let player_value: i8 = get_hand_value(&state.player_hand);

    println!("\x1b[3F\x1b[0J");

    print_prompt(prompt, &state);

    print_header("Dealer's cards:", dealer_value, width);
    print_cards(&state.dealer_hand, width);

    println!();

    print_header("Player's cards:", player_value, width);
    print_cards(&state.player_hand, width);

    print!("\x1b[u");
    io::stdout().flush().unwrap();
}

fn print_prompt(prompt: &str, state: &State) {
    let balance_len: i8 = state.balance.to_string().len() as i8;
    let bet_len: i8 = state.bet.to_string().len() as i8;
    let prompt_len: i8 = prompt.len() as i8;

    let width_len: i8 = get_width(&state) - (24 + balance_len + prompt_len + bet_len);

    let balance_str: &str = &gen_dashes(balance_len);
    let bet_str: &str = &gen_dashes(bet_len);
    let prompt_str: &str = &gen_dashes(prompt_len);

    let space_str: &str = &gen_spaces(width_len);
    let dash_str: &str = &gen_dashes(width_len);

    println!("┌──────────{}─┬──────{}─┬─{}{}─┐", balance_str, bet_str, prompt_str, dash_str);
    println!("│ Balance: {} │ Bet: {} │ {}{} │", state.balance, state.bet, prompt, space_str);
    println!("└──────────{}─┴──────{}─┴─{}{}─┘", balance_str, bet_str, prompt_str, dash_str);

    println!("\x1b[2F\x1b[{}C\x1b[s\x1b[2E", 23 + balance_len + prompt_len + bet_len);
}

fn print_header(header: &str, hand_value: i8, width: i8) {
    let value: String = format!("Value = {}", hand_value);

    let header_spaces: String = gen_spaces((width - header.len() as i8) / 2);
    let value_spaces: String = gen_spaces((width - value.len() as i8) / 2);

    println!("{}{}", header_spaces, header);
    println!("{}{}", value_spaces, value);
}

fn print_cards(hand: &Vec<Card>, width: i8) {
    let cards_width: i8 = (hand.len() as i8 * 14) - 1;
    let cards: Vec<[String; 9]> = gen_card_strings(hand);

    for row in 0..9 {
        let mut string: String = gen_spaces((width - cards_width) / 2);

        for card in &cards {
            string += &card[row];

            if cards.ends_with(&[card.clone()]) == false {
                string += " ";
            }
        }

        println!("{}", string);
    }
}

fn gen_card_strings(hand: &Vec<Card>) -> Vec<[String; 9]> {
    const RANKS: [&str; 13] = ["A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K"];
    const SUITS: [&str; 4] = ["♥", "♦", "♠", "♣"];
    const STRING: String = String::new();

    let mut cards: Vec<[String; 9]> = Vec::new();

    for card in hand {
        let mut card_str: [String; 9] = [STRING; 9];

        card_str[0] = String::from("┌───────────┐");
        card_str[8] = String::from("└───────────┘");

        if card.flip == true {
            let color: &str = if card.suit < 2 { "\x1b[1;31m" } else { "\x1b[1;90m" };
            let reset: &str = "\x1b[0m";

            let rank: &str = RANKS[card.rank as usize];
            let suit: &str = SUITS[card.suit as usize];

            let formatted_top_rank: String = format!("{} ", rank);
            let formatted_bottom_rank: String = format!(" {}", rank);

            let top_rank: &str = if card.rank == 9 { rank } else { &formatted_top_rank };
            let bottom_rank: &str = if card.rank == 9 { rank } else { &formatted_bottom_rank };

            card_str[1] = format!("│ {}{}      {}{} │", color, top_rank, suit, reset);
            card_str[2] = format!("│ {}         {} │", color, reset);
            card_str[3] = format!("│ {}         {} │", color, reset);
            card_str[4] = format!("│ {}    {}    {} │", color, suit, reset);
            card_str[5] = format!("│ {}         {} │", color, reset);
            card_str[6] = format!("│ {}         {} │", color, reset);
            card_str[7] = format!("│ {}{}      {}{} │", color, suit, bottom_rank, reset);

        } else if card.flip == false {
            for i in 1..8 {
                card_str[i] = String::from("│ ░░░░░░░░░ │");
            }
        }

        cards.push(card_str);
    }

    cards
}

fn gen_spaces(num: i8) -> String {
    let mut string: String = String::new();

    for _ in 0..num {
        string.push_str(" ");
    }

    string
}

fn gen_dashes(num: i8) -> String {
    let mut string: String = String::new();

    for _ in 0..num {
        string.push_str("─");
    }

    string
}

fn get_width(state: &State) -> i8 {
    let dealer_cards: i8 = state.dealer_hand.len() as i8;
    let player_cards: i8 = state.player_hand.len() as i8;

    let cards: i8 = if player_cards > dealer_cards { player_cards } else { dealer_cards };

    let width: i8 = (cards * 14) - 1;
    if width > 78 { width } else { 78 }
}
