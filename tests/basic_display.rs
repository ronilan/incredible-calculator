use std::rc::Rc;

use incredible::*;
use incredible_calculator::state::State;
use incredible_calculator::ui::screens::basic::basic;
use incredible_elements::{Rectangle, Text};

struct MockOutput;

impl PlatformOutput for MockOutput {
    fn output_platform(&self) -> OutputPlatform {
        OutputPlatform::Terminal
    }

    fn is_true_color(&self) -> bool {
        false
    }

    fn size(&self) -> (usize, usize) {
        (80, 24)
    }

    fn draw_blocks(&self, _x: isize, _y: isize, _blocks: &[Vec<Block>]) {}
}

fn now() -> f64 {
    0.0
}

fn screen() -> Rectangle<State> {
    Globals::set_now(now);
    Platform::set_output_provider(MockOutput);
    basic::build()
}

fn display_text(screen: &Rectangle<State>) -> Rc<Text<State>> {
    screen
        .elements()
        .dcot::<Text<State>>()
        .into_iter()
        .find(|text| text.get_style_handle() == "Display_digit")
        .expect("the basic calculator should have a Display_digit text")
}

fn on_state(text: &Text<State>, state: &State) {
    text.trigger_on_state(
        state,
        &EventState {
            loop_count: 0,
            prev_state: State::new(),
            timestamp: 0.0,
        },
    );
}

#[test]
fn basic_display_starts_at_zero_in_a_fixed_field() {
    let screen = screen();
    let display = display_text(&screen);

    on_state(&display, &State::new());

    assert_eq!(display.get_text(), format!(" {:>24} ", "0"));
    assert_eq!(display.get_text().chars().count(), 26);
}

#[test]
fn basic_display_right_aligns_the_typed_value() {
    let screen = screen();
    let display = display_text(&screen);
    let mut state = State::new();
    state.display = "120.75".into();

    on_state(&display, &state);

    assert_eq!(display.get_text(), format!(" {:>24} ", "120.75"));
    assert!(display.get_text().starts_with(' '));
    assert!(display.get_text().ends_with("120.75 "));
}

#[test]
fn basic_display_shows_zero_for_an_empty_value() {
    let screen = screen();
    let display = display_text(&screen);
    let mut state = State::new();
    state.display.clear();

    on_state(&display, &state);

    assert_eq!(display.get_text(), format!(" {:>24} ", "0"));
}

#[test]
fn basic_display_fits_the_longest_typed_value() {
    let screen = screen();
    let display = display_text(&screen);
    let longest = "123456789012345678901234";
    let mut state = State::new();
    state.display = longest.into();

    on_state(&display, &state);

    assert_eq!(display.get_text(), format!(" {:>24} ", longest));
    assert_eq!(display.get_text().chars().count(), 26);
}
