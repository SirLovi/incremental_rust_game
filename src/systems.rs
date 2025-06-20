use crate::achievements::Achievements;
use crate::buildings::{BuildingType, Buildings};
use crate::events::check_random_events;
use crate::research::{Research, Tech};
#[allow(unused_imports)]
use crate::resources::{res, Resources};
use crate::upgrades::{UpgradeType, Upgrades};
use base64::Engine;
use rand::thread_rng;
use serde::{Deserialize, Serialize};

/// Current save format version
pub const SAVE_VERSION: u32 = 1;

/// Game state containing all persistent data
#[derive(Serialize, Deserialize)]
pub struct GameState {
    /// Version of save
    pub version: u32,
    /// Player resources
    pub resources: Resources,
    /// Owned buildings
    pub buildings: Buildings,
    /// Purchased upgrades
    pub upgrades: Upgrades,
    /// Researched techs
    pub research: Research,
    /// Achievements unlocked
    pub achievements: Achievements,
    /// Prestige information
    pub prestige: crate::prestige::Prestige,
    /// Pending event log messages
    #[serde(skip)]
    pub event_log: Vec<String>,
    /// Chance of a random event each tick
    #[serde(skip)]
    pub event_chance: f64,
    /// Tick rate in seconds
    pub tick_rate: f64,
    /// Last update timestamp in seconds
    pub last_update: Option<f64>,
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

impl GameState {
    /// Create a new game state
    pub fn new() -> Self {
        GameState {
            version: SAVE_VERSION,
            // Provide some starting supplies so the player can build initial tools
            resources: res(20.0, 20.0, 0.0, 0.0, 0.0),
            buildings: Buildings::default(),
            upgrades: Upgrades::default(),
            research: Research::default(),
            achievements: Achievements::default(),
            prestige: crate::prestige::Prestige::default(),
            event_log: Vec::new(),
            event_chance: crate::events::FARM_LOSS_CHANCE,
            tick_rate: 1.0,
            last_update: None,
        }
    }

    /// Compute building yield with upgrades
    fn tick_yield(&self) -> Resources {
        let mut r = self.buildings.total_yield();
        let m = self.upgrades.multiplier(UpgradeType::Efficiency);
        r = r.scale(m);
        r = r.scale(self.prestige.bonus_multiplier());
        if r.mana > 0.0 {
            r.mana *= self.upgrades.multiplier(UpgradeType::AlchemyBoost);
        }
        r
    }

    /// Net change of each resource per second
    pub fn resource_rate(&self) -> Resources {
        self.tick_yield().scale(1.0 / self.tick_rate)
    }

    /// Perform a prestige reset gaining permanent bonuses
    pub fn prestige(&mut self) {
        let gained = ((self.resources.gold / 1e6).sqrt().floor()) as u32;
        if gained > 0 {
            self.prestige.points += gained;
        }
        let base = GameState::new();
        self.resources = base.resources;
        self.buildings = base.buildings;
        self.upgrades = base.upgrades;
        self.research = base.research;
        self.achievements = base.achievements;
        self.last_update = base.last_update;
    }

    /// Advance the game by delta seconds
    pub fn tick(&mut self, now: f64) {
        if let Some(prev) = self.last_update {
            let elapsed = (now - prev).min(8.0 * 3600.0);
            let ticks = (elapsed / self.tick_rate).floor() as u64;
            for _ in 0..ticks {
                let y = self.tick_yield();
                self.resources.add(&y);
                self.resources.clamp_non_negative();
                if let Some(msg) = check_random_events(
                    &mut self.buildings,
                    &mut self.resources,
                    &mut thread_rng(),
                    self.event_chance,
                ) {
                    self.event_log.push(msg);
                }
                let new_ach = self.achievements.check(&self.buildings, &self.research);
                self.event_log.extend(new_ach);
            }
            self.last_update = Some(prev + ticks as f64 * self.tick_rate);
            if self.last_update.unwrap() < now {
                self.last_update = Some(now);
            }
        } else {
            self.last_update = Some(now);
        }
    }

    /// Build a building by name
    pub fn build(&mut self, name: String) -> bool {
        let ty = match name.as_str() {
            "farm" => BuildingType::Farm,
            "lumber_mill" => BuildingType::LumberMill,
            "quarry" => BuildingType::Quarry,
            "mine" => BuildingType::Mine,
            "bakery" => BuildingType::Bakery,
            "generator" => BuildingType::Generator,
            "lab" => BuildingType::Lab,
            "shrine" => BuildingType::Shrine,
            _ => return false,
        };
        // Check research requirements
        if matches!(ty, BuildingType::Mine) && !self.research.is_unlocked(Tech::Mining) {
            return false;
        }
        if matches!(ty, BuildingType::Bakery) && !self.research.is_unlocked(Tech::Baking) {
            return false;
        }
        if matches!(ty, BuildingType::Generator) && !self.research.is_unlocked(Tech::Electricity) {
            return false;
        }
        if matches!(ty, BuildingType::Lab) && !self.research.is_unlocked(Tech::Education) {
            return false;
        }
        if matches!(ty, BuildingType::Shrine) && !self.research.is_unlocked(Tech::Alchemy) {
            return false;
        }
        self.buildings.build(ty, &mut self.resources)
    }

    /// Get the current cost to build the next level of a building by name
    pub fn build_cost(&self, name: String) -> Resources {
        let ty = match name.as_str() {
            "farm" => BuildingType::Farm,
            "lumber_mill" => BuildingType::LumberMill,
            "quarry" => BuildingType::Quarry,
            "mine" => BuildingType::Mine,
            "bakery" => BuildingType::Bakery,
            "generator" => BuildingType::Generator,
            "lab" => BuildingType::Lab,
            "shrine" => BuildingType::Shrine,
            _ => return Resources::default(),
        };
        self.buildings.cost(ty)
    }

    /// Number of buildings of the given type
    pub fn building_count(&self, name: String) -> u32 {
        let ty = match name.as_str() {
            "farm" => BuildingType::Farm,
            "lumber_mill" => BuildingType::LumberMill,
            "quarry" => BuildingType::Quarry,
            "mine" => BuildingType::Mine,
            "bakery" => BuildingType::Bakery,
            "generator" => BuildingType::Generator,
            "lab" => BuildingType::Lab,
            "shrine" => BuildingType::Shrine,
            _ => return 0,
        };
        self.buildings.level(ty)
    }

    /// Get resource by name
    pub fn get_resource(&self, name: String) -> f64 {
        match name.as_str() {
            "wood" => self.resources.wood,
            "stone" => self.resources.stone,
            "food" => self.resources.food,
            "iron" => self.resources.iron,
            "gold" => self.resources.gold,
            "energy" => self.resources.energy,
            "science" => self.resources.science,
            "mana" => self.resources.mana,
            _ => 0.0,
        }
    }

    /// Net resource change per second by name
    pub fn get_resource_rate(&self, name: String) -> f64 {
        let rates = self.resource_rate();
        match name.as_str() {
            "wood" => rates.wood,
            "stone" => rates.stone,
            "food" => rates.food,
            "iron" => rates.iron,
            "gold" => rates.gold,
            "energy" => rates.energy,
            "science" => rates.science,
            "mana" => rates.mana,
            _ => 0.0,
        }
    }

    /// Save state to base64 string
    pub fn save_string(&self) -> String {
        base64::engine::general_purpose::STANDARD
            .encode(serde_json::to_vec(self).expect("serialize"))
    }

    /// Load state from base64 string
    pub fn load_string(data: &str) -> Option<Self> {
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(data)
            .ok()?;
        serde_json::from_slice(&bytes).ok()
    }

    /// Pop the next log message generated by events or achievements
    pub fn pop_log(&mut self) -> Option<String> {
        if self.event_log.is_empty() {
            None
        } else {
            Some(self.event_log.remove(0))
        }
    }

    /// List achievements as JSON serializable vector
    pub fn achievements_list(&self) -> Vec<String> {
        self.achievements.list()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn cost_curve() {
        let c0 = Buildings::cost_for_level(BuildingType::Farm, 0).wood;
        let c1 = Buildings::cost_for_level(BuildingType::Farm, 1).wood;
        assert!(c1 > c0);
        assert!((c1 - c0 * 1.15).abs() < 1e-6);
    }

    #[wasm_bindgen_test]
    fn offline_progress() {
        let mut g = GameState::new();
        g.event_chance = 0.0;
        g.resources = res(20.0, 20.0, 0.0, 0.0, 0.0);
        assert!(g.build("farm".into()));
        let start = g.resources.food;
        g.tick(10.0);
        g.tick(20.0);
        assert_eq!(g.resources.food, start + 10.0);
    }

    #[wasm_bindgen_test]
    fn bakery_food_depletion() {
        let mut g = GameState::new();
        g.event_chance = 0.0;
        g.resources = res(100.0, 100.0, 100.5, 100.0, 100.0);
        g.research.unlock(Tech::Baking);
        assert!(g.build("bakery".into()));
        g.resources.food = 0.5;
        g.tick(0.0);
        g.tick(1.0);
        assert_eq!(g.resources.food, 0.0);
        assert!((g.resources.gold - 100.2).abs() < 1e-6);
    }

    #[wasm_bindgen_test]
    fn starting_resources_nonzero() {
        let g = GameState::new();
        // Player should start with enough wood and stone for a first farm
        assert!(g.resources.wood >= 10.0);
        assert!(g.resources.stone >= 10.0);
    }
}
