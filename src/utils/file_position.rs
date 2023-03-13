#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct FilePosition {
    pub(crate) line: usize,
    pub(crate) column: usize,
    pub(crate) line_text: String
}

impl FilePosition {
    pub fn position(&self) -> String {
        let mut pos_string = String::from("\n");
        pos_string += &("Line: ".to_string() + &self.line.to_string() + ", column: " + &self.column.to_string() + "\n");
        pos_string += &self.line_text;
        pos_string += "\n";
        pos_string += &"-".to_string().repeat(self.column);
        pos_string += &"^".to_string();
        pos_string
    }
}