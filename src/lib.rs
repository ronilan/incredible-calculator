pub mod logic;
pub mod logic_scientific;
pub mod settings;
pub mod state;
pub mod themes;
pub mod ui;

#[cfg(target_arch = "wasm32")]
mod platform;
#[cfg(target_arch = "wasm32")]
mod runtime;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn main() {
    runtime::run();
}
