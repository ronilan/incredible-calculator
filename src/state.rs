#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Number(f64),
    Op(char),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ClearMode {
    All,
    Entry,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CalcMode {
    Basic,
    Scientific,
}

#[derive(Clone, Debug, PartialEq)]
pub struct State {
    pub theme_index: u8,
    pub display: String,
    pub current: String,
    pub formula: Vec<Token>,
    pub repeat_op: Option<char>,
    pub repeat_rhs: Option<f64>,
    pub just_evaluated: bool,
    pub clear_mode: ClearMode,
    pub calc_mode: CalcMode,
    // Scientific mode fields
    pub history: String,
    pub expression: String,
    pub is_radian: bool,
    pub is_inverse: bool,
    pub last_ans: f64,
    pub should_reset: bool,
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    /// Switches to the other calculator and starts it from a clean sheet.
    /// Any expression in progress, the history, and the stored answer are
    /// dropped so the new screen shows a fresh `0`, the keypad starts in
    /// `Deg`, and `Inv` is off. The theme is a preference, so it survives.
    pub fn switch_mode(&mut self, mode: CalcMode) {
        self.calc_mode = mode;

        // Basic screen
        self.display = "0".into();
        self.current.clear();
        self.formula.clear();
        self.repeat_op = None;
        self.repeat_rhs = None;
        self.just_evaluated = false;
        self.clear_mode = ClearMode::All;

        // Scientific screen
        self.history.clear();
        self.expression = "0".into();
        self.is_radian = false;
        self.is_inverse = false;
        self.last_ans = 0.0;
        self.should_reset = false;
    }

    pub fn new() -> Self {
        Self {
            theme_index: 0,
            display: "0".into(),
            current: String::new(),
            formula: Vec::new(),
            repeat_op: None,
            repeat_rhs: None,
            just_evaluated: false,
            clear_mode: ClearMode::All,
            calc_mode: CalcMode::Basic,
            history: String::new(),
            expression: "0".into(),
            is_radian: false,
            is_inverse: false,
            last_ans: 0.0,
            should_reset: false,
        }
    }
}
