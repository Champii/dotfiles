use std::path::PathBuf;

#[derive(Debug, Clone, Default)]
pub struct Span {
    pub file_path: PathBuf,
    pub start: usize,
    pub end: usize,
}
