use incredible_calculator::state::CalcMode;
use incredible_calculator::ui::app::window_size;

#[test]
fn each_mode_reports_the_calculator_size() {
    assert_eq!(window_size(CalcMode::Basic), (28, 19));
    assert_eq!(window_size(CalcMode::Scientific), (72, 20));
}

#[test]
fn the_two_calculators_need_different_windows() {
    assert_ne!(
        window_size(CalcMode::Basic),
        window_size(CalcMode::Scientific)
    );
}
