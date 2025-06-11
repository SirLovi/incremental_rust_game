use crate::buildings::{BuildingType, Buildings};
#[allow(unused_imports)]
use crate::resources::{res, Resources};
use crate::upgrades::{UpgradeType, Upgrades};
use crate::research::{Research, Tech};
use serde::{Deserialize, Serialize};
use base64::Engine;

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
            resources: Resources::default(),
            buildings: Buildings::default(),
            upgrades: Upgrades::default(),
            research: Research::default(),
            tick_rate: 1.0,
            last_update: None,
        }
    }

    /// Compute building yield with upgrades
    fn tick_yield(&self) -> Resources {
        let mut r = self.buildings.total_yield();
        let m = self.upgrades.multiplier(UpgradeType::Efficiency);
        r = r.scale(m);
        r
    }

    /// Advance the game by delta seconds
    pub fn tick(&mut self, now: f64) {
        if let Some(prev) = self.last_update {
            let elapsed = now - prev;
            let ticks = (elapsed / self.tick_rate).floor() as u64;
            for _ in 0..ticks {
                let y = self.tick_yield();
                self.resources.add(&y);
            }
            self.last_update = Some(prev + ticks as f64 * self.tick_rate);
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
            _ => return false,
        };
        // Check research requirements
        if matches!(ty, BuildingType::Mine) && !self.research.is_unlocked(Tech::Mining) {
            return false;
        }
        if matches!(ty, BuildingType::Bakery) && !self.research.is_unlocked(Tech::Baking) {
            return false;
        }
        self.buildings.build(ty, &mut self.resources)
    }

    /// Get resource by name
    pub fn get_resource(&self, name: String) -> f64 {
        match name.as_str() {
            "wood" => self.resources.wood,
            "stone" => self.resources.stone,
            "food" => self.resources.food,
            "iron" => self.resources.iron,
            "gold" => self.resources.gold,
            _ => 0.0,
        }
    }

    /// Save state to base64 string
    pub fn save_string(&self) -> String {
        base64::engine::general_purpose::STANDARD.encode(
            serde_json::to_vec(self).expect("serialize"),
        )
    }

    /// Load state from base64 string
    pub fn load_string(data: &str) -> Option<Self> {
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(data)
            .ok()?;
        serde_json::from_slice(&bytes).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cost_curve() {
        let c0 = Buildings::cost_for_level(BuildingType::Farm, 0).wood;
        let c1 = Buildings::cost_for_level(BuildingType::Farm, 1).wood;
        assert!(c1 > c0);
        assert!((c1 - c0 * 1.15).abs() < 1e-6);
    }

    #[test]
    fn offline_progress() {
        let mut g = GameState::new();
        g.resources = res(20.0, 20.0, 0.0, 0.0, 0.0);
        assert!(g.build("farm".into()));
        let start = g.resources.food;
        g.tick(10.0);
        g.tick(20.0);
        assert_eq!(g.resources.food, start + 10.0);
    }
}
