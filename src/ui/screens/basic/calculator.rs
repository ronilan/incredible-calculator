use incredible::*;
use incredible_elements::Rectangle;

use crate::state::State;

use super::{additional, display, numbers, operations, title};

/// The basic calculator's exact grid size. The native window is sized to
/// match it, so the calculator fills the window without spare rows or columns.
pub(crate) const WIDTH: usize = 28;
pub(crate) const HEIGHT: usize = 19;

// Keypad grid. Buttons are built unpositioned by the modules above and placed
// here, so all layout lives in one file.
const COLUMN_X: isize = 0;
const COLUMN_STEP: isize = 7;
const ROW_STEP: isize = 3;
const ADDITIONAL_Y: isize = 4;
const NUMBERS_Y: isize = 7;
const OPERATIONS_X: isize = 21;
const OPERATIONS_Y: isize = 4;

/// Builds the basic calculator. Unpositioned; the screen centers it.
pub(crate) fn build() -> Rectangle<State> {
    let calculator = Rectangle::new();
    calculator.width(WIDTH).height(HEIGHT);

    calculator.add(title::build());
    calculator.add(display::build());

    // Additional row (AC, ±, %)
    let clear = additional::build_clear();
    clear.x(COLUMN_X).y(ADDITIONAL_Y);
    calculator.add(clear);

    let sign = additional::build_sign();
    sign.x(COLUMN_X + COLUMN_STEP).y(ADDITIONAL_Y);
    calculator.add(sign);

    let percentage = additional::build_percentage();
    percentage.x(COLUMN_X + 2 * COLUMN_STEP).y(ADDITIONAL_Y);
    calculator.add(percentage);

    // Number rows (1-9)
    let numbers_grid = [["9", "8", "7"], ["4", "5", "6"], ["1", "2", "3"]];
    for (row, digits) in numbers_grid.iter().enumerate() {
        for (column, text) in digits.iter().enumerate() {
            let button = numbers::build_digit(text);
            button
                .x(COLUMN_X + column as isize * COLUMN_STEP)
                .y(NUMBERS_Y + row as isize * ROW_STEP);
            calculator.add(button);
        }
    }

    // Zero and decimal point
    let zero = numbers::build_zero();
    zero.x(COLUMN_X).y(NUMBERS_Y + 3 * ROW_STEP);
    calculator.add(zero);

    let dot = numbers::build_dot();
    dot.x(COLUMN_X + 2 * COLUMN_STEP)
        .y(NUMBERS_Y + 3 * ROW_STEP);
    calculator.add(dot);

    // Operator column (÷, ×, -, +) and equals below it
    for (row, text) in ["÷", "×", "-", "+"].iter().enumerate() {
        let button = operations::build_operator(text);
        button
            .x(OPERATIONS_X)
            .y(OPERATIONS_Y + row as isize * ROW_STEP);
        calculator.add(button);
    }

    let equals = operations::build_equals();
    equals.x(OPERATIONS_X).y(OPERATIONS_Y + 4 * ROW_STEP);
    calculator.add(equals);

    calculator
}
