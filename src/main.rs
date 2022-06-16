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
    width: i8,
    bet: i32,
    balance: i32,
    deck: Vec<Card>,
    dealer_hand: Vec<Card>,
    player_hand: Vec<Card>
}

fn main() {
    println!("\n██████╗ ██╗      █████╗  █████╗ ██╗  ██╗     ██╗ █████╗  █████╗ ██╗  ██╗");
    println!("██╔══██╗██║     ██╔══██╗██╔══██╗██║ ██╔╝     ██║██╔══██╗██╔══██╗██║ ██╔╝");
    println!("██████╦╝██║     ███████║██║  ╚═╝█████═╝      ██║███████║██║  ╚═╝█████═╝ ");
    println!("██╔══██╗██║     ██╔══██║██║  ██╗██╔═██╗ ██╗  ██║██╔══██║██║  ██╗██╔═██╗ ");
    println!("██████╦╝███████╗██║  ██║╚█████╔╝██║ ╚██╗╚█████╔╝██║  ██║╚█████╔╝██║ ╚██╗");
    println!("╚═════╝ ╚══════╝╚═╝  ╚═╝ ╚════╝ ╚═╝  ╚═╝ ╚════╝ ╚═╝  ╚═╝ ╚════╝ ╚═╝  ╚═╝\n");

    let mut balance: i32 = 0;

    loop {
        let mut state = State {
            width: 73,
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

        for _ in 0..26 {
            println!();
        }

        let table_str: String = String::new();
        println!("{}", build_table_str(table_str, &state));

        let prompt_str: String = String::from("Bet amount?");
        print!("{}", build_prompt_str(prompt_str, &state));
        io::stdout().flush().unwrap();

        let mut bet: String = String::new();
        io::stdin().read_line(&mut bet).unwrap();

        let bet: i32 = bet.trim().parse::<i32>().unwrap();
        state.bet = bet;

        loop {
            let table_str: String = String::new();
            println!("{}", build_table_str(table_str, &state));

            if state.player_hand.len() == 2 && get_hand_value(&state.player_hand) == 21 {
                break;
            }

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

        settle(&mut state);

        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.to_lowercase().chars().next().unwrap() {
            'y' => {
                println!("\x1b[27F\x1b[0J");
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

fn settle(state: &mut State) {
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

    let player_natural: bool = state.player_hand.len() == 2 && player_value == 21;
    let dealer_natural: bool;

    let dealer_ace_or_ten: bool = state.dealer_hand[0].rank == 0 || state.dealer_hand[0].rank == 9;

    if state.dealer_hand.len() == 2 && dealer_ace_or_ten {
        dealer_natural = state.dealer_hand.len() == 2 && player_value == 21;

    } else {
        dealer_natural = false;
    }

    if  player_natural == true && dealer_natural == false {
        state.balance += state.bet + (state.bet / 2);

        let prompt_str: String = String::from("You blackjack! Deal?");
        print!("{}", build_prompt_str(prompt_str, &state));

    } else if dealer_natural == true && player_natural == false {
        state.balance -= state.bet;

        let prompt_str: String = String::from("Dealer blackjack! Deal?");
        print!("{}", build_prompt_str(prompt_str, &state));

    } else if player_natural == true && dealer_natural == true {
        let prompt_str: String = String::from("You draw! Deal?");
        print!("{}", build_prompt_str(prompt_str, &state));

    } else if player_value > 21 {
        state.balance -= state.bet;

        let prompt_str: String = String::from("You bust! Deal?");
        print!("{}", build_prompt_str(prompt_str, &state));

    } else if dealer_value > 21 {
        state.balance += state.bet;

        let prompt_str: String = String::from("Dealer bust! Deal?");
        print!("{}", build_prompt_str(prompt_str, &state));

    } else if player_value < dealer_value {
        state.balance -= state.bet;

        let prompt_str: String = String::from("You lose! Deal?");
        print!("{}", build_prompt_str(prompt_str, &state));

    } else if player_value > dealer_value {
        state.balance += state.bet;

        let prompt_str: String = String::from("You win! Deal?");
        print!("{}", build_prompt_str(prompt_str, &state));

    } else if player_value == dealer_value {
        let prompt_str: String = String::from("You draw! Deal?");
        print!("{}", build_prompt_str(prompt_str, &state));
    }

    io::stdout().flush().unwrap();
}

fn get_hand_value(hand: &Vec<Card>) -> i8 {
    let mut value: i8 = 0;
    let mut num_aces: i8 = 0;

    for card in hand {
        if card.flip == false {
            continue;

        } else if card.rank == 0 {
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
    let bet_len: i8 = state.bet.to_string().len() as i8;

    let width_len: i8 = get_widest_row(&state) - (26 + balance_len + prompt_len + bet_len);

    let balance_str: &str = &get_dash_str(balance_len);
    let prompt_str: &str = &get_dash_str(prompt_len);
    let bet_str: &str = &get_dash_str(bet_len);

    let space_str: &str = &get_space_str(width_len);
    let dash_str: &str = &get_dash_str(width_len);

    string += &format!("┌──────────{}─┬──────{}─┬─{}──{}─┐\n", balance_str, bet_str, prompt_str, dash_str);
    string += &format!("│ Balance: {} │ Bet: {} │ {}  {} │\n", state.balance, state.bet, prompt, space_str);
    string += &format!("└──────────{}─┴──────{}─┴─{}──{}─┘", balance_str, bet_str, prompt_str, dash_str);

    string += &format!("\x1b[1F\x1b[{}C", 23 + balance_len + prompt_len + bet_len);

    string
}

fn build_table_str(mut string: String, state: &State) -> String {
    let width: i8 = get_widest_row(&state);

    let dealer_value: i8 = get_hand_value(&state.dealer_hand);
    let player_value: i8 = get_hand_value(&state.player_hand);

    string += "\x1b[26F\x1b[0J";

    string = build_header_str(string, width, "Dealer's cards:");
    string = build_header_str(string, width, &format!("Value = {}", dealer_value));
    string = build_card_str(string, width, &state.dealer_hand);

    string += "\n";

    string = build_header_str(string, width, "Player's cards:");
    string = build_header_str(string, width, &format!("Value = {}", player_value));
    string = build_card_str(string, width, &state.player_hand);

    string
}

fn build_header_str(mut string: String, width: i8, header: &str) -> String {
    string += &get_space_str((width - header.len() as i8) / 2);
    string += header;
    string += "\n";

    string
}

fn build_card_str(mut string: String, width: i8, hand: &Vec<Card>) -> String {
    let cards_width: i8 = (hand.len() as i8 * 14) - 1;

    for row in 0..9 {
        string += &get_space_str((width - cards_width) / 2);

        for card in hand {
            string = build_card_row_str(string, card, row);
            string += " ";
        }

        string += "\n";
    }

    string
}

fn build_card_row_str(mut string: String, card: &Card, row: usize) -> String {
    const NEW_STRING: String = String::new();
    let mut card_str: [String; 9] = [NEW_STRING; 9];

    card_str[0] = String::from("┌───────────┐");
    card_str[8] = String::from("└───────────┘");

    if card.flip == true {
        const RANKS: [&str; 13] = ["A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K"];
        const SUITS: [&str; 4] = ["♥", "♦", "♠", "♣"];

        let rank: &str = RANKS[card.rank as usize];
        let suit: &str = SUITS[card.suit as usize];

        let color: &str = if card.suit < 2 { "\x1b[1;31m" } else { "\x1b[1;90m" };
        let reset: &str = "\x1b[0m";

        let top_rank_formatted: String = format!("{} ", rank);
        let bottom_rank_formatted: String = format!(" {}", rank);

        let top_rank: &str = if card.rank == 9 { rank } else { &top_rank_formatted };
        let bottom_rank: &str = if card.rank == 9 { rank } else { &bottom_rank_formatted };

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

    string += &card_str[row];

    string
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

    let mut width: i8 = (if player_cards > dealer_cards { player_cards } else { dealer_cards } * 14) - 1;
    if width < state.width { width = state.width; }

    width
}
