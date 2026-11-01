use std::rc::Rc;

use incredible::*;
use incredible_calculator::state::State;
use incredible_calculator::ui::screens::basic::basic;
use incredible_elements::{FancyButton, Rectangle};

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

fn button(screen: &Rectangle<State>, label: &str) -> Rc<FancyButton<State>> {
    screen
        .elements()
        .dcot::<FancyButton<State>>()
        .into_iter()
        .find(|button| button.get_text() == label)
        .unwrap_or_else(|| panic!("the basic keypad should have a {label:?} button"))
}

fn key_event(key: Key) -> EventKey {
    EventKey {
        loop_count: 0,
        key,
        modifiers: vec![],
        repeated: 0,
        timestamp: 0.0,
    }
}

fn state_event() -> EventState<State> {
    EventState {
        loop_count: 0,
        prev_state: State::new(),
        timestamp: 0.0,
    }
}

fn press(screen: &Rectangle<State>, state: &mut State, label: &str, key: Key) {
    button(screen, label).trigger_on_key(state, &key_event(key));
}

fn click(screen: &Rectangle<State>, state: &mut State, label: &str) {
    let button = button(screen, label);
    let pointer = |mouse| EventMouse {
        loop_count: 0,
        mouse,
        modifiers: vec![],
        x: button.get_x(),
        y: button.get_y(),
        timestamp: 0.0,
        source: PointerSource::Mouse,
    };

    // The framework synthesizes `Click` on release, after a press that
    // started over the same element. The screen is never flattened in a unit
    // test, so mark the button visible and give it the cached bounds
    // flattening would set.
    button.status().computed_showed.set(true);
    button.container().effective_bounds.set(&Bounds::new(
        button.get_x(),
        button.get_y(),
        button.get_width(),
        button.get_height(),
    ));
    button.trigger_on_mouse(state, &pointer(Mouse::Down));
    button.trigger_on_mouse(state, &pointer(Mouse::Up));
}

#[test]
fn basic_number_keys_type_digits() {
    let screen = screen();
    let mut state = State::new();

    for digit in "789".chars() {
        press(&screen, &mut state, &digit.to_string(), Key::Char(digit));
    }

    assert_eq!(state.display, "789");
}

#[test]
fn basic_operator_keys_chain_and_evaluate() {
    let screen = screen();
    let mut state = State::new();

    press(&screen, &mut state, "2", Key::Char('2'));
    press(&screen, &mut state, "+", Key::Char('+'));
    press(&screen, &mut state, "3", Key::Char('3'));
    press(&screen, &mut state, "×", Key::Char('*'));
    assert_eq!(state.display, "5");

    press(&screen, &mut state, "4", Key::Char('4'));
    press(&screen, &mut state, "=", Key::Char('='));

    assert_eq!(state.display, "20");
}

#[test]
fn basic_equals_button_also_accepts_enter() {
    let screen = screen();
    let mut state = State::new();

    press(&screen, &mut state, "2", Key::Char('2'));
    press(&screen, &mut state, "+", Key::Char('+'));
    press(&screen, &mut state, "3", Key::Char('3'));
    press(&screen, &mut state, "=", Key::Enter);

    assert_eq!(state.display, "5");
}

#[test]
fn basic_dot_key_starts_a_decimal() {
    let screen = screen();
    let mut state = State::new();

    press(&screen, &mut state, ".", Key::Char('.'));
    press(&screen, &mut state, "5", Key::Char('5'));

    assert_eq!(state.display, "0.5");
}

#[test]
fn basic_buttons_respond_to_mouse_clicks() {
    let screen = screen();
    let mut state = State::new();

    click(&screen, &mut state, "5");
    click(&screen, &mut state, "×");
    click(&screen, &mut state, "4");
    click(&screen, &mut state, "=");

    assert_eq!(state.display, "20");
}

#[test]
fn basic_clear_button_relabels_and_clears() {
    let screen = screen();
    let mut state = State::new();

    press(&screen, &mut state, "1", Key::Char('1'));
    press(&screen, &mut state, "2", Key::Char('2'));

    let clear = button(&screen, "AC");
    clear.trigger_on_state(&state, &state_event());
    assert_eq!(clear.get_text(), "C");

    clear.trigger_on_key(&mut state, &key_event(Key::Delete));
    assert_eq!(state.display, "0");

    clear.trigger_on_state(&state, &state_event());
    assert_eq!(clear.get_text(), "AC");
}

#[test]
fn basic_percentage_and_sign_keys_are_wired() {
    let screen = screen();
    let mut state = State::new();

    press(&screen, &mut state, "5", Key::Char('5'));
    press(&screen, &mut state, "0", Key::Char('0'));
    press(&screen, &mut state, "%", Key::Char('%'));
    assert_eq!(state.display, "0.5");

    click(&screen, &mut state, "±");
    assert_eq!(state.display, "-0.5");
}
