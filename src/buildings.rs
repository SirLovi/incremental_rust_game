use crate::resources::{res, Resources};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Types of buildings available in the game
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum BuildingType {
    /// Produces food
    Farm,
    /// Produces wood passively
    LumberMill,
    /// Produces stone passively
    Quarry,
    /// Produces iron passively
    Mine,
    /// Converts food into gold
    Bakery,
}

/// Static data for a building
pub struct BuildingInfo {
    pub base_cost: Resources,
    pub growth: f64,
    pub yield_per_tick: Resources,
}

fn info(ty: BuildingType) -> BuildingInfo {
    match ty {
        BuildingType::Farm => BuildingInfo {
            base_cost: res(10.0, 10.0, 0.0, 0.0, 0.0),
            growth: 1.15,
            yield_per_tick: res(0.0, 0.0, 1.0, 0.0, 0.0),
        },
        BuildingType::LumberMill => BuildingInfo {
            base_cost: res(15.0, 5.0, 0.0, 0.0, 0.0),
            growth: 1.15,
            yield_per_tick: res(1.0, 0.0, 0.0, 0.0, 0.0),
        },
        BuildingType::Quarry => BuildingInfo {
            base_cost: res(5.0, 15.0, 0.0, 0.0, 0.0),
            growth: 1.15,
            yield_per_tick: res(0.0, 1.0, 0.0, 0.0, 0.0),
        },
        BuildingType::Mine => BuildingInfo {
            base_cost: res(20.0, 20.0, 0.0, 0.0, 0.0),
            growth: 1.2,
            yield_per_tick: res(0.0, 0.0, 0.0, 1.0, 0.0),
        },
        BuildingType::Bakery => BuildingInfo {
            base_cost: res(50.0, 25.0, 100.0, 10.0, 0.0),
            growth: 1.2,
            yield_per_tick: res(0.0, 0.0, -1.0, 0.0, 0.2),
        },
    }
}

fn cost_for(ty: BuildingType, level: u32) -> Resources {
    let info = info(ty);
    let factor = info.growth.powi(level as i32);
    info.base_cost.scale(factor)
}

/// Player owned buildings
#[derive(Default, Serialize, Deserialize)]
pub struct Buildings {
    levels: HashMap<BuildingType, u32>,
}

impl Buildings {
    /// Current level of building
    pub fn level(&self, ty: BuildingType) -> u32 {
        *self.levels.get(&ty).unwrap_or(&0)
    }

    /// Cost to build next level
    pub fn cost(&self, ty: BuildingType) -> Resources {
        cost_for(ty, self.level(ty))
    }

    /// Increase level
    pub fn increment(&mut self, ty: BuildingType) {
        *self.levels.entry(ty).or_insert(0) += 1;
    }

    /// Total yield per tick of all buildings
    pub fn total_yield(&self) -> Resources {
        let mut r = Resources::default();
        for (&ty, &level) in &self.levels {
            let info = info(ty);
            r.add(&info.yield_per_tick.scale(level as f64));
        }
        r
    }

    /// Attempt to build if resources are sufficient
    pub fn build(&mut self, ty: BuildingType, res: &mut Resources) -> bool {
        let cost = self.cost(ty);
        if res.subtract(&cost) {
            self.increment(ty);
            true
        } else {
            false
        }
    }

    /// Public helper for tests
    pub fn cost_for_level(ty: BuildingType, level: u32) -> Resources {
        cost_for(ty, level)
    }
}
