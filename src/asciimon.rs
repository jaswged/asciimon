use rand::Rng;

#[derive(Debug)]
pub struct Asciimon{
    pub name: String,
    pub sprite: String,
    pub health: i32,
    pub is_dead: bool,
    sprite_ln1: String,
    sprite_ln2: String,
    sprite_ln3: String,
    sprite_ln4: String,
    sprite_ln5: String,
    // vec<Attacks> todo
}

impl Asciimon{
    // randomize health to 3d6 for enemies. always start with 20 on player
    pub fn new(name: String, sprite: String) -> Self{
        let mut it = sprite.lines();
        let sprite_ln1 = it.next().unwrap().to_string();
        let sprite_ln2 = it.next().unwrap().to_string();
        let sprite_ln3 = it.next().unwrap().to_string();
        let sprite_ln4 = it.next().unwrap().to_string();
        let sprite_ln5 = it.next().unwrap().to_string();

         // effectively 3d6 but not really. different distribution
        let health = rand::thread_rng().gen_range(3..=18); 

        Asciimon{
            name,
            sprite,
            // sprite: r"muh sprite".to_string(),
            health,
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

    pub fn get_enemy(enemies: Vec<Asciimon>) -> Asciimon {
        enemies.get(0).unwrap_or(create_enemy())
    }

    pub fn get_list() -> Vec<Asciimon> {
        let mut asciis = Vec::new();
        // todo read this in from a file instead. look into json parsing in rust
        
        let x = Asciimon{
            name: "TurboFish".to_string(),
            sprite: r"::<>".to_string(),
            health,
            is_dead: false
        };
        asciis.push(x);

        asciis.push(y);

        asciis.push(z);
    }

    pub fn health_pretty(&self) -> String {
        "#".repeat(self.health as usize)
    }

    pub fn take_damage(&mut self, amount: i32) {
        println!("Pokemon took {} damage", amount);
        self.health -= amount; 
        if self.health < 0 {
            self.health = 0;
            // todo set is_dead or trigger something
        }
    }
}