use incredible::*;
use incredible_calculator::state::{CalcMode, State};
use incredible_calculator::ui::screens::basic::basic;
use incredible_calculator::ui::screens::scientific::scientific;
use incredible_helpers_layout::sizing::Sizing;

struct MockOutput;

impl PlatformOutput for MockOutput {
    fn output_platform(&self) -> OutputPlatform {
        OutputPlatform::Terminal
    }

    fn is_true_color(&self) -> bool {
        false
    }

    fn size(&self) -> (usize, usize) {
        (80, 24)
    }

    fn draw_blocks(&self, _x: isize, _y: isize, _blocks: &[Vec<Block>]) {}
}

fn resize(columns: usize, rows: usize) -> EventWindow {
    EventWindow {
        loop_count: 0,
        window: Window::Resize,
        rows,
        columns,
        focus: true,
        timestamp: 0.0,
    }
}

#[test]
fn screen_resizes_and_recenters_the_calculator() {
    Platform::set_output_provider(MockOutput);

    let screen = basic::build();
    let calculator = screen.elements().get(0).unwrap();
    let mut state = State::new();

    screen.trigger_on_window(&mut state, &resize(80, 24));
    assert_eq!((screen.get_width(), screen.get_height()), (80, 24));
    assert_eq!(calculator.get_xy(), (26, 3));

    screen.trigger_on_window(&mut state, &resize(100, 30));
    assert_eq!((screen.get_width(), screen.get_height()), (100, 30));
    assert_eq!(calculator.get_xy(), (36, 6));
}

fn content_size(screen: &incredible_elements::Rectangle<State>) -> (usize, usize) {
    let calculator = screen.elements().get(0).unwrap();
    let (origin_x, origin_y) = calculator.get_xy();

    let mut right = 0isize;
    let mut bottom = 0isize;

    for child in calculator.elements().iter() {
        let bounds = child.get_outside();
        right = right.max(bounds.x.get() - origin_x + bounds.width.get() as isize);
        bottom = bottom.max(bounds.y.get() - origin_y + bounds.height.get() as isize);
    }

    (right as usize, bottom as usize)
}

#[test]
fn calculators_are_exactly_as_large_as_their_content() {
    Platform::set_output_provider(MockOutput);

    let screens = [
        ("basic", basic::build()),
        ("scientific", scientific::build()),
    ];

    for (name, screen) in screens {
        let calculator = screen.elements().get(0).unwrap();

        assert_eq!(
            (calculator.get_width(), calculator.get_height()),
            content_size(&screen),
            "{name} calculator should be sized to its content"
        );
    }
}

fn paste(text: &str) -> EventWindow {
    EventWindow {
        loop_count: 0,
        window: Window::Paste(text.to_string()),
        rows: 0,
        columns: 0,
        focus: true,
        timestamp: 0.0,
    }
}

#[test]
fn paste_replaces_the_expression_only_in_scientific_mode() {
    Platform::set_output_provider(MockOutput);

    let screen = scientific::build();
    let mut state = State::new();

    state.calc_mode = CalcMode::Scientific;
    screen.trigger_on_window(&mut state, &paste(" 2 +\n3 "));
    assert_eq!(state.expression, "2+3");

    state.expression = "9".into();
    state.calc_mode = CalcMode::Basic;
    screen.trigger_on_window(&mut state, &paste("4+4"));
    assert_eq!(state.expression, "9");
}

#[test]
fn scientific_screen_resizes_and_recenters_the_calculator() {
    Platform::set_output_provider(MockOutput);

    let screen = scientific::build();
    let calculator = screen.elements().get(0).unwrap();
    let mut state = State::new();

    screen.trigger_on_window(&mut state, &resize(80, 24));
    assert_eq!((screen.get_width(), screen.get_height()), (80, 24));
    assert_eq!(calculator.get_xy(), (4, 2));

    screen.trigger_on_window(&mut state, &resize(100, 30));
    assert_eq!((screen.get_width(), screen.get_height()), (100, 30));
    assert_eq!(calculator.get_xy(), (14, 5));
}
