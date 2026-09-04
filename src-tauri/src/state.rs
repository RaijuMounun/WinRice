use std::sync::Mutex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeState {
    pub current_theme: String,
}

impl Default for ThemeState {
    fn default() -> Self {
        Self {
            current_theme: "default".to_string(),
        }
    }
}

pub struct WinRiceState {
    pub theme: Mutex<ThemeState>,
}

impl WinRiceState {
    pub fn new() -> Self {
        Self {
            theme: Mutex::new(ThemeState::default()),
        }
    }
}

impl Default for WinRiceState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_winrice_state_initialization() {
        let state = WinRiceState::new();
        let theme = state.theme.lock().unwrap();
        assert_eq!(theme.current_theme, "default", "Initial theme should be 'default'");
    }

    #[test]
    fn test_winrice_state_mutation() {
        let state = WinRiceState::new();
        {
            let mut theme = state.theme.lock().unwrap();
            theme.current_theme = "dracula".to_string();
        }
        let theme = state.theme.lock().unwrap();
        assert_eq!(theme.current_theme, "dracula", "State mutation failed across locks");
    }

    #[test]
    fn test_theme_state_serialization() {
        let theme = ThemeState { current_theme: "tokyo_night".to_string() };
        let json = serde_json::to_string(&theme).unwrap();
        assert_eq!(json, r#"{"current_theme":"tokyo_night"}"#, "Incorrect JSON serialization");
        
        let deserialized: ThemeState = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.current_theme, "tokyo_night", "Incorrect JSON deserialization");
    }
}
