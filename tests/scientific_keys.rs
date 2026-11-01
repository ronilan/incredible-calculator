use std::rc::Rc;

use incredible::*;
use incredible_calculator::state::State;
use incredible_calculator::ui::screens::scientific::scientific;
use incredible_elements::{FancyButton, FancyButtonGroup, Rectangle};

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
    scientific::build()
}

fn button(screen: &Rectangle<State>, label: &str) -> Rc<FancyButton<State>> {
    screen
        .elements()
        .dcot::<FancyButton<State>>()
        .into_iter()
        .find(|button| button.get_text() == label)
        .unwrap_or_else(|| panic!("the scientific keypad should have a {label:?} button"))
}

fn angle_group(screen: &Rectangle<State>) -> Rc<FancyButtonGroup<State>> {
    screen
        .elements()
        .dcot::<FancyButtonGroup<State>>()
        .into_iter()
        .next()
        .expect("the scientific keypad should have a Deg/Rad group")
}

fn key_event(key: Key, modifiers: &[KeyMod]) -> EventKey {
    EventKey {
        loop_count: 0,
        key,
        modifiers: modifiers.to_vec(),
        repeated: 0,
        timestamp: 0.0,
    }
}

fn press_with(
    screen: &Rectangle<State>,
    state: &mut State,
    label: &str,
    key: Key,
    modifiers: &[KeyMod],
) {
    button(screen, label).trigger_on_key(state, &key_event(key, modifiers));
}

fn press(screen: &Rectangle<State>, state: &mut State, label: &str, key: Key) {
    press_with(screen, state, label, key, &[]);
}

fn type_digits(screen: &Rectangle<State>, state: &mut State, digits: &str) {
    for digit in digits.chars() {
        press(screen, state, &digit.to_string(), Key::Char(digit));
    }
}

fn evaluate(screen: &Rectangle<State>, state: &mut State) {
    press(screen, state, "=", Key::Char('='));
}

#[test]
fn every_formerly_mouse_only_button_has_a_key() {
    let screen = screen();

    for label in [
        "Ans", "1/x", "EE", "Inv", "π", "e", "x!", "sin", "cos", "tan", "log", "ln", "√",
    ] {
        let button = button(&screen, label);
        assert!(
            button.internal_state.key_combination.borrow().is_some(),
            "{label} should have a keyboard binding"
        );
    }
}

#[test]
fn ans_key_reuses_the_last_result() {
    let screen = screen();
    let mut state = State::new();
    state.last_ans = 5.0;
    state.should_reset = true;

    press(&screen, &mut state, "Ans", Key::Char('a'));
    assert_eq!(state.expression, "Ans");

    evaluate(&screen, &mut state);
    assert_eq!(state.display, "5");
}

#[test]
fn inv_key_toggles_the_inverse_functions() {
    let screen = screen();
    let mut state = State::new();

    press(&screen, &mut state, "Inv", Key::Char('i'));
    assert!(state.is_inverse);

    press(&screen, &mut state, "Inv", Key::Char('i'));
    assert!(!state.is_inverse);
}

#[test]
fn reciprocal_key_inverts_the_value_being_typed() {
    let screen = screen();
    let mut state = State::new();

    type_digits(&screen, &mut state, "4");
    press(&screen, &mut state, "1/x", Key::Char('x'));

    assert_eq!(state.expression, "0.25");
}

#[test]
fn ee_key_enters_the_exponent_marker() {
    let screen = screen();
    let mut state = State::new();

    type_digits(&screen, &mut state, "2");
    press(&screen, &mut state, "EE", Key::Char('E'));
    type_digits(&screen, &mut state, "3");

    assert_eq!(state.expression, "2 E 3");
    evaluate(&screen, &mut state);
    assert_eq!(state.display, "2000");
}

#[test]
fn constant_keys_insert_pi_and_e() {
    let screen = screen();

    for (label, key, expected) in [("π", Key::Char('p'), "π"), ("e", Key::Char('e'), "e")] {
        let mut state = State::new();
        press(&screen, &mut state, label, key);

        assert_eq!(state.expression, expected);
    }
}

#[test]
fn function_keys_open_their_function_call() {
    let screen = screen();

    for (label, key, expected) in [
        ("sin", Key::Char('s'), "sin("),
        ("cos", Key::Char('c'), "cos("),
        ("tan", Key::Char('t'), "tan("),
        ("log", Key::Char('l'), "log("),
        ("ln", Key::Char('n'), "ln("),
        ("√", Key::Char('q'), "√("),
        ("x!", Key::Char('!'), "fact("),
    ] {
        let mut state = State::new();
        press(&screen, &mut state, label, key);

        assert_eq!(state.expression, expected, "{label} key");
    }
}

#[test]
fn function_keys_evaluate_through_the_keypad() {
    let screen = screen();

    let cases = [
        ("sin", Key::Char('s'), "30", "0.5"),
        ("√", Key::Char('q'), "9", "3"),
        ("x!", Key::Char('!'), "5", "120"),
    ];

    for (label, key, argument, expected) in cases {
        let mut state = State::new();
        press(&screen, &mut state, label, key);
        type_digits(&screen, &mut state, argument);
        evaluate(&screen, &mut state);

        assert_eq!(state.display, expected, "{label} key");
    }
}

#[test]
fn inverse_changes_what_the_square_root_key_does() {
    let screen = screen();
    let mut state = State::new();

    press(&screen, &mut state, "Inv", Key::Char('i'));
    type_digits(&screen, &mut state, "5");
    press(&screen, &mut state, "√", Key::Char('q'));
    evaluate(&screen, &mut state);

    // With Inv on, √ becomes x².
    assert_eq!(state.display, "25");
}

#[test]
fn backspace_is_delete_and_clear_entry_is_ctrl_delete() {
    let screen = screen();
    let mut state = State::new();

    type_digits(&screen, &mut state, "12");
    press(&screen, &mut state, "+", Key::Char('+'));
    type_digits(&screen, &mut state, "34");
    assert_eq!(state.expression, "12 + 34");

    press(&screen, &mut state, "⌫", Key::Delete);
    assert_eq!(state.expression, "12 + 3");

    press_with(&screen, &mut state, "C", Key::Delete, &[KeyMod::Ctrl]);
    assert_eq!(state.expression, "12 + 0");
}

#[test]
fn escape_clears_everything() {
    let screen = screen();
    let mut state = State::new();

    type_digits(&screen, &mut state, "12");
    press(&screen, &mut state, "+", Key::Char('+'));
    type_digits(&screen, &mut state, "34");
    state.last_ans = 9.0;

    press(&screen, &mut state, "AC", Key::Escape);

    assert_eq!(state.expression, "0");
    assert_eq!(state.display, "0");
    assert_eq!(state.last_ans, 0.0);
    assert!(state.history.is_empty());
}

#[test]
fn angle_key_toggles_between_degrees_and_radians() {
    let screen = screen();
    let mut state = State::new();
    let group = angle_group(&screen);

    group.trigger_on_key(&mut state, &key_event(Key::Char('d'), &[]));
    assert!(state.is_radian);

    group.trigger_on_key(&mut state, &key_event(Key::Char('d'), &[]));
    assert!(!state.is_radian);
}
