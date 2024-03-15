use std::path::PathBuf;

#[derive(Debug, Clone, Default)]
pub struct Span {
    pub file_path: PathBuf,
    pub start: usize,
    pub end: usize,
}

impl PartialEq for Span {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Eq for Span {}
