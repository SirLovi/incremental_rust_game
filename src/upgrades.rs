use crate::resources::{res, Resources};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Upgrades obtainable in game
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum UpgradeType {
    /// Increases all yields
    Efficiency,
    /// Increases resource storage cap
    Storage,
    /// Boost mana production
    AlchemyBoost,
}

struct UpgradeInfo {
    base_cost: Resources,
    growth: f64,
    multiplier: f64,
}

fn info(ty: UpgradeType) -> UpgradeInfo {
    match ty {
        UpgradeType::Efficiency => UpgradeInfo {
            base_cost: res(50.0, 50.0, 0.0, 0.0, 0.0),
            growth: 1.5,
            multiplier: 1.1,
        },
        UpgradeType::Storage => UpgradeInfo {
            base_cost: res(100.0, 100.0, 0.0, 0.0, 0.0),
            growth: 1.7,
            multiplier: 1.2,
        },
        UpgradeType::AlchemyBoost => UpgradeInfo {
            base_cost: res(0.0, 0.0, 0.0, 0.0, 200.0),
            growth: 2.0,
            multiplier: 1.5,
        },
    }
}

/// Owned upgrades
#[derive(Default, Serialize, Deserialize)]
pub struct Upgrades {
    levels: HashMap<UpgradeType, u32>,
}

impl Upgrades {
    /// Level of upgrade
    pub fn level(&self, ty: UpgradeType) -> u32 {
        *self.levels.get(&ty).unwrap_or(&0)
    }

    /// Current multiplier for upgrade
    pub fn multiplier(&self, ty: UpgradeType) -> f64 {
        info(ty).multiplier.powi(self.level(ty) as i32)
    }

    /// Cost to purchase next level
    pub fn cost(&self, ty: UpgradeType) -> Resources {
        let i = info(ty);
        let factor = i.growth.powi(self.level(ty) as i32);
        i.base_cost.scale(factor)
    }

    /// Attempt to purchase
    pub fn purchase(&mut self, ty: UpgradeType, res: &mut Resources) -> bool {
        let cost = self.cost(ty);
        if res.subtract(&cost) {
            *self.levels.entry(ty).or_insert(0) += 1;
            true
        } else {
            false
        }
    }
}
