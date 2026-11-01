use incredible::*;
use incredible_calculator::state::{CalcMode, State};
use incredible_calculator::ui::screens::{basic, scientific};
use incredible_elements::{Rectangle, Text};

const TITLE_WIDTH_BASIC: usize = 28;
const TITLE_WIDTH_SCIENTIFIC: usize = 72;
const MODE_BUTTON_FROM_LEFT: isize = 0;
const NAME_FROM_LEFT: isize = 3;
const SETTING_BUTTON_FROM_RIGHT: isize = 3;

fn on_state(title: &Rectangle<State>, state: &State) {
    title.trigger_on_state(
        state,
        &EventState {
            loop_count: 0,
            prev_state: State::new(),
            timestamp: 0.0,
        },
    );
}

fn click(x: isize, y: isize) -> EventMouse {
    EventMouse {
        loop_count: 0,
        mouse: Mouse::Click,
        modifiers: vec![],
        x,
        y,
        timestamp: 0.0,
        source: PointerSource::Mouse,
    }
}

fn key_event(key: Key) -> EventKey {
    EventKey {
        loop_count: 0,
        key,
        modifiers: vec![],
        repeated: 0,
        timestamp: 0.0,
    }
}

fn now() -> f64 {
    0.0
}

#[test]
fn the_gear_sits_before_the_name() {
    for (title, name) in [
        (basic::title::build(), "Basic"),
        (scientific::title::build(), "Scientific"),
    ] {
        title.x(100);
        on_state(&title, &State::new());

        let texts = title.elements().cot::<Text<State>>();
        assert_eq!(texts[0].get_text(), name);
        assert_eq!(texts[0].get_x(), 100 + NAME_FROM_LEFT);
        assert_eq!(texts[1].get_text(), "⚙️");
        assert_eq!(texts[1].get_x(), 100 + MODE_BUTTON_FROM_LEFT);
    }
}

#[test]
fn basic_title_buttons_follow_the_title_when_it_moves() {
    let title = basic::title::build();
    title.x(100);

    let state = State::new();
    on_state(&title, &state);

    let texts = title.elements().cot::<Text<State>>();
    let right = 100 + TITLE_WIDTH_BASIC as isize;
    assert_eq!(texts[0].get_x(), 100 + NAME_FROM_LEFT);
    assert_eq!(texts[1].get_x(), 100 + MODE_BUTTON_FROM_LEFT);
    assert_eq!(texts[2].get_x(), right - SETTING_BUTTON_FROM_RIGHT);

    // The containing element can still move after the title's on_state
    // runs; captured children must move with it.
    title.x(200);
    let right = 200 + TITLE_WIDTH_BASIC as isize;
    assert_eq!(texts[0].get_x(), 200 + NAME_FROM_LEFT);
    assert_eq!(texts[1].get_x(), 200 + MODE_BUTTON_FROM_LEFT);
    assert_eq!(texts[2].get_x(), right - SETTING_BUTTON_FROM_RIGHT);
}

#[test]
fn scientific_title_buttons_follow_the_title_when_it_moves() {
    let title = scientific::title::build();
    title.x(100);

    let mut state = State::new();
    state.calc_mode = CalcMode::Scientific;
    on_state(&title, &state);

    let texts = title.elements().cot::<Text<State>>();
    let right = 100 + TITLE_WIDTH_SCIENTIFIC as isize;
    assert_eq!(texts[0].get_x(), 100 + NAME_FROM_LEFT);
    assert_eq!(texts[1].get_x(), 100 + MODE_BUTTON_FROM_LEFT);
    assert_eq!(texts[2].get_x(), right - SETTING_BUTTON_FROM_RIGHT);

    title.x(200);
    let right = 200 + TITLE_WIDTH_SCIENTIFIC as isize;
    assert_eq!(texts[0].get_x(), 200 + NAME_FROM_LEFT);
    assert_eq!(texts[1].get_x(), 200 + MODE_BUTTON_FROM_LEFT);
    assert_eq!(texts[2].get_x(), right - SETTING_BUTTON_FROM_RIGHT);
}

#[test]
fn the_theme_shortcut_is_shift_t() {
    Globals::set_now(now);

    for title in [basic::title::build(), scientific::title::build()] {
        let mut state = State::new();
        let texts = title.elements().cot::<Text<State>>();
        let setting_button = &texts[2];

        setting_button.trigger_on_key(&mut state, &key_event(Key::Char('t')));
        assert_eq!(state.theme_index, 0);

        setting_button.trigger_on_key(&mut state, &key_event(Key::Char('T')));
        assert_eq!(state.theme_index, 1);
    }
}

#[test]
fn mode_buttons_switch_the_calculator_mode() {
    let title = basic::title::build();
    let mut state = State::new();
    let texts = title.elements().cot::<Text<State>>();
    let mode_button = &texts[1];
    mode_button.trigger_on_mouse(&mut state, &click(mode_button.get_x(), mode_button.get_y()));
    assert_eq!(state.calc_mode, CalcMode::Scientific);

    let title = scientific::title::build();
    let mut state = State::new();
    state.calc_mode = CalcMode::Scientific;
    let texts = title.elements().cot::<Text<State>>();
    let mode_button = &texts[1];
    mode_button.trigger_on_mouse(&mut state, &click(mode_button.get_x(), mode_button.get_y()));
    assert_eq!(state.calc_mode, CalcMode::Basic);
}
