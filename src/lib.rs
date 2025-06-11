//! Re-export modules and provide wasm bindings

mod achievements;
mod buildings;
mod events;
mod prestige;
mod research;
mod resources;
mod systems;
mod upgrades;

pub use achievements::*;
pub use buildings::*;
pub use events::*;
pub use prestige::*;
pub use research::*;
pub use resources::*;
pub use systems::*;
pub use upgrades::*;

use std::cell::RefCell;
use wasm_bindgen::prelude::*;

thread_local! {
    static GAME: RefCell<GameState> = RefCell::new(GameState::new());
}

/// JS facing game API
#[wasm_bindgen]
pub struct Game;

#[wasm_bindgen]
impl Game {
    /// Create a new game instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> Game {
        Game
    }

    /// Advance the game state based on the current timestamp (seconds)
    pub fn tick(now: f64) {
        GAME.with(|g| g.borrow_mut().tick(now));
    }

    /// Attempt to build a building by name
    pub fn build(name: &str) -> bool {
        GAME.with(|g| g.borrow_mut().build(name.into()))
    }

    /// Get a resource amount by name
    pub fn get_resource(name: &str) -> f64 {
        GAME.with(|g| g.borrow().get_resource(name.into()))
    }

    /// Get the cost of constructing the next level of a building as a JSON string
    pub fn building_cost(name: &str) -> String {
        GAME.with(|g| {
            let cost = g.borrow().build_cost(name.into());
            serde_json::to_string(&cost).expect("serialize cost")
        })
    }

    /// Save game to a base64 string
    pub fn save() -> String {
        GAME.with(|g| g.borrow().save_string())
    }

    /// Load game from a base64 string
    pub fn load(data: &str) {
        if let Some(state) = GameState::load_string(data) {
            GAME.with(|g| *g.borrow_mut() = state);
        }
    }

    /// Change tick rate in seconds
    pub fn set_tick_rate(rate: f64) {
        GAME.with(|g| g.borrow_mut().tick_rate = rate.max(0.2).min(10.0));
    }

    /// Retrieve the next log message from the game if available
    pub fn pop_log() -> Option<String> {
        GAME.with(|g| g.borrow_mut().pop_log())
    }

    /// Get the list of unlocked achievements as a JSON string
    pub fn achievements() -> String {
        GAME.with(|g| {
            let list = g.borrow().achievements_list();
            serde_json::to_string(&list).expect("serialize achievements")
        })
    }

    /// Attempt to research a technology using science
    pub fn research(name: &str) -> bool {
        let tech = match name {
            "mining" => Tech::Mining,
            "baking" => Tech::Baking,
            "electricity" => Tech::Electricity,
            "education" => Tech::Education,
            "alchemy" => Tech::Alchemy,
            _ => return false,
        };
        GAME.with(|g| {
            let mut state = g.borrow_mut();
            let res_ptr = &mut state.resources as *mut _;
            // SAFETY: no other mutable borrows exist
            unsafe { state.research.try_unlock(tech, &mut *res_ptr) }
        })
    }

    /// Perform a prestige reset
    pub fn prestige() {
        GAME.with(|g| g.borrow_mut().prestige());
    }

    /// Current prestige points
    pub fn prestige_points() -> u32 {
        GAME.with(|g| g.borrow().prestige.points)
    }
}
