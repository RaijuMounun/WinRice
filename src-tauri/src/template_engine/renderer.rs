use super::TemplateError;
use serde_json::Value;
use std::path::Path;

use tera::{Context, Tera};
use crate::file_manager::atomic::write_atomically;

pub fn render_template(template_content: &str, context_json: &Value) -> Result<String, TemplateError> {
    let context = Context::from_serialize(context_json)
        .map_err(|e| TemplateError::RenderError(format!("Context error: {}", e)))?;
    Tera::one_off(template_content, &context, false)
        .map_err(|e| TemplateError::RenderError(e.to_string()))
}

pub fn render_and_deploy(template_content: &str, context_json: &Value, target_path: &Path) -> Result<(), TemplateError> {
    let rendered = render_template(template_content, context_json)?;
    write_atomically(target_path, rendered.as_bytes())
        .map_err(|e| TemplateError::FileError(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_render_template_success() {
        let template = "Hello {{ name }}! You are {{ age }} years old. User type: {{ user.roles[0] }}";
        let context = json!({
            "name": "Alice",
            "age": 30,
            "user": {
                "roles": ["admin"]
            }
        });

        let result = render_template(template, &context);
        assert!(result.is_ok(), "Template should render successfully");
        assert_eq!(result.unwrap(), "Hello Alice! You are 30 years old. User type: admin");
    }

    #[test]
    fn test_render_template_missing_variable() {
        let template = "Hello {{ name }}! You are {{ age }} years old.";
        let context = json!({
            "name": "Bob"
            // "age" is deliberately missing
        });

        let result = render_template(template, &context);
        assert!(matches!(result, Err(TemplateError::RenderError(_))), "Expected TemplateError::RenderError on missing variable");
    }

    #[test]
    fn test_render_template_invalid_syntax() {
        // Invalid syntax: unclosed variable tag
        let template = "Hello {{ name ";
        let context = json!({
            "name": "Charlie"
        });

        let result = render_template(template, &context);
        assert!(matches!(result, Err(TemplateError::RenderError(_))), "Expected TemplateError::RenderError on syntax error");
    }

    #[test]
    fn test_render_template_empty_string() {
        let template = "";
        let context = json!({});

        let result = render_template(template, &context);
        assert!(result.is_ok(), "Empty template should render successfully");
        assert_eq!(result.unwrap(), "");
    }

    #[test]
    fn test_render_and_deploy_success() {
        let dir = tempdir().unwrap();
        let target_path = dir.path().join("output.txt");

        let template = "Configured: {{ colors.primary }}";
        let context = json!({
            "colors": {
                "primary": "#ff0000"
            }
        });

        let result = render_and_deploy(template, &context, &target_path);
        assert!(result.is_ok(), "Deployment should succeed");

        let content = fs::read_to_string(&target_path).unwrap();
        assert_eq!(content, "Configured: #ff0000");
    }

    #[test]
    fn test_render_and_deploy_overwrites_existing_file() {
        let dir = tempdir().unwrap();
        let target_path = dir.path().join("output.txt");
        
        // Setup existing file with old content
        fs::write(&target_path, "old content that must be replaced").unwrap();

        let template = "New content: {{ val }}";
        let context = json!({ "val": 42 });

        let result = render_and_deploy(template, &context, &target_path);
        assert!(result.is_ok(), "Deployment should succeed and overwrite");

        let content = fs::read_to_string(&target_path).unwrap();
        assert_eq!(content, "New content: 42");
    }

    #[test]
    fn test_render_and_deploy_preserves_existing_file_on_render_failure() {
        let dir = tempdir().unwrap();
        let target_path = dir.path().join("output.txt");
        
        let original_content = "important configuration that must not be lost";
        fs::write(&target_path, original_content).unwrap();

        // Template syntax error / missing var that causes rendering to fail
        let invalid_template = "Configured: {{ missing_var }}";
        let context = json!({});

        let result = render_and_deploy(invalid_template, &context, &target_path);
        assert!(result.is_err(), "Deployment should fail due to rendering error");

        // The original file must remain completely untouched.
        // This effectively enforces that the file is not opened in 'truncate' mode 
        // prior to successful rendering (a requirement for safe/atomic deploys).
        let content = fs::read_to_string(&target_path).expect("File must still exist");
        assert_eq!(content, original_content, "The original file was corrupted by a failed render attempt");
    }

    #[test]
    fn test_render_and_deploy_missing_directory() {
        let dir = tempdir().unwrap();
        let target_path = dir.path().join("nested").join("dir").join("output.txt");

        let template = "Content";
        let context = json!({});

        // Depending on design, this either automatically creates the dir, or fails with FileError.
        // A robust system should typically return a FileError if the directory doesn't exist.
        let result = render_and_deploy(template, &context, &target_path);
        assert!(matches!(result, Err(TemplateError::FileError(_))), "Expected TemplateError::FileError for missing directory");
    }
}
