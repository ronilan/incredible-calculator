use crate::state::State;
use incredible::*;
use incredible_elements::Rectangle;

use super::{buttons, display, title};

/// The scientific calculator's exact grid size. The native window is sized to
/// match it, so the calculator fills the window without spare rows or columns.
pub(crate) const WIDTH: usize = 72;
pub(crate) const HEIGHT: usize = 20;

/// Builds the scientific calculator. Unpositioned; the screen centers it.
pub(crate) fn build() -> Rectangle<State> {
    let calculator = Rectangle::new();
    // As large as the content, so the background is centered with it:
    // title (1 row) + display (4 rows) + five button rows (3 rows each).
    calculator.width(WIDTH).height(HEIGHT);

    calculator.add(title::build());
    calculator.add(display::build());
    calculator.add_many(buttons::build());

    calculator
}
