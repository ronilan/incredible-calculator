use incredible::*;
use incredible_elements::FancyButton;

use crate::logic::*;
use crate::state::{ClearMode, State};
use crate::ui::key::pressed;

static X: isize = 0;
static Y: isize = 4;

static ADD_X_STEP: isize = 7;

pub fn build<'a>() -> Elements<State> {
    let elements = Elements::new();

    let additional = ["AC", "±", "%"];

    // Additional buttons (AC, ±, %)
    for (i, &text) in additional.iter().enumerate() {
        let button = FancyButton::default();
        button
            .x(X + i as isize * ADD_X_STEP)
            .y(Y)
            .text(text)
            .width(7)
            .style_handle("AdditionalButton");

        if text.contains('C') {
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
        } else if text.contains('%') {
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
        } else if text.contains('±') {
            button.key_combination(Some(KeyCombination::new(Key::Char('s'), &[KeyMod::Ctrl])));

            button
                .on_key(|el, state: &mut State, event| {
                    let stored = el.internal_state.key_combination.borrow().clone();
                    if let Some(stored) = stored {
                        if pressed(&stored, event) {
                            handle_plus_minus(state);
                        }
                    }
                })
                .on_mouse(|_, state: &mut State, event| {
                    if event.mouse == Mouse::Click {
                        handle_plus_minus(state);
                    }
                });
        }

        elements.push(button);
    }

    elements
}
