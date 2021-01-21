use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct TemplateContext {
    pub page: String,
    pub message: String,
    pub ok: bool
}