use incredible_calculator::logic::basic::{
    handle_equals, handle_number as basic_number, handle_operator,
};
use incredible_calculator::logic::scientific::{
    calculate, handle_number as sci_number, input_func, input_op,
};
use incredible_calculator::state::{CalcMode, State};

fn basic_calculation(state: &mut State) {
    basic_number(state, '1');
    basic_number(state, '2');
    handle_operator(state, '+');
    basic_number(state, '3');
    handle_equals(state);
    assert_eq!(state.display, "15");
}

fn scientific_calculation(state: &mut State) {
    sci_number(state, '3');
    input_op(state, "÷");
    sci_number(state, '2');
    calculate(state);
    assert_eq!(state.display, "1.5");
}

#[test]
fn switching_to_scientific_clears_the_basic_calculation() {
    let mut state = State::new();
    basic_calculation(&mut state);

    state.switch_mode(CalcMode::Scientific);

    assert_eq!(state.display, "0");
    assert!(state.current.is_empty());
    assert!(state.formula.is_empty());
    assert_eq!(state.repeat_op, None);
    assert_eq!(state.repeat_rhs, None);
    assert!(!state.just_evaluated);
}

#[test]
fn switching_to_basic_clears_the_scientific_calculation() {
    let mut state = State::new();
    scientific_calculation(&mut state);
    assert_eq!(state.last_ans, 1.5);

    state.switch_mode(CalcMode::Basic);

    assert_eq!(state.display, "0");
    assert_eq!(state.expression, "0");
    assert!(state.history.is_empty());
    assert_eq!(state.last_ans, 0.0);
    assert!(!state.should_reset);
    assert!(!state.is_inverse);
}

#[test]
fn switching_modes_keeps_the_theme_and_resets_the_angle_mode() {
    let mut state = State::new();
    state.theme_index = 2;
    state.is_radian = true;

    state.switch_mode(CalcMode::Scientific);
    assert_eq!(state.theme_index, 2);
    assert!(!state.is_radian);

    state.is_radian = true;
    state.switch_mode(CalcMode::Basic);
    assert_eq!(state.theme_index, 2);
    assert!(!state.is_radian);
}

#[test]
fn a_switched_calculator_has_no_stored_answer() {
    let mut state = State::new();
    scientific_calculation(&mut state);

    state.switch_mode(CalcMode::Basic);
    state.switch_mode(CalcMode::Scientific);

    // `Ans` now refers to the fresh zero instead of the old 1.5.
    input_func(&mut state, "Ans");
    calculate(&mut state);
    assert_eq!(state.display, "0");
}
