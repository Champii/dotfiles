#[derive(Debug, Clone)]
pub struct ParseCtx {
    pub indent_level: u8,
    pub diagnostics: Vec<String>,
}

impl ParseCtx {
    pub fn new() -> Self {
        ParseCtx {
            indent_level: 0,
            diagnostics: Vec::new(),
        }
    }
}
