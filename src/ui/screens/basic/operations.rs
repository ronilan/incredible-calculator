use incredible::*;
use incredible_elements::FancyButton;

use crate::logic::*;
use crate::state::State;
use crate::ui::key::pressed;

static X: isize = 21;
static Y: isize = 4;

static OP_Y_STEP: isize = 3;

pub fn build<'a>() -> Elements<State> {
    let elements = Elements::new();

    let operations = ["÷", "×", "-", "+"];

    // Operation buttons (÷, ×, -, +)
    for (i, &text) in operations.iter().enumerate() {
        let val = match text {
            "÷" => '/',
            "×" => '*',
            "-" => '-',
            "+" => '+',
            _ => unreachable!(),
        };

        let button = FancyButton::default();
        button
            .x(X)
            .y(Y + i as isize * OP_Y_STEP)
            .text(text)
            .width(7)
            .key_combination(Some(KeyCombination::new(Key::Char(val), &[])))
            .style_handle("OperationButton");

        button
            .on_key(|el, state: &mut State, event| {
                let stored = el.internal_state.key_combination.borrow().clone();
                if let Some(stored) = stored {
                    if pressed(&stored, event) {
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

        elements.push(button);
    }

    // Equals button
    let eq_button = FancyButton::default();
    eq_button
        .x(X)
        .y(Y + 4 * OP_Y_STEP)
        .text("=")
        .width(7)
        .key_combination(Some(KeyCombination::new(Key::Char('='), &[])))
        .style_handle("OperationButton");

    eq_button.on_key(|el, state: &mut State, event| {
        let stored = el.internal_state.key_combination.borrow().clone();
        let mut matched = false;
        if let Some(stored) = stored {
            if pressed(&stored, event) {
                matched = true;
            }
        }
        if matched || event.key == Key::Enter {
            handle_equals(state);
        }
    });
    eq_button.on_mouse(|_, state: &mut State, event| {
        if event.mouse == Mouse::Click {
            handle_equals(state);
        }
    });

    elements.push(eq_button);

    elements
}
