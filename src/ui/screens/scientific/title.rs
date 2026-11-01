use incredible::*;
use incredible_elements::{Rectangle, Text};

use crate::state::{CalcMode, State};
use crate::themes;

static X: isize = 0;
static Y: isize = 0;

static TITLE_WIDTH: usize = 72;
static TITLE_HEIGHT: usize = 1;

// The gear sits in front of the name; one column keeps the two apart because
// most terminals render the glyph as a single column.
static MODE_BUTTON_X: isize = 0;
static NAME_X: isize = 3;

// Distance from the title's right edge for the palette icon.
static SETTING_BUTTON_FROM_RIGHT: isize = 3;

pub fn build<'a>() -> Rectangle<State> {
    // Title
    let title = Rectangle::new();
    title
        .x(X)
        .y(Y)
        .width(TITLE_WIDTH)
        .height(TITLE_HEIGHT)
        .style_handle("Title");

    let name = Text::default();
    name.x(X + NAME_X)
        .y(Y)
        .text("Scientific")
        .capture(Capture::all());
    title.add(name);

    let mode_button = Text::default();
    mode_button
        .x(X + MODE_BUTTON_X)
        .y(Y)
        .text("⚙️")
        .capture(Capture::all());

    mode_button.on_mouse(|_el, state: &mut State, event| {
        if event.mouse == Mouse::Click {
            state.switch_mode(CalcMode::Basic);
        }
    });

    title.add(mode_button);

    let setting_button = Text::default();
    setting_button
        .x(X + TITLE_WIDTH as isize - SETTING_BUTTON_FROM_RIGHT)
        .y(Y)
        .text("🎨")
        .capture(Capture::all());

    setting_button
        .on_mouse(|_el, state: &mut State, event| {
            if event.mouse == Mouse::Click {
                state.theme_index = (state.theme_index + 1) % 4;
                themes::apply(state.theme_index);
            }
        })
        .on_key(|_el, state: &mut State, event| {
            if event.key == Key::Char('T') {
                state.theme_index = (state.theme_index + 1) % 4;
                themes::apply(state.theme_index);
            }
        });

    title.add(setting_button);

    // The title may have been moved (e.g. centered inside a screen), so the
    // buttons must be re-anchored to the title's current position rather than
    // to the coordinates they had when they were added.
    title.on_state(|el, _state: &State, _ev| {
        let left = el.get_x();
        let right = left + TITLE_WIDTH as isize;
        let texts = el.elements().cot::<Text<State>>();

        if let Some(name) = texts.first() {
            name.x(left + NAME_X);
        }

        if let Some(mode_button) = texts.get(1) {
            mode_button.x(left + MODE_BUTTON_X);
        }

        if let Some(setting_button) = texts.get(2) {
            setting_button.x(right - SETTING_BUTTON_FROM_RIGHT);
        }
    });

    title
}
