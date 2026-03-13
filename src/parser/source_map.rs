use crate::parser::lexer::Span;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SourceMap {
    pub file: String,
    spans: Vec<(usize, Span)>,
    line_offsets: Vec<usize>,
}

impl SourceMap {
    pub fn new(file: String) -> Self {
        SourceMap {
            file,
            spans: Vec::new(),
            line_offsets: Vec::new(),
        }
    }

    pub fn record(&mut self, start: usize, span: Span) {
        self.spans.push((start, span));
    }

    pub fn get(&self, index: usize) -> Option<&Span> {
        self.spans.get(index).map(|(_, s)| s)
    }

    pub fn build_line_offsets(&mut self, source: &str) {
        self.line_offsets.clear();
        self.line_offsets.push(0);
        for (i, c) in source.char_indices() {
            if c == '\n' {
                self.line_offsets.push(i + 1);
            }
        }
    }

    pub fn location(&self, pos: usize) -> (u32, u32) {
        for (i, &offset) in self.line_offsets.iter().enumerate().rev() {
            if pos >= offset {
                return ((i + 1) as u32, (pos - offset + 1) as u32);
            }
        }
        (1, 1)
    }
}
