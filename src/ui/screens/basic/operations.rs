use incredible::*;
use incredible_elements::FancyButton;

use crate::logic::basic::*;
use crate::state::State;

const BUTTON_WIDTH: usize = 7;

/// Builds an operator button (÷, ×, -, +). Unpositioned; the calculator
/// places it in the operator column.
pub(crate) fn build_operator(text: &str) -> FancyButton<State> {
    let val = match text {
        "÷" => '/',
        "×" => '*',
        "-" => '-',
        "+" => '+',
        _ => unreachable!(),
    };

    let button = operation_button(text);
    button.key_combination(Some(KeyCombination::new(Key::Char(val), &[])));

    button
        .on_key(|el, state: &mut State, event| {
            let stored = el.internal_state.key_combination.borrow().clone();
            if let Some(stored) = stored {
                if stored.matches(event) {
                    if let Key::Char(c) = stored.key {
                        handle_operator(state, c);
                    }
                }
            }
        })
        .on_mouse(|el, state: &mut State, event| {
            if event.mouse == Mouse::Click {
                let stored = el.internal_state.key_combination.borrow().clone();
                if let Some(stored) = stored {
                    if let Key::Char(c) = stored.key {
                        handle_operator(state, c);
                    }
                }
            }
        });
    button
}

/// Builds the equals button. Unpositioned; the calculator places it below the
/// operator column.
pub(crate) fn build_equals() -> FancyButton<State> {
    let button = operation_button("=");
    button.key_combination(Some(KeyCombination::new(Key::Char('='), &[])));

    button
        .on_key(|el, state: &mut State, event| {
            let stored = el.internal_state.key_combination.borrow().clone();
            let mut matched = false;
            if let Some(stored) = stored {
                if stored.matches(event) {
                    matched = true;
                }
            }
            if matched || event.key == Key::Enter {
                handle_equals(state);
            }
        })
        .on_mouse(|_, state: &mut State, event| {
            if event.mouse == Mouse::Click {
                handle_equals(state);
            }
        });
    button
}

fn operation_button(text: &str) -> FancyButton<State> {
    let button = FancyButton::default();
    button
        .text(text)
        .width(BUTTON_WIDTH)
        .style_handle("OperationButton");
    button
}
