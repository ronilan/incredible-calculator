use crate::platform;
use crate::settings;
use crate::state::State;
use crate::themes;
use crate::ui::app;

pub(crate) fn run() -> incredible::tui::DeferredValue<State> {
    platform::init();
    let app = app::build();

    let mut state = State::new();
    settings::apply_to_state(&settings::load(), &mut state);
    themes::apply(state.theme_index);

    // Native windows are resized to the calculator; terminals and the web
    // output ignore this.
    let (width, height) = app::window_size(state.calc_mode);
    app.native_window_size(width, height);

    app.run(state)
}
