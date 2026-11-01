use incredible_calculator::logic_scientific::{
    calculate, handle_number, input_op, input_percent, input_sign,
};
use incredible_calculator::state::State;

#[test]
fn sign_toggles_the_value_being_typed() {
    let mut state = State::new();

    handle_number(&mut state, '1');
    handle_number(&mut state, '2');
    input_sign(&mut state);
    assert_eq!(state.display, "-12");

    input_sign(&mut state);
    assert_eq!(state.display, "12");
}

#[test]
fn sign_applies_to_the_last_operand_only() {
    let mut state = State::new();

    handle_number(&mut state, '1');
    handle_number(&mut state, '2');
    input_op(&mut state, "+");
    handle_number(&mut state, '3');
    handle_number(&mut state, '4');
    input_sign(&mut state);
    assert_eq!(state.expression, "12 + -34");

    calculate(&mut state);
    assert_eq!(state.display, "-22");
}

#[test]
fn sign_does_nothing_after_an_operator() {
    let mut state = State::new();

    handle_number(&mut state, '1');
    handle_number(&mut state, '2');
    input_op(&mut state, "+");

    let expression = state.expression.clone();
    input_sign(&mut state);
    assert_eq!(state.expression, expression);
}

#[test]
fn percent_turns_the_value_into_hundredths() {
    let mut state = State::new();

    handle_number(&mut state, '5');
    handle_number(&mut state, '0');
    input_percent(&mut state);
    assert_eq!(state.display, "0.5");

    calculate(&mut state);
    assert_eq!(state.display, "0.5");
}

#[test]
fn percent_applies_to_the_last_operand_only() {
    let mut state = State::new();

    handle_number(&mut state, '1');
    handle_number(&mut state, '2');
    input_op(&mut state, "+");
    handle_number(&mut state, '3');
    handle_number(&mut state, '4');
    input_percent(&mut state);
    assert_eq!(state.expression, "12 + 0.34");

    calculate(&mut state);
    assert_eq!(state.display, "12.34");
}

#[test]
fn percent_does_nothing_after_an_operator() {
    let mut state = State::new();

    handle_number(&mut state, '1');
    input_op(&mut state, "+");

    let expression = state.expression.clone();
    input_percent(&mut state);
    assert_eq!(state.expression, expression);
}
