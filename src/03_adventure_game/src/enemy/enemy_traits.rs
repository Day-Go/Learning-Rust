#[derive(Clone, Copy)]
pub enum EnemyType {
    Goblin,
    Orc,
    Troll,
    Wolf,
    Bear,
    Dragon,
}

pub struct Stats {
    pub health: u32,
    pub attack: u32,
    pub attack_speed: u32,
    pub physical_resistance: u32,
    pub magical_resistance: u32,
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
        base_stats: Stats,
    ) -> Self {
        let mut enemy = Enemy {
            enemy_type, 
            health: base_stats.health,
            attack: base_stats.attack,
            attack_speed: base_stats.attack_speed,
            physical_resistance: base_stats.physical_resistance,
            magical_resistance: base_stats.magical_resistance,
        };
        
        enemy.apply_stat_multipliers();

        enemy
    }
    
    fn apply_stat_multipliers(&mut self) {
        match self.enemy_type {
            EnemyType::Goblin => {
                self.health *= 1;
                self.attack *= 2;
                self.attack_speed *= 2;
                self.physical_resistance *= 1;
                self.magical_resistance *= 1;
            },
            EnemyType::Orc => {
                self.health *= 2;
                self.attack *= 2;
                self.attack_speed *= 1;
                self.physical_resistance *= 2;
                self.magical_resistance *= 1;
            },
            EnemyType::Troll => {
                self.health *= 3;
                self.attack *= 1;
                self.attack_speed *= 1;
                self.physical_resistance *= 1;
                self.magical_resistance *= 2;
            },
            EnemyType::Wolf => {
                self.health *= 1;
                self.attack *= 1;
                self.attack_speed *= 3;
                self.physical_resistance *= 1;
                self.magical_resistance *= 1;
            },
            EnemyType::Bear => {
                self.health *= 4;
                self.attack *= 3;
                self.attack_speed *= 1;
                self.physical_resistance *= 2;
                self.magical_resistance *= 1;
            },
            EnemyType::Dragon => {
                self.health *= 5;
                self.attack *= 4;
                self.attack_speed *= 2;
                self.physical_resistance *= 3;
                self.magical_resistance *= 3;
            },
        }
    }

    pub fn attack(&self) -> u32 {
        self.attack
    }

    pub fn die(&self) {
        println!("The {} has died!", self.enemy_type);
    }
}

impl std::fmt::Display for EnemyType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            EnemyType::Goblin => write!(f, "Goblin"),
            EnemyType::Orc => write!(f, "Orc"),
            EnemyType::Troll => write!(f, "Troll"),
            EnemyType::Wolf => write!(f, "Wolf"),
            EnemyType::Bear => write!(f, "Bear"),
            EnemyType::Dragon => write!(f, "Dragon"),
        }
    }
}