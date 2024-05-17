use crate::asciimon::Asciimon;
use rand::Rng;

#[derive(Debug)]
pub struct State {
    pub contender: Asciimon,
    pub opponent: Asciimon,
    pub opponents: Vec<Asciimon>,
    // pub asciimons: Vec<Asciimon>
}

impl State{
    pub fn new(contender: Asciimon, mut opponents: Vec<Asciimon>) -> Self{
        // Choose random opponent
        let rand_opt = rand::thread_rng().gen_range(0..opponents.len());
        let opponent = opponents.remove(rand_opt);
        State{
            contender, opponent, opponents
        }
    }
}