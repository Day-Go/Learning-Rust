pub enum EnemyType {
    Goblin,
    Orc,
    Troll,
    Wolf,
    Bear,
    Dragon,
}

pub struct Enemy {
    pub enemy_type: EnemyType,
    pub health: u32,
    pub attack: u32,
    pub attack_speed: u32,
    pub physical_resistance: u32,
    pub magical_resistance: u32
}

impl Enemy {
    pub fn new(
        enemy_type: EnemyType,
        health: u32,
        attack: u32, 
        attack_speed: u32, 
        physical_resistance: u32, 
        magical_resistance: u32,
    ) -> Self {
        Enemy {
            enemy_type, 
            health, 
            attack, 
            attack_speed, 
            physical_resistance, 
            magical_resistance
        }
    }
    
    pub fn attack(&self) -> u32 {
        self.attack
    }

    pub fn die(&self) {
        println!("The {} has died!", self.enemy_type);
    }
}