use incredible::*;
use incredible_elements::App;

use crate::state::{CalcMode, State};
use crate::themes::apple_theme;
use crate::ui::screens;

/// The native window size that exactly fits the active calculator.
pub fn window_size(mode: CalcMode) -> (usize, usize) {
    match mode {
        CalcMode::Basic => (
            screens::basic::calculator::WIDTH,
            screens::basic::calculator::HEIGHT,
        ),
        CalcMode::Scientific => (
            screens::scientific::calculator::WIDTH,
            screens::scientific::calculator::HEIGHT,
        ),
    }
}

pub fn build() -> App<State> {
    let app = App::default();
    app.title("Calculator").auto_surface(false);

    let basic = screens::basic::basic::build();
    let sci = screens::scientific::scientific::build();

    app.add(basic);
    app.add(sci);

    app.on_state(|el, state: &State, event| {
        let previous = &event.prev_state;

        // Draw on theme change
        if state.theme_index != previous.theme_index {
            el.draw();
        }

        // Keep the native window exactly as large as the visible calculator.
        // Terminals and the web output ignore this.
        if state.calc_mode != previous.calc_mode {
            let (width, height) = window_size(state.calc_mode);
            el.native_window_size(width, height);
        }

        // Preferences are written only when one of them changes; calculations
        // change the state far more often than these three fields.
        if state.theme_index != previous.theme_index
            || state.calc_mode != previous.calc_mode
            || state.is_radian != previous.is_radian
        {
            crate::settings::persist_now(state);
        }
    });

    apple_theme();

    app
}
