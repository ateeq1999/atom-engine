use std::fmt;

#[derive(Clone)]
pub struct OutputBuffer {
    inner: String,
}

impl OutputBuffer {
    pub fn new() -> Self {
        OutputBuffer {
            inner: String::new(),
        }
    }

    pub fn push(&mut self, s: &str) {
        self.inner.push_str(s);
    }

    pub fn push_str(&mut self, s: &str) {
        self.inner.push_str(s);
    }

    pub fn push_escaped(&mut self, s: &str) {
        self.inner.push_str(&Self::escape_html(s));
    }

    pub fn push_char(&mut self, c: char) {
        self.inner.push(c);
    }

    pub fn finish(self) -> String {
        self.inner
    }

    pub fn as_str(&self) -> &str {
        &self.inner
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    pub fn escape_html(s: &str) -> String {
        let mut result = String::with_capacity(s.len());
        for c in s.chars() {
            match c {
                '&' => result.push_str("&amp;"),
                '<' => result.push_str("&lt;"),
                '>' => result.push_str("&gt;"),
                '"' => result.push_str("&quot;"),
                '\'' => result.push_str("&#39;"),
                _ => result.push(c),
            }
        }
        result
    }
}

impl Default for OutputBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for OutputBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "OutputBuffer({:?})", self.inner)
    }
}

impl fmt::Display for OutputBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_and_finish() {
        let mut buf = OutputBuffer::new();
        buf.push("hello");
        buf.push(" world");
        assert_eq!(buf.finish(), "hello world");
    }

    #[test]
    fn test_push_char() {
        let mut buf = OutputBuffer::new();
        buf.push("hello");
        buf.push_char('!');
        assert_eq!(buf.finish(), "hello!");
    }

    #[test]
    fn test_escape_html() {
        assert_eq!(
            OutputBuffer::escape_html("&<>\"'"),
            "&amp;&lt;&gt;&quot;&#39;"
        );
    }

    #[test]
    fn test_escape_html_no_escapes() {
        assert_eq!(OutputBuffer::escape_html("hello world"), "hello world");
    }
}
