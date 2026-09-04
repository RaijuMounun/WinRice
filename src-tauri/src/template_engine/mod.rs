pub mod renderer;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TemplateError {
    #[error("Template rendering error: {0}")]
    RenderError(String),
    #[error("File operation error: {0}")]
    FileError(String),
}

pub use renderer::render_template;
pub use renderer::render_and_deploy;
