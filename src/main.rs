use std::{io, thread, time::Duration};
use std::cmp::Ordering;
use rand::Rng;
use crate::asciimon::Asciimon;
use crate::state::State;
use std::ops::Range;

mod asciimon;
mod state;

const TITLE: &str = r"_______ _______ _______ _____ _____ _______  _____  __   _
|_____| |______ |         |     |   |  |  | |     | | \  |
|     | ______| |_____  __|__ __|__ |  |  | |_____| |  \_|
";
const SPACING: &str = "                             ";

fn print_title() {
    print!("\x1B[2J");
    println!("{}", TITLE);
}

fn print_state(state: &State) {
    print!("\x1B[2J"); // works to clear screen
    print!("{}", state.contender.sprite);
    print!("{SPACING}");
    println!("{}", state.opponent.sprite);
    print!("{}", state.contender.name);
    print!("{SPACING}");
    println!("{}", state.opponent.name);
    print!("{}", state.contender.health_pretty());
    print!("{SPACING}");
    println!("{}", state.opponent.health_pretty());

    println!("\n\nChoose what to do...\n");
    println!(" 1) Attack 1{SPACING}2) Attack");
    println!("1 turn cooldown{SPACING}2 turn cooldown");
}

fn main() {
    print_title();
    thread::sleep(Duration::from_secs(2));
    // use a second thread here to setup the game while showing title.

    // todo get from list of availables
    let asciis: Vec<Asciimon> = Asciimon::get_list();

    // todo let user pick thier ascii
    println!("Choose your AsciiMon!");
    // Would list them here.
    let rn = 1u8..4u8;
    let choice = get_input("Choose! 1-3", rn);
    println!("Your choice was: {choice}");

    let sprite = String::from("(^_^)");
    let name = "Floppy".to_string();
    let contender = Asciimon::new(name, sprite);
 
    let mut state = setup_game(contender);

    // Play the game
    loop {
        print_state(&state);
        // todo put attacks in own method
        // todo add initiave order per battle
        let attack = get_input("text", 1u8..3u8);
        // todo need to map u8 to an "attack" enum?
        let x = rand::thread_rng().gen_range(1..=6);
        println!("Your attack is: {attack} does {x} damage");
        state.opponent.take_damage(x);

        if state.opponent.is_dead {
            println!("You win!");
            break;
        }

        let y = rand::thread_rng().gen_range(1..=6);
        println!("Now they would attack back for {y} damage");
        state.contender.take_damage(y);
        thread::sleep(Duration::from_secs(2));

        if state.contender.is_dead {
            println!("You Lose!\nYou get nothing!");
            break;
        }
    }
}

fn get_input(text: &str, range: Range<u8>) -> u8 { //  range: range
    let mut choice = String::new();
    println!("{text}");
    loop {
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let opt: u8 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // Make sure choice is within specified range
        if range.contains(&opt) {
            println!("You chose: {opt}");
            return opt
        }
    }
}

fn setup_game(contender: Asciimon) -> State {
    State {
        contender,
        opponent: Asciimon::get_enemy()
    }
}
