pub mod logic;
pub mod logic_scientific;
pub mod platform;
pub mod runtime;
pub mod settings;
pub mod state;
pub mod themes;
pub mod ui;

pub fn run() {
    runtime::run();
}

pub fn main() {
    run();
}
