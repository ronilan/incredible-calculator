use incredible::*;

use crate::state::State;

/// Applies the theme at `index`, falling back to Apple for unknown values.
pub fn apply(index: u8) {
    match index {
        1 => little_theme(),
        2 => basic_16_theme(),
        3 => none_theme(),
        _ => apple_theme(),
    }
}

// -----------------------------
// 0) Apple Theme
// -----------------------------
pub fn apple_theme() {
    Globals::theme_reset::<State>();

    theme_rule::<Style>("Display", |s| {
        s.base.decor.color.set(Some(Color::Ansi(231)));
        s.base
            .decor
            .background
            .set(Some(Color::Ansi(Colors::hex_to_ansi8("#333333"))));
    });

    theme_rule::<Style>("Title", |s| {
        s.base.decor.italic.set(Some(true));
        s.base.decor.color.set(Some(Color::Ansi(231)));
        s.base
            .decor
            .background
            .set(Some(Color::Ansi(Colors::hex_to_ansi8("#222222"))));
    });

    theme_rule::<Style>("OperationButton", |s| {
        s.base.decor.color.set(Some(Color::Ansi(231)));
        s.base
            .decor
            .background
            .set(Some(Color::Ansi(Colors::hex_to_ansi8("#FF9500"))));
        s.focused.decor.bold.set(Some(false));
        s.entered.decor.bold.set(Some(true));
        s.entered.pointer.set(Some(PointerShape::Pointer));
    });

    theme_rule::<Style>("NumberButton", |s| {
        s.base.decor.color.set(Some(Color::Ansi(231)));
        s.base
            .decor
            .background
            .set(Some(Color::Ansi(Colors::hex_to_ansi8("#666666"))));
        s.focused.decor.bold.set(Some(false));
        s.entered.decor.bold.set(Some(true));
        s.entered.pointer.set(Some(PointerShape::Pointer));
    });

    theme_rule::<Style>("AdditionalButton", |s| {
        s.base.decor.color.set(Some(Color::Ansi(231)));
        s.base
            .decor
            .background
            .set(Some(Color::Ansi(Colors::hex_to_ansi8("#444444"))));
        s.focused.decor.bold.set(Some(false));
        s.entered.decor.bold.set(Some(true));
        s.entered.pointer.set(Some(PointerShape::Pointer));
    });

    theme_rule::<Style>("ScientificButton", |s| {
        s.base.decor.color.set(Some(Color::Ansi(231)));
        s.base
            .decor
            .background
            .set(Some(Color::Ansi(Colors::hex_to_ansi8("#555555"))));
        s.focused.decor.bold.set(Some(false));
        s.entered.decor.bold.set(Some(true));
        s.entered.pointer.set(Some(PointerShape::Pointer));
    });
}

// -----------------------------
// 2) Little Theme Two
// -----------------------------
pub fn little_theme() {
    Globals::theme_reset::<State>();

    theme_rule::<Style>("Display", |s| {
        s.base.decor.color.set(Some(Color::Ansi(0)));
        s.base.decor.background.set(Some(Color::Ansi(7)));
    });

    theme_rule::<Style>("Title", |s| {
        s.base.decor.italic.set(Some(true));
        s.base.decor.bold.set(Some(true));
        s.base.decor.color.set(Some(Color::Ansi(15)));
        s.base.decor.background.set(Some(Color::Ansi(8)));
    });

    theme_rule::<Style>("OperationButton", |s| {
        s.base.decor.color.set(Some(Color::Ansi(0)));
        s.base.decor.background.set(Some(Color::Ansi(9)));
        s.focused.decor.bold.set(Some(false));
        s.entered.decor.bold.set(Some(true));
    });

    theme_rule::<Style>("NumberButton", |s| {
        s.base.decor.color.set(Some(Color::Ansi(0)));
        s.base.decor.background.set(Some(Color::Ansi(10)));
        s.focused.decor.bold.set(Some(false));
        s.entered.decor.bold.set(Some(true));
    });

    theme_rule::<Style>("AdditionalButton", |s| {
        s.base.decor.color.set(Some(Color::Ansi(0)));
        s.base.decor.background.set(Some(Color::Ansi(14)));
        s.focused.decor.bold.set(Some(false));
        s.entered.decor.bold.set(Some(true));
    });

    theme_rule::<Style>("ScientificButton", |s| {
        s.base.decor.color.set(Some(Color::Ansi(15)));
        s.base.decor.background.set(Some(Color::Ansi(8)));
        s.focused.decor.bold.set(Some(false));
        s.entered.decor.bold.set(Some(true));
    });
}

// -----------------------------
// 3) Basic 16 Theme (Apple approximation)
// -----------------------------
pub fn basic_16_theme() {
    Globals::theme_reset::<State>();

    theme_rule::<Style>("Display", |s| {
        s.base.decor.background.set(Some(Color::Ansi(0)));
    });

    theme_rule::<Style>("Display_digit", |_s| {});

    theme_rule::<Style>("Title", |s| {
        s.base.decor.color.set(Some(Color::Ansi(15)));
        s.base.decor.background.set(Some(Color::Ansi(8)));
    });

    theme_rule::<Style>("OperationButton", |s| {
        s.base.decor.color.set(Some(Color::Ansi(15)));
        s.base.decor.background.set(Some(Color::Ansi(1)));
        s.focused.decor.bold.set(Some(false));
        s.entered.decor.bold.set(Some(true));
    });

    theme_rule::<Style>("NumberButton", |s| {
        s.base.decor.color.set(Some(Color::Ansi(0)));
        s.base.decor.background.set(Some(Color::Ansi(7)));
        s.focused.decor.bold.set(Some(false));
        s.entered.decor.bold.set(Some(true));
    });

    theme_rule::<Style>("AdditionalButton", |s| {
        s.base.decor.color.set(Some(Color::Ansi(15)));
        s.base.decor.background.set(Some(Color::Ansi(8)));
        s.focused.decor.bold.set(Some(false));
        s.entered.decor.bold.set(Some(true));
    });

    theme_rule::<Style>("ScientificButton", |s| {
        s.base.decor.color.set(Some(Color::Ansi(15)));
        s.base.decor.background.set(Some(Color::Ansi(0)));
        s.focused.decor.bold.set(Some(false));
        s.entered.decor.bold.set(Some(true));
    });
}

// -----------------------------
// 4) None Theme
// -----------------------------
pub fn none_theme() {
    Globals::theme_reset::<State>();

    theme_rule::<Style>("Display", |_s| {});
    theme_rule::<Style>("Title", |_s| {});
    theme_rule::<Style>("OperationButton", |_s| {});
    theme_rule::<Style>("NumberButton", |_s| {});
    theme_rule::<Style>("AdditionalButton", |_s| {});
    theme_rule::<Style>("ScientificButton", |_s| {});
}
