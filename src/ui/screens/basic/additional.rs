use incredible::*;
use incredible_elements::FancyButton;

use crate::logic::basic::*;
use crate::state::{ClearMode, State};

// Keeps the clear button's label (AC/C) and digit width consistent with the
// rest of the keypad.
const BUTTON_WIDTH: usize = 7;

/// Builds the clear button. Unpositioned; the calculator places it.
pub(crate) fn build_clear() -> FancyButton<State> {
    let button = additional_button("AC");
    button.key_combination(Some(KeyCombination::new(Key::Delete, &[])));

    button
        .on_key(|el, state: &mut State, event| {
            let stored = el.internal_state.key_combination.borrow().clone();
            if let Some(stored) = stored {
                if event.key == stored.key {
                    // If display is 0 or formula is empty → AC
                    let all = state.display == "0" && state.current.is_empty();
                    handle_clear(state, all);
                }
            }
        })
        .on_mouse(|_, state: &mut State, event| {
            if event.mouse == Mouse::Click {
                // If display is 0 or formula is empty → AC
                let all = state.display == "0" && state.current.is_empty();
                handle_clear(state, all);
            }
        })
        .on_state(|el, state, _ev| {
            if state.clear_mode == ClearMode::All {
                el.text("AC");
            } else {
                el.text("C");
            }
        });
    button
}

/// Builds the sign toggle button. Unpositioned; the calculator places it.
pub(crate) fn build_sign() -> FancyButton<State> {
    let button = additional_button("±");
    button.key_combination(Some(KeyCombination::new(Key::Char('s'), &[KeyMod::Ctrl])));

    button
        .on_key(|el, state: &mut State, event| {
            let stored = el.internal_state.key_combination.borrow().clone();
            if let Some(stored) = stored {
                if stored.matches(event) {
                    handle_plus_minus(state);
                }
            }
        })
        .on_mouse(|_, state: &mut State, event| {
            if event.mouse == Mouse::Click {
                handle_plus_minus(state);
            }
        });
    button
}

/// Builds the percentage button. Unpositioned; the calculator places it.
pub(crate) fn build_percentage() -> FancyButton<State> {
    let button = additional_button("%");
    button.key_combination(Some(KeyCombination::new(Key::Char('%'), &[])));

    button
        .on_key(|el, state: &mut State, event| {
            let stored = el.internal_state.key_combination.borrow().clone();
            if let Some(stored) = stored {
                if event.key == stored.key {
                    handle_percentage(state);
                }
            }
        })
        .on_mouse(|_, state: &mut State, event| {
            if event.mouse == Mouse::Click {
                handle_percentage(state);
            }
        });
    button
}

fn additional_button(text: &str) -> FancyButton<State> {
    let button = FancyButton::default();
    button
        .text(text)
        .width(BUTTON_WIDTH)
        .style_handle("AdditionalButton");
    button
}
