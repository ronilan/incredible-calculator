use incredible::*;
use incredible_elements::{Rectangle, Text};
use incredible_elements_text_fonts::FancyASCII;

use crate::state::State;

static X: isize = 0;
static Y: isize = 1;

static FRAME_WIDTH: usize = 72;
static FRAME_HEIGHT: usize = 4;

// The fancy font is three rows tall. Most glyphs are three columns wide, but a
// few ('.', '/', '*', ...) are a single column, so characters advance by their
// own glyph width. Composed rows are always DISPLAY_COLUMNS wide, which keeps
// the painted field fixed and lets old digits be overwritten cleanly.
static DISPLAY_ROWS: usize = 3;
static DISPLAY_COLUMNS: usize = FRAME_WIDTH - 2;

pub fn build() -> Rectangle<State> {
    let display = Rectangle::new();
    display
        .x(X)
        .y(Y)
        .width(FRAME_WIDTH)
        .height(FRAME_HEIGHT)
        .style_handle("Display");

    let history_display = Text::default();
    history_display
        .x(1)
        .y(0)
        .text("")
        .capture(Capture::all())
        .style_handle("Display_digit");

    history_display.on_state(|el, state: &State, _ev| {
        el.text(history_text(&state.history).as_str());
    });
    display.add(history_display);

    let digit_display = Text::default();
    digit_display
        .x(1)
        .y(1)
        .text(fancy_text("0").as_str())
        .capture(Capture::all())
        .style_handle("Display_digit");

    digit_display.on_state(|el, state: &State, _ev| {
        let shown = if state.display.is_empty() {
            "0"
        } else {
            &state.display
        };
        el.text(fancy_text(shown).as_str());
    });

    display.add(digit_display);

    display
}

/// Compacts the calculation shown above the digits and right-aligns it in the
/// same fixed field as the glyph rows.
///
/// Spaces are dropped for the same reason as in `translated`: the expression is
/// stored with spaces around operators, and here they only make the line sparse.
fn history_text(history: &str) -> String {
    let compact: String = history.chars().filter(|ch| *ch != ' ').collect();
    let chars: Vec<char> = compact.chars().collect();

    let shown: String = if chars.len() > DISPLAY_COLUMNS {
        chars[chars.len() - DISPLAY_COLUMNS..].iter().collect()
    } else {
        compact
    };

    format!("{:>width$}", shown, width = DISPLAY_COLUMNS)
}

/// Renders `text` with the fancy font: symbols translated, spaces dropped, and
/// only the tail that fits. The result is DISPLAY_ROWS lines, each exactly
/// DISPLAY_COLUMNS characters wide and right-aligned.
fn fancy_text(text: &str) -> String {
    let glyphs = fitted_glyphs(text);

    let width: usize = glyphs.iter().map(|glyph| glyph_width(glyph)).sum();
    let padding = " ".repeat(DISPLAY_COLUMNS - width);

    (0..DISPLAY_ROWS)
        .map(|row| {
            let mut line = padding.clone();
            for glyph in &glyphs {
                line.push_str(glyph.lines().nth(row).unwrap_or(""));
            }
            line
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Keeps the tail of the text whose glyphs fit in the display.
fn fitted_glyphs(text: &str) -> Vec<&'static str> {
    let mut glyphs = Vec::new();
    let mut width = 0;

    for ch in translated(text).chars().rev() {
        let glyph = glyph(ch);
        let width_of_glyph = glyph_width(glyph);
        if width + width_of_glyph > DISPLAY_COLUMNS {
            break;
        }
        width += width_of_glyph;
        glyphs.push(glyph);
    }

    glyphs.reverse();
    glyphs
}

/// Converts calculator symbols into characters the font knows, and drops
/// spaces: the font draws every character three columns wide, so a space is as
/// wide as a digit and the expression would otherwise look sparse.
///
/// Function names are swapped for private-use characters that map to the thin
/// font, so `sin(` reads as small letters instead of three tall glyphs.
fn translated(text: &str) -> String {
    let mut mapped = String::new();

    for ch in text.chars() {
        match ch {
            ' ' => {}
            '×' => mapped.push('*'),
            '÷' => mapped.push('/'),
            '−' => mapped.push('-'),
            _ => mapped.push(ch),
        }
    }

    let mut out = String::new();
    let mut rest = mapped.as_str();
    let mut prev_letter = false;

    'scan: while !rest.is_empty() {
        // `Ans` is a word too: draw it on the thin row like a function name.
        if let Some(tail) = rest.strip_prefix("Ans") {
            out.push('A');
            out.push(small_letter('n'));
            out.push(small_letter('s'));
            rest = tail;
            prev_letter = false;
            continue 'scan;
        }

        for name in FUNCTION_NAMES {
            if rest.starts_with(name) {
                for ch in name[..name.len() - 1].chars() {
                    out.push(small_letter(ch));
                }
                // The parenthesis stays a normal glyph; only the name shrinks.
                out.push('(');
                rest = &rest[name.len()..];
                prev_letter = false;
                continue 'scan;
            }
        }

        let ch = rest.chars().next().unwrap();
        let next_letter = rest[ch.len_utf8()..]
            .chars()
            .next()
            .is_some_and(|next| next.is_ascii_alphabetic());

        // A standalone uppercase `E` is the exponent marker; keep it thin.
        // The `E` in a word such as `Error` stays a normal glyph.
        if ch == 'E' && !prev_letter && !next_letter {
            out.push(EXP_MARKER);
        } else {
            out.push(ch);
        }

        prev_letter = ch.is_ascii_alphabetic();
        rest = &rest[ch.len_utf8()..];
    }

    out
}

fn glyph(ch: char) -> &'static str {
    if let Some(index) = small_letter_index(ch) {
        SMALL_FONT[index]
    } else if ch == '√' {
        ROOT_GLYPH
    } else if ch == 'π' {
        PI_GLYPH
    } else if ch == EXP_MARKER {
        EXP_GLYPH
    } else if ch == 'e' {
        // Euler's constant reads as one thin letter.
        SMALL_FONT[4]
    } else if ch == 'A' {
        // The capital of `Ans`; its other letters are thin markers.
        SMALL_A_GLYPH
    } else if (' '..='~').contains(&ch) {
        FancyASCII::<State>::ASCII_FONT[(ch as u8 - b' ') as usize]
    } else {
        EMPTY_GLYPH
    }
}

/// Function names that are drawn in the thin font. Longest first, so `asin`
/// wins over `sin`.
static FUNCTION_NAMES: [&str; 10] = [
    "asin(", "acos(", "atan(", "sqrt(", "fact(", "sin(", "cos(", "tan(", "log(", "ln(",
];

// Private-use block used to mark letters that should use the small font.
const SMALL_BASE: u32 = 0xE000;

fn small_letter(ch: char) -> char {
    char::from_u32(SMALL_BASE + (ch as u32 - 'a' as u32)).expect("function names are lowercase")
}

fn small_letter_index(ch: char) -> Option<usize> {
    let code = ch as u32;
    if (SMALL_BASE..SMALL_BASE + SMALL_FONT.len() as u32).contains(&code) {
        Some((code - SMALL_BASE) as usize)
    } else {
        None
    }
}

/// One-column lowercase letters, sitting on the middle row like `*` and `/`.
const SMALL_FONT: [&str; 26] = [
    " \na\n ", " \nb\n ", " \nc\n ", " \nd\n ", " \ne\n ", " \nf\n ", " \ng\n ", " \nh\n ",
    " \ni\n ", " \nj\n ", " \nk\n ", " \nl\n ", " \nm\n ", " \nn\n ", " \no\n ", " \np\n ",
    " \nq\n ", " \nr\n ", " \ns\n ", " \nt\n ", " \nu\n ", " \nv\n ", " \nw\n ", " \nx\n ",
    " \ny\n ", " \nz\n ",
];

fn glyph_width(glyph: &str) -> usize {
    glyph
        .lines()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0)
}

/// The fancy font only covers ASCII; this is the root sign it lacks. One
/// column wide, on the same row as the other thin symbols (`*`, `/`).
const ROOT_GLYPH: &str = " \n√\n ";

/// `π` is outside the ASCII font; draw it thin like the other symbols.
const PI_GLYPH: &str = " \nπ\n ";

/// The exponent marker is an uppercase `E`; draw it one column wide, like
/// the other thin symbols.
const EXP_MARKER: char = '\u{E100}';
const EXP_GLYPH: &str = " \nE\n ";

/// The capital `A` of `Ans`, one column wide like its other letters.
const SMALL_A_GLYPH: &str = " \nA\n ";

const EMPTY_GLYPH: &str = "   \n   \n   ";
