//! Document management for the language server

use ropey::Rope;
use solscript_ast::Program;

/// Represents an open document in the editor
pub struct Document {
    /// The document text
    pub text: String,
    /// Rope for efficient text manipulation
    pub rope: Rope,
    /// Document version
    pub version: i32,
    /// Cached parsed AST (if parsing succeeded)
    pub ast: Option<Program>,
    /// Parse errors (if any)
    pub parse_errors: Vec<String>,
    /// Type check errors (if any)
    pub type_errors: Vec<solscript_typeck::TypeError>,
}

impl Document {
    /// Create a new document
    pub fn new(text: String, version: i32) -> Self {
        let rope = Rope::from_str(&text);
        let mut doc = Self {
            text: text.clone(),
            rope,
            version,
            ast: None,
            parse_errors: Vec::new(),
            type_errors: Vec::new(),
        };
        doc.analyze();
        doc
    }

    /// Update the document content
    pub fn update(&mut self, text: String, version: i32) {
        self.text = text.clone();
        self.rope = Rope::from_str(&text);
        self.version = version;
        self.analyze();
    }

    /// Analyze the document (parse and type check)
    fn analyze(&mut self) {
        self.parse_errors.clear();
        self.type_errors.clear();
        self.ast = None;

        // Parse
        match solscript_parser::parse(&self.text) {
            Ok(program) => {
                // Type check
                if let Err(errors) = solscript_typeck::typecheck(&program, &self.text) {
                    self.type_errors = errors;
                }
                self.ast = Some(program);
            }
            Err(e) => {
                self.parse_errors.push(format!("{:?}", e));
            }
        }
    }

    /// Get the byte offset for a position
    pub fn offset_at(&self, line: u32, character: u32) -> Option<usize> {
        let line_idx = line as usize;
        if line_idx >= self.rope.len_lines() {
            return None;
        }

        let line_start = self.rope.line_to_byte(line_idx);
        let line_text = self.rope.line(line_idx);
        let char_offset = (character as usize).min(line_text.len_chars());

        Some(line_start + char_offset)
    }

    /// Get the position for a byte offset
    pub fn position_at(&self, offset: usize) -> (u32, u32) {
        let line = self.rope.byte_to_line(offset);
        let line_start = self.rope.line_to_byte(line);
        let character = offset - line_start;
        (line as u32, character as u32)
    }

    /// Get the word at a position
    pub fn word_at(&self, line: u32, character: u32) -> Option<String> {
        let offset = self.offset_at(line, character)?;

        // Find word boundaries
        let bytes = self.text.as_bytes();
        let mut start = offset;
        let mut end = offset;

        // Scan backwards to find start of word
        while start > 0 && is_identifier_char(bytes[start - 1] as char) {
            start -= 1;
        }

        // Scan forwards to find end of word
        while end < bytes.len() && is_identifier_char(bytes[end] as char) {
            end += 1;
        }

        if start < end {
            Some(self.text[start..end].to_string())
        } else {
            None
        }
    }

    /// Get the line text at a line number
    pub fn line_text(&self, line: u32) -> Option<String> {
        let line_idx = line as usize;
        if line_idx >= self.rope.len_lines() {
            return None;
        }
        Some(self.rope.line(line_idx).to_string())
    }
}

fn is_identifier_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}
