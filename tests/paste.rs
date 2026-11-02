use incredible_calculator::logic::scientific::{calculate, paste_expression};
use incredible_calculator::state::State;

fn pasted(text: &str) -> State {
    let mut state = State::new();
    paste_expression(&mut state, text);
    state
}

fn evaluated(text: &str) -> String {
    let mut state = pasted(text);
    calculate(&mut state);
    state.display
}

#[test]
fn paste_replaces_the_whole_expression() {
    let mut state = State::new();
    paste_expression(&mut state, "7×8");
    paste_expression(&mut state, "2+3");

    assert_eq!(state.expression, "2+3");
    assert_eq!(state.display, "2+3");
    assert!(!state.should_reset);
}

#[test]
fn line_feeds_and_spaces_are_dropped() {
    let state = pasted("(2 +\n 3)\r\n\t× 4");

    assert_eq!(state.expression, "(2+3)×4");
    assert_eq!(evaluated("(2 +\n 3)\r\n\t× 4"), "20");
}

#[test]
fn operators_are_normalized() {
    let state = pasted("2 * 3 / 4 - 1");

    assert_eq!(state.expression, "2×3÷4−1");
    assert_eq!(evaluated("2 * 3 / 4 - 1"), "0.5");
}

#[test]
fn constants_and_ans_are_recognized() {
    let state = pasted("pi * ans + 1");

    assert_eq!(state.expression, "π×Ans+1");
}

#[test]
fn function_names_are_kept_and_lowercased() {
    let state = pasted("Sin(30) + COS(60)");

    assert_eq!(state.expression, "sin(30)+cos(60)");
    assert_eq!(evaluated("Sin(30) + COS(60)"), "1");
}

#[test]
fn sqrt_is_drawn_as_a_root() {
    let state = pasted("sqrt(9)");

    assert_eq!(state.expression, "√(9)");
    assert_eq!(evaluated("sqrt(9)"), "3");
}

#[test]
fn postfix_factorial_becomes_a_call() {
    assert_eq!(pasted("5!").expression, "fact(5)");
    assert_eq!(evaluated("5!"), "120");

    assert_eq!(pasted("(2+3)!").expression, "fact((2+3))");
    assert_eq!(evaluated("(2+3)!"), "120");
}

#[test]
fn percent_divides_by_a_hundred() {
    let state = pasted("50%+1");

    assert_eq!(state.expression, "50÷100+1");
    assert_eq!(evaluated("50%+1"), "1.5");
}

#[test]
fn lowercase_e_is_an_exponent_after_a_value() {
    assert_eq!(pasted("1e3").expression, "1E3");
    assert_eq!(evaluated("1e3"), "1000");

    // With no value in front of it, `e` is Euler's constant.
    assert_eq!(pasted("e^2").expression, "e^2");
    assert_eq!(evaluated("e^2"), "7.389056098931");
}

#[test]
fn unsupported_characters_are_discarded() {
    let state = pasted("2 apples + 3 #@$");

    assert_eq!(state.expression, "2+3");
    assert_eq!(evaluated("2 apples + 3 #@$"), "5");
}

#[test]
fn an_empty_paste_leaves_the_expression_alone() {
    let mut state = State::new();
    state.expression = "9".into();
    state.display = "9".into();

    paste_expression(&mut state, " \n\t#@$");

    assert_eq!(state.expression, "9");
    assert_eq!(state.display, "9");
}

#[test]
fn a_long_paste_keeps_the_first_part() {
    let state = pasted(&"1".repeat(600));

    assert_eq!(state.expression.chars().count(), 512);
}

fn assert_close(text: &str, expected: f64) {
    let display = evaluated(text);
    let value: f64 = display
        .parse()
        .unwrap_or_else(|_| panic!("expected a number, got {display:?}"));
    assert!(
        (value - expected).abs() < 1e-9,
        "{display} should be {expected}"
    );
}

#[test]
fn unicode_operators_are_kept_as_the_keypad_spellings() {
    let state = pasted("6 × 7 − 2 ÷ 2");

    assert_eq!(state.expression, "6×7−2÷2");
    assert_eq!(evaluated("6 × 7 − 2 ÷ 2"), "41");
}

#[test]
fn root_signs_and_constants_pass_through() {
    let state = pasted("√(16) + π");

    assert_eq!(state.expression, "√(16)+π");
    assert_close("√(16) + π", 4.0 + std::f64::consts::PI);
}

#[test]
fn uppercase_words_are_recognized() {
    let state = pasted("SQRT(81) + SIN(30)");

    assert_eq!(state.expression, "√(81)+sin(30)");
    assert_eq!(evaluated("SQRT(81) + SIN(30)"), "9.5");
}

#[test]
fn signed_exponents_are_kept() {
    let state = pasted("2e-3");

    assert_eq!(state.expression, "2E−3");
    assert_eq!(evaluated("2e-3"), "0.002");
}

#[test]
fn paste_after_an_error_starts_over() {
    let mut state = State::new();
    state.expression = "Error".into();
    state.display = "Error".into();

    paste_expression(&mut state, "4+4");

    assert_eq!(state.expression, "4+4");
    assert!(!state.should_reset);

    calculate(&mut state);
    assert_eq!(state.display, "8");
}

#[test]
fn paste_refreshes_the_history_line() {
    let mut state = State::new();
    state.last_ans = 12.0;

    paste_expression(&mut state, "Ans+1");

    assert_eq!(state.history, "Ans = 12");

    calculate(&mut state);
    assert_eq!(state.display, "13");
}

#[test]
fn thousand_separators_are_dropped() {
    let state = pasted("1,234 + 5");

    assert_eq!(state.expression, "1234+5");
    assert_eq!(evaluated("1,234 + 5"), "1239");
}

#[test]
fn bare_functions_and_roots_open_their_parenthesis() {
    assert_eq!(pasted("sin30").expression, "sin(30");
    assert_eq!(evaluated("sin30"), "0.5");

    assert_eq!(pasted("sqrt9").expression, "√(9");
    assert_eq!(evaluated("sqrt9"), "3");

    assert_eq!(pasted("√9").expression, "√(9");
    assert_eq!(evaluated("√9"), "3");

    assert_eq!(pasted("fact5").expression, "fact(5");
    assert_eq!(evaluated("fact5"), "120");
}

#[test]
fn a_percent_without_a_value_is_dropped() {
    let mut state = State::new();
    paste_expression(&mut state, "%");
    assert_eq!(state.expression, "0");
    assert_eq!(state.display, "0");

    assert_eq!(pasted("%5").expression, "5");
    assert_eq!(evaluated("%5"), "5");
}

#[test]
fn a_percent_after_a_group_divides_by_a_hundred() {
    let state = pasted("(2+3)%");

    assert_eq!(state.expression, "(2+3)÷100");
    assert_eq!(evaluated("(2+3)%"), "0.05");
}
