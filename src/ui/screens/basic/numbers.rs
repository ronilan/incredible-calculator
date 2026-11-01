use incredible::*;
use incredible_elements::FancyButton;

use crate::logic::*;
use crate::state::State;
use crate::ui::key::pressed;

static X: isize = 0;
static Y: isize = 7;

static NUM_X_STEP: isize = 7;
static NUM_Y_STEP: isize = 3;

pub fn build<'a>() -> Elements<State> {
    let elements = Elements::new();

    let numbers = [["9", "8", "7"], ["4", "5", "6"], ["1", "2", "3"]];

    // Number buttons (1-9)
    for (row_idx, row) in numbers.iter().enumerate() {
        for (col_idx, &text) in row.iter().enumerate() {
            let button = FancyButton::default();
            let val = text.chars().find(|c| !c.is_whitespace()).unwrap();
            button
                .x(X + col_idx as isize * NUM_X_STEP)
                .y(Y + row_idx as isize * NUM_Y_STEP)
                .text(text)
                .width(7)
                .key_combination(Some(KeyCombination::new(Key::Char(val), &[])))
                .style_handle("NumberButton");

            button
                .on_key(|el, state: &mut State, event| {
                    let stored = el.internal_state.key_combination.borrow().clone();
                    if let Some(stored) = stored {
                        if pressed(&stored, event) {
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

            elements.push(button);
        }
    }

    // Zero button
    let zero = FancyButton::default();
    zero.x(X)
        .y(Y + 3 * NUM_Y_STEP)
        .text("0")
        .width(14)
        .key_combination(Some(KeyCombination::new(Key::Char('0'), &[])))
        .style_handle("NumberButton");

    zero.on_key(|el, state: &mut State, event| {
        let stored = el.internal_state.key_combination.borrow().clone();
        if let Some(stored) = stored {
            if pressed(&stored, event) {
                if let Key::Char(c) = stored.key {
                    handle_number(state, c);
                }
            }
        }
    });
    zero.on_mouse(|el, state: &mut State, event| {
        if event.mouse == Mouse::Click {
            let stored = el.internal_state.key_combination.borrow().clone();
            if let Some(stored) = stored {
                if let Key::Char(c) = stored.key {
                    handle_number(state, c);
                }
            }
        }
    });
    elements.push(zero);

    // Decimal point button
    let dot = FancyButton::default();
    dot.x(X + 2 * NUM_X_STEP)
        .y(Y + 3 * NUM_Y_STEP)
        .text(".")
        .width(7)
        .key_combination(Some(KeyCombination::new(Key::Char('.'), &[])))
        .style_handle("NumberButton");

    dot.on_key(|el, state: &mut State, event| {
        let stored = el.internal_state.key_combination.borrow().clone();
        if let Some(stored) = stored {
            if pressed(&stored, event) {
                handle_dot(state);
            }
        }
    });
    dot.on_mouse(|_, state: &mut State, event| {
        if event.mouse == Mouse::Click {
            handle_dot(state);
        }
    });
    elements.push(dot);

    elements
}
