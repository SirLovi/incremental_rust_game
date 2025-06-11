use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Window;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct Game {
    wood: u32,
    stone: u32,
    food: u32,
    farms: u32,
    axe: bool,
    pickaxe: bool,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            wood: 0,
            stone: 0,
            food: 0,
            farms: 0,
            axe: false,
            pickaxe: false,
        }
    }

    pub fn collect_wood(&mut self) {
        self.wood += 1;
    }

    pub fn collect_stone(&mut self) {
        self.stone += 1;
    }

    pub fn collect_food(&mut self) {
        self.food += 1;
    }

    pub fn build_farm(&mut self) -> bool {
        if self.wood >= 10 && self.stone >= 10 {
            self.wood -= 10;
            self.stone -= 10;
            self.farms += 1;
            true
        } else {
            false
        }
    }

    pub fn craft_axe(&mut self) -> bool {
        if self.axe {
            return false;
        }
        if self.wood >= 10 && self.stone >= 5 {
            self.wood -= 10;
            self.stone -= 5;
            self.axe = true;
            true
        } else {
            false
        }
    }

    pub fn craft_pickaxe(&mut self) -> bool {
        if self.pickaxe {
            return false;
        }
        if self.wood >= 5 && self.stone >= 10 {
            self.wood -= 5;
            self.stone -= 10;
            self.pickaxe = true;
            true
        } else {
            false
        }
    }

    pub fn get_wood(&self) -> u32 {
        self.wood
    }

    pub fn get_stone(&self) -> u32 {
        self.stone
    }

    pub fn has_axe(&self) -> bool {
        self.axe
    }

    pub fn has_pickaxe(&self) -> bool {
        self.pickaxe
    }

    pub fn get_food(&self) -> u32 {
        self.food
    }

    pub fn get_farms(&self) -> u32 {
        self.farms
    }

    pub fn passive_food_generation(&mut self) {
        if self.farms > 0 {
            self.food += self.farms;
        }
    }
}

thread_local! {
    static GAME: RefCell<Game> = RefCell::new(Game::new());
}

pub fn reset_game() {
    GAME.with(|game| {
        *game.borrow_mut() = Game::new();
    });
}

#[wasm_bindgen]
pub fn collect_wood_global() -> u32 {
    GAME.with(|game| {
        let mut game = game.borrow_mut();
        game.collect_wood();
        game.get_wood()
    })
}

#[wasm_bindgen]
pub fn collect_stone_global() -> u32 {
    GAME.with(|game| {
        let mut game = game.borrow_mut();
        game.collect_stone();
        game.get_stone()
    })
}

#[wasm_bindgen]
pub fn craft_axe_global() -> bool {
    GAME.with(|game| {
        let mut game = game.borrow_mut();
        game.craft_axe()
    })
}

#[wasm_bindgen]
pub fn craft_pickaxe_global() -> bool {
    GAME.with(|game| {
        let mut game = game.borrow_mut();
        game.craft_pickaxe()
    })
}

#[wasm_bindgen]
pub fn has_axe_global() -> bool {
    GAME.with(|game| game.borrow().has_axe())
}

#[wasm_bindgen]
pub fn has_pickaxe_global() -> bool {
    GAME.with(|game| game.borrow().has_pickaxe())
}

#[wasm_bindgen]
pub fn get_wood_global() -> u32 {
    GAME.with(|game| game.borrow().get_wood())
}

#[wasm_bindgen]
pub fn get_stone_global() -> u32 {
    GAME.with(|game| game.borrow().get_stone())
}

#[wasm_bindgen]
pub fn collect_food_global() -> u32 {
    GAME.with(|game| {
        let mut game = game.borrow_mut();
        game.collect_food();
        game.get_food()
    })
}

#[wasm_bindgen]
pub fn build_farm_global() -> bool {
    GAME.with(|game| {
        let mut game = game.borrow_mut();
        game.build_farm()
    })
}

#[wasm_bindgen]
pub fn get_food_global() -> u32 {
    GAME.with(|game| game.borrow().get_food())
}

#[wasm_bindgen]
pub fn get_farms_global() -> u32 {
    GAME.with(|game| game.borrow().get_farms())
}

#[wasm_bindgen]
pub fn passive_food_generation() {
    GAME.with(|game| {
        let mut game = game.borrow_mut();
        game.passive_food_generation();
    });
}

#[wasm_bindgen]
pub fn passive_wood_collection() {
    GAME.with(|game| {
        let mut game = game.borrow_mut();
        if game.axe {
            game.wood += 1;
        }
    });
}

#[wasm_bindgen]
pub fn passive_stone_collection() {
    GAME.with(|game| {
        let mut game = game.borrow_mut();
        if game.pickaxe {
            game.stone += 1;
        }
    });
}

#[wasm_bindgen(start)]
pub fn run() {
    log("Game initialized!");

    let window: Window = web_sys::window().expect("no global `window` exists");
    let closure = Closure::wrap(Box::new(move || {
        passive_wood_collection();
        passive_stone_collection();
        passive_food_generation();
    }) as Box<dyn Fn()>);

    window
        .set_interval_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            1000, // Update every second
        )
        .expect("should register `setInterval`");

    closure.forget(); // Prevent closure from being garbage collected
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn craft_axe_only_once() {
        let mut game = Game::new();
        game.wood = 20;
        game.stone = 20;
        assert!(game.craft_axe());
        assert!(!game.craft_axe());
        assert_eq!(game.get_wood(), 10);
        assert_eq!(game.get_stone(), 15);
    }

    #[test]
    fn craft_pickaxe_only_once() {
        let mut game = Game::new();
        game.wood = 20;
        game.stone = 20;
        assert!(game.craft_pickaxe());
        assert!(!game.craft_pickaxe());
        assert_eq!(game.get_wood(), 15);
        assert_eq!(game.get_stone(), 10);
    }

    #[test]
    fn build_farm_and_generate_food() {
        let mut game = Game::new();
        game.wood = 20;
        game.stone = 20;
        assert!(game.build_farm());
        assert_eq!(game.get_farms(), 1);
        assert_eq!(game.get_wood(), 10);
        assert_eq!(game.get_stone(), 10);
        game.passive_food_generation();
        assert_eq!(game.get_food(), 1);
    }
}
