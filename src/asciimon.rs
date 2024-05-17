use rand::Rng;

#[derive(Debug)]
pub struct Asciimon{
    pub name: String,
    pub sprite: String,
    pub health: i32,
    pub max_health: i32,
    pub is_dead: bool,
    pub sprite_ln1: String,
    pub sprite_ln2: String,
    pub sprite_ln3: String,
    pub sprite_ln4: String,
    pub sprite_ln5: String,
    // vec<Attacks> todo
}

impl Asciimon {
    pub(crate) fn print_battle(asci1: &Asciimon, asci2: &Asciimon) {
        // todo reverse for right side  `.chars().rev().collect::<String>())`
        let spaces_num = 35 - asci1.sprite_ln1.len();
        let spacing = " ".repeat(spaces_num);
        println!("{} {} {}", asci1.sprite_ln1, spacing, asci2.sprite_ln1);
        println!("{} {} {}", asci1.sprite_ln2, spacing, asci2.sprite_ln2);
        println!("{} {} {}", asci1.sprite_ln3, spacing, asci2.sprite_ln3);
        println!("{} {} {}", asci1.sprite_ln4, spacing, asci2.sprite_ln4);
        println!("{} {} {}", asci1.sprite_ln5, spacing, asci2.sprite_ln5);
    }
}

impl Asciimon{
    // randomize health to 3d6 for enemies. always start with 20 on player
    pub fn new(name: String, sprite: String) -> Self{
        let mut it = sprite.lines();
        let sprite_ln1 = it.next().unwrap().to_string();
        // println!("{}", sprite_ln1);
        let sprite_ln2 = it.next().unwrap().to_string();
        // println!("{}", sprite_ln2);
        let sprite_ln3 = it.next().unwrap().to_string();
        // println!("{}", sprite_ln3);
        let sprite_ln4 = it.next().unwrap().to_string();
        // println!("{}", sprite_ln4);
        let sprite_ln5 = it.next().unwrap().to_string();
        // println!("{}", sprite_ln5);

         // effectively 3d6 but not really. different distribution
        let health = rand::thread_rng().gen_range(3..=18); 

        Asciimon{
            name,
            sprite,
            // sprite: r"muh sprite".to_string(),
            health,
            max_health: health,
            is_dead: false,
            sprite_ln1,
            sprite_ln2,
            sprite_ln3,
            sprite_ln4,
            sprite_ln5,

        }
    }

    pub fn create_enemy() -> Self{
        // todo random from list
        Asciimon::new(
            "TurboFish".to_string(),
            r"::<>".to_string()
        )
    }

    // pub fn get_enemy(enemies: Vec<Asciimon>) -> Asciimon {
    //     enemies.get(0).unwrap_or(create_enemy())
    // }

    pub fn get_list() -> Vec<Asciimon> {
        let mut asciis = Vec::new();
        // todo read this in from a file instead. look into json parsing in rust
        let dolph = Asciimon::new(
            "Dolfy".to_string(),
            "        ,       \n      __)\\___   \n  _.-'      .`-.\n.'/~~```\"~z/~'\"`\n^^              ".to_string()
        );
        asciis.push(dolph);
        let x = Asciimon::new(
            "Bunbun".to_string(),
            "  \\\\_   \n  ( _\\  \n  / \\__ \n / _/`\"`\n{\\  )_  ".to_string()
        );
        asciis.push(x);
        let y = Asciimon::new(
            "Gingy".to_string(),
            "   ,-.   \n _(*_*)_ \n(_  o  _)\n  / o \\  \n (_/ \\_) ".to_string()
        );
        asciis.push(y);
        let z = Asciimon::new(
            "S-Man".to_string(),
            "     __    \n   _|==|_  \n    ('')__/\n>--(`^^')  \n  (`^'^'`) ".to_string()
        );
        asciis.push(z);
        asciis
    }

    pub fn health_pretty(&self) -> String {
        // ToDo. Show health as a portion of max health
        let mut to_return = "#".repeat(self.health as usize);
        let remaining = self.max_health - self.health;
        if remaining != 0{
            let a = "_".repeat(remaining as usize);
            to_return.push_str(&a);
        }
        to_return
    }

    pub fn take_damage(&mut self, amount: i32) {
        // println!("Asciimon took {} damage", amount);
        self.health -= amount; 
        if self.health <= 0 {
            self.health = 0;
            // todo set is_dead or trigger something
            self.is_dead = true;
        }
    }
}