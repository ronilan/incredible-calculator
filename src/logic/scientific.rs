use crate::state::{ClearMode, State};

const MAX_DISPLAY_CHARS: usize = 30;
const SNAP_THRESHOLD: f64 = 1e-10;

fn factorial(n: i64) -> f64 {
    if n < 0 {
        return f64::NAN;
    }
    if n == 0 || n == 1 {
        return 1.0;
    }
    let mut res: f64 = 1.0;
    let mut i: i64 = 2;
    while i <= n {
        res *= i as f64;
        i += 1;
    }
    res
}

/// Selects an angle mode directly, for the `Deg` / `Rad` push buttons.
pub fn set_angle_mode(state: &mut State, radian: bool) {
    state.is_radian = radian;
}

pub fn toggle_inverse(state: &mut State) {
    state.is_inverse = !state.is_inverse;
}

/// While the main display holds an expression being typed, the history line
/// shows the previous answer instead of the last evaluated expression.
fn show_previous_answer(state: &mut State) {
    state.history = format!("Ans = {}", format_number(state.last_ans));
}

/// `AC`: starts over, including the stored answer that `Ans` refers to.
pub fn handle_clear_all(state: &mut State) {
    state.expression = "0".into();
    state.display = "0".into();
    state.history = String::new();
    state.should_reset = false;
    state.last_ans = 0.0;
    state.clear_mode = ClearMode::All;
}

/// `C`: clears the value currently being typed and leaves the rest of the
/// expression alone, so `12` becomes `0` and `12 + 34` becomes `12 + 0`.
/// When the expression ends in an operator there is no value yet, and `C`
/// does nothing.
pub fn handle_clear_entry(state: &mut State) {
    if state.expression == "Error" {
        state.expression = "0".into();
        state.display = "0".into();
        state.should_reset = false;
        state.clear_mode = ClearMode::Entry;
        return;
    }

    // After `=` the value on the display is the result; clear it.
    if state.should_reset {
        state.expression = "0".into();
        state.display = "0".into();
        state.should_reset = false;
        state.clear_mode = ClearMode::Entry;
        return;
    }

    let trimmed = state.expression.trim_end();
    let prefix = trimmed.trim_end_matches(|c: char| c.is_ascii_digit() || c == '.');
    if prefix.len() == trimmed.len() {
        // Ends in an operator, parenthesis, or symbol: no value to clear.
        return;
    }

    state.expression = format!("{prefix}0");
    state.display = state.expression.clone();
    show_previous_answer(state);
    state.clear_mode = ClearMode::Entry;
}

/// Names that are inserted with an opening parenthesis and drawn in the thin
/// font. `⌫` removes each name together with its `(`, because a bare name
/// would be painted in tall letters.
const CALL_NAMES: [&str; 9] = [
    "asin", "acos", "atan", "fact", "sin", "cos", "tan", "log", "ln",
];

/// `⌫`: removes the last visible character. Operators are stored with
/// surrounding spaces, which the display hides, so those are skipped.
/// A function call (`cos(`) is removed as one unit.
pub fn handle_delete(state: &mut State) {
    if state.expression == "Error" {
        state.expression = "0".into();
        state.display = "0".into();
        state.should_reset = false;
        return;
    }

    // A result is not being edited; leave it alone until a new input starts.
    if state.should_reset {
        return;
    }

    let mut trimmed = state.expression.trim_end().to_string();

    // `Ans` is a single token, so one press removes the whole word.
    let removed_ans = trimmed.strip_suffix("Ans").is_some_and(|prefix| {
        !prefix
            .chars()
            .next_back()
            .is_some_and(|ch| ch.is_ascii_alphanumeric())
    });
    if removed_ans {
        trimmed.truncate(trimmed.len() - "Ans".len());
    } else {
        let Some(last) = trimmed.pop() else {
            return;
        };

        if last == '(' {
            let name = CALL_NAMES.iter().find(|name| {
                let Some(start) = trimmed.len().checked_sub(name.len()) else {
                    return false;
                };
                trimmed.ends_with(**name)
                    && !trimmed[..start]
                        .chars()
                        .next_back()
                        .is_some_and(|ch| ch.is_ascii_alphabetic())
            });

            if let Some(name) = name {
                trimmed.truncate(trimmed.len() - name.len());
            } else if trimmed.ends_with('√') {
                trimmed.pop();
            }
        }
    }

    let trimmed = trimmed.trim_end();
    state.expression = if trimmed.is_empty() {
        "0".into()
    } else {
        trimmed.to_string()
    };
    state.display = state.expression.clone();
    show_previous_answer(state);
    state.clear_mode = ClearMode::Entry;
}

pub fn handle_number(state: &mut State, c: char) {
    if state.expression == "0" || state.should_reset {
        state.expression = if c == '.' {
            "0.".to_string()
        } else {
            c.to_string()
        };
        state.should_reset = false;
    } else {
        if c == '.' {
            let mut has_dot = false;
            let bytes = state.expression.as_bytes();
            let mut i = bytes.len() as i32 - 1;
            while i >= 0 {
                let ch = bytes[i as usize] as char;
                if ch == ' '
                    || ch == '('
                    || ch == '+'
                    || ch == '-'
                    || ch == '×'
                    || ch == '÷'
                    || ch == '^'
                {
                    break;
                }
                if ch == '.' {
                    has_dot = true;
                    break;
                }
                i -= 1;
            }
            if has_dot {
                return;
            }
        }
        state.expression.push(c);
    }
    state.display = state.expression.clone();
    show_previous_answer(state);
    state.clear_mode = ClearMode::Entry;
}

pub fn handle_decimal(state: &mut State) {
    handle_number(state, '.');
}

pub fn input_op(state: &mut State, op: &str) {
    state.should_reset = false;
    if state.expression == "Error" {
        state.expression = "0".into();
    }
    state.expression.push(' ');
    state.expression.push_str(op);
    state.expression.push(' ');
    state.display = state.expression.clone();
    show_previous_answer(state);
}

/// Applies a postfix square to the expression, e.g. `5` becomes `5 ^ 2`.
pub fn input_square(state: &mut State) {
    state.should_reset = false;
    if state.expression == "Error" {
        state.expression = "0".into();
    }
    state.expression.push_str(" ^ 2");
    state.display = state.expression.clone();
    show_previous_answer(state);
}

/// Starts an nth root: `8` becomes `8 ^ ( 1 / ` and the user enters the
/// degree. The closing parenthesis is supplied by [`calculate`].
pub fn input_root(state: &mut State) {
    state.should_reset = false;
    if state.expression == "Error" {
        state.expression = "0".into();
    }
    state.expression.push_str(" ^ ( 1 / ");
    state.display = state.expression.clone();
    show_previous_answer(state);
}

pub fn input_paren(state: &mut State, paren: char) {
    state.should_reset = false;
    if state.expression == "0" || state.should_reset {
        state.expression = paren.to_string();
        state.should_reset = false;
    } else {
        state.expression.push(paren);
    }
    state.display = state.expression.clone();
    show_previous_answer(state);
}

pub fn input_const(state: &mut State, c: &str) {
    if state.expression == "0" || state.should_reset {
        state.expression = c.to_string();
        state.should_reset = false;
    } else {
        state.expression.push_str(c);
    }
    state.display = state.expression.clone();
    show_previous_answer(state);
}

/// Inserts `Ans` as a symbolic value, the way Google's calculator does.
///
/// A result on the display is replaced by `Ans`; after a value an implicit
/// multiplication is added (`6` becomes `6 × Ans`); after an operator it is
/// appended directly (`6 − ` becomes `6 − Ans`).
pub fn input_ans(state: &mut State) {
    if state.expression == "Error" {
        state.expression = "0".into();
    }

    if state.should_reset || state.expression == "0" {
        state.expression = "Ans".into();
    } else if ends_with_value(&state.expression) {
        state.expression.push_str(" × Ans");
    } else {
        state.expression.push_str("Ans");
    }

    state.should_reset = false;
    state.display = state.expression.clone();
    show_previous_answer(state);
}

/// Replaces the expression with text pasted from the system clipboard.
///
/// A paste is treated as a fresh expression rather than an insertion: line
/// feeds and other whitespace are dropped, common notation is normalised
/// (`*` to `×`, `pi` to `π`, `sqrt` to `√`, postfix `!` to `fact(...)`), and
/// characters the expression editor cannot represent are discarded. Pasting
/// only whitespace or unsupported characters leaves the current expression
/// alone.
pub fn paste_expression(state: &mut State, text: &str) {
    let cleaned = clean_pasted_expression(text);
    if cleaned.is_empty() {
        return;
    }

    state.expression = cleaned;
    state.display = state.expression.clone();
    state.should_reset = false;
    show_previous_answer(state);
}

/// Whether the expression currently ends in an operand, in which case another
/// value needs an explicit multiplication sign.
fn ends_with_value(expr: &str) -> bool {
    match expr.trim_end().chars().last() {
        Some(ch) => !matches!(ch, '+' | '-' | '−' | '*' | '×' | '/' | '÷' | '^' | '('),
        None => false,
    }
}

/// Byte range of the numeric literal at the end of `expr`, if there is one.
fn trailing_number(expr: &str) -> Option<(usize, usize)> {
    let trimmed = expr.trim_end();
    let start = trimmed
        .trim_end_matches(|c: char| c.is_ascii_digit() || c == '.')
        .len();
    if start == trimmed.len() {
        None
    } else {
        Some((start, trimmed.len()))
    }
}

/// `±` toggles the sign of the value being typed by adding or removing a
/// unary minus in front of it.
pub fn input_sign(state: &mut State) {
    if state.expression == "Error" {
        return;
    }
    let Some((start, _)) = trailing_number(&state.expression) else {
        return;
    };

    let trimmed = state.expression.trim_end();
    let prefix = &trimmed[..start];
    let number = &trimmed[start..];
    // `0` stays `0`, matching the basic calculator.
    if number.parse::<f64>().map(|n| n == 0.0).unwrap_or(true) {
        return;
    }

    state.expression = match prefix.strip_suffix('-') {
        // A unary minus: remove it. A binary minus keeps its preceding space.
        Some(unsigned)
            if unsigned.is_empty()
                || matches!(
                    unsigned.chars().last(),
                    Some(' ' | '(' | '+' | '−' | '×' | '÷' | '^')
                ) =>
        {
            format!("{unsigned}{number}")
        }
        _ => format!("{prefix}-{number}"),
    };
    state.display = state.expression.clone();
    if !state.should_reset {
        show_previous_answer(state);
    }
}

/// `%` turns the value being typed into hundredths.
pub fn input_percent(state: &mut State) {
    if state.expression == "Error" {
        return;
    }
    let Some((start, end)) = trailing_number(&state.expression) else {
        return;
    };

    let value = state.expression[start..end].parse::<f64>().unwrap_or(0.0);
    state.expression = format!(
        "{}{}",
        &state.expression[..start],
        format_number(value / 100.0)
    );
    state.display = state.expression.clone();
    if !state.should_reset {
        show_previous_answer(state);
    }
}

/// `1/x` replaces the value being typed with its reciprocal, so `8` becomes
/// `0.125`. Constants, `Ans` and closed groups keep their symbol and are
/// wrapped (`π` becomes `1/(π)`). With no value yet the reciprocal is opened
/// (`12 + ` becomes `12 + 1/(`) for the value typed next.
pub fn input_reciprocal(state: &mut State) {
    if state.expression == "Error" {
        state.expression = "0".into();
        state.display = "0".into();
        state.should_reset = false;
    }

    // The untouched zero is "no value yet", not a value to invert.
    let no_value = state.expression == "0" && !state.should_reset;

    if !no_value {
        if let Some((start, end)) = trailing_number(&state.expression) {
            let value = state.expression[start..end].parse::<f64>().unwrap_or(0.0);
            if value != 0.0 {
                state.expression = format!(
                    "{}{}",
                    &state.expression[..start],
                    format_number(1.0 / value)
                );
                state.should_reset = false;
                state.display = state.expression.clone();
                show_previous_answer(state);
                return;
            }
        }

        if ends_with_value(&state.expression) {
            if let Some(start) = trailing_value_start(&state.expression) {
                let trimmed = state.expression.trim_end();
                state.expression = format!("{}1/({})", &trimmed[..start], &trimmed[start..]);
                state.should_reset = false;
                state.display = state.expression.clone();
                show_previous_answer(state);
                return;
            }
        }
    }

    if state.expression == "0" || state.should_reset {
        state.expression = "1/(".into();
    } else {
        state.expression.push_str("1/(");
    }
    state.should_reset = false;
    state.display = state.expression.clone();
    show_previous_answer(state);
}

/// A random value in `[0, 1)`, mixed from the clock so the calculator does
/// not need a random-number dependency.
fn random_fraction() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};

    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos() as u64)
        .unwrap_or(0);
    // SplitMix64 finalizer.
    let mut x = nanos;
    x = (x ^ (x >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    x ^= x >> 31;
    (x >> 11) as f64 / (1u64 << 53) as f64
}

/// Byte offset where the value at the end of the expression starts, if there
/// is one: a number (including `E` notation), a constant, `Ans`, or a closed
/// function/parenthesis group.
fn trailing_value_start(expr: &str) -> Option<usize> {
    let trimmed = expr.trim_end();
    if trimmed.is_empty() {
        return None;
    }

    let mut start = trimmed.len();

    if trimmed.ends_with(')') {
        // Walk back to the parenthesis that opened this group.
        let mut depth = 0;
        let mut open = None;
        for (i, ch) in trimmed.char_indices().rev() {
            match ch {
                ')' => depth += 1,
                '(' => {
                    depth -= 1;
                    if depth == 0 {
                        open = Some(i);
                        break;
                    }
                }
                _ => {}
            }
        }
        start = open?;

        // A function name directly before the parenthesis belongs to the call.
        while start > 0 {
            let ch = trimmed[..start].chars().next_back().unwrap();
            if ch.is_ascii_alphabetic() || ch == '√' {
                start -= ch.len_utf8();
            } else {
                break;
            }
        }
        return Some(start);
    }

    // A literal such as `12`, `0.5`, `2E3`, `π`, or `Ans`.
    let literal_start = start;
    while start > 0 {
        let ch = trimmed[..start].chars().next_back().unwrap();
        if ch.is_ascii_alphanumeric() || ch == '.' || ch == 'π' {
            start -= ch.len_utf8();
        } else {
            break;
        }
    }
    if start == literal_start {
        return None;
    }

    // Include the unary minus directly in front of the literal, so replacing
    // `-9` gives a positive random value rather than `-0.xxx`.
    if start > 0 {
        let before = &trimmed[..start];
        if before.ends_with('-') {
            let sign_at = start - 1;
            let unary = sign_at == 0
                || matches!(
                    trimmed[..sign_at].chars().next_back(),
                    Some(' ' | '(' | '+' | '−' | '×' | '÷' | '^')
                );
            if unary {
                start = sign_at;
            }
        }
    }

    Some(start)
}

/// A random value in `[0, 1)` with at most six decimals, so that two of them
/// and an operator fit the scientific display (`0.123456×0.654321` is 45 of
/// its 61 columns wide).
fn random_value() -> f64 {
    (random_fraction() * 1_000_000.0).round() / 1_000_000.0
}

/// `Rand` inserts a random value between `0` and `1` at the current position.
/// It replaces only the last value (`8 × 9` becomes `8 × 0.xxx`) and inserts
/// one when there is none yet (`8 × ` becomes `8 × 0.xxx`).
pub fn input_rand(state: &mut State) {
    let value = format_number(random_value());
    if state.expression == "Error" {
        state.expression = "0".into();
    }

    if state.should_reset {
        state.expression = value;
    } else if let Some(start) = trailing_value_start(&state.expression) {
        let trimmed = state.expression.trim_end();
        state.expression = format!("{}{}", &trimmed[..start], value);
    } else if state.expression == "0" {
        state.expression = value;
    } else {
        state.expression.push_str(&value);
    }

    state.should_reset = false;
    state.display = state.expression.clone();
    show_previous_answer(state);
}

pub fn input_func(state: &mut State, func: &str) {
    if func == "inv" {
        state.is_inverse = !state.is_inverse;
        return;
    }
    if func == "Ans" {
        input_ans(state);
        return;
    }
    if func == "fact" {
        if state.expression == "0" || state.should_reset {
            state.expression = "fact(".to_string();
            state.should_reset = false;
        } else {
            state.expression.push_str("fact(");
        }
        state.display = state.expression.clone();
        show_previous_answer(state);
        return;
    }

    let mut func_name = func.to_string();

    if state.is_inverse {
        match func {
            "sin" => func_name = "asin".to_string(),
            "cos" => func_name = "acos".to_string(),
            "tan" => func_name = "atan".to_string(),
            // Inverse of ln(x) is e^x; lowercase `e` is Euler's constant.
            "ln" => func_name = "e^".to_string(),
            // Inverse of log(x) is 10^x.
            "log" => func_name = "10^".to_string(),
            // Square is postfix, so it extends the expression instead of
            // opening a function call.
            "sqrt" => {
                input_square(state);
                return;
            }
            _ => {}
        }
    } else if func == "sqrt" {
        func_name = "√".to_string();
    }

    if state.expression == "0" || state.should_reset {
        state.expression = format!("{}(", func_name);
        state.should_reset = false;
    } else {
        state.expression.push_str(&format!("{}(", func_name));
    }
    state.display = state.expression.clone();
    show_previous_answer(state);
}

pub fn calculate(state: &mut State) {
    // Prefix functions and nth roots leave their parentheses open while the
    // user types; close them when evaluating.
    let raw = balanced(&state.expression);
    state.history = format!("{} =", raw);

    match eval_expression(&raw, state.is_radian, state.last_ans) {
        Ok(result) => {
            let rounded = round_number(result);
            if rounded.is_nan() || rounded.is_infinite() {
                state.expression = "Error".into();
                state.display = "Error".into();
            } else {
                state.last_ans = rounded;
                state.expression = format_number(rounded);
                state.display = state.expression.clone();
            }
        }
        Err(_) => {
            state.expression = "Error".into();
            state.display = "Error".into();
        }
    }
    state.should_reset = true;
}

fn eval_expression(expr: &str, is_radian: bool, last_ans: f64) -> Result<f64, ()> {
    let mut parsed = expr.to_string();
    parsed = parsed.replace('÷', "/");
    parsed = parsed.replace('×', "*");
    parsed = parsed.replace('−', "-");
    parsed = parsed.replace('π', &format!("({})", std::f64::consts::PI));
    parsed = parsed.replace('√', "sqrt");
    // Lowercase `e` is Euler's constant; uppercase `E` is the exponent
    // marker inserted by `EE`.
    parsed = parsed.replace('e', &format!("({})", std::f64::consts::E));
    parsed = parsed.replace('E', "*10^");
    // `Ans` stays symbolic while editing; it resolves to the last result here.
    parsed = parsed.replace("Ans", &format!("({})", last_ans));
    parsed = insert_implicit_multiplication(&parsed);
    parsed = evaluate_functions(&parsed, is_radian)?;
    simple_eval(&parsed)
}

/// Inserts an explicit `*` where two values are written next to each other.
///
/// The keypad lets these be typed without an operator: `4π`, `(2 + 3)4`,
/// `2sin(30)`, `4√(9)`. Calculators conventionally read that as
/// multiplication, and without the sign the parser would silently drop the
/// second value (`√(4π)` would become `√(4)`).
fn insert_implicit_multiplication(expr: &str) -> String {
    let mut out = String::with_capacity(expr.len());
    let mut prev: Option<char> = None;

    for ch in expr.chars() {
        if !ch.is_whitespace() {
            if let Some(prev) = prev {
                let after_number = prev.is_ascii_digit() || prev == '.';
                let after_group = prev == ')';
                let starts_group = ch == '(';
                let starts_function = ch.is_ascii_alphabetic();
                let starts_number = ch.is_ascii_digit() || ch == '.';

                let multiply = (after_number && (starts_group || starts_function))
                    || (after_group && (starts_group || starts_function || starts_number));
                if multiply {
                    out.push('*');
                }
            }
            prev = Some(ch);
        }
        out.push(ch);
    }

    out
}

/// Longest cleaned paste kept. The display shows only the tail, and a
/// clipboard may hold an entire document rather than a calculation.
const MAX_PASTE_CHARS: usize = 512;

/// Reduces pasted text to the characters and words the expression editor can
/// hold, keeping only the first part of an oversized paste.
fn clean_pasted_expression(text: &str) -> String {
    let mut out = String::new();
    let mut rest = text;

    while let Some(ch) = rest.chars().next() {
        if ch.is_whitespace() {
            rest = &rest[ch.len_utf8()..];
            continue;
        }

        if ch.is_ascii_alphabetic() {
            let end = rest
                .find(|next: char| !next.is_ascii_alphabetic())
                .unwrap_or(rest.len());
            let word = &rest[..end];

            if word.eq_ignore_ascii_case("ans") {
                out.push_str("Ans");
            } else if word.eq_ignore_ascii_case("pi") {
                out.push('π');
            } else if word.eq_ignore_ascii_case("sqrt") {
                out.push('√');
                open_missing_paren(&mut out, &rest[end..]);
            } else if is_function_name(word) {
                out.push_str(&word.to_ascii_lowercase());
                open_missing_paren(&mut out, &rest[end..]);
            } else if word == "e" {
                let marker = if is_exponent_marker(&out, &rest[end..]) {
                    'E'
                } else {
                    'e'
                };
                out.push(marker);
            } else if word == "E" {
                out.push('E');
            }

            rest = &rest[end..];
            continue;
        }

        if ch == '!' {
            wrap_trailing_factorial(&mut out);
            rest = &rest[ch.len_utf8()..];
            continue;
        }

        // A root sign is a prefix function like `sin`; without a parenthesis
        // the parser would read `√9` as the word `sqrt9`.
        if ch == '√' {
            out.push('√');
            open_missing_paren(&mut out, &rest[ch.len_utf8()..]);
            rest = &rest[ch.len_utf8()..];
            continue;
        }

        let mapped = match ch {
            '*' | '×' | '·' => '×',
            '/' | '÷' => '÷',
            '-' | '−' | '–' | '—' => '−',
            '%' => {
                // `%` is not part of the stored expression, so keep its
                // meaning: a value divided by a hundred. Without a value in
                // front of it there is nothing to divide, so it is dropped.
                if ends_with_value(&out) {
                    out.push_str("÷100");
                }
                rest = &rest[ch.len_utf8()..];
                continue;
            }
            'π' | '+' | '^' | '(' | ')' | '.' => ch,
            '0'..='9' => ch,
            _ => {
                rest = &rest[ch.len_utf8()..];
                continue;
            }
        };
        out.push(mapped);
        rest = &rest[ch.len_utf8()..];
    }

    if out.chars().count() > MAX_PASTE_CHARS {
        out = out.chars().take(MAX_PASTE_CHARS).collect();
    }
    out
}

/// Turns the value before a pasted `!` into a `fact(...)` call. The keypad
/// writes factorials in prefix form, so `5!` and `(2+3)!` paste as calls.
fn wrap_trailing_factorial(expr: &mut String) {
    let Some(start) = trailing_value_start(expr) else {
        return;
    };
    let value = expr[start..].to_string();
    expr.truncate(start);
    expr.push_str("fact(");
    expr.push_str(&value);
    expr.push(')');
}

fn is_function_name(word: &str) -> bool {
    FUNCTIONS.iter().any(|name| word.eq_ignore_ascii_case(name))
}

/// Functions and roots are typed in prefix form, so the keypad always opens a
/// parenthesis after them. Pasted shorthand such as `sin30`, `sqrt9` or `√9`
/// gets the same treatment unless a `(` already follows.
fn open_missing_paren(out: &mut String, rest: &str) {
    let has_paren = rest
        .chars()
        .find(|ch| !ch.is_whitespace())
        .is_some_and(|ch| ch == '(');
    if !has_paren {
        out.push('(');
    }
}

/// Treats `e` as the `EE` marker (`1e3`, `2e-3`) when it sits after a value
/// and before a number; otherwise it is Euler's constant.
fn is_exponent_marker(before: &str, after: &str) -> bool {
    let Some(prev) = before.chars().next_back() else {
        return false;
    };
    if !(prev.is_ascii_digit() || prev == '.' || prev == ')') {
        return false;
    }

    let mut chars = after.chars().peekable();
    if matches!(chars.peek(), Some('+') | Some('-') | Some('−')) {
        chars.next();
    }
    chars.peek().is_some_and(|next| next.is_ascii_digit())
}

/// Appends any missing closing parentheses so partially typed expressions
/// (for example `sin(30` or `8 ^ ( 1 / 3`) can be evaluated.
fn balanced(expr: &str) -> String {
    let mut open = 0i32;
    for ch in expr.chars() {
        match ch {
            '(' => open += 1,
            ')' => open -= 1,
            _ => {}
        }
    }

    let mut balanced = expr.to_string();
    for _ in 0..open.max(0) {
        balanced.push(')');
    }
    balanced
}

/// The functions the scientific calculator can evaluate. Longest names come
/// first, and `find_last_call` also rejects a name that is the tail of a
/// longer one, so `cos(` is never read as part of `acos(`.
const FUNCTIONS: [&str; 10] = [
    "fact", "sqrt", "asin", "acos", "atan", "log", "sin", "cos", "tan", "ln",
];

fn evaluate_functions(expr: &str, is_radian: bool) -> Result<String, ()> {
    let to_rad = if is_radian {
        1.0f64
    } else {
        std::f64::consts::PI / 180.0
    };
    let from_rad = if is_radian {
        1.0f64
    } else {
        180.0 / std::f64::consts::PI
    };

    let mut result = expr.to_string();

    // Work from the rightmost call inward: a call nested in an argument is
    // farther right, so it is resolved before its parent.
    while let Some((start, name)) = find_last_call(&result) {
        let open = start + name.len();
        let Some(close) = matching_paren(&result, open) else {
            break;
        };
        let Ok(n) = simple_eval(&result[open + 1..close]) else {
            break;
        };

        let value = match name {
            "fact" => factorial(n as i64),
            "sqrt" => n.sqrt(),
            "log" => n.log10(),
            "ln" => n.ln(),
            "sin" => (n * to_rad).sin(),
            "cos" => (n * to_rad).cos(),
            "tan" => (n * to_rad).tan(),
            "asin" => n.asin() * from_rad,
            "acos" => n.acos() * from_rad,
            "atan" => n.atan() * from_rad,
            _ => return Err(()),
        };

        result.replace_range(start..=close, &value.to_string());
    }

    Ok(result)
}

/// The rightmost function call that starts a name rather than ending one.
fn find_last_call(haystack: &str) -> Option<(usize, &'static str)> {
    let mut best: Option<(usize, &'static str)> = None;

    for name in FUNCTIONS {
        let needle = format!("{name}(");
        for (start, _) in haystack.match_indices(&needle) {
            let tail_of_a_name = haystack[..start]
                .chars()
                .next_back()
                .is_some_and(|ch| ch.is_ascii_alphanumeric());
            if tail_of_a_name {
                continue;
            }
            if best.is_none_or(|(best_start, _)| start > best_start) {
                best = Some((start, name));
            }
        }
    }

    best
}

/// Index of the `)` that closes the `(` at `open`.
fn matching_paren(text: &str, open: usize) -> Option<usize> {
    let mut depth = 0i32;

    for (offset, ch) in text[open..].char_indices() {
        match ch {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(open + offset);
                }
            }
            _ => {}
        }
    }

    None
}

fn simple_eval(expr: &str) -> Result<f64, ()> {
    let expr = expr.trim();
    if expr.is_empty() {
        return Err(());
    }
    if let Ok(n) = expr.parse::<f64>() {
        return Ok(n);
    }
    eval_shunting_yard(expr)
}

fn eval_shunting_yard(expr: &str) -> Result<f64, ()> {
    let mut tokens: Vec<String> = Vec::new();
    let mut i = 0;
    let chars: Vec<char> = expr.chars().collect();
    while i < chars.len() {
        let c = chars[i];
        if c.is_whitespace() {
            i += 1;
            continue;
        }
        if c.is_ascii_digit() || c == '.' {
            let mut num = String::new();
            while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                num.push(chars[i]);
                i += 1;
            }
            tokens.push(num);
            continue;
        }
        if c == '+' || c == '-' || c == '*' || c == '/' || c == '(' || c == ')' || c == '^' {
            tokens.push(c.to_string());
            i += 1;
            continue;
        }
        i += 1;
    }
    if tokens.is_empty() {
        return Err(());
    }
    let mut output: Vec<f64> = Vec::new();
    let mut ops: Vec<String> = Vec::new();
    fn precedence(op: &str) -> i32 {
        match op {
            "+" | "-" => 1,
            "*" | "/" => 2,
            "^" | "**" => 3,
            "u-" | "u+" => 4,
            _ => 0,
        }
    }
    fn right_assoc(op: &str) -> bool {
        op == "^" || op == "**" || op == "u-"
    }
    let mut expect_operand = true;
    for token in &tokens {
        if let Ok(n) = token.parse::<f64>() {
            output.push(n);
            expect_operand = false;
            continue;
        }
        if token == "(" {
            ops.push(token.clone());
            expect_operand = true;
            continue;
        }
        if token == ")" {
            while !ops.is_empty() && ops.last().unwrap() != "(" {
                apply_op(&mut output, &mut ops)?;
            }
            if !ops.is_empty() && ops.last().unwrap() == "(" {
                ops.pop();
            }
            expect_operand = false;
            continue;
        }
        // A sign where an operand is expected is unary: `2^-3`, `(-1.5)`.
        let op = if expect_operand && (token == "-" || token == "+") {
            if token == "-" { "u-" } else { "u+" }
        } else {
            token.as_str()
        };
        while !ops.is_empty() {
            let top = ops.last().unwrap().clone();
            if top == "(" {
                break;
            }
            if (!right_assoc(op) && precedence(&top) >= precedence(op))
                || (right_assoc(op) && precedence(&top) > precedence(op))
            {
                apply_op(&mut output, &mut ops)?;
            } else {
                break;
            }
        }
        ops.push(op.to_string());
        expect_operand = true;
    }
    while !ops.is_empty() {
        let top = ops.last().unwrap().clone();
        if top == "(" || top == ")" {
            ops.pop();
            continue;
        }
        apply_op(&mut output, &mut ops)?;
    }
    if output.is_empty() {
        Err(())
    } else {
        Ok(output[0])
    }
}

fn apply_op(output: &mut Vec<f64>, ops: &mut Vec<String>) -> Result<(), ()> {
    let op = ops.pop().ok_or(())?;
    if op == "u-" {
        let a = output.pop().ok_or(())?;
        output.push(-a);
        return Ok(());
    }
    if op == "u+" {
        return Ok(());
    }
    if op == "**" || op == "^" {
        let b = output.pop().ok_or(())?;
        let a = output.pop().ok_or(())?;
        output.push(a.powf(b));
        return Ok(());
    }
    let b = output.pop().ok_or(())?;
    let a = output.pop().ok_or(())?;
    match op.as_str() {
        "+" => output.push(a + b),
        "-" => output.push(a - b),
        "*" => output.push(a * b),
        "/" => {
            if b == 0.0 {
                output.push(f64::NAN);
            } else {
                output.push(a / b);
            }
        }
        _ => {}
    }
    Ok(())
}

fn round_number(x: f64) -> f64 {
    if x.is_nan() || x.is_infinite() {
        return x;
    }
    let nearest_int = x.round();
    if (x - nearest_int).abs() < SNAP_THRESHOLD {
        return nearest_int;
    }
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
    if abs_n >= 1e15 || (abs_n < 1e-7 && abs_n != 0.0) {
        let s = format!("{:.12e}", n).replace('e', "E");
        return trim_scientific(&s);
    }
    let s = format!("{:.12}", n);
    let trimmed = s.trim_end_matches('0').trim_end_matches('.').to_string();
    if trimmed.len() > MAX_DISPLAY_CHARS {
        return format!("{:.6e}", n).replace('e', "E");
    }
    if trimmed == "-0" || trimmed.is_empty() {
        "0".into()
    } else {
        trimmed
    }
}

fn trim_scientific(s: &str) -> String {
    if let Some(e_pos) = s.find('E') {
        let mantissa = &s[..e_pos];
        let exponent = &s[e_pos..];
        let trimmed_mantissa = mantissa.trim_end_matches('0').trim_end_matches('.');
        format!("{}{}", trimmed_mantissa, exponent)
    } else {
        s.to_string()
    }
}
