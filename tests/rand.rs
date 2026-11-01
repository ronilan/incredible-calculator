use incredible_calculator::logic_scientific::{
    calculate, handle_number, input_const, input_op, input_rand, input_sign,
};
use incredible_calculator::state::State;

fn value(text: &str) -> f64 {
    text.parse()
        .unwrap_or_else(|_| panic!("not a number: {text:?}"))
}

fn random_tail(prefix: &str, expression: &str) -> f64 {
    let tail = expression
        .strip_prefix(prefix)
        .unwrap_or_else(|| panic!("{expression:?} does not start with {prefix:?}"));
    value(tail)
}

#[test]
fn rand_inserts_when_the_position_is_empty() {
    let mut state = State::new();

    input_rand(&mut state);
    let n = value(&state.display);
    assert!((0.0..1.0).contains(&n), "{}", state.display);

    let mut state = State::new();
    handle_number(&mut state, '8');
    input_op(&mut state, "×");
    input_rand(&mut state);
    let n = random_tail("8 × ", &state.expression);
    assert!((0.0..1.0).contains(&n), "{}", state.expression);

    calculate(&mut state);
    let n = value(&state.display);
    assert!((0.0..8.0).contains(&n), "{}", state.display);
}

#[test]
fn rand_replaces_only_the_last_value() {
    let mut state = State::new();

    handle_number(&mut state, '8');
    input_op(&mut state, "×");
    handle_number(&mut state, '9');
    input_rand(&mut state);

    let n = random_tail("8 × ", &state.expression);
    assert!((0.0..1.0).contains(&n), "{}", state.expression);

    calculate(&mut state);
    let n = value(&state.display);
    assert!((0.0..8.0).contains(&n), "{}", state.display);
}

#[test]
fn rand_leaves_earlier_values_alone() {
    let mut state = State::new();

    handle_number(&mut state, '8');
    input_op(&mut state, "×");
    handle_number(&mut state, '9');
    input_op(&mut state, "+");
    handle_number(&mut state, '3');
    input_rand(&mut state);

    let n = random_tail("8 × 9 + ", &state.expression);
    assert!((0.0..1.0).contains(&n), "{}", state.expression);

    calculate(&mut state);
    let n = value(&state.display);
    assert!((72.0..73.0).contains(&n), "{}", state.display);
}

#[test]
fn rand_replaces_a_negative_value_with_a_positive_one() {
    let mut state = State::new();

    handle_number(&mut state, '8');
    input_op(&mut state, "×");
    handle_number(&mut state, '3');
    input_sign(&mut state);
    assert_eq!(state.expression, "8 × -3");

    input_rand(&mut state);
    let n = random_tail("8 × ", &state.expression);
    assert!((0.0..1.0).contains(&n), "{}", state.expression);
}

#[test]
fn rand_replaces_a_constant() {
    let mut state = State::new();

    input_const(&mut state, "π");
    input_rand(&mut state);

    let n = value(&state.expression);
    assert!((0.0..1.0).contains(&n), "{}", state.expression);
}

#[test]
fn rand_replaces_a_finished_result() {
    let mut state = State::new();

    handle_number(&mut state, '6');
    input_rand(&mut state);
    calculate(&mut state);
    let first = value(&state.display);
    assert!((0.0..6.0).contains(&first), "{}", state.display);

    input_rand(&mut state);
    let n = value(&state.expression);
    assert!((0.0..1.0).contains(&n), "{}", state.expression);
}

#[test]
fn rand_values_are_short_enough_to_pair() {
    for _ in 0..100 {
        let mut state = State::new();
        input_rand(&mut state);

        assert!(state.expression.len() <= 8, "{}", state.expression);
        let decimals = state.expression.split('.').nth(1).unwrap_or("");
        assert!(decimals.len() <= 6, "{}", state.expression);
    }
}
