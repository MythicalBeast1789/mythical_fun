use serde::{Serialize};
#[derive(Serialize)]
pub struct TemplateContext {
    pub page: String,
    pub ok: bool,
    pub message: String,
}