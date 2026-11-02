use crate::state::State;
use incredible::*;
use incredible_elements::Rectangle;

use super::{buttons, display, title};

/// The scientific calculator's exact grid size. The native window is sized to
/// match it, so the calculator fills the window without spare rows or columns.
pub(crate) const WIDTH: usize = 72;
pub(crate) const HEIGHT: usize = 20;

// Keypad grid. Buttons are built unpositioned by `buttons` and placed here,
// so all layout lives in one file.
const COLUMN_X: isize = 0;
const COLUMN_STEP: isize = 9;
const ROW_Y: isize = 5;
const ROW_STEP: isize = 3;

// Column that holds the first digit (7, 4, 1).
const NUMBER_COLUMN: isize = 4;
// Row that holds the first digit row (7, 8, 9).
const NUMBER_ROW: isize = 1;

/// Builds the scientific calculator. Unpositioned; the screen centers it.
pub(crate) fn build() -> Rectangle<State> {
    let calculator = Rectangle::new();
    // As large as the content, so the background is centered with it:
    // title (1 row) + display (4 rows) + five button rows (3 rows each).
    calculator.width(WIDTH).height(HEIGHT);

    calculator.add(title::build());
    calculator.add(display::build());

    // Left column: the angle-mode pair and the extra values.
    let angle = buttons::build_angle_buttons();
    angle.x(COLUMN_X).y(ROW_Y);
    calculator.add(angle);

    let inverse = buttons::build_inv_button();
    inverse.x(COLUMN_X).y(ROW_Y + 2 * ROW_STEP);
    calculator.add(inverse);

    let rand = buttons::build_rand_button();
    rand.x(COLUMN_X).y(ROW_Y + 3 * ROW_STEP);
    calculator.add(rand);

    let ans = buttons::build_ans_button();
    ans.x(COLUMN_X).y(ROW_Y + 4 * ROW_STEP);
    calculator.add(ans);

    // Row 1: parens, percent, delete, clear pair, divide.
    let open = buttons::build_op_button("(");
    open.x(COLUMN_X + COLUMN_STEP).y(ROW_Y);
    calculator.add(open);

    let close = buttons::build_op_button(")");
    close.x(COLUMN_X + 2 * COLUMN_STEP).y(ROW_Y);
    calculator.add(close);

    let percentage = buttons::build_percent_button();
    percentage.x(COLUMN_X + 3 * COLUMN_STEP).y(ROW_Y);
    calculator.add(percentage);

    let delete = buttons::build_delete_button();
    delete.x(COLUMN_X + 4 * COLUMN_STEP).y(ROW_Y);
    calculator.add(delete);

    let clear_entry = buttons::build_clear_button("C", false);
    clear_entry.x(COLUMN_X + 5 * COLUMN_STEP).y(ROW_Y);
    calculator.add(clear_entry);

    let clear_all = buttons::build_clear_button("AC", true);
    clear_all.x(COLUMN_X + 6 * COLUMN_STEP).y(ROW_Y);
    calculator.add(clear_all);

    let divide = buttons::build_op_button("÷");
    divide.x(COLUMN_X + 7 * COLUMN_STEP).y(ROW_Y);
    calculator.add(divide);

    // Row 2: factorial, sin, ln.
    let factorial = buttons::build_func_button("x!", "fact", Key::Char('!'), None);
    factorial.x(COLUMN_X + COLUMN_STEP).y(ROW_Y + ROW_STEP);
    calculator.add(factorial);

    let sin = buttons::build_func_button("sin", "sin", Key::Char('s'), Some("sin⁻¹"));
    sin.x(COLUMN_X + 2 * COLUMN_STEP).y(ROW_Y + ROW_STEP);
    calculator.add(sin);

    let ln = buttons::build_func_button("ln", "ln", Key::Char('n'), Some("eˣ"));
    ln.x(COLUMN_X + 3 * COLUMN_STEP).y(ROW_Y + ROW_STEP);
    calculator.add(ln);

    let multiply = buttons::build_op_button("×");
    multiply.x(COLUMN_X + 7 * COLUMN_STEP).y(ROW_Y + ROW_STEP);
    calculator.add(multiply);

    // Row 3: pi, cos, log.
    let pi = buttons::build_const_button("π", Key::Char('p'));
    pi.x(COLUMN_X + COLUMN_STEP).y(ROW_Y + 2 * ROW_STEP);
    calculator.add(pi);

    let cos = buttons::build_func_button("cos", "cos", Key::Char('c'), Some("cos⁻¹"));
    cos.x(COLUMN_X + 2 * COLUMN_STEP).y(ROW_Y + 2 * ROW_STEP);
    calculator.add(cos);

    let log = buttons::build_func_button("log", "log", Key::Char('l'), Some("10ˣ"));
    log.x(COLUMN_X + 3 * COLUMN_STEP).y(ROW_Y + 2 * ROW_STEP);
    calculator.add(log);

    let subtract = buttons::build_op_button("−");
    subtract
        .x(COLUMN_X + 7 * COLUMN_STEP)
        .y(ROW_Y + 2 * ROW_STEP);
    calculator.add(subtract);

    // Number rows (7-9, 4-6, 1-3)
    let number_rows = [["7", "8", "9"], ["4", "5", "6"], ["1", "2", "3"]];
    for (row, digits) in number_rows.iter().enumerate() {
        for (column, text) in digits.iter().enumerate() {
            let button = buttons::build_num_button(text);
            button
                .x(COLUMN_X + (NUMBER_COLUMN + column as isize) * COLUMN_STEP)
                .y(ROW_Y + (NUMBER_ROW + row as isize) * ROW_STEP);
            calculator.add(button);
        }
    }

    // Row 4: e, tan, sqrt, add.
    let e = buttons::build_const_button("e", Key::Char('e'));
    e.x(COLUMN_X + COLUMN_STEP).y(ROW_Y + 3 * ROW_STEP);
    calculator.add(e);

    let tan = buttons::build_func_button("tan", "tan", Key::Char('t'), Some("tan⁻¹"));
    tan.x(COLUMN_X + 2 * COLUMN_STEP).y(ROW_Y + 3 * ROW_STEP);
    calculator.add(tan);

    let sqrt = buttons::build_func_button("√", "sqrt", Key::Char('q'), Some("x²"));
    sqrt.x(COLUMN_X + 3 * COLUMN_STEP).y(ROW_Y + 3 * ROW_STEP);
    calculator.add(sqrt);

    let add = buttons::build_op_button("+");
    add.x(COLUMN_X + 7 * COLUMN_STEP).y(ROW_Y + 3 * ROW_STEP);
    calculator.add(add);

    // Row 5: reciprocal, EE, power, sign, zero, decimal, equals.
    let reciprocal = buttons::build_reciprocal_button();
    reciprocal.x(COLUMN_X + COLUMN_STEP).y(ROW_Y + 4 * ROW_STEP);
    calculator.add(reciprocal);

    let ee = buttons::build_op_button("EE");
    ee.x(COLUMN_X + 2 * COLUMN_STEP).y(ROW_Y + 4 * ROW_STEP);
    calculator.add(ee);

    let power = buttons::build_op_button("^");
    power.x(COLUMN_X + 3 * COLUMN_STEP).y(ROW_Y + 4 * ROW_STEP);
    calculator.add(power);

    let sign = buttons::build_sign_button();
    sign.x(COLUMN_X + 4 * COLUMN_STEP).y(ROW_Y + 4 * ROW_STEP);
    calculator.add(sign);

    let zero = buttons::build_num_button("0");
    zero.x(COLUMN_X + 5 * COLUMN_STEP).y(ROW_Y + 4 * ROW_STEP);
    calculator.add(zero);

    let decimal = buttons::build_decimal_button();
    decimal
        .x(COLUMN_X + 6 * COLUMN_STEP)
        .y(ROW_Y + 4 * ROW_STEP);
    calculator.add(decimal);

    let equals = buttons::build_equals_button();
    equals.x(COLUMN_X + 7 * COLUMN_STEP).y(ROW_Y + 4 * ROW_STEP);
    calculator.add(equals);

    calculator
}
