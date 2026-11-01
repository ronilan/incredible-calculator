use incredible::*;

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
    assert!(combination(Key::Char('7'), &[]).matches(&event(Key::Char('7'), &[])));
}

#[test]
fn shifted_symbol_matches_its_binding() {
    // Terminals report `*`, `+`, `(` and `)` with Shift because the keyboard
    // layout needs it to produce the character.
    for ch in ['*', '+', '(', ')'] {
        assert!(
            combination(Key::Char(ch), &[]).matches(&event(Key::Char(ch), &[KeyMod::Shift])),
            "{ch} should match"
        );
    }
}

#[test]
fn uppercase_character_ignores_synthesised_shift() {
    // The terminal adds Shift for `E`, which is bound as `Key::Char('E')`.
    assert!(combination(Key::Char('E'), &[]).matches(&event(Key::Char('E'), &[KeyMod::Shift])));
}

#[test]
fn different_key_does_not_match() {
    assert!(!combination(Key::Char('7'), &[]).matches(&event(Key::Char('8'), &[])));
    assert!(!combination(Key::Char('e'), &[]).matches(&event(Key::Char('E'), &[])));
}

#[test]
fn modifier_binding_requires_its_modifier() {
    let stored = combination(Key::Char('s'), &[KeyMod::Ctrl]);

    assert!(!stored.matches(&event(Key::Char('s'), &[])));
    assert!(stored.matches(&event(Key::Char('s'), &[KeyMod::Ctrl])));
}

#[test]
fn extra_shift_does_not_replace_ctrl() {
    let stored = combination(Key::Char('s'), &[KeyMod::Ctrl]);

    assert!(!stored.matches(&event(Key::Char('s'), &[KeyMod::Shift])));
    assert!(stored.matches(&event(Key::Char('s'), &[KeyMod::Ctrl, KeyMod::Shift])));
}

#[test]
fn shift_stays_significant_for_non_character_keys() {
    assert!(!combination(Key::Tab, &[]).matches(&event(Key::Tab, &[KeyMod::Shift])));
    assert!(
        !combination(Key::Delete, &[KeyMod::Ctrl])
            .matches(&event(Key::Delete, &[KeyMod::Ctrl, KeyMod::Shift]))
    );
}
