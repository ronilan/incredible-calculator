use incredible::*;
use incredible_calculator::ui::key::pressed;

fn event(key: Key, modifiers: &[KeyMod]) -> EventKey {
    EventKey {
        loop_count: 0,
        key,
        modifiers: modifiers.to_vec(),
        repeated: 0,
        timestamp: 0.0,
    }
}

fn combination(key: Key, modifiers: &[KeyMod]) -> KeyCombination {
    KeyCombination::new(key, modifiers)
}

#[test]
fn key_without_modifiers_matches_a_plain_press() {
    assert!(pressed(
        &combination(Key::Char('7'), &[]),
        &event(Key::Char('7'), &[])
    ));
}

#[test]
fn shifted_symbol_matches_its_binding() {
    // Terminals report `*`, `+`, `(` and `)` with Shift because the keyboard
    // layout needs it to produce the character.
    for ch in ['*', '+', '(', ')'] {
        assert!(
            pressed(
                &combination(Key::Char(ch), &[]),
                &event(Key::Char(ch), &[KeyMod::Shift])
            ),
            "{ch} should match"
        );
    }
}

#[test]
fn different_key_does_not_match() {
    assert!(!pressed(
        &combination(Key::Char('7'), &[]),
        &event(Key::Char('8'), &[])
    ));
}

#[test]
fn modifier_binding_requires_its_modifier() {
    let stored = combination(Key::Char('s'), &[KeyMod::Ctrl]);

    assert!(!pressed(&stored, &event(Key::Char('s'), &[])));
    assert!(pressed(&stored, &event(Key::Char('s'), &[KeyMod::Ctrl])));
}

#[test]
fn shifted_modifier_binding_still_requires_it() {
    let stored = combination(Key::Char('s'), &[KeyMod::Ctrl]);

    assert!(!pressed(&stored, &event(Key::Char('s'), &[KeyMod::Shift])));
    assert!(pressed(
        &stored,
        &event(Key::Char('s'), &[KeyMod::Ctrl, KeyMod::Shift])
    ));
}
