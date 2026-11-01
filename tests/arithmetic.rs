use incredible_calculator::logic_scientific::{
    calculate, handle_number, input_const, input_func, input_op, input_paren, input_percent,
    input_sign, set_angle_mode, toggle_inverse,
};
use incredible_calculator::state::State;

fn type_digits(state: &mut State, digits: &str) {
    for digit in digits.chars() {
        handle_number(state, digit);
    }
}

fn result(state: &mut State) -> String {
    calculate(state);
    state.display.clone()
}

#[test]
fn multiplication_binds_tighter_than_addition() {
    let mut state = State::new();

    type_digits(&mut state, "2");
    input_op(&mut state, "+");
    type_digits(&mut state, "3");
    input_op(&mut state, "×");
    type_digits(&mut state, "4");

    assert_eq!(result(&mut state), "14");
}

#[test]
fn parentheses_override_precedence() {
    let mut state = State::new();

    input_paren(&mut state, '(');
    type_digits(&mut state, "2");
    input_op(&mut state, "+");
    type_digits(&mut state, "3");
    input_paren(&mut state, ')');
    input_op(&mut state, "×");
    type_digits(&mut state, "4");

    assert_eq!(result(&mut state), "20");
}

#[test]
fn nested_parentheses_are_evaluated_inside_out() {
    let mut state = State::new();

    type_digits(&mut state, "2");
    input_op(&mut state, "×");
    input_paren(&mut state, '(');
    type_digits(&mut state, "3");
    input_op(&mut state, "+");
    input_paren(&mut state, '(');
    type_digits(&mut state, "4");
    input_op(&mut state, "−");
    type_digits(&mut state, "1");
    input_paren(&mut state, ')');
    input_paren(&mut state, ')');

    assert_eq!(result(&mut state), "12");
}

#[test]
fn subtraction_is_left_associative() {
    let mut state = State::new();

    type_digits(&mut state, "10");
    input_op(&mut state, "−");
    type_digits(&mut state, "3");
    input_op(&mut state, "−");
    type_digits(&mut state, "2");

    assert_eq!(result(&mut state), "5");
}

#[test]
fn division_is_left_associative() {
    let mut state = State::new();

    type_digits(&mut state, "100");
    input_op(&mut state, "÷");
    type_digits(&mut state, "5");
    input_op(&mut state, "÷");
    type_digits(&mut state, "2");

    assert_eq!(result(&mut state), "10");
}

#[test]
fn powers_are_right_associative() {
    let mut state = State::new();

    type_digits(&mut state, "2");
    input_op(&mut state, "^");
    type_digits(&mut state, "3");
    input_op(&mut state, "^");
    type_digits(&mut state, "2");

    // 2 ^ (3 ^ 2), not (2 ^ 3) ^ 2.
    assert_eq!(result(&mut state), "512");
}

#[test]
fn power_binds_tighter_than_multiplication() {
    let mut state = State::new();

    type_digits(&mut state, "2");
    input_op(&mut state, "×");
    type_digits(&mut state, "3");
    input_op(&mut state, "^");
    type_digits(&mut state, "2");

    assert_eq!(result(&mut state), "18");
}

#[test]
fn a_unary_minus_starts_a_negative_number() {
    let mut state = State::new();

    type_digits(&mut state, "3");
    input_sign(&mut state);
    input_op(&mut state, "+");
    type_digits(&mut state, "5");

    assert_eq!(result(&mut state), "2");
}

#[test]
fn a_unary_minus_is_accepted_inside_a_power() {
    let mut state = State::new();

    type_digits(&mut state, "2");
    input_op(&mut state, "^");
    input_paren(&mut state, '(');
    input_op(&mut state, "−");
    type_digits(&mut state, "3");
    input_paren(&mut state, ')');

    assert_eq!(result(&mut state), "0.125");
}

#[test]
fn division_by_zero_is_an_error() {
    let mut state = State::new();

    type_digits(&mut state, "1");
    input_op(&mut state, "÷");
    type_digits(&mut state, "0");

    assert_eq!(result(&mut state), "Error");
}

#[test]
fn a_division_by_zero_inside_a_sum_is_an_error() {
    let mut state = State::new();

    type_digits(&mut state, "1");
    input_op(&mut state, "+");
    type_digits(&mut state, "1");
    input_op(&mut state, "÷");
    type_digits(&mut state, "0");

    assert_eq!(result(&mut state), "Error");
}

#[test]
fn a_result_can_be_used_by_the_next_expression() {
    let mut state = State::new();

    type_digits(&mut state, "2");
    input_op(&mut state, "+");
    type_digits(&mut state, "3");
    assert_eq!(result(&mut state), "5");

    input_op(&mut state, "×");
    type_digits(&mut state, "4");
    assert_eq!(result(&mut state), "20");
}

#[test]
fn percentages_become_hundredths_inside_an_expression() {
    let mut state = State::new();

    type_digits(&mut state, "200");
    input_op(&mut state, "×");
    type_digits(&mut state, "50");
    input_percent(&mut state);

    assert_eq!(state.expression, "200 × 0.5");
    assert_eq!(result(&mut state), "100");
}

#[test]
fn a_standalone_percentage_is_hundredths() {
    let mut state = State::new();

    type_digits(&mut state, "25");
    input_percent(&mut state);

    assert_eq!(result(&mut state), "0.25");
}

#[test]
fn pi_can_be_used_as_a_number() {
    let mut state = State::new();

    input_const(&mut state, "π");
    input_op(&mut state, "×");
    type_digits(&mut state, "2");

    assert_eq!(result(&mut state), "6.28318530718");
}

#[test]
fn e_is_eulers_number() {
    let mut state = State::new();
    input_const(&mut state, "e");

    assert_eq!(result(&mut state), "2.718281828459");
}

#[test]
fn square_root_of_a_sum_uses_the_whole_group() {
    let mut state = State::new();

    input_func(&mut state, "sqrt");
    type_digits(&mut state, "9");
    input_op(&mut state, "+");
    type_digits(&mut state, "16");

    assert_eq!(result(&mut state), "5");
}

#[test]
fn square_root_of_a_negative_number_is_an_error() {
    let mut state = State::new();

    input_func(&mut state, "sqrt");
    type_digits(&mut state, "9");
    input_sign(&mut state);

    assert_eq!(result(&mut state), "Error");
}

#[test]
fn a_factorial_multiplies_the_numbers_below_it() {
    let mut state = State::new();

    input_func(&mut state, "fact");
    type_digits(&mut state, "5");

    assert_eq!(result(&mut state), "120");
}

#[test]
fn zero_factorial_is_one() {
    let mut state = State::new();

    input_func(&mut state, "fact");
    type_digits(&mut state, "0");

    assert_eq!(result(&mut state), "1");
}

#[test]
fn a_factorial_of_a_negative_number_is_an_error() {
    let mut state = State::new();

    input_func(&mut state, "fact");
    type_digits(&mut state, "3");
    input_sign(&mut state);

    assert_eq!(result(&mut state), "Error");
}

#[test]
fn log_is_base_ten() {
    let mut state = State::new();

    input_func(&mut state, "log");
    type_digits(&mut state, "1000");

    assert_eq!(result(&mut state), "3");
}

#[test]
fn natural_log_of_e_is_one() {
    let mut state = State::new();

    input_func(&mut state, "ln");
    input_const(&mut state, "e");

    assert_eq!(result(&mut state), "1");
}

#[test]
fn trigonometry_uses_radians_when_selected() {
    let mut state = State::new();
    set_angle_mode(&mut state, true);

    input_func(&mut state, "sin");
    input_const(&mut state, "π");
    input_op(&mut state, "÷");
    type_digits(&mut state, "2");

    assert_eq!(result(&mut state), "1");
}

#[test]
fn arc_sine_returns_radians_when_selected() {
    let mut state = State::new();
    set_angle_mode(&mut state, true);

    toggle_inverse(&mut state);
    input_func(&mut state, "sin");
    type_digits(&mut state, "1");

    assert_eq!(result(&mut state), "1.570796326795");
}

#[test]
fn tangent_of_forty_five_degrees_is_one() {
    let mut state = State::new();

    input_func(&mut state, "tan");
    type_digits(&mut state, "45");

    assert_eq!(result(&mut state), "1");
}

#[test]
fn the_exponent_marker_can_be_multiplied() {
    let mut state = State::new();

    type_digits(&mut state, "2");
    input_op(&mut state, "E");
    type_digits(&mut state, "3");
    input_op(&mut state, "×");
    type_digits(&mut state, "2");

    assert_eq!(result(&mut state), "4000");
}

#[test]
fn history_keeps_the_expression_with_closed_parentheses() {
    let mut state = State::new();

    type_digits(&mut state, "2");
    input_op(&mut state, "×");
    input_paren(&mut state, '(');
    type_digits(&mut state, "3");
    input_op(&mut state, "+");
    type_digits(&mut state, "4");

    assert_eq!(result(&mut state), "14");
    assert_eq!(state.history, "2 × (3 + 4) =");
}

#[test]
fn a_fully_typed_parenthesis_is_not_closed_twice() {
    let mut state = State::new();

    input_paren(&mut state, '(');
    type_digits(&mut state, "8");
    input_op(&mut state, "÷");
    type_digits(&mut state, "2");
    input_paren(&mut state, ')');

    assert_eq!(result(&mut state), "4");
    assert_eq!(state.history, "(8 ÷ 2) =");
}

#[test]
fn a_function_argument_can_hold_another_function_call() {
    let mut state = State::new();

    input_func(&mut state, "sin");
    input_func(&mut state, "cos");
    type_digits(&mut state, "60");

    // cos(60°) = 0.5, and sin(0.5°) is evaluated in degrees as well.
    assert_eq!(result(&mut state), "0.008726535498");
}

#[test]
fn a_function_argument_can_hold_a_parenthesised_expression() {
    let mut state = State::new();

    input_func(&mut state, "sin");
    input_paren(&mut state, '(');
    type_digits(&mut state, "1");
    input_op(&mut state, "+");
    type_digits(&mut state, "2");
    input_paren(&mut state, ')');
    input_op(&mut state, "×");
    type_digits(&mut state, "3");

    // sin(9°) = 0.156434465040...
    assert_eq!(result(&mut state), "0.15643446504");
}
