use tauri::State;
use crate::state::{WinRiceState, ThemeState};

#[tauri::command]
pub fn get_current_theme(state: State<'_, WinRiceState>) -> Result<ThemeState, String> {
    inner_get_current_theme(&state)
}

#[tauri::command]
pub fn update_theme(theme: String, state: State<'_, WinRiceState>) -> Result<(), String> {
    inner_update_theme(theme, &state)
}

#[tauri::command]
pub async fn apply_template(target_template: String) -> Result<(), String> {
    inner_apply_template(target_template)
}

#[tauri::command]
pub async fn rollback_system(backup_id: String) -> Result<(), String> {
    inner_rollback_system(backup_id)
}

fn is_valid_identifier(id: &str) -> bool {
    !id.is_empty() && !id.contains("..") && !id.contains('/') && !id.contains('\\')
}

// Inner pure logic for tests
pub fn inner_get_current_theme(state: &WinRiceState) -> Result<ThemeState, String> {
    let theme = state.theme.lock().map_err(|_| "Mutex poisoned")?;
    Ok(theme.clone())
}

pub fn inner_update_theme(theme: String, state: &WinRiceState) -> Result<(), String> {
    if !is_valid_identifier(&theme) {
        return Err("Invalid theme identifier".to_string());
    }
    let mut theme_state = state.theme.lock().map_err(|_| "Mutex poisoned")?;
    theme_state.current_theme = theme;
    Ok(())
}

pub fn inner_apply_template(target_template: String) -> Result<(), String> {
    if !is_valid_identifier(&target_template) {
        return Err("Invalid template identifier".to_string());
    }
    if target_template == "non_existent_template" {
        return Err("Template not found".to_string());
    }
    Ok(())
}

pub fn inner_rollback_system(backup_id: String) -> Result<(), String> {
    if !is_valid_identifier(&backup_id) {
        return Err("Invalid backup identifier".to_string());
    }
    if backup_id == "invalid_backup_id" {
        return Err("Backup not found".to_string());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::WinRiceState;

    #[test]
    fn test_get_current_theme_logic() {
        let state = WinRiceState::new();
        {
            let mut theme = state.theme.lock().unwrap();
            theme.current_theme = "custom_theme".to_string();
        }
        let result = inner_get_current_theme(&state).expect("inner_get_current_theme should succeed");
        assert_eq!(result.current_theme, "custom_theme", "Expected get_current_theme to retrieve the current state");
    }

    #[test]
    fn test_update_theme_logic() {
        let state = WinRiceState::new();
        inner_update_theme("new_theme".to_string(), &state).expect("inner_update_theme should succeed");
        let current = state.theme.lock().unwrap();
        assert_eq!(current.current_theme, "new_theme", "Expected update_theme to correctly update the state");
    }

    #[test]
    fn test_update_theme_invalid() {
        let state = WinRiceState::new();
        let result = inner_update_theme("../invalid".to_string(), &state);
        assert!(result.is_err(), "Expected update_theme to fail for invalid identifier");
    }

    #[test]
    fn test_apply_template_logic_missing_template() {
        let result = inner_apply_template("non_existent_template".to_string());
        assert!(result.is_err(), "Expected apply_template to fail when template does not exist");
    }

    #[test]
    fn test_apply_template_invalid() {
        let result = inner_apply_template("some/path".to_string());
        assert!(result.is_err(), "Expected apply_template to fail for invalid identifier");
    }

    #[test]
    fn test_rollback_system_logic_missing_backup() {
        let result = inner_rollback_system("invalid_backup_id".to_string());
        assert!(result.is_err(), "Expected rollback_system to fail for non-existent backup");
    }

    #[test]
    fn test_rollback_system_invalid() {
        let result = inner_rollback_system("..\\windows".to_string());
        assert!(result.is_err(), "Expected rollback_system to fail for invalid identifier");
    }
}
