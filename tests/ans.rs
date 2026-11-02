use incredible_calculator::logic::scientific::{calculate, handle_number, input_func, input_op};
use incredible_calculator::state::State;

fn three_halves(state: &mut State) {
    handle_number(state, '3');
    input_op(state, "÷");
    handle_number(state, '2');
    calculate(state);
    assert_eq!(state.display, "1.5");
}

#[test]
fn ans_replaces_a_result_and_stays_symbolic() {
    let mut state = State::new();
    three_halves(&mut state);

    // The result is replaced by the symbol, not pasted as digits.
    input_func(&mut state, "Ans");
    assert_eq!(state.display, "Ans");
    assert_eq!(state.history, "Ans = 1.5");

    input_op(&mut state, "×");
    handle_number(&mut state, '2');
    calculate(&mut state);
    assert_eq!(state.display, "3");
    assert_eq!(state.history, "Ans × 2 =");
}

#[test]
fn ans_after_a_value_adds_an_implicit_multiplication() {
    let mut state = State::new();
    handle_number(&mut state, '6');
    input_func(&mut state, "Ans");
    assert_eq!(state.display, "6 × Ans");
}

#[test]
fn ans_follows_a_pending_operation() {
    let mut state = State::new();
    three_halves(&mut state);

    handle_number(&mut state, '6');
    input_op(&mut state, "−");
    input_func(&mut state, "Ans");
    assert_eq!(state.display, "6 − Ans");

    calculate(&mut state);
    assert_eq!(state.display, "4.5");
    assert_eq!(state.history, "6 − Ans =");
}

#[test]
fn ans_after_an_open_parenthesis_is_appended() {
    let mut state = State::new();
    input_func(&mut state, "sqrt");
    input_func(&mut state, "Ans");
    assert_eq!(state.display, "√(Ans");
}
