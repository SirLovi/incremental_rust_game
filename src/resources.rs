use serde::{Deserialize, Serialize};

/// Collection of all game resources
#[derive(Default, Clone, Copy, Serialize, Deserialize, Debug)]
pub struct Resources {
    /// Amount of wood
    pub wood: f64,
    /// Amount of stone
    pub stone: f64,
    /// Amount of food
    pub food: f64,
    /// Amount of iron
    pub iron: f64,
    /// Amount of gold
    pub gold: f64,
}

impl Resources {
    /// Add other resources to self
    pub fn add(&mut self, other: &Resources) {
        self.wood += other.wood;
        self.stone += other.stone;
        self.food += other.food;
        self.iron += other.iron;
        self.gold += other.gold;
    }

    /// Subtract other resources if affordable
    pub fn subtract(&mut self, cost: &Resources) -> bool {
        if self.can_afford(cost) {
            self.wood -= cost.wood;
            self.stone -= cost.stone;
            self.food -= cost.food;
            self.iron -= cost.iron;
            self.gold -= cost.gold;
            true
        } else {
            false
        }
    }

    /// Check if self has at least cost of each resource
    pub fn can_afford(&self, cost: &Resources) -> bool {
        self.wood >= cost.wood
            && self.stone >= cost.stone
            && self.food >= cost.food
            && self.iron >= cost.iron
            && self.gold >= cost.gold
    }

    /// Scale resources by factor
    pub fn scale(&self, factor: f64) -> Resources {
        Resources {
            wood: self.wood * factor,
            stone: self.stone * factor,
            food: self.food * factor,
            iron: self.iron * factor,
            gold: self.gold * factor,
        }
    }
}

/// Helper to create resource bundle
pub fn res(wood: f64, stone: f64, food: f64, iron: f64, gold: f64) -> Resources {
    Resources {
        wood,
        stone,
        food,
        iron,
        gold,
    }
}
