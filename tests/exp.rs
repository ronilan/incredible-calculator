use incredible_calculator::logic::scientific::{calculate, handle_number, input_const, input_op};
use incredible_calculator::state::State;

#[test]
fn ee_inserts_an_exponent_marker() {
    let mut state = State::new();

    handle_number(&mut state, '2');
    input_op(&mut state, "E");
    handle_number(&mut state, '3');

    // Stored with spaces like other operators; the display drops them.
    assert_eq!(state.expression, "2 E 3");

    calculate(&mut state);
    assert_eq!(state.display, "2000");
    assert_eq!(state.history, "2 E 3 =");
}

#[test]
fn ee_accepts_a_negative_exponent() {
    let mut state = State::new();

    handle_number(&mut state, '2');
    input_op(&mut state, "E");
    input_op(&mut state, "−");
    handle_number(&mut state, '3');

    calculate(&mut state);
    assert_eq!(state.display, "0.002");
}

#[test]
fn euler_constant_is_plain_lowercase_e() {
    let mut state = State::new();

    input_const(&mut state, "e");
    assert_eq!(state.display, "e");

    calculate(&mut state);
    assert!(
        state.display.starts_with("2.718281828"),
        "{}",
        state.display
    );
}

#[test]
fn scientific_results_use_the_exponent_marker() {
    let mut state = State::new();

    handle_number(&mut state, '1');
    input_op(&mut state, "÷");
    handle_number(&mut state, '1');
    for _ in 0..8 {
        handle_number(&mut state, '0');
    }

    calculate(&mut state);
    assert_eq!(state.display, "1E-8");

    // The result uses the same marker and can be reused as an expression.
    calculate(&mut state);
    assert_eq!(state.display, "1E-8");
}
