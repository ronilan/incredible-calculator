use incredible_calculator::logic::{
    handle_clear, handle_dot, handle_equals, handle_number, handle_operator, handle_percentage,
    handle_plus_minus,
};
use incredible_calculator::state::{ClearMode, State};

fn type_digits(state: &mut State, digits: &str) {
    for digit in digits.chars() {
        handle_number(state, digit);
    }
}

/// Types `left op right =` and returns the display.
fn calculate(state: &mut State, left: &str, op: char, right: &str) -> String {
    type_digits(state, left);
    handle_operator(state, op);
    type_digits(state, right);
    handle_equals(state);
    state.display.clone()
}

#[test]
fn a_new_basic_calculator_starts_at_zero() {
    let state = State::new();

    assert_eq!(state.display, "0");
    assert!(state.current.is_empty());
    assert!(state.formula.is_empty());
    assert_eq!(state.clear_mode, ClearMode::All);
    assert!(!state.just_evaluated);
}

#[test]
fn digits_and_a_decimal_form_the_display() {
    let mut state = State::new();
    type_digits(&mut state, "120.75");

    assert_eq!(state.current, "120.75");
    assert_eq!(state.display, "120.75");
}

#[test]
fn a_dot_on_an_empty_display_starts_at_zero_point() {
    let mut state = State::new();
    handle_dot(&mut state);

    assert_eq!(state.current, "0.");
    assert_eq!(state.display, "0.");
}

#[test]
fn a_second_dot_in_the_same_number_is_ignored() {
    let mut state = State::new();

    type_digits(&mut state, "1");
    handle_dot(&mut state);
    handle_dot(&mut state);
    type_digits(&mut state, "5");

    assert_eq!(state.display, "1.5");
}

#[test]
fn a_dot_after_an_operator_starts_a_new_value() {
    let mut state = State::new();

    type_digits(&mut state, "12");
    handle_operator(&mut state, '+');
    handle_dot(&mut state);
    assert_eq!(state.display, "0.");

    type_digits(&mut state, "5");
    handle_equals(&mut state);
    assert_eq!(state.display, "12.5");
}

#[test]
fn equals_adds_the_operands() {
    let mut state = State::new();
    assert_eq!(calculate(&mut state, "12", '+', "3"), "15");
}

#[test]
fn every_operator_evaluates_the_chain_like_a_pocket_calculator() {
    let mut state = State::new();

    type_digits(&mut state, "2");
    handle_operator(&mut state, '+');
    type_digits(&mut state, "3");
    // The pending `2 + 3` is resolved when the next operator is pressed.
    handle_operator(&mut state, '*');
    assert_eq!(state.display, "5");

    type_digits(&mut state, "4");
    handle_equals(&mut state);
    assert_eq!(state.display, "20");
}

#[test]
fn repeated_equals_repeats_the_last_operation() {
    let mut state = State::new();

    type_digits(&mut state, "2");
    handle_operator(&mut state, '+');
    type_digits(&mut state, "3");

    handle_equals(&mut state);
    assert_eq!(state.display, "5");

    handle_equals(&mut state);
    assert_eq!(state.display, "8");

    handle_equals(&mut state);
    assert_eq!(state.display, "11");
}

#[test]
fn repeated_equals_without_an_operation_keeps_the_value() {
    let mut state = State::new();

    type_digits(&mut state, "5");
    handle_equals(&mut state);
    assert_eq!(state.display, "5");

    handle_equals(&mut state);
    assert_eq!(state.display, "5");
}

#[test]
fn a_new_digit_after_equals_starts_a_new_number() {
    let mut state = State::new();

    type_digits(&mut state, "2");
    handle_operator(&mut state, '+');
    type_digits(&mut state, "3");
    handle_equals(&mut state);
    assert_eq!(state.display, "5");

    type_digits(&mut state, "7");
    assert_eq!(state.display, "7");
    assert!(state.formula.is_empty());

    handle_operator(&mut state, '+');
    type_digits(&mut state, "3");
    handle_equals(&mut state);
    assert_eq!(state.display, "10");
}

#[test]
fn an_operator_after_equals_continues_from_the_result() {
    let mut state = State::new();

    type_digits(&mut state, "2");
    handle_operator(&mut state, '+');
    type_digits(&mut state, "3");
    handle_equals(&mut state);

    handle_operator(&mut state, '*');
    type_digits(&mut state, "4");
    handle_equals(&mut state);
    assert_eq!(state.display, "20");
}

#[test]
fn an_operator_pressed_twice_replaces_itself() {
    let mut state = State::new();

    type_digits(&mut state, "8");
    handle_operator(&mut state, '+');
    handle_operator(&mut state, '*');
    type_digits(&mut state, "3");
    handle_equals(&mut state);

    assert_eq!(state.display, "24");
}

#[test]
fn equals_with_nothing_typed_does_nothing() {
    let mut state = State::new();
    handle_equals(&mut state);

    assert_eq!(state.display, "0");
    assert!(state.formula.is_empty());
}

#[test]
fn division_by_zero_shows_error_and_recovers() {
    let mut state = State::new();
    assert_eq!(calculate(&mut state, "5", '/', "0"), "Error");

    type_digits(&mut state, "2");
    handle_operator(&mut state, '+');
    type_digits(&mut state, "2");
    handle_equals(&mut state);
    assert_eq!(state.display, "4");
}

#[test]
fn decimal_addition_snaps_away_float_noise() {
    let mut state = State::new();
    assert_eq!(calculate(&mut state, "0.1", '+', "0.2"), "0.3");
}

#[test]
fn a_dot_alone_evaluates_to_zero() {
    let mut state = State::new();

    handle_dot(&mut state);
    handle_equals(&mut state);

    assert_eq!(state.display, "0");
}

#[test]
fn percentage_with_a_pending_operation_uses_the_base() {
    let mut state = State::new();

    type_digits(&mut state, "200");
    handle_operator(&mut state, '*');
    type_digits(&mut state, "10");
    handle_percentage(&mut state);

    assert_eq!(state.current, "20");
    assert_eq!(state.display, "20");
}

#[test]
fn percentage_without_an_operation_is_hundredths() {
    let mut state = State::new();

    type_digits(&mut state, "50");
    handle_percentage(&mut state);

    assert_eq!(state.display, "0.5");
}

#[test]
fn percentage_of_a_fresh_display_is_zero() {
    let mut state = State::new();
    handle_percentage(&mut state);

    assert_eq!(state.display, "0");
}

#[test]
fn sign_toggles_the_number_being_typed() {
    let mut state = State::new();

    type_digits(&mut state, "12");
    handle_plus_minus(&mut state);
    assert_eq!(state.display, "-12");

    handle_plus_minus(&mut state);
    assert_eq!(state.display, "12");
}

#[test]
fn sign_does_nothing_to_zero() {
    let mut state = State::new();
    handle_plus_minus(&mut state);

    assert_eq!(state.display, "0");
}

#[test]
fn sign_after_equals_flips_the_result() {
    let mut state = State::new();

    type_digits(&mut state, "2");
    handle_operator(&mut state, '+');
    type_digits(&mut state, "3");
    handle_equals(&mut state);

    handle_plus_minus(&mut state);
    assert_eq!(state.display, "-5");
}

#[test]
fn clear_entry_keeps_the_pending_operation() {
    let mut state = State::new();

    type_digits(&mut state, "12");
    handle_operator(&mut state, '+');
    type_digits(&mut state, "34");

    // The key labels itself `C` while a value is being typed.
    assert_eq!(state.clear_mode, ClearMode::Entry);
    handle_clear(&mut state, false);
    assert_eq!(state.display, "0");
    assert_eq!(state.formula.len(), 2);

    type_digits(&mut state, "5");
    handle_equals(&mut state);
    assert_eq!(state.display, "17");
}

#[test]
fn clear_all_wipes_the_calculation() {
    let mut state = State::new();

    type_digits(&mut state, "12");
    handle_operator(&mut state, '+');
    type_digits(&mut state, "34");
    handle_clear(&mut state, true);

    assert_eq!(state.display, "0");
    assert!(state.current.is_empty());
    assert!(state.formula.is_empty());
    assert_eq!(state.repeat_op, None);
    assert_eq!(state.repeat_rhs, None);
    assert!(!state.just_evaluated);
    assert_eq!(state.clear_mode, ClearMode::All);
}

#[test]
fn clear_after_equals_clears_the_result() {
    let mut state = State::new();

    type_digits(&mut state, "2");
    handle_operator(&mut state, '+');
    type_digits(&mut state, "3");
    handle_equals(&mut state);
    assert_eq!(state.display, "5");

    handle_clear(&mut state, false);
    assert_eq!(state.display, "0");
}

#[test]
fn typing_is_capped_at_twenty_four_characters() {
    let mut state = State::new();
    type_digits(&mut state, "123456789012345678901234567890");

    assert_eq!(state.current.len(), 24);
    assert_eq!(state.display, "123456789012345678901234");
}

#[test]
fn a_dot_is_not_added_once_the_display_is_full() {
    let mut state = State::new();
    type_digits(&mut state, "123456789012345678901234");

    handle_dot(&mut state);

    assert_eq!(state.current.len(), 24);
    assert!(!state.display.contains('.'));
}
