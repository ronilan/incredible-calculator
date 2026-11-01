use incredible_calculator::logic_scientific::{
    calculate, handle_clear_all, handle_clear_entry, handle_delete, handle_number, input_ans,
    input_func, input_op, input_paren,
};
use incredible_calculator::state::State;

#[test]
fn delete_removes_one_character() {
    let mut state = State::new();

    handle_number(&mut state, '1');
    handle_number(&mut state, '2');
    assert_eq!(state.display, "12");

    handle_delete(&mut state);
    assert_eq!(state.display, "1");

    handle_delete(&mut state);
    assert_eq!(state.display, "0");
}

#[test]
fn delete_skips_the_spaces_around_operators() {
    let mut state = State::new();

    handle_number(&mut state, '1');
    handle_number(&mut state, '2');
    input_op(&mut state, "+");
    handle_number(&mut state, '3');

    // One press removes the visible digit, not a stored space.
    handle_delete(&mut state);
    assert_eq!(state.display, "12 +");

    // The next press removes the visible operator (and its spaces).
    handle_delete(&mut state);
    assert_eq!(state.display, "12");
}

#[test]
fn clear_entry_clears_only_the_current_value() {
    let mut state = State::new();

    handle_number(&mut state, '1');
    handle_number(&mut state, '2');
    handle_clear_entry(&mut state);
    assert_eq!(state.display, "0");

    handle_number(&mut state, '1');
    handle_number(&mut state, '2');
    input_op(&mut state, "+");
    handle_number(&mut state, '3');
    handle_number(&mut state, '4');
    handle_clear_entry(&mut state);
    assert_eq!(state.display, "12 + 0");
}

#[test]
fn clear_entry_does_nothing_after_an_operator() {
    let mut state = State::new();

    handle_number(&mut state, '1');
    handle_number(&mut state, '2');
    input_op(&mut state, "+");

    let expression = state.expression.clone();
    let history = state.history.clone();

    handle_clear_entry(&mut state);

    assert_eq!(state.expression, expression);
    assert_eq!(state.history, history);
}

#[test]
fn clear_entry_after_equals_zeroes_the_result_but_keeps_the_answer() {
    let mut state = State::new();

    handle_number(&mut state, '2');
    input_op(&mut state, "+");
    handle_number(&mut state, '3');
    calculate(&mut state);
    assert_eq!(state.display, "5");

    handle_clear_entry(&mut state);
    assert_eq!(state.display, "0");
    assert_eq!(state.last_ans, 5.0);
}

#[test]
fn clear_all_resets_the_answer() {
    let mut state = State::new();

    handle_number(&mut state, '2');
    input_op(&mut state, "+");
    handle_number(&mut state, '3');
    calculate(&mut state);
    assert_eq!(state.last_ans, 5.0);

    handle_clear_all(&mut state);
    assert_eq!(state.display, "0");
    assert_eq!(state.expression, "0");
    assert_eq!(state.history, "");
    assert_eq!(state.last_ans, 0.0);

    // `Ans` now refers to the fresh value.
    handle_number(&mut state, '7');
    assert_eq!(state.history, "Ans = 0");
}

#[test]
fn delete_leaves_a_fresh_result_alone() {
    let mut state = State::new();

    handle_number(&mut state, '2');
    input_op(&mut state, "+");
    handle_number(&mut state, '3');
    calculate(&mut state);

    handle_delete(&mut state);
    assert_eq!(state.display, "5");
    assert_eq!(state.last_ans, 5.0);
}

#[test]
fn delete_removes_a_function_name_with_its_parenthesis() {
    let mut state = State::new();

    input_func(&mut state, "cos");
    assert_eq!(state.expression, "cos(");

    // The whole call opener goes, so no bare `cos` is left on the display.
    handle_delete(&mut state);
    assert_eq!(state.display, "0");
}

#[test]
fn delete_removes_a_function_call_after_a_value() {
    let mut state = State::new();

    handle_number(&mut state, '1');
    handle_number(&mut state, '2');
    input_op(&mut state, "+");
    input_func(&mut state, "sin");
    assert_eq!(state.expression, "12 + sin(");

    handle_delete(&mut state);
    assert_eq!(state.display, "12 +");
}

#[test]
fn delete_removes_the_longer_inverse_name() {
    let mut state = State::new();
    state.is_inverse = true;

    input_func(&mut state, "sin");
    assert_eq!(state.expression, "asin(");

    handle_delete(&mut state);
    assert_eq!(state.display, "0");
}

#[test]
fn delete_removes_the_root_sign_with_its_parenthesis() {
    let mut state = State::new();

    input_func(&mut state, "sqrt");
    assert_eq!(state.expression, "√(");

    handle_delete(&mut state);
    assert_eq!(state.display, "0");
}

#[test]
fn delete_leaves_a_plain_parenthesis_alone() {
    let mut state = State::new();

    handle_number(&mut state, '1');
    handle_number(&mut state, '2');
    input_op(&mut state, "+");
    input_paren(&mut state, '(');
    assert_eq!(state.expression, "12 + (");

    handle_delete(&mut state);
    assert_eq!(state.display, "12 +");
}

#[test]
fn delete_removes_ans_as_one_word() {
    let mut state = State::new();

    input_ans(&mut state);
    assert_eq!(state.expression, "Ans");

    handle_delete(&mut state);
    assert_eq!(state.display, "0");
}

#[test]
fn delete_removes_ans_after_a_value() {
    let mut state = State::new();

    handle_number(&mut state, '6');
    input_ans(&mut state);
    assert_eq!(state.expression, "6 × Ans");

    handle_delete(&mut state);
    assert_eq!(state.display, "6 ×");
}
