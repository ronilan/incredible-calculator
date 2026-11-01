use incredible_calculator::logic_scientific::{
    calculate, handle_number, input_const, input_op, input_reciprocal, input_sign,
};
use incredible_calculator::state::State;

fn value(text: &str) -> f64 {
    text.parse()
        .unwrap_or_else(|_| panic!("not a number: {text:?}"))
}

#[test]
fn reciprocal_inverts_the_value_being_typed() {
    let mut state = State::new();

    handle_number(&mut state, '8');
    input_reciprocal(&mut state);
    assert_eq!(state.display, "0.125");

    calculate(&mut state);
    assert_eq!(state.display, "0.125");
}

#[test]
fn reciprocal_applies_only_to_the_last_value() {
    let mut state = State::new();

    handle_number(&mut state, '8');
    input_op(&mut state, "×");
    handle_number(&mut state, '9');
    input_reciprocal(&mut state);

    assert_eq!(state.expression, "8 × 0.111111111111");

    calculate(&mut state);
    let n = value(&state.display);
    assert!((n - 8.0 / 9.0).abs() < 1e-9, "{n}");
}

#[test]
fn reciprocal_opens_when_there_is_no_value_yet() {
    let mut state = State::new();

    handle_number(&mut state, '1');
    handle_number(&mut state, '2');
    input_op(&mut state, "+");
    input_reciprocal(&mut state);
    assert_eq!(state.expression, "12 + 1/(");

    handle_number(&mut state, '8');
    calculate(&mut state);
    assert_eq!(state.display, "12.125");
}

#[test]
fn reciprocal_opens_on_a_clear_display() {
    let mut state = State::new();

    input_reciprocal(&mut state);
    assert_eq!(state.expression, "1/(");

    handle_number(&mut state, '4');
    calculate(&mut state);
    assert_eq!(state.display, "0.25");
}

#[test]
fn reciprocal_wraps_a_constant_instead_of_replacing_it() {
    let mut state = State::new();

    input_const(&mut state, "π");
    input_reciprocal(&mut state);
    assert_eq!(state.expression, "1/(π)");

    calculate(&mut state);
    let n = value(&state.display);
    assert!((n - 1.0 / std::f64::consts::PI).abs() < 1e-9, "{n}");
}

#[test]
fn reciprocal_keeps_a_negative_sign() {
    let mut state = State::new();

    handle_number(&mut state, '3');
    input_sign(&mut state);
    assert_eq!(state.expression, "-3");

    input_reciprocal(&mut state);
    assert_eq!(state.expression, "-0.333333333333");
}
