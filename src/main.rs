use std::{io, thread, time::Duration};
use std::{
    error::Error,
    process::Command,
};
use rand::Rng;
use crate::asciimon::Asciimon;
use crate::state::State;
use std::ops::{Range, RangeInclusive};

mod asciimon;
mod state;

const TITLE: &str = r"_______ _______ _______ _____ _____ _______  _____  __   _
|_____| |______ |         |     |   |  |  | |     | | \  |
|     | ______| |_____  __|__ __|__ |  |  | |_____| |  \_|
";

fn print_title() {
    // print!("\x1B[2J");
    println!("{}", TITLE);
}

fn print_state(state: &State) {
    // print!("\x1B[2J"); // works to clear screen only in powershell. not in exe itself.
    Command::new("cmd")
        .args(["/c", "cls"])
        .spawn()
        .expect("cls command failed to start")
        .wait()
        .expect("failed to wait");
    println!("\n\n");
    Asciimon::print_battle(&state.contender, &state.opponent);

    let spaces_num = 35 - (state.contender.name.len());
    let spacing = " ".repeat(spaces_num);
    println!("{}{}{}", state.contender.name, spacing, state.opponent.name);

    let health1 = state.contender.health_pretty();
    let spaces_num = 35 -health1.len();
    let spacing = " ".repeat(spaces_num);
    println!("{}{}{}", health1, spacing, state.opponent.health_pretty());
}

fn main() {
    print_title();
    thread::sleep(Duration::from_secs(2));
    // use a second thread here to setup the game while showing title.

    // todo get from list of availables
    let mut asciis: Vec<Asciimon> = Asciimon::get_list();

    // todo let user pick thier ascii
    // 70 wide for spacing? 35 each side
    println!("Choose your AsciiMon!");
    let l = asciis.len() as u8;

    for i in (0u8..l).step_by(2) {
        let asci1: &Asciimon = asciis.get(i as usize).unwrap();
        let asci2: &Asciimon = asciis.get(i as usize + 1).unwrap();

        let spaces_num = 35 - (3 +  asci1.name.len());
        let spacing = " ".repeat(spaces_num);
        println!("{}: {} {} {}: {}", i + 1, asci1.name, spacing, i + 2, asci2.name);
        Asciimon::print_battle(asci1, asci2);
        println!("\n");
    }

    let rang = 1u8..=l;
    let choice = get_input(format!("Choose! 1-{l}"), rang) as usize;
    println!("Your choice was: {choice}");

    let mut contender = asciis.remove(choice-1);
    // Set player health to the maxiumum
    contender.max_health = 18;
    contender.health = 18;
    let mut state = setup_game(contender, asciis);

    // Play the game
    loop {
        print_state(&state);

        // todo put attacks in own method
        // todo add initiative order per battle
        let attacks = &state.contender.attacks;
        for i in (0..attacks.len()).step_by(2) {
            let spacing = 35 - attacks.get(i).unwrap().name.len();
            let spaces = " ".repeat(spacing - 5);
            println!(" {}) {}{}{}){}", i + 1, attacks.get(i).unwrap().name, spaces, i+2, attacks.get(i+1).unwrap().name);
        }

        let atk_ind = get_input(format!("\nChoose which attack {} will use...\n", &state.contender.name), 1u8..=attacks.len() as u8 + 1) as usize;
        let attack = attacks.get(atk_ind - 1).unwrap();
        let dmg = rand::thread_rng().gen_range(attack.damage_range.clone());
        println!("Your {} did {} damage", &attack.name, dmg);
        state.opponent.take_damage(dmg);

        if state.opponent.is_dead {
            println!("\nYou win!");
            break;
        }

        let y = rand::thread_rng().gen_range(1..=6);
        println!("They attack back for {y} damage!");
        state.contender.take_damage(y);
        thread::sleep(Duration::from_secs(2));

        if state.contender.is_dead {
            println!("\nYou Lose!\nYou get nothing!");
            break;
        }
    }

    // Break line to pause on the final screen
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
}

fn get_input(text: String, range: RangeInclusive<u8>) -> u8 {
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

fn setup_game(contender: Asciimon, opponents: Vec<Asciimon>) -> State {
    State::new(
        contender,
        opponents
    )
}
