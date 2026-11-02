use crate::state::{ClearMode, State, Token};

const MAX_DISPLAY_CHARS: usize = 24;
const SNAP_THRESHOLD: f64 = 1e-10; // Threshold to snap 9.9999999999 -> 10.0

// Clear
pub fn handle_clear(state: &mut State, all: bool) {
    if all || state.clear_mode == ClearMode::All {
        state.display = "0".into();
        state.current.clear();
        state.formula.clear();
        state.repeat_op = None;
        state.repeat_rhs = None;
        state.just_evaluated = false;
        state.clear_mode = ClearMode::All;
    } else {
        state.current.clear();
        state.display = "0".into();
        state.clear_mode = ClearMode::All;
    }
}

// Plus / Minus
pub fn handle_plus_minus(state: &mut State) {
    let target = if state.current.is_empty() {
        &mut state.display
    } else {
        &mut state.current
    };

    if let Ok(n) = target.parse::<f64>() {
        if n != 0.0 {
            let v = -n;
            *target = format_number(v);
            state.display = target.clone();
        }
    }
}

// Percentage
pub fn handle_percentage(state: &mut State) {
    let curr = state
        .current
        .parse::<f64>()
        .unwrap_or_else(|_| state.display.parse().unwrap_or(0.0));

    let base = last_number(&state.formula).unwrap_or(0.0);

    let result = if has_pending_op(&state.formula) {
        base * curr / 100.0
    } else {
        curr / 100.0
    };

    state.current = format_number(result);
    state.display = state.current.clone();
}

// Operator
pub fn handle_operator(state: &mut State, op: char) {
    commit_current(state);

    // FIXED: Evaluate chain immediately (Pocket Calculator style)
    // This prevents floating point noise from building up in long formulas
    if let Some(res) = eval_formula(&state.formula) {
        let rounded = round(res);
        state.display = format_number(rounded);
        state.formula = vec![Token::Number(rounded)];
    }

    // Replace operator if pressed twice
    if let Some(Token::Op(_)) = state.formula.last() {
        state.formula.pop();
    }

    state.formula.push(Token::Op(op));
    state.just_evaluated = false;
    state.repeat_op = None;
    state.repeat_rhs = None;
}

// Equals
pub fn handle_equals(state: &mut State) {
    commit_current(state);

    if !state.just_evaluated {
        if let Some(result) = eval_formula(&state.formula) {
            let final_val = round(result);

            if let Some(last_num) = last_number(&state.formula) {
                if let Some(Token::Op(op)) = state
                    .formula
                    .iter()
                    .rev()
                    .find(|t| matches!(t, Token::Op(_)))
                {
                    state.repeat_op = Some(*op);
                    state.repeat_rhs = Some(last_num);
                }
            }

            state.display = format_number(final_val);
            state.formula = vec![Token::Number(final_val)];
            state.just_evaluated = true;
        }
    } else {
        // Repeated equals
        if let (Some(op), Some(rhs)) = (state.repeat_op, state.repeat_rhs) {
            if let Ok(lhs) = state.display.parse::<f64>() {
                let v = apply_op(op, lhs, rhs);
                state.display = format_number(v);
                state.formula = vec![Token::Number(v)];
            }
        }
    }
}

// Number
pub fn handle_number(state: &mut State, c: char) {
    if state.just_evaluated {
        state.formula.clear();
        state.just_evaluated = false;
    }

    if state.current.len() >= MAX_DISPLAY_CHARS {
        return;
    }

    state.current.push(c);
    state.display = state.current.clone();
    state.clear_mode = ClearMode::Entry;
}

// Dot
pub fn handle_dot(state: &mut State) {
    if state.just_evaluated {
        state.formula.clear();
        state.just_evaluated = false;
    }

    if state.current.len() >= MAX_DISPLAY_CHARS {
        return;
    }

    if !state.current.contains('.') {
        if state.current.is_empty() {
            state.current.push('0');
        }
        state.current.push('.');
        state.display = state.current.clone();
    }
}

// Helpers
fn commit_current(state: &mut State) {
    if !state.current.is_empty() {
        if let Ok(n) = state.current.parse::<f64>() {
            state.formula.push(Token::Number(n));
        }
        state.current.clear();
    }
}

fn last_number(tokens: &[Token]) -> Option<f64> {
    tokens.iter().rev().find_map(|t| match t {
        Token::Number(n) => Some(*n),
        _ => None,
    })
}

fn has_pending_op(tokens: &[Token]) -> bool {
    tokens.iter().any(|t| matches!(t, Token::Op(_)))
}

fn precedence(op: char) -> u8 {
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        _ => 0,
    }
}

fn eval_formula(tokens: &[Token]) -> Option<f64> {
    if tokens.is_empty() {
        return None;
    }
    let mut values = Vec::new();
    let mut ops = Vec::new();

    for t in tokens {
        match *t {
            Token::Number(n) => values.push(n),
            Token::Op(op) => {
                while let Some(&top) = ops.last() {
                    if precedence(top) >= precedence(op) {
                        apply_top(&mut values, &mut ops)?;
                    } else {
                        break;
                    }
                }
                ops.push(op);
            }
        }
    }

    while !ops.is_empty() {
        apply_top(&mut values, &mut ops)?;
    }

    values.pop()
}

fn apply_top(values: &mut Vec<f64>, ops: &mut Vec<char>) -> Option<()> {
    let op = ops.pop()?;
    let r = values.pop()?;
    let l = values.pop()?;
    values.push(apply_op(op, l, r));
    Some(())
}

fn apply_op(op: char, l: f64, r: f64) -> f64 {
    let res = match op {
        '+' => l + r,
        '-' => l - r,
        '*' => l * r,
        '/' => {
            if r == 0.0 {
                f64::NAN
            } else {
                l / r
            }
        }
        _ => r,
    };
    round(res)
}

// FIXED: Snapping logic to fix 10^10 artifacts
fn round(x: f64) -> f64 {
    if x.is_nan() || x.is_infinite() {
        return x;
    }

    // 1. Snapping: If value is extremely close to an integer, it's noise.
    let nearest_int = x.round();
    if (x - nearest_int).abs() < SNAP_THRESHOLD {
        return nearest_int;
    }

    // 2. Precision rounding for decimals
    (x * 1e12).round() / 1e12
}

fn format_number(n: f64) -> String {
    if n.is_nan() {
        return "Error".into();
    }
    if n.is_infinite() {
        return "Overflow".into();
    }

    let abs_n = n.abs();

    // Large or very small → scientific notation
    if abs_n >= 1e15 || (abs_n < 1e-7 && abs_n != 0.0) {
        let s = format!("{:.12e}", n);
        return trim_scientific(&s);
    }

    // Standard fixed-point display
    let s = format!("{:.12}", n);
    let trimmed = s.trim_end_matches('0').trim_end_matches('.').to_string();

    if trimmed.len() > MAX_DISPLAY_CHARS {
        return format!("{:.6e}", n);
    }

    if trimmed == "-0" || trimmed == "" {
        "0".into()
    } else {
        trimmed
    }
}

fn trim_scientific(s: &str) -> String {
    if let Some(e_pos) = s.find('e') {
        let mantissa = &s[..e_pos];
        let exponent = &s[e_pos..];
        let trimmed_mantissa = mantissa.trim_end_matches('0').trim_end_matches('.');
        format!("{}{}", trimmed_mantissa, exponent)
    } else {
        s.to_string()
    }
}
