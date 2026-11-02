use incredible_calculator::logic::scientific::{
    calculate, handle_number, input_const, input_func, input_op, input_paren,
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

fn assert_close(state: &mut State, expected: f64) {
    let display = result(state);
    let value: f64 = display
        .parse()
        .unwrap_or_else(|_| panic!("expected a number, got {display:?}"));
    assert!(
        (value - expected).abs() < 1e-9,
        "{display} should be {expected}"
    );
}

#[test]
fn a_constant_after_a_number_multiplies() {
    let mut state = State::new();
    type_digits(&mut state, "4");
    input_const(&mut state, "π");

    assert_close(&mut state, 4.0 * std::f64::consts::PI);
}

#[test]
fn adjacent_constants_multiply() {
    let mut state = State::new();
    input_const(&mut state, "π");
    input_const(&mut state, "e");

    assert_close(&mut state, std::f64::consts::PI * std::f64::consts::E);
}

#[test]
fn a_constant_inside_a_square_root_multiplies() {
    let mut state = State::new();
    input_func(&mut state, "sqrt");
    type_digits(&mut state, "4");
    input_const(&mut state, "π");

    assert_close(&mut state, (4.0 * std::f64::consts::PI).sqrt());
}

#[test]
fn a_number_before_parentheses_multiplies() {
    let mut state = State::new();
    type_digits(&mut state, "2");
    input_paren(&mut state, '(');
    type_digits(&mut state, "3");
    input_op(&mut state, "+");
    type_digits(&mut state, "4");
    input_paren(&mut state, ')');

    assert_eq!(result(&mut state), "14");
}

#[test]
fn a_number_after_parentheses_multiplies() {
    let mut state = State::new();
    input_paren(&mut state, '(');
    type_digits(&mut state, "2");
    input_op(&mut state, "+");
    type_digits(&mut state, "3");
    input_paren(&mut state, ')');
    type_digits(&mut state, "4");

    assert_eq!(result(&mut state), "20");
}

#[test]
fn a_decimal_before_parentheses_multiplies() {
    let mut state = State::new();
    type_digits(&mut state, "2.5");
    input_paren(&mut state, '(');
    type_digits(&mut state, "4");
    input_paren(&mut state, ')');

    assert_eq!(result(&mut state), "10");
}

#[test]
fn a_function_after_a_number_multiplies() {
    let mut state = State::new();
    type_digits(&mut state, "2");
    input_func(&mut state, "sin");
    type_digits(&mut state, "30");

    assert_eq!(result(&mut state), "1");
}

#[test]
fn adjacent_function_calls_multiply() {
    let mut state = State::new();
    input_func(&mut state, "sin");
    type_digits(&mut state, "30");
    input_paren(&mut state, ')');
    input_func(&mut state, "cos");
    type_digits(&mut state, "60");

    assert_eq!(result(&mut state), "0.25");
}

#[test]
fn a_root_after_a_number_multiplies() {
    let mut state = State::new();
    type_digits(&mut state, "4");
    input_func(&mut state, "sqrt");
    type_digits(&mut state, "9");

    assert_eq!(result(&mut state), "12");
}

#[test]
fn a_constant_before_a_function_multiplies() {
    let mut state = State::new();
    input_const(&mut state, "π");
    input_func(&mut state, "sin");
    type_digits(&mut state, "30");

    assert_close(&mut state, std::f64::consts::PI * 0.5);
}

#[test]
fn exponents_are_not_read_as_multiplication() {
    let mut state = State::new();
    type_digits(&mut state, "2");
    input_op(&mut state, "E");
    type_digits(&mut state, "3");

    assert_eq!(result(&mut state), "2000");
}

#[test]
fn adjacent_groups_multiply() {
    let mut state = State::new();
    input_paren(&mut state, '(');
    type_digits(&mut state, "2");
    input_op(&mut state, "+");
    type_digits(&mut state, "3");
    input_paren(&mut state, ')');
    input_paren(&mut state, '(');
    type_digits(&mut state, "4");
    input_op(&mut state, "+");
    type_digits(&mut state, "5");
    input_paren(&mut state, ')');

    assert_eq!(result(&mut state), "45");
}

#[test]
fn a_constant_before_a_group_multiplies() {
    let mut state = State::new();
    input_const(&mut state, "π");
    input_paren(&mut state, '(');
    type_digits(&mut state, "2");
    input_paren(&mut state, ')');

    assert_close(&mut state, 2.0 * std::f64::consts::PI);
}

#[test]
fn a_constant_after_a_group_multiplies() {
    let mut state = State::new();
    input_paren(&mut state, '(');
    type_digits(&mut state, "2");
    input_paren(&mut state, ')');
    input_const(&mut state, "π");

    assert_close(&mut state, 2.0 * std::f64::consts::PI);
}

#[test]
fn a_number_after_a_function_call_multiplies() {
    let mut state = State::new();
    input_func(&mut state, "fact");
    type_digits(&mut state, "3");
    input_paren(&mut state, ')');
    type_digits(&mut state, "2");

    assert_eq!(result(&mut state), "12");
}

#[test]
fn a_group_after_an_exponent_multiplies() {
    let mut state = State::new();
    type_digits(&mut state, "2");
    input_op(&mut state, "^");
    type_digits(&mut state, "3");
    input_paren(&mut state, '(');
    type_digits(&mut state, "4");
    input_paren(&mut state, ')');

    assert_eq!(result(&mut state), "32");
}

#[test]
fn division_and_implicit_multiplication_run_left_to_right() {
    let mut state = State::new();
    type_digits(&mut state, "8");
    input_op(&mut state, "÷");
    type_digits(&mut state, "2");
    input_paren(&mut state, '(');
    type_digits(&mut state, "4");
    input_paren(&mut state, ')');

    assert_eq!(result(&mut state), "16");
}
