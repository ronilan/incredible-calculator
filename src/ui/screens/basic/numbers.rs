use incredible::*;
use incredible_elements::FancyButton;

use crate::logic::basic::*;
use crate::state::State;

/// Builds a digit button (1-9). Unpositioned; the calculator lays out the grid.
pub(crate) fn build_digit(text: &str) -> FancyButton<State> {
    let val = text.chars().find(|c| !c.is_whitespace()).unwrap();
    let button = number_button(text, 7);
    button.key_combination(Some(KeyCombination::new(Key::Char(val), &[])));
    bind_number(&button);
    button
}

/// Builds the wide zero button. Unpositioned; the calculator places it.
pub(crate) fn build_zero() -> FancyButton<State> {
    let button = number_button("0", 14);
    button.key_combination(Some(KeyCombination::new(Key::Char('0'), &[])));
    bind_number(&button);
    button
}

/// Builds the decimal point button. Unpositioned; the calculator places it.
pub(crate) fn build_dot() -> FancyButton<State> {
    let button = number_button(".", 7);
    button.key_combination(Some(KeyCombination::new(Key::Char('.'), &[])));
    button
        .on_key(|el, state: &mut State, event| {
            let stored = el.internal_state.key_combination.borrow().clone();
            if let Some(stored) = stored {
                if stored.matches(event) {
                    handle_dot(state);
                }
            }
        })
        .on_mouse(|_, state: &mut State, event| {
            if event.mouse == Mouse::Click {
                handle_dot(state);
            }
        });
    button
}

fn number_button(text: &str, width: usize) -> FancyButton<State> {
    let button = FancyButton::default();
    button.text(text).width(width).style_handle("NumberButton");
    button
}

fn bind_number(button: &FancyButton<State>) {
    button
        .on_key(|el, state: &mut State, event| {
            let stored = el.internal_state.key_combination.borrow().clone();
            if let Some(stored) = stored {
                if stored.matches(event) {
                    if let Key::Char(c) = stored.key {
                        handle_number(state, c);
                    }
                }
            }
        })
        .on_mouse(|el, state: &mut State, event| {
            if event.mouse == Mouse::Click {
                let stored = el.internal_state.key_combination.borrow().clone();
                if let Some(stored) = stored {
                    if let Key::Char(c) = stored.key {
                        handle_number(state, c);
                    }
                }
            }
        });
}
