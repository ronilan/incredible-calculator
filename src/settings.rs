use std::fs;
use std::path::PathBuf;

use crate::state::{CalcMode, State};

#[cfg(not(target_arch = "wasm32"))]
const DATA_DIR: &str = "incredible-calculator";
#[cfg(not(target_arch = "wasm32"))]
const DATA_FILE: &str = ".incredible-calculator";

/// Number of themes the palette cycles through.
pub const THEME_COUNT: u8 = 4;

/// Preferences that survive a restart: the palette, which calculator was
/// open, and its angle mode. Calculations are deliberately not persisted.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Persisted {
    pub theme_index: u8,
    pub calc_mode: CalcMode,
    pub is_radian: bool,
}

impl Default for Persisted {
    fn default() -> Self {
        Self {
            theme_index: 0,
            calc_mode: CalcMode::Basic,
            is_radian: false,
        }
    }
}

impl Persisted {
    /// The preferences currently held by `state`.
    pub fn from_state(state: &State) -> Self {
        Self {
            theme_index: state.theme_index.min(THEME_COUNT - 1),
            calc_mode: state.calc_mode,
            is_radian: state.is_radian,
        }
    }

    /// Reads the `theme,mode,angle` line. Missing or unreadable fields fall
    /// back to their defaults, so an old or damaged file never blocks startup.
    pub fn parse_content(content: &str) -> Self {
        let mut parts = content.trim().split(',').map(str::trim);

        let theme_index = parts
            .next()
            .and_then(|part| part.parse::<u8>().ok())
            .filter(|index| *index < THEME_COUNT)
            .unwrap_or(0);

        let calc_mode = match parts.next().unwrap_or("") {
            "1" | "scientific" => CalcMode::Scientific,
            _ => CalcMode::Basic,
        };

        let is_radian = matches!(parts.next().unwrap_or(""), "1" | "true");

        Self {
            theme_index,
            calc_mode,
            is_radian,
        }
    }

    /// The `theme,mode,angle` line written to disk.
    pub fn content(&self) -> String {
        let mode = match self.calc_mode {
            CalcMode::Basic => 0,
            CalcMode::Scientific => 1,
        };
        format!("{},{},{}", self.theme_index, mode, self.is_radian)
    }
}

/// Per-user data file location, following platform conventions
/// (macOS: ~/Library/Application Support, Linux: $XDG_DATA_HOME,
/// Windows: %APPDATA%). `None` when no base data directory can be
/// determined (e.g. HOME is unset).
#[cfg(not(target_arch = "wasm32"))]
fn data_file_path() -> Option<PathBuf> {
    Some(dirs::data_dir()?.join(DATA_DIR).join(DATA_FILE))
}

/// Native file persistence does not exist on the web.
#[cfg(target_arch = "wasm32")]
fn data_file_path() -> Option<PathBuf> {
    None
}

/// Loads persisted preferences. Defaults when missing or unreadable.
pub fn load() -> Persisted {
    match data_file_path().and_then(|path| fs::read_to_string(path).ok()) {
        Some(content) => Persisted::parse_content(&content),
        None => Persisted::default(),
    }
}

/// Applies persisted preferences onto a fresh state.
pub fn apply_to_state(data: &Persisted, state: &mut State) {
    state.theme_index = if data.theme_index < THEME_COUNT {
        data.theme_index
    } else {
        0
    };
    state.calc_mode = data.calc_mode;
    state.is_radian = data.is_radian;
}

/// Writes the current preferences. Silent no-op when the path cannot be
/// determined or created.
pub fn persist_now(state: &State) {
    let Some(path) = data_file_path() else { return };
    if let Some(parent) = path.parent() {
        if fs::create_dir_all(parent).is_err() {
            return;
        }
    }
    let _ = fs::write(path, Persisted::from_state(state).content());
}
