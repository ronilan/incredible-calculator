use incredible::*;
use incredible_elements::Rectangle;
use incredible_helpers_layout::*;

use crate::state::{CalcMode, State};
use crate::themes::*;

use super::calculator;

/// Builds the whole basic screen: a full-window rectangle with the
/// calculator centered inside it.
pub fn build() -> Rectangle<State> {
    let screen = Rectangle::new();
    screen
        .width(Platform::columns())
        .height(Platform::rows())
        .handle("basic_screen")
        .showed(false);

    screen.add(calculator::build());

    apple_theme();

    screen
        .on_window(|el, _state, event| {
            if event.window == Window::Resize {
                el.width(event.columns).height(event.rows);
                el.elements_to_center();
                el.draw();
            }
        })
        .on_state(|el, state, event| {
            if event.loop_count == 0 || event.prev_state.calc_mode != state.calc_mode {
                el.showed(matches!(state.calc_mode, CalcMode::Basic));
                el.elements_to_center();
                el.draw();
            }
        });

    screen
}
