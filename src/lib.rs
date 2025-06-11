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
}

thread_local! {
    static GAME: RefCell<Game> = RefCell::new(Game::new());
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
}
