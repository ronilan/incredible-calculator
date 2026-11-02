use incredible_calculator::logic::scientific::{
    calculate, handle_number, input_func, set_angle_mode,
};
use incredible_calculator::state::State;

fn sine_of_30(state: &mut State) -> f64 {
    input_func(state, "sin");
    handle_number(state, '3');
    handle_number(state, '0');
    calculate(state);

    state
        .display
        .parse()
        .unwrap_or_else(|_| panic!("not a number: {:?}", state.display))
}

#[test]
fn a_new_calculator_starts_in_degrees() {
    assert!(!State::new().is_radian);
}

#[test]
fn angle_mode_can_be_selected_directly() {
    let mut state = State::new();

    set_angle_mode(&mut state, false);
    assert!(!state.is_radian);

    set_angle_mode(&mut state, true);
    assert!(state.is_radian);

    // Selecting the mode that is already active keeps it.
    set_angle_mode(&mut state, true);
    assert!(state.is_radian);
}

#[test]
fn angle_mode_changes_how_trigonometry_is_evaluated() {
    let mut state = State::new();
    set_angle_mode(&mut state, false);
    assert!((sine_of_30(&mut state) - 0.5).abs() < 1e-9);

    let mut state = State::new();
    set_angle_mode(&mut state, true);
    let radians = 30.0f64.sin();
    assert!((sine_of_30(&mut state) - radians).abs() < 1e-9);
}
