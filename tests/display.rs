use std::rc::Rc;

use incredible::*;
use incredible_calculator::state::State;
use incredible_calculator::ui::screens::scientific::display;
use incredible_elements::{Rectangle, Text};

const DISPLAY_COLUMNS: usize = 70;

fn on_state(display: &Rectangle<State>, state: &State) {
    let event = EventState {
        loop_count: 0,
        prev_state: State::new(),
        timestamp: 0.0,
    };

    propagate(display, state, &event);
}

fn propagate(el: &dyn ElementTrait<State>, state: &State, event: &EventState<State>) {
    for sub in el.elements().iter() {
        propagate(sub.as_ref(), state, event);
    }
    el.trigger_on_state(state, event);
}

fn history_display(display: &Rectangle<State>) -> Rc<Text<State>> {
    let texts = display.elements().cot::<Text<State>>();
    assert_eq!(texts.len(), 2, "display should hold history and digits");
    texts[0].clone()
}

fn history_row(display: &Rectangle<State>) -> String {
    history_display(display).get_text()
}

fn digit_display(display: &Rectangle<State>) -> Rc<Text<State>> {
    let texts = display.elements().cot::<Text<State>>();
    assert_eq!(texts.len(), 2, "display should hold history and digits");
    texts[1].clone()
}

fn rows(display: &Rectangle<State>) -> Vec<String> {
    digit_display(display)
        .get_text()
        .lines()
        .map(str::to_string)
        .collect()
}

fn leading_spaces(display: &Rectangle<State>) -> usize {
    rows(display)[0].chars().take_while(|c| *c == ' ').count()
}

#[test]
fn scientific_display_renders_three_fancy_rows() {
    let display = display::build();
    let state = State::new();

    on_state(&display, &state);

    let rows = rows(&display);
    assert_eq!(rows.len(), 3);
    assert!(rows[0].ends_with("╭─╮"), "{}", rows[0]);
    assert!(rows[1].ends_with("│ │"), "{}", rows[1]);
    assert!(rows[2].ends_with("╰─╯"), "{}", rows[2]);
}

#[test]
fn scientific_display_paints_a_fixed_field() {
    let display = display::build();
    let mut state = State::new();

    for value in [
        "0",
        "12.5",
        "1234567890",
        "1/2",
        "1234567890123456789012345",
    ] {
        state.display = value.into();
        on_state(&display, &state);

        let rows = rows(&display);
        assert_eq!(rows.len(), 3);
        for row in &rows {
            assert_eq!(row.chars().count(), DISPLAY_COLUMNS, "{row:?}");
        }
        assert_eq!(digit_display(&display).get_width(), DISPLAY_COLUMNS);
    }
}

#[test]
fn scientific_display_advances_thin_glyphs_one_column() {
    let display = display::build();
    let mut state = State::new();

    state.display = "132".into();
    on_state(&display, &state);
    let wide = leading_spaces(&display);

    // '/' is one column wide where a digit is three, so the content starts
    // two columns further right.
    state.display = "1/2".into();
    on_state(&display, &state);
    let thin = leading_spaces(&display);

    assert_eq!(thin, wide + 2);
}

#[test]
fn scientific_display_drops_spaces_and_maps_symbols() {
    let spaced = display::build();
    let compact = display::build();
    let mut state = State::new();

    state.display = "4 * (75 - 3)".into();
    on_state(&spaced, &state);
    state.display = "4*(75-3)".into();
    on_state(&compact, &state);
    assert_eq!(
        digit_display(&spaced).get_text(),
        digit_display(&compact).get_text()
    );

    state.display = "2×3".into();
    on_state(&spaced, &state);
    state.display = "2*3".into();
    on_state(&compact, &state);
    assert_eq!(
        digit_display(&spaced).get_text(),
        digit_display(&compact).get_text()
    );
}

#[test]
fn scientific_display_keeps_the_tail_that_fits() {
    let display = display::build();
    let expected = display::build();
    let mut state = State::new();

    let full = "1234567890123456789012345";
    // Only whole three-column digits fit, so the leading ones are dropped.
    let fits = DISPLAY_COLUMNS / 3;

    state.display = full.into();
    on_state(&display, &state);
    state.display = full[full.len() - fits..].into();
    on_state(&expected, &state);

    assert_eq!(
        digit_display(&display).get_text(),
        digit_display(&expected).get_text()
    );
}

#[test]
fn scientific_history_drops_expression_spaces() {
    let display = display::build();
    let mut state = State::new();
    state.history = "4 × (75 − 3) =".into();

    on_state(&display, &state);

    assert_eq!(
        history_row(&display).trim_start(),
        "4×(75−3)=",
        "{}",
        history_row(&display)
    );
}

#[test]
fn scientific_history_paints_a_fixed_field() {
    let display = display::build();
    let mut state = State::new();

    for value in [
        "",
        "1 ÷ 2 =",
        "sin(1) =",
        "123456789012345678901234567890 =",
    ] {
        state.history = value.into();
        on_state(&display, &state);

        let row = history_row(&display);
        assert_eq!(row.chars().count(), DISPLAY_COLUMNS, "{row:?}");
        assert!(
            !row.trim().contains(' '),
            "history should be compact: {row:?}"
        );
    }
}

#[test]
fn scientific_history_keeps_the_tail_that_fits() {
    let display = display::build();
    let mut state = State::new();
    let digits = "1234567890".repeat(7);
    state.history = format!("{digits} =");

    on_state(&display, &state);

    // The compact history is longer than the field, so its first columns are
    // dropped.
    let compact = format!("{digits}=");
    assert!(compact.len() > DISPLAY_COLUMNS);
    let expected = &compact[compact.len() - DISPLAY_COLUMNS..];
    let row = history_row(&display);
    assert_eq!(row.chars().count(), DISPLAY_COLUMNS);
    assert_eq!(row, expected);
}

#[test]
fn scientific_display_pads_parentheses_symmetrically() {
    let display = display::build();
    let mut state = State::new();

    state.display = "(4+8)".into();
    on_state(&display, &state);

    // `(` 2 + `4` 3 + `+` 3 + `8` 3 + `)` 2 = 13 columns.
    let left = DISPLAY_COLUMNS - 13;
    let right = DISPLAY_COLUMNS - 1;

    for (row, (left_glyph, right_glyph)) in
        rows(&display)
            .iter()
            .zip([('╭', '╮'), ('│', '│'), ('╰', '╯')])
    {
        let chars: Vec<char> = row.chars().collect();
        assert_eq!(chars[left], left_glyph, "{row:?}");
        assert_eq!(chars[right], right_glyph, "{row:?}");
    }
}

#[test]
fn scientific_display_draws_the_root_sign() {
    let display = display::build();
    let mut state = State::new();

    state.display = "√(9)".into();
    on_state(&display, &state);

    let rows = rows(&display);
    for row in &rows {
        assert_eq!(row.chars().count(), DISPLAY_COLUMNS, "{row:?}");
    }

    let roots: usize = rows
        .iter()
        .map(|row| row.chars().filter(|c| *c == '√').count())
        .sum();
    assert_eq!(roots, 1, "the root sign should be drawn once: {rows:?}");
    assert!(
        !digit_display(&display).get_text().contains("sqrt"),
        "the root sign should not fall back to the word sqrt"
    );
}

#[test]
fn scientific_display_renders_function_names_in_small_letters() {
    let display = display::build();
    let mut state = State::new();

    state.display = "sin(1)".into();
    on_state(&display, &state);

    let painted = rows(&display);
    // The letters share the middle row instead of being three tall glyphs.
    assert!(painted[1].contains("sin"), "{painted:?}");

    // Three thin letters + '(' (2) + '1' (3) + ')' (2) = 10 columns. The
    // middle row starts with the letters themselves, so it measures the width.
    let middle_lead = painted[1].chars().take_while(|c| *c == ' ').count();
    assert_eq!(middle_lead, DISPLAY_COLUMNS - 10, "{painted:?}");

    // The whole name is matched, not just its tail.
    state.display = "asin(1)".into();
    on_state(&display, &state);
    let painted = rows(&display);
    assert!(painted[1].contains("asin"), "{painted:?}");
}

#[test]
fn scientific_display_draws_constants_as_single_characters() {
    let display = display::build();
    let mut state = State::new();

    state.display = "πe".into();
    on_state(&display, &state);

    let painted = rows(&display);
    // Both constants share the middle row, one column each.
    assert!(painted[1].contains("πe"), "{painted:?}");

    let middle_lead = painted[1].chars().take_while(|c| *c == ' ').count();
    assert_eq!(middle_lead, DISPLAY_COLUMNS - 2, "{painted:?}");
}

#[test]
fn scientific_display_draws_the_exponent_marker_thin() {
    let display = display::build();
    let mut state = State::new();

    state.display = "2E3".into();
    on_state(&display, &state);

    let painted = rows(&display);
    // Middle row: '2' middle + thin E + '3' middle.
    assert!(painted[1].contains("┌─┘E ─┤"), "{painted:?}");

    // The E in a word is not an exponent marker and stays a normal glyph.
    state.display = "Error".into();
    on_state(&display, &state);
    let painted = rows(&display);
    assert!(!painted[1].contains('E'), "{painted:?}");
}

#[test]
fn scientific_display_fits_rand_times_rand() {
    let display = display::build();
    let mut state = State::new();

    // Two six-decimal randoms and a multiply sign: 14 digits (3 columns),
    // two dots and the sign (1 column each).
    state.display = "0.123456×0.654321".into();
    on_state(&display, &state);

    let painted = rows(&display);
    let lead = painted[0].chars().take_while(|c| *c == ' ').count();
    assert_eq!(lead, DISPLAY_COLUMNS - 45, "{painted:?}");
}

#[test]
fn scientific_display_draws_ans_as_thin_letters() {
    let display = display::build();
    let mut state = State::new();

    state.display = "Ans".into();
    on_state(&display, &state);

    let painted = rows(&display);
    // The whole word shares the middle row, one column per letter.
    assert!(painted[1].contains("Ans"), "{painted:?}");

    let middle_lead = painted[1].chars().take_while(|c| *c == ' ').count();
    assert_eq!(middle_lead, DISPLAY_COLUMNS - 3, "{painted:?}");
}
