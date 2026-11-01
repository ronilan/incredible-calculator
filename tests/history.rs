use incredible_calculator::logic_scientific::{
    calculate, handle_clear_all, handle_number, input_op,
};
use incredible_calculator::state::State;

#[test]
fn history_is_empty_before_typing() {
    let state = State::new();
    assert_eq!(state.display, "0");
    assert_eq!(state.history, "");
}

#[test]
fn history_shows_the_previous_answer_while_typing() {
    let mut state = State::new();

    // The first keystroke turns the display into an expression, so the top
    // line switches from "nothing" to the previous answer.
    handle_number(&mut state, '2');
    assert_eq!(state.history, "Ans = 0");

    input_op(&mut state, "×");
    handle_number(&mut state, '3');
    assert_eq!(state.history, "Ans = 0");

    // A result flips it back to the evaluated expression.
    calculate(&mut state);
    assert_eq!(state.display, "6");
    assert_eq!(state.history, "2 × 3 =");

    // Typing again switches to the answer that produced this display.
    input_op(&mut state, "×");
    assert_eq!(state.display, "6 × ");
    assert_eq!(state.history, "Ans = 6");
}

#[test]
fn clearing_all_empties_the_history_until_typing_again() {
    let mut state = State::new();
    handle_number(&mut state, '5');
    input_op(&mut state, "+");
    handle_number(&mut state, '5');
    calculate(&mut state);
    assert_eq!(state.display, "10");

    handle_clear_all(&mut state);
    assert_eq!(state.history, "");
    assert_eq!(state.last_ans, 0.0);

    handle_number(&mut state, '7');
    assert_eq!(state.history, "Ans = 0");
}
