use crate::asciimon::Asciimon;

#[derive(Debug)]
pub struct State {
    pub contender: Asciimon, 
    pub opponent: Asciimon  // todo make a list of contenders.
    // asciimons: Vec<Asciimon>
}

impl State{
    fn new(contender: Asciimon, opponent: Asciimon) -> Self{
        State{
            contender, opponent
        }
    }
}