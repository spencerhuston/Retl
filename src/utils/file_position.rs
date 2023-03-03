#[derive(Clone, Debug)]
pub struct FilePosition {
    pub(crate) line: usize,
    pub(crate) column: usize,
    pub(crate) line_text: String
}

impl FilePosition {
    fn position() {
        () // TODO
    }
}