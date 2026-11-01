use incredible::*;
use incredible_elements::{Rectangle, Text};

use crate::state::State;

static X: isize = 0;
static Y: isize = 1;

static FRAME_WIDTH: usize = 28;
static FRAME_HEIGHT: usize = 3;

pub(crate) fn build<'a>() -> Rectangle<State> {
    let display = Rectangle::new();
    display
        .x(X)
        .y(Y)
        .width(FRAME_WIDTH)
        .height(FRAME_HEIGHT)
        .style_handle("Display");

    let digit_display = Text::default();
    digit_display
        .x(1)
        .y(1)
        .text(format!(" {:>24} ", "0").as_str())
        .capture(Capture::all())
        .style_handle("Display_digit");

    digit_display.on_state(|el, state: &State, _ev| {
        let shown = if state.display.is_empty() {
            "0"
        } else {
            &state.display
        };
        el.text(format!(" {:>24} ", shown).as_str());
    });
    display.add(digit_display);

    display
}
