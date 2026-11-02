use incredible::*;
use incredible_elements::{FancyButton, FancyButtonGroup};
use incredible_helpers_layout::Direction;

use crate::state::State;

static BTN_WIDTH: usize = 9;

/// Binds a plain key press to a button, matching the keypad's existing
/// combination-based handlers.
fn bind_key<F>(button: &FancyButton<State>, key: Key, action: F)
where
    F: Fn(&mut State) + 'static,
{
    button.key_combination(Some(KeyCombination::new(key, &[])));
    button.on_key(move |el, state: &mut State, event| {
        let stored = el.internal_state.key_combination.borrow().clone();
        if let Some(stored) = stored {
            if stored.matches(event) {
                action(state);
            }
        }
    });
}

/// `Deg` and `Rad` are a radio pair: `FancyButtonGroup` keeps exactly one of
/// them pressed, and clicking either one selects that angle mode.
/// Unpositioned; the calculator places the group.
pub(crate) fn build_angle_buttons() -> FancyButtonGroup<State> {
    let group = FancyButtonGroup::default();
    group
        .allow_unselect(false)
        .key_combination(None)
        .prev_combination(None)
        .next_combination(None)
        .add_item("Deg", "deg")
        .add_item("Rad", "rad");

    // The group styles its children itself; use the calculator's extra-key
    // style and the keypad's fixed width instead.
    for button in group.elements().cot::<FancyButton<State>>() {
        button.width(BTN_WIDTH).style_handle("AdditionalButton");
    }
    group.direction(Direction::Down);

    group
        .on_mouse(|el, state: &mut State, event| {
            if event.mouse == Mouse::Click {
                crate::logic::scientific::set_angle_mode(state, el.get_value() == "rad");
            }
        })
        .on_key(|_el, state, event| {
            if KeyCombination::new(Key::Char('d'), &[]).matches(event) {
                crate::logic::scientific::set_angle_mode(state, !state.is_radian);
            }
        })
        .on_state(|el, state, _ev| {
            // Re-apply on every update: showing the screen and changing the
            // theme both repaint the button shadows, which would otherwise
            // leave both angle buttons looking released.
            let active = if state.is_radian { "rad" } else { "deg" };
            el.value(active);
        });

    group
}

/// Unpositioned; the calculator places it.
pub(crate) fn build_inv_button() -> FancyButton<State> {
    let button = FancyButton::default();
    button
        .text("Inv")
        .width(BTN_WIDTH)
        .toggle_mode(true)
        .style_handle("AdditionalButton");
    bind_key(&button, Key::Char('i'), |state| {
        crate::logic::scientific::toggle_inverse(state);
    });
    button
        .on_mouse(|el, state: &mut State, event| {
            if event.mouse == Mouse::Click {
                crate::logic::scientific::toggle_inverse(state);
                el.pressed(state.is_inverse);
            }
        })
        .on_state(|el, state, _ev| {
            el.pressed(state.is_inverse);
        });
    button
}

/// Unpositioned; the calculator places it.
pub(crate) fn build_ans_button() -> FancyButton<State> {
    let button = FancyButton::default();
    button
        .text("Ans")
        .width(BTN_WIDTH)
        .style_handle("ScientificButton");
    bind_key(&button, Key::Char('a'), |state| {
        crate::logic::scientific::input_ans(state);
    });
    button.on_mouse(|_, state: &mut State, event| {
        if event.mouse == Mouse::Click {
            crate::logic::scientific::input_ans(state);
        }
    });
    button
}

/// Unpositioned; the calculator places it.
pub(crate) fn build_reciprocal_button() -> FancyButton<State> {
    let button = FancyButton::default();
    button
        .text("1/x")
        .width(BTN_WIDTH)
        .style_handle("ScientificButton");
    bind_key(&button, Key::Char('x'), |state| {
        crate::logic::scientific::input_reciprocal(state);
    });
    button.on_mouse(|_, state: &mut State, event| {
        if event.mouse == Mouse::Click {
            crate::logic::scientific::input_reciprocal(state);
        }
    });
    button
}

/// Unpositioned; the calculator places it.
pub(crate) fn build_num_button(text: &str) -> FancyButton<State> {
    let button = FancyButton::default();
    let val = text.chars().next().unwrap();
    button
        .text(text)
        .width(BTN_WIDTH)
        .key_combination(Some(KeyCombination::new(Key::Char(val), &[])))
        .style_handle("NumberButton");
    button
        .on_key(|el, state: &mut State, event| {
            let stored = el.internal_state.key_combination.borrow().clone();
            if let Some(stored) = stored {
                if stored.matches(event) {
                    if let Key::Char(c) = stored.key {
                        crate::logic::scientific::handle_number(state, c);
                    }
                }
            }
        })
        .on_mouse(|el, state: &mut State, event| {
            if event.mouse == Mouse::Click {
                let stored = el.internal_state.key_combination.borrow().clone();
                if let Some(stored) = stored {
                    if let Key::Char(c) = stored.key {
                        crate::logic::scientific::handle_number(state, c);
                    }
                }
            }
        });
    button
}

/// Unpositioned; the calculator places it.
pub(crate) fn build_decimal_button() -> FancyButton<State> {
    let button = FancyButton::default();
    button
        .text(".")
        .width(BTN_WIDTH)
        .key_combination(Some(KeyCombination::new(Key::Char('.'), &[])))
        .style_handle("NumberButton");
    button
        .on_key(|el, state: &mut State, event| {
            let stored = el.internal_state.key_combination.borrow().clone();
            if let Some(stored) = stored {
                if stored.matches(event) {
                    crate::logic::scientific::handle_decimal(state);
                }
            }
        })
        .on_mouse(|_, state: &mut State, event| {
            if event.mouse == Mouse::Click {
                crate::logic::scientific::handle_decimal(state);
            }
        });
    button
}

/// Unpositioned; the calculator places it.
pub(crate) fn build_op_button(text: &str) -> FancyButton<State> {
    let button = FancyButton::default();
    let style = match text {
        "÷" | "×" | "−" | "-" | "+" | "=" => "OperationButton",
        _ => "ScientificButton",
    };
    button.text(text).width(BTN_WIDTH).style_handle(style);

    // The power key becomes an nth-root key while Inv is active.
    if text == "^" {
        button.on_state(|el, state: &State, _ev| {
            el.text(if state.is_inverse { "ʸ√x" } else { "^" });
        });
    }
    if text == "÷" {
        button.key_combination(Some(KeyCombination::new(Key::Char('/'), &[])));
    } else if text == "×" {
        button.key_combination(Some(KeyCombination::new(Key::Char('*'), &[])));
    } else if text == "−" || text == "-" {
        button.key_combination(Some(KeyCombination::new(Key::Char('-'), &[])));
    } else if text == "+" {
        button.key_combination(Some(KeyCombination::new(Key::Char('+'), &[])));
    } else if text == "(" {
        button.key_combination(Some(KeyCombination::new(Key::Char('('), &[])));
    } else if text == ")" {
        button.key_combination(Some(KeyCombination::new(Key::Char(')'), &[])));
    } else if text == "^" {
        button.key_combination(Some(KeyCombination::new(Key::Char('^'), &[])));
    } else if text == "=" {
        button.key_combination(Some(KeyCombination::new(Key::Char('='), &[])));
    } else if text == "EE" {
        button.key_combination(Some(KeyCombination::new(Key::Char('E'), &[])));
    }
    let text_owned1 = text.to_string();
    let text_owned2 = text.to_string();
    button
        .on_key(move |el, state: &mut State, event| {
            let stored = el.internal_state.key_combination.borrow().clone();
            let mut matched = false;
            if let Some(stored) = stored {
                if stored.matches(event) {
                    matched = true;
                }
            }
            if matched {
                if text_owned1 == "=" {
                    crate::logic::scientific::calculate(state);
                } else if text_owned1 == "÷" {
                    crate::logic::scientific::input_op(state, "÷");
                } else if text_owned1 == "×" {
                    crate::logic::scientific::input_op(state, "×");
                } else if text_owned1 == "−" {
                    crate::logic::scientific::input_op(state, "−");
                } else if text_owned1 == "+" {
                    crate::logic::scientific::input_op(state, "+");
                } else if text_owned1 == "^" {
                    if state.is_inverse {
                        crate::logic::scientific::input_root(state);
                    } else {
                        crate::logic::scientific::input_op(state, "^");
                    }
                } else if text_owned1 == "(" {
                    crate::logic::scientific::input_paren(state, '(');
                } else if text_owned1 == ")" {
                    crate::logic::scientific::input_paren(state, ')');
                } else if text_owned1 == "EE" {
                    crate::logic::scientific::input_op(state, "E");
                }
            }
        })
        .on_mouse(move |_, state: &mut State, event| {
            if event.mouse == Mouse::Click {
                if text_owned2 == "=" {
                    crate::logic::scientific::calculate(state);
                } else if text_owned2 == "÷" {
                    crate::logic::scientific::input_op(state, "÷");
                } else if text_owned2 == "×" {
                    crate::logic::scientific::input_op(state, "×");
                } else if text_owned2 == "−" {
                    crate::logic::scientific::input_op(state, "−");
                } else if text_owned2 == "+" {
                    crate::logic::scientific::input_op(state, "+");
                } else if text_owned2 == "^" {
                    if state.is_inverse {
                        crate::logic::scientific::input_root(state);
                    } else {
                        crate::logic::scientific::input_op(state, "^");
                    }
                } else if text_owned2 == "(" {
                    crate::logic::scientific::input_paren(state, '(');
                } else if text_owned2 == ")" {
                    crate::logic::scientific::input_paren(state, ')');
                } else if text_owned2 == "EE" {
                    crate::logic::scientific::input_op(state, "E");
                }
            }
        });
    button
}

/// Unpositioned; the calculator places it.
pub(crate) fn build_func_button(
    text: &str,
    func: &str,
    key: Key,
    inverse_text: Option<&str>,
) -> FancyButton<State> {
    let button = FancyButton::default();
    button
        .text(text)
        .width(BTN_WIDTH)
        .style_handle("ScientificButton");
    let func_for_key = func.to_string();
    bind_key(&button, key, move |state| {
        crate::logic::scientific::input_func(state, &func_for_key);
    });
    let func_owned = func.to_string();
    button.on_mouse(move |_, state: &mut State, event| {
        if event.mouse == Mouse::Click {
            crate::logic::scientific::input_func(state, &func_owned);
        }
    });
    if let Some(inverse) = inverse_text {
        let normal = text.to_string();
        let inverse = inverse.to_string();
        button.on_state(move |el, state: &State, _ev| {
            el.text(if state.is_inverse { &inverse } else { &normal });
        });
    }
    button
}

/// Unpositioned; the calculator places it.
pub(crate) fn build_const_button(text: &str, key: Key) -> FancyButton<State> {
    let button = FancyButton::default();
    button
        .text(text)
        .width(BTN_WIDTH)
        .style_handle("ScientificButton");
    let text_for_key = text.to_string();
    bind_key(&button, key, move |state| {
        crate::logic::scientific::input_const(state, &text_for_key);
    });
    let text_str = text.to_string();
    button.on_mouse(move |_, state: &mut State, event| {
        if event.mouse == Mouse::Click {
            crate::logic::scientific::input_const(state, &text_str);
        }
    });
    button
}

/// Unpositioned; the calculator places it.
pub(crate) fn build_clear_button(text: &str, all: bool) -> FancyButton<State> {
    let button = FancyButton::default();
    button
        .text(text)
        .width(BTN_WIDTH)
        .style_handle("AdditionalButton");
    // `AC` is the full reset; `C` clears only the value being typed.
    if all {
        button.key_combination(Some(KeyCombination::new(Key::Escape, &[])));
    } else {
        button.key_combination(Some(KeyCombination::new(Key::Delete, &[KeyMod::Ctrl])));
    }
    button
        .on_key(move |el, state: &mut State, event| {
            let stored = el.internal_state.key_combination.borrow().clone();
            if let Some(stored) = stored {
                if stored.matches(event) {
                    if all {
                        crate::logic::scientific::handle_clear_all(state);
                    } else {
                        crate::logic::scientific::handle_clear_entry(state);
                    }
                }
            }
        })
        .on_mouse(move |_, state: &mut State, event| {
            if event.mouse == Mouse::Click {
                if all {
                    crate::logic::scientific::handle_clear_all(state);
                } else {
                    crate::logic::scientific::handle_clear_entry(state);
                }
            }
        });
    button
}

/// Unpositioned; the calculator places it.
pub(crate) fn build_delete_button() -> FancyButton<State> {
    let button = FancyButton::default();
    button
        .text("⌫")
        .width(BTN_WIDTH)
        .key_combination(Some(KeyCombination::new(Key::Delete, &[])))
        .style_handle("AdditionalButton");
    button
        .on_key(|el, state: &mut State, event| {
            let stored = el.internal_state.key_combination.borrow().clone();
            if let Some(stored) = stored {
                if stored.matches(event) {
                    crate::logic::scientific::handle_delete(state);
                }
            }
        })
        .on_mouse(|_, state: &mut State, event| {
            if event.mouse == Mouse::Click {
                crate::logic::scientific::handle_delete(state);
            }
        });
    button
}

/// Unpositioned; the calculator places it.
pub(crate) fn build_sign_button() -> FancyButton<State> {
    let button = FancyButton::default();
    button
        .text("±")
        .width(BTN_WIDTH)
        .key_combination(Some(KeyCombination::new(Key::Char('s'), &[KeyMod::Ctrl])))
        .style_handle("NumberButton");
    button
        .on_key(|el, state: &mut State, event| {
            let stored = el.internal_state.key_combination.borrow().clone();
            if let Some(stored) = stored {
                if stored.matches(event) {
                    crate::logic::scientific::input_sign(state);
                }
            }
        })
        .on_mouse(|_, state: &mut State, event| {
            if event.mouse == Mouse::Click {
                crate::logic::scientific::input_sign(state);
            }
        });
    button
}

/// Unpositioned; the calculator places it.
pub(crate) fn build_percent_button() -> FancyButton<State> {
    let button = FancyButton::default();
    button
        .text("%")
        .width(BTN_WIDTH)
        .key_combination(Some(KeyCombination::new(Key::Char('%'), &[])))
        .style_handle("ScientificButton");
    button
        .on_key(|el, state: &mut State, event| {
            let stored = el.internal_state.key_combination.borrow().clone();
            if let Some(stored) = stored {
                if stored.matches(event) {
                    crate::logic::scientific::input_percent(state);
                }
            }
        })
        .on_mouse(|_, state: &mut State, event| {
            if event.mouse == Mouse::Click {
                crate::logic::scientific::input_percent(state);
            }
        });
    button
}

/// Unpositioned; the calculator places it.
pub(crate) fn build_rand_button() -> FancyButton<State> {
    let button = FancyButton::default();
    button
        .text("Rand")
        .width(BTN_WIDTH)
        .key_combination(Some(KeyCombination::new(Key::Char('r'), &[])))
        .style_handle("ScientificButton");
    button
        .on_key(|el, state: &mut State, event| {
            let stored = el.internal_state.key_combination.borrow().clone();
            if let Some(stored) = stored {
                if stored.matches(event) {
                    crate::logic::scientific::input_rand(state);
                }
            }
        })
        .on_mouse(|_, state: &mut State, event| {
            if event.mouse == Mouse::Click {
                crate::logic::scientific::input_rand(state);
            }
        });
    button
}

/// Unpositioned; the calculator places it.
pub(crate) fn build_equals_button() -> FancyButton<State> {
    let button = FancyButton::default();
    button
        .text("=")
        .width(BTN_WIDTH)
        .key_combination(Some(KeyCombination::new(Key::Char('='), &[])))
        .style_handle("OperationButton");
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
                crate::logic::scientific::calculate(state);
            }
        })
        .on_mouse(|_, state: &mut State, event| {
            if event.mouse == Mouse::Click {
                crate::logic::scientific::calculate(state);
            }
        });
    button
}
