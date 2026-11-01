use incredible::*;
use incredible_elements::Rectangle;
use incredible_helpers_layout::*;

use crate::state::{CalcMode, State};
use crate::themes::*;

use super::calculator;

/// Builds the whole scientific screen: a full-window rectangle with the
/// calculator centered inside it.
pub fn build() -> Rectangle<State> {
    let screen = Rectangle::new();
    screen
        .width(Platform::columns())
        .height(Platform::rows())
        .handle("scientific_screen")
        .showed(false);

    screen.add(calculator::build());

    apple_theme();

    screen
        .on_window(|el, state, event| match &event.window {
            Window::Resize => {
                el.width(event.columns).height(event.rows);
                el.elements_to_center();
                el.draw();
            }
            Window::Paste(text) => {
                // The screen stays in the tree while Basic is showing, so the
                // mode check is what keeps pasting Scientific-only.
                if matches!(state.calc_mode, CalcMode::Scientific) {
                    crate::logic_scientific::paste_expression(state, text);
                }
            }
            _ => {}
        })
        .on_state(|el, state, event| {
            if event.loop_count == 0 || event.prev_state.calc_mode != state.calc_mode {
                el.showed(matches!(state.calc_mode, CalcMode::Scientific));
                el.elements_to_center();
                el.draw();
            }
        });

    screen
}
