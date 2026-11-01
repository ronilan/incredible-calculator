use incredible_calculator::logic::{
    handle_dot, handle_equals, handle_number as basic_number, handle_operator,
    handle_plus_minus as basic_sign,
};
use incredible_calculator::logic_scientific::{
    calculate, handle_number as sci_number, input_op, input_sign as sci_sign,
};
use incredible_calculator::state::State;

#[derive(Clone, Copy, Debug)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    fn basic(self) -> char {
        match self {
            Op::Add => '+',
            Op::Sub => '-',
            Op::Mul => '*',
            Op::Div => '/',
        }
    }

    fn scientific(self) -> &'static str {
        match self {
            Op::Add => "+",
            Op::Sub => "−",
            Op::Mul => "×",
            Op::Div => "÷",
        }
    }
}

fn basic_type(state: &mut State, value: &str) {
    let (negative, digits) = match value.strip_prefix('-') {
        Some(digits) => (true, digits),
        None => (false, value),
    };

    for digit in digits.chars() {
        if digit == '.' {
            handle_dot(state);
        } else {
            basic_number(state, digit);
        }
    }

    if negative {
        basic_sign(state);
    }
}

fn scientific_type(state: &mut State, value: &str) {
    let (negative, digits) = match value.strip_prefix('-') {
        Some(digits) => (true, digits),
        None => (false, value),
    };

    for digit in digits.chars() {
        sci_number(state, digit);
    }

    if negative {
        sci_sign(state);
    }
}

fn basic_eval(left: &str, op: Op, right: &str) -> String {
    let mut state = State::new();
    basic_type(&mut state, left);
    handle_operator(&mut state, op.basic());
    basic_type(&mut state, right);
    handle_equals(&mut state);
    state.display
}

fn scientific_eval(left: &str, op: Op, right: &str) -> String {
    let mut state = State::new();
    scientific_type(&mut state, left);
    input_op(&mut state, op.scientific());
    scientific_type(&mut state, right);
    calculate(&mut state);
    state.display
}

fn assert_both(left: &str, op: Op, right: &str, expected: &str) {
    assert_eq!(
        basic_eval(left, op, right),
        expected,
        "basic: {left} {op:?} {right}"
    );
    assert_eq!(
        scientific_eval(left, op, right),
        expected,
        "scientific: {left} {op:?} {right}"
    );
}

#[test]
fn both_calculators_add() {
    assert_both("12", Op::Add, "3", "15");
}

#[test]
fn both_calculators_subtract() {
    assert_both("12", Op::Sub, "3", "9");
    assert_both("3", Op::Sub, "12", "-9");
}

#[test]
fn both_calculators_multiply() {
    assert_both("12", Op::Mul, "3", "36");
}

#[test]
fn both_calculators_divide() {
    assert_both("12", Op::Div, "3", "4");
}

#[test]
fn both_calculators_handle_decimals() {
    assert_both("0.1", Op::Add, "0.2", "0.3");
    assert_both("0.5", Op::Mul, "0.4", "0.2");
}

#[test]
fn both_calculators_accept_a_negative_operand() {
    assert_both("-2", Op::Mul, "3", "-6");
    assert_both("-2", Op::Add, "-3", "-5");
}

#[test]
fn both_calculators_report_division_by_zero() {
    assert_both("5", Op::Div, "0", "Error");
}

#[test]
fn basic_resolves_operations_as_they_are_typed() {
    // Pocket-calculator chaining: `2 + 3` is resolved before `× 4`.
    let mut state = State::new();
    basic_type(&mut state, "2");
    handle_operator(&mut state, '+');
    basic_type(&mut state, "3");
    handle_operator(&mut state, '*');
    basic_type(&mut state, "4");
    handle_equals(&mut state);

    assert_eq!(state.display, "20");
}

#[test]
fn scientific_follows_operator_precedence() {
    // Same keys, different rule: multiplication binds tighter than addition.
    let mut state = State::new();
    scientific_type(&mut state, "2");
    input_op(&mut state, "+");
    scientific_type(&mut state, "3");
    input_op(&mut state, "×");
    scientific_type(&mut state, "4");
    calculate(&mut state);

    assert_eq!(state.display, "14");
}
