//! Re-export modules and provide wasm bindings

mod resources;
mod buildings;
mod upgrades;
mod research;
mod systems;

pub use resources::*;
pub use buildings::*;
pub use upgrades::*;
pub use research::*;
pub use systems::*;

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
}
