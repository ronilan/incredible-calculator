use incredible::*;
use incredible_elements::{FancyButton, FancyButtonGroup};
use incredible_helpers_layout::Direction;

use crate::state::State;

static X: isize = 0;
static Y: isize = 5;

static BTN_WIDTH: usize = 9;
static BTN_X_STEP: isize = 9;
static BTN_Y_STEP: isize = 3;

pub fn build<'a>() -> Elements<State> {
    let mut elements = Elements::new();

    // Left column: the angle-mode pair and the extra values.
    add_angle_buttons(&mut elements, X, Y);
    add_inv_button(&mut elements, X, Y + 2 * BTN_Y_STEP);
    add_rand_button(&mut elements, X, Y + 3 * BTN_Y_STEP);
    add_ans_button(&mut elements, X, Y + 4 * BTN_Y_STEP);

    // Row 1
    add_op_button(&mut elements, X + BTN_X_STEP, Y, "(");
    add_op_button(&mut elements, X + 2 * BTN_X_STEP, Y, ")");
    add_percent_button(&mut elements, X + 3 * BTN_X_STEP, Y);
    add_delete_button(&mut elements, X + 4 * BTN_X_STEP, Y);
    add_clear_button(&mut elements, X + 5 * BTN_X_STEP, Y, "C", false);
    add_clear_button(&mut elements, X + 6 * BTN_X_STEP, Y, "AC", true);
    add_op_button(&mut elements, X + 7 * BTN_X_STEP, Y, "÷");

    // Row 2
    add_func_button(
        &mut elements,
        X + BTN_X_STEP,
        Y + BTN_Y_STEP,
        "x!",
        "fact",
        Key::Char('!'),
        None,
    );
    add_func_button(
        &mut elements,
        X + 2 * BTN_X_STEP,
        Y + BTN_Y_STEP,
        "sin",
        "sin",
        Key::Char('s'),
        Some("sin⁻¹"),
    );
    add_func_button(
        &mut elements,
        X + 3 * BTN_X_STEP,
        Y + BTN_Y_STEP,
        "ln",
        "ln",
        Key::Char('n'),
        Some("eˣ"),
    );
    add_num_button(&mut elements, X + 4 * BTN_X_STEP, Y + BTN_Y_STEP, "7");
    add_num_button(&mut elements, X + 5 * BTN_X_STEP, Y + BTN_Y_STEP, "8");
    add_num_button(&mut elements, X + 6 * BTN_X_STEP, Y + BTN_Y_STEP, "9");
    add_op_button(&mut elements, X + 7 * BTN_X_STEP, Y + BTN_Y_STEP, "×");

    // Row 3
    add_const_button(
        &mut elements,
        X + BTN_X_STEP,
        Y + 2 * BTN_Y_STEP,
        "π",
        Key::Char('p'),
    );
    add_func_button(
        &mut elements,
        X + 2 * BTN_X_STEP,
        Y + 2 * BTN_Y_STEP,
        "cos",
        "cos",
        Key::Char('c'),
        Some("cos⁻¹"),
    );
    add_func_button(
        &mut elements,
        X + 3 * BTN_X_STEP,
        Y + 2 * BTN_Y_STEP,
        "log",
        "log",
        Key::Char('l'),
        Some("10ˣ"),
    );
    add_num_button(&mut elements, X + 4 * BTN_X_STEP, Y + 2 * BTN_Y_STEP, "4");
    add_num_button(&mut elements, X + 5 * BTN_X_STEP, Y + 2 * BTN_Y_STEP, "5");
    add_num_button(&mut elements, X + 6 * BTN_X_STEP, Y + 2 * BTN_Y_STEP, "6");
    add_op_button(&mut elements, X + 7 * BTN_X_STEP, Y + 2 * BTN_Y_STEP, "−");

    // Row 4
    add_const_button(
        &mut elements,
        X + BTN_X_STEP,
        Y + 3 * BTN_Y_STEP,
        "e",
        Key::Char('e'),
    );
    add_func_button(
        &mut elements,
        X + 2 * BTN_X_STEP,
        Y + 3 * BTN_Y_STEP,
        "tan",
        "tan",
        Key::Char('t'),
        Some("tan⁻¹"),
    );
    add_func_button(
        &mut elements,
        X + 3 * BTN_X_STEP,
        Y + 3 * BTN_Y_STEP,
        "√",
        "sqrt",
        Key::Char('q'),
        Some("x²"),
    );
    add_num_button(&mut elements, X + 4 * BTN_X_STEP, Y + 3 * BTN_Y_STEP, "1");
    add_num_button(&mut elements, X + 5 * BTN_X_STEP, Y + 3 * BTN_Y_STEP, "2");
    add_num_button(&mut elements, X + 6 * BTN_X_STEP, Y + 3 * BTN_Y_STEP, "3");
    add_op_button(&mut elements, X + 7 * BTN_X_STEP, Y + 3 * BTN_Y_STEP, "+");

    // Row 5
    add_reciprocal_button(&mut elements, X + BTN_X_STEP, Y + 4 * BTN_Y_STEP);
    add_op_button(&mut elements, X + 2 * BTN_X_STEP, Y + 4 * BTN_Y_STEP, "EE");
    add_op_button(&mut elements, X + 3 * BTN_X_STEP, Y + 4 * BTN_Y_STEP, "^");
    add_sign_button(&mut elements, X + 4 * BTN_X_STEP, Y + 4 * BTN_Y_STEP);
    add_num_button(&mut elements, X + 5 * BTN_X_STEP, Y + 4 * BTN_Y_STEP, "0");
    add_decimal_button(&mut elements, X + 6 * BTN_X_STEP, Y + 4 * BTN_Y_STEP);
    add_equals_button(&mut elements, X + 7 * BTN_X_STEP, Y + 4 * BTN_Y_STEP);

    elements
}

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
fn add_angle_buttons(elements: &mut Elements<State>, x: isize, y: isize) {
    let group = FancyButtonGroup::default();
    group
        .x(x)
        .y(y)
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
                crate::logic_scientific::set_angle_mode(state, el.get_value() == "rad");
            }
        })
        .on_key(|_el, state, event| {
            if KeyCombination::new(Key::Char('d'), &[]).matches(event) {
                crate::logic_scientific::set_angle_mode(state, !state.is_radian);
            }
        })
        .on_state(|el, state, _ev| {
            // Re-apply on every update: showing the screen and changing the
            // theme both repaint the button shadows, which would otherwise
            // leave both angle buttons looking released.
            let active = if state.is_radian { "rad" } else { "deg" };
            el.value(active);
        });

    elements.push(group);
}

fn add_inv_button(elements: &mut Elements<State>, x: isize, y: isize) {
    let button = FancyButton::default();
    button
        .x(x)
        .y(y)
        .text("Inv")
        .width(BTN_WIDTH)
        .toggle_mode(true)
        .style_handle("AdditionalButton");
    bind_key(&button, Key::Char('i'), |state| {
        crate::logic_scientific::toggle_inverse(state);
    });
    button
        .on_mouse(|el, state: &mut State, event| {
            if event.mouse == Mouse::Click {
                crate::logic_scientific::toggle_inverse(state);
                el.pressed(state.is_inverse);
            }
        })
        .on_state(|el, state, _ev| {
            el.pressed(state.is_inverse);
        });
    elements.push(button);
}

fn add_ans_button(elements: &mut Elements<State>, x: isize, y: isize) {
    let button = FancyButton::default();
    button
        .x(x)
        .y(y)
        .text("Ans")
        .width(BTN_WIDTH)
        .style_handle("ScientificButton");
    bind_key(&button, Key::Char('a'), |state| {
        crate::logic_scientific::input_ans(state);
    });
    button.on_mouse(|_, state: &mut State, event| {
        if event.mouse == Mouse::Click {
            crate::logic_scientific::input_ans(state);
        }
    });
    elements.push(button);
}

fn add_reciprocal_button(elements: &mut Elements<State>, x: isize, y: isize) {
    let button = FancyButton::default();
    button
        .x(x)
        .y(y)
        .text("1/x")
        .width(BTN_WIDTH)
        .style_handle("ScientificButton");
    bind_key(&button, Key::Char('x'), |state| {
        crate::logic_scientific::input_reciprocal(state);
    });
    button.on_mouse(|_, state: &mut State, event| {
        if event.mouse == Mouse::Click {
            crate::logic_scientific::input_reciprocal(state);
        }
    });
    elements.push(button);
}

fn add_num_button(elements: &mut Elements<State>, x: isize, y: isize, text: &str) {
    let button = FancyButton::default();
    let val = text.chars().next().unwrap();
    button
        .x(x)
        .y(y)
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
                        crate::logic_scientific::handle_number(state, c);
                    }
                }
            }
        })
        .on_mouse(|el, state: &mut State, event| {
            if event.mouse == Mouse::Click {
                let stored = el.internal_state.key_combination.borrow().clone();
                if let Some(stored) = stored {
                    if let Key::Char(c) = stored.key {
                        crate::logic_scientific::handle_number(state, c);
                    }
                }
            }
        });
    elements.push(button);
}

fn add_decimal_button(elements: &mut Elements<State>, x: isize, y: isize) {
    let button = FancyButton::default();
    button
        .x(x)
        .y(y)
        .text(".")
        .width(BTN_WIDTH)
        .key_combination(Some(KeyCombination::new(Key::Char('.'), &[])))
        .style_handle("NumberButton");
    button
        .on_key(|el, state: &mut State, event| {
            let stored = el.internal_state.key_combination.borrow().clone();
            if let Some(stored) = stored {
                if stored.matches(event) {
                    crate::logic_scientific::handle_decimal(state);
                }
            }
        })
        .on_mouse(|_, state: &mut State, event| {
            if event.mouse == Mouse::Click {
                crate::logic_scientific::handle_decimal(state);
            }
        });
    elements.push(button);
}

fn add_op_button(elements: &mut Elements<State>, x: isize, y: isize, text: &str) {
    let button = FancyButton::default();
    let style = match text {
        "÷" | "×" | "−" | "-" | "+" | "=" => "OperationButton",
        _ => "ScientificButton",
    };
    button
        .x(x)
        .y(y)
        .text(text)
        .width(BTN_WIDTH)
        .style_handle(style);

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
                    crate::logic_scientific::calculate(state);
                } else if text_owned1 == "÷" {
                    crate::logic_scientific::input_op(state, "÷");
                } else if text_owned1 == "×" {
                    crate::logic_scientific::input_op(state, "×");
                } else if text_owned1 == "−" {
                    crate::logic_scientific::input_op(state, "−");
                } else if text_owned1 == "+" {
                    crate::logic_scientific::input_op(state, "+");
                } else if text_owned1 == "^" {
                    if state.is_inverse {
                        crate::logic_scientific::input_root(state);
                    } else {
                        crate::logic_scientific::input_op(state, "^");
                    }
                } else if text_owned1 == "(" {
                    crate::logic_scientific::input_paren(state, '(');
                } else if text_owned1 == ")" {
                    crate::logic_scientific::input_paren(state, ')');
                } else if text_owned1 == "EE" {
                    crate::logic_scientific::input_op(state, "E");
                }
            }
        })
        .on_mouse(move |_, state: &mut State, event| {
            if event.mouse == Mouse::Click {
                if text_owned2 == "=" {
                    crate::logic_scientific::calculate(state);
                } else if text_owned2 == "÷" {
                    crate::logic_scientific::input_op(state, "÷");
                } else if text_owned2 == "×" {
                    crate::logic_scientific::input_op(state, "×");
                } else if text_owned2 == "−" {
                    crate::logic_scientific::input_op(state, "−");
                } else if text_owned2 == "+" {
                    crate::logic_scientific::input_op(state, "+");
                } else if text_owned2 == "^" {
                    if state.is_inverse {
                        crate::logic_scientific::input_root(state);
                    } else {
                        crate::logic_scientific::input_op(state, "^");
                    }
                } else if text_owned2 == "(" {
                    crate::logic_scientific::input_paren(state, '(');
                } else if text_owned2 == ")" {
                    crate::logic_scientific::input_paren(state, ')');
                } else if text_owned2 == "EE" {
                    crate::logic_scientific::input_op(state, "E");
                }
            }
        });
    elements.push(button);
}

fn add_func_button(
    elements: &mut Elements<State>,
    x: isize,
    y: isize,
    text: &str,
    func: &str,
    key: Key,
    inverse_text: Option<&str>,
) {
    let button = FancyButton::default();
    button
        .x(x)
        .y(y)
        .text(text)
        .width(BTN_WIDTH)
        .style_handle("ScientificButton");
    let func_for_key = func.to_string();
    bind_key(&button, key, move |state| {
        crate::logic_scientific::input_func(state, &func_for_key);
    });
    let func_owned = func.to_string();
    button.on_mouse(move |_, state: &mut State, event| {
        if event.mouse == Mouse::Click {
            crate::logic_scientific::input_func(state, &func_owned);
        }
    });
    if let Some(inverse) = inverse_text {
        let normal = text.to_string();
        let inverse = inverse.to_string();
        button.on_state(move |el, state: &State, _ev| {
            el.text(if state.is_inverse { &inverse } else { &normal });
        });
    }
    elements.push(button);
}

fn add_const_button(elements: &mut Elements<State>, x: isize, y: isize, text: &str, key: Key) {
    let button = FancyButton::default();
    button
        .x(x)
        .y(y)
        .text(text)
        .width(BTN_WIDTH)
        .style_handle("ScientificButton");
    let text_for_key = text.to_string();
    bind_key(&button, key, move |state| {
        crate::logic_scientific::input_const(state, &text_for_key);
    });
    let text_str = text.to_string();
    button.on_mouse(move |_, state: &mut State, event| {
        if event.mouse == Mouse::Click {
            crate::logic_scientific::input_const(state, &text_str);
        }
    });
    elements.push(button);
}

fn add_clear_button(elements: &mut Elements<State>, x: isize, y: isize, text: &str, all: bool) {
    let button = FancyButton::default();
    button
        .x(x)
        .y(y)
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
                        crate::logic_scientific::handle_clear_all(state);
                    } else {
                        crate::logic_scientific::handle_clear_entry(state);
                    }
                }
            }
        })
        .on_mouse(move |_, state: &mut State, event| {
            if event.mouse == Mouse::Click {
                if all {
                    crate::logic_scientific::handle_clear_all(state);
                } else {
                    crate::logic_scientific::handle_clear_entry(state);
                }
            }
        });
    elements.push(button);
}

fn add_delete_button(elements: &mut Elements<State>, x: isize, y: isize) {
    let button = FancyButton::default();
    button
        .x(x)
        .y(y)
        .text("⌫")
        .width(BTN_WIDTH)
        .key_combination(Some(KeyCombination::new(Key::Delete, &[])))
        .style_handle("AdditionalButton");
    button
        .on_key(|el, state: &mut State, event| {
            let stored = el.internal_state.key_combination.borrow().clone();
            if let Some(stored) = stored {
                if stored.matches(event) {
                    crate::logic_scientific::handle_delete(state);
                }
            }
        })
        .on_mouse(|_, state: &mut State, event| {
            if event.mouse == Mouse::Click {
                crate::logic_scientific::handle_delete(state);
            }
        });
    elements.push(button);
}

fn add_sign_button(elements: &mut Elements<State>, x: isize, y: isize) {
    let button = FancyButton::default();
    button
        .x(x)
        .y(y)
        .text("±")
        .width(BTN_WIDTH)
        .key_combination(Some(KeyCombination::new(Key::Char('s'), &[KeyMod::Ctrl])))
        .style_handle("NumberButton");
    button
        .on_key(|el, state: &mut State, event| {
            let stored = el.internal_state.key_combination.borrow().clone();
            if let Some(stored) = stored {
                if stored.matches(event) {
                    crate::logic_scientific::input_sign(state);
                }
            }
        })
        .on_mouse(|_, state: &mut State, event| {
            if event.mouse == Mouse::Click {
                crate::logic_scientific::input_sign(state);
            }
        });
    elements.push(button);
}

fn add_percent_button(elements: &mut Elements<State>, x: isize, y: isize) {
    let button = FancyButton::default();
    button
        .x(x)
        .y(y)
        .text("%")
        .width(BTN_WIDTH)
        .key_combination(Some(KeyCombination::new(Key::Char('%'), &[])))
        .style_handle("ScientificButton");
    button
        .on_key(|el, state: &mut State, event| {
            let stored = el.internal_state.key_combination.borrow().clone();
            if let Some(stored) = stored {
                if stored.matches(event) {
                    crate::logic_scientific::input_percent(state);
                }
            }
        })
        .on_mouse(|_, state: &mut State, event| {
            if event.mouse == Mouse::Click {
                crate::logic_scientific::input_percent(state);
            }
        });
    elements.push(button);
}

fn add_rand_button(elements: &mut Elements<State>, x: isize, y: isize) {
    let button = FancyButton::default();
    button
        .x(x)
        .y(y)
        .text("Rand")
        .width(BTN_WIDTH)
        .key_combination(Some(KeyCombination::new(Key::Char('r'), &[])))
        .style_handle("ScientificButton");
    button
        .on_key(|el, state: &mut State, event| {
            let stored = el.internal_state.key_combination.borrow().clone();
            if let Some(stored) = stored {
                if stored.matches(event) {
                    crate::logic_scientific::input_rand(state);
                }
            }
        })
        .on_mouse(|_, state: &mut State, event| {
            if event.mouse == Mouse::Click {
                crate::logic_scientific::input_rand(state);
            }
        });
    elements.push(button);
}

fn add_equals_button(elements: &mut Elements<State>, x: isize, y: isize) {
    let button = FancyButton::default();
    button
        .x(x)
        .y(y)
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
                crate::logic_scientific::calculate(state);
            }
        })
        .on_mouse(|_, state: &mut State, event| {
            if event.mouse == Mouse::Click {
                crate::logic_scientific::calculate(state);
            }
        });
    elements.push(button);
}
