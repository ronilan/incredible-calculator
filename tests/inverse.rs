use incredible_calculator::logic::scientific::{
    calculate, handle_number, input_func, input_root, toggle_inverse,
};
use incredible_calculator::state::State;

fn type_digits(state: &mut State, digits: &str) {
    for digit in digits.chars() {
        handle_number(state, digit);
    }
}

fn inverse_state() -> State {
    let mut state = State::new();
    toggle_inverse(&mut state);
    state
}

#[test]
fn inverse_turns_trig_and_logs_into_their_arc_and_exp_forms() {
    let cases = [
        ("sin", "asin("),
        ("cos", "acos("),
        ("tan", "atan("),
        ("ln", "e^("),
        ("log", "10^("),
    ];

    for (func, expected) in cases {
        let mut state = inverse_state();
        input_func(&mut state, func);
        assert_eq!(state.expression, expected, "inverse of {func}");
    }
}

#[test]
fn inverse_of_sqrt_squares_the_value() {
    let mut state = inverse_state();
    type_digits(&mut state, "5");
    input_func(&mut state, "sqrt");
    assert_eq!(state.expression, "5 ^ 2");

    calculate(&mut state);
    assert_eq!(state.display, "25");
}

#[test]
fn inverse_of_power_takes_an_nth_root() {
    let mut state = inverse_state();
    type_digits(&mut state, "8");
    input_root(&mut state);
    type_digits(&mut state, "3");

    // The open parenthesis is closed when evaluating.
    calculate(&mut state);
    assert_eq!(state.display, "2");
}

#[test]
fn sqrt_still_displays_as_a_root_sign() {
    let mut state = State::new();
    input_func(&mut state, "sqrt");
    assert_eq!(state.expression, "√(");

    type_digits(&mut state, "9");
    // The open parenthesis is closed when evaluating.
    calculate(&mut state);
    assert_eq!(state.display, "3");
}

#[test]
fn the_arc_functions_take_a_ratio_and_return_degrees() {
    let cases = [
        ("sin", "0.5", "30"),
        ("cos", "0.5", "60"),
        ("tan", "1", "45"),
    ];

    for (func, ratio, expected) in cases {
        let mut state = inverse_state();
        input_func(&mut state, func);
        type_digits(&mut state, ratio);

        calculate(&mut state);
        assert_eq!(state.display, expected, "{func}({ratio})");
    }
}

#[test]
fn arc_cosine_is_not_read_as_cosine() {
    let mut state = inverse_state();
    input_func(&mut state, "cos");
    type_digits(&mut state, "60");

    // acos takes a ratio in [-1, 1], so 60 is out of range. Before the fix
    // this was evaluated as cos(60) and answered 0.5.
    calculate(&mut state);
    assert_eq!(state.display, "Error");
}
