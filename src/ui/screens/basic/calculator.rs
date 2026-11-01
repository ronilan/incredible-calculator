use incredible::*;
use incredible_elements::Rectangle;

use crate::state::State;

use super::{additional, display, numbers, operations, title};

/// The basic calculator's exact grid size. The native window is sized to
/// match it, so the calculator fills the window without spare rows or columns.
pub(crate) const WIDTH: usize = 28;
pub(crate) const HEIGHT: usize = 19;

/// Builds the basic calculator. Unpositioned; the screen centers it.
pub(crate) fn build() -> Rectangle<State> {
    let calculator = Rectangle::new();
    calculator.width(WIDTH).height(HEIGHT);

    calculator.add(title::build());
    calculator.add(display::build());
    calculator.add_many(additional::build());
    calculator.add_many(numbers::build());
    calculator.add_many(operations::build());

    calculator
}
