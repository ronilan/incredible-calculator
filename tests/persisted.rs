use incredible_calculator::settings::{Persisted, apply_to_state};
use incredible_calculator::state::{CalcMode, State};

#[test]
fn defaults_are_apple_basic_and_degrees() {
    let data = Persisted::default();
    assert_eq!(data.theme_index, 0);
    assert_eq!(data.calc_mode, CalcMode::Basic);
    assert!(!data.is_radian);
}

#[test]
fn parses_saved_preferences() {
    let data = Persisted::parse_content("2,1,true");
    assert_eq!(data.theme_index, 2);
    assert_eq!(data.calc_mode, CalcMode::Scientific);
    assert!(data.is_radian);
}

#[test]
fn accepts_named_mode_and_boolean_forms() {
    let data = Persisted::parse_content("3,scientific,true");
    assert_eq!(data.theme_index, 3);
    assert_eq!(data.calc_mode, CalcMode::Scientific);
    assert!(data.is_radian);

    let data = Persisted::parse_content("0,basic,false");
    assert_eq!(data.theme_index, 0);
    assert_eq!(data.calc_mode, CalcMode::Basic);
    assert!(!data.is_radian);
}

#[test]
fn unknown_values_fall_back_to_defaults() {
    let data = Persisted::parse_content("9,sideways,maybe");
    assert_eq!(data.theme_index, 0);
    assert_eq!(data.calc_mode, CalcMode::Basic);
    assert!(!data.is_radian);
}

#[test]
fn missing_fields_fall_back_individually() {
    let data = Persisted::parse_content("1");
    assert_eq!(data.theme_index, 1);
    assert_eq!(data.calc_mode, CalcMode::Basic);
    assert!(!data.is_radian);

    let data = Persisted::parse_content("1,1");
    assert_eq!(data.theme_index, 1);
    assert_eq!(data.calc_mode, CalcMode::Scientific);
    assert!(!data.is_radian);
}

#[test]
fn empty_or_damaged_content_uses_defaults() {
    assert_eq!(Persisted::parse_content(""), Persisted::default());
    assert_eq!(Persisted::parse_content("   "), Persisted::default());
    assert_eq!(Persisted::parse_content(",,,"), Persisted::default());
}

#[test]
fn extra_fields_are_ignored() {
    let data = Persisted::parse_content("2,1,true,999,anything");
    assert_eq!(data.theme_index, 2);
    assert_eq!(data.calc_mode, CalcMode::Scientific);
    assert!(data.is_radian);
}

#[test]
fn round_trips_the_preferences_in_state() {
    let mut state = State::new();
    state.theme_index = 3;
    state.calc_mode = CalcMode::Scientific;
    state.is_radian = true;

    let data = Persisted::from_state(&state);
    let parsed = Persisted::parse_content(&data.content());

    assert_eq!(parsed, data);
}

#[test]
fn content_uses_the_three_field_format() {
    let mut state = State::new();
    state.theme_index = 2;
    state.calc_mode = CalcMode::Scientific;
    state.is_radian = true;
    assert_eq!(Persisted::from_state(&state).content(), "2,1,true");

    state.theme_index = 0;
    state.calc_mode = CalcMode::Basic;
    state.is_radian = false;
    assert_eq!(Persisted::from_state(&state).content(), "0,0,false");
}

#[test]
fn applies_preferences_to_a_fresh_state() {
    let data = Persisted {
        theme_index: 1,
        calc_mode: CalcMode::Scientific,
        is_radian: true,
    };

    let mut state = State::new();
    apply_to_state(&data, &mut state);

    assert_eq!(state.theme_index, 1);
    assert_eq!(state.calc_mode, CalcMode::Scientific);
    assert!(state.is_radian);
}

#[test]
fn an_out_of_range_theme_is_clamped_to_apple() {
    let data = Persisted {
        theme_index: 99,
        calc_mode: CalcMode::Basic,
        is_radian: false,
    };

    let mut state = State::new();
    state.theme_index = 2;
    apply_to_state(&data, &mut state);

    assert_eq!(state.theme_index, 0);
}
