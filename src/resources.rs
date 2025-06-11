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
    /// Amount of energy
    pub energy: f64,
    /// Amount of science
    pub science: f64,
    /// Amount of mana
    pub mana: f64,
}

impl Resources {
    /// Add other resources to self
    pub fn add(&mut self, other: &Resources) {
        self.wood += other.wood;
        self.stone += other.stone;
        self.food += other.food;
        self.iron += other.iron;
        self.gold += other.gold;
        self.energy += other.energy;
        self.science += other.science;
        self.mana += other.mana;
    }

    /// Ensure all resources are non-negative
    pub fn clamp_non_negative(&mut self) {
        if self.wood < 0.0 {
            self.wood = 0.0;
        }
        if self.stone < 0.0 {
            self.stone = 0.0;
        }
        if self.food < 0.0 {
            self.food = 0.0;
        }
        if self.iron < 0.0 {
            self.iron = 0.0;
        }
        if self.gold < 0.0 {
            self.gold = 0.0;
        }
        if self.energy < 0.0 {
            self.energy = 0.0;
        }
        if self.science < 0.0 {
            self.science = 0.0;
        }
        if self.mana < 0.0 {
            self.mana = 0.0;
        }
    }

    /// Subtract other resources if affordable
    pub fn subtract(&mut self, cost: &Resources) -> bool {
        if self.can_afford(cost) {
            self.wood -= cost.wood;
            self.stone -= cost.stone;
            self.food -= cost.food;
            self.iron -= cost.iron;
            self.gold -= cost.gold;
            self.energy -= cost.energy;
            self.science -= cost.science;
            self.mana -= cost.mana;
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
            && self.energy >= cost.energy
            && self.science >= cost.science
            && self.mana >= cost.mana
    }

    /// Scale resources by factor
    pub fn scale(&self, factor: f64) -> Resources {
        Resources {
            wood: self.wood * factor,
            stone: self.stone * factor,
            food: self.food * factor,
            iron: self.iron * factor,
            gold: self.gold * factor,
            energy: self.energy * factor,
            science: self.science * factor,
            mana: self.mana * factor,
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
        energy: 0.0,
        science: 0.0,
        mana: 0.0,
    }
}

/// Create a resource bundle with all fields specified
pub fn res_ext(
    wood: f64,
    stone: f64,
    food: f64,
    iron: f64,
    gold: f64,
    energy: f64,
    science: f64,
    mana: f64,
) -> Resources {
    Resources {
        wood,
        stone,
        food,
        iron,
        gold,
        energy,
        science,
        mana,
    }
}
