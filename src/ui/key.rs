use incredible::*;

// -----------------------------
// Key bindings
// -----------------------------

/// Whether `event` presses the key bound by `combination`.
///
/// Terminals report the Shift modifier for symbols that need it on the keyboard
/// layout (`*`, `+`, `(` and `)` for example), which would leave those bindings
/// unreachable if the whole combination were compared. Shift is therefore
/// ignored; every other modifier still has to match, so combinations such as
/// `Ctrl+S` stay distinct from a plain `s`.
pub fn pressed(combination: &KeyCombination, event: &EventKey) -> bool {
    if combination.key != event.key {
        return false;
    }

    let wanted: Vec<&KeyMod> = combination
        .modifiers
        .iter()
        .filter(|modifier| **modifier != KeyMod::Shift)
        .collect();
    let actual: Vec<&KeyMod> = event
        .modifiers
        .iter()
        .filter(|modifier| **modifier != KeyMod::Shift)
        .collect();

    wanted.len() == actual.len() && wanted.iter().all(|modifier| actual.contains(modifier))
}
