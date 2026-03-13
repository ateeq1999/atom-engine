use std::iter::FromIterator;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: u32,
    pub col: u32,
}

impl Span {
    pub fn new(start: usize, end: usize, line: u32, col: u32) -> Self {
        Span {
            start,
            end,
            line,
            col,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Text(Span, String),
    OpenInterp(Span),
    OpenRawInterp(Span),
    CloseInterp(Span),
    CloseRawInterp(Span),
    OpenComment(Span),
    CloseComment(Span),
    At(Span),
    Bang(Span),
    Ident(Span, String),
    LParen(Span),
    RParen(Span),
    LBrace(Span),
    RBrace(Span),
    LBracket(Span),
    RBracket(Span),
    Comma(Span),
    Dot(Span),
    Colon(Span),
    Equals(Span),
    StringLit(Span, String),
    NumLit(Span, String),
    BoolLit(Span, bool),
    NullLit(Span),
    Whitespace(Span, String),
    Newline(Span),
    Eof(Span),
}

pub struct Lexer<'a> {
    source: &'a str,
    chars: std::iter::Peekable<std::str::Chars<'a>>,
    pos: usize,
    line: u32,
    col: u32,
    start_pos: usize,
    start_line: u32,
    start_col: u32,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Lexer {
            source: src,
            chars: src.chars().peekable(),
            pos: 0,
            line: 1,
            col: 1,
            start_pos: 0,
            start_line: 1,
            start_col: 1,
        }
    }

    fn make_span(&self) -> Span {
        Span::new(self.start_pos, self.pos, self.start_line, self.start_col)
    }

    fn make_token(&self, token: Token) -> Token {
        token
    }

    fn advance(&mut self) -> Option<char> {
        match self.chars.next() {
            Some(c) => {
                self.pos += 1;
                if c == '\n' {
                    self.line += 1;
                    self.col = 1;
                } else {
                    self.col += 1;
                }
                Some(c)
            }
            None => None,
        }
    }

    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    fn peek2(&mut self) -> (Option<char>, Option<char>) {
        let mut iter = self.chars.clone();
        let first = iter.next();
        let second = iter.next();
        (first, second)
    }

    fn matches(&mut self, expected: char) -> bool {
        if let Some(&c) = self.peek() {
            if c == expected {
                self.advance();
                return true;
            }
        }
        false
    }

    fn read_string(&mut self, delimiter: char) -> String {
        let mut result = String::new();
        loop {
            let c = match self.advance() {
                Some(c) => c,
                None => break,
            };

            if c == delimiter {
                break;
            }

            if c == '\\' {
                let next = self.peek().copied();
                match next {
                    Some('n') => {
                        self.advance();
                        result.push('\n');
                    }
                    Some('r') => {
                        self.advance();
                        result.push('\r');
                    }
                    Some('t') => {
                        self.advance();
                        result.push('\t');
                    }
                    Some('\\') => {
                        self.advance();
                        result.push('\\');
                    }
                    Some('\'') => {
                        self.advance();
                        result.push('\'');
                    }
                    Some('"') => {
                        self.advance();
                        result.push('"');
                    }
                    Some(other) => {
                        self.advance();
                        result.push('\\');
                        result.push(other);
                    }
                    None => {
                        result.push('\\');
                        break;
                    }
                }
            } else {
                result.push(c);
            }
        }
        result
    }

    fn read_number(&mut self, first: char) -> String {
        let mut result = String::new();
        result.push(first);

        loop {
            match self.peek() {
                Some(&c)
                    if c.is_ascii_digit()
                        || c == '.'
                        || c == 'e'
                        || c == 'E'
                        || c == '+'
                        || c == '-' =>
                {
                    result.push(c);
                    self.advance();
                }
                _ => break,
            }
        }
        result
    }

    fn read_ident(&mut self, first: char) -> String {
        let mut result = String::new();
        result.push(first);

        loop {
            match self.peek() {
                Some(&c) if c.is_ascii_alphanumeric() || c == '_' || c == '-' => {
                    result.push(c);
                    self.advance();
                }
                _ => break,
            }
        }
        result
    }

    fn skip_whitespace(&mut self) -> String {
        let mut result = String::new();
        loop {
            match self.peek() {
                Some(&c) if c == ' ' || c == '\t' || c == '\r' => {
                    result.push(c);
                    self.advance();
                }
                _ => break,
            }
        }
        result
    }

    fn read_text(&mut self) -> String {
        let mut result = String::new();
        loop {
            match self.peek() {
                Some(&c) => {
                    // Stop at special characters
                    if c == '{'
                        || c == '}'
                        || c == '@'
                        || c == '\n'
                        || c == ' '
                        || c == '\t'
                        || c == '\r'
                        || c == '('
                        || c == ')'
                        || c == '['
                        || c == ']'
                        || c == ','
                        || c == '.'
                        || c == ':'
                        || c == '='
                        || c == '"'
                        || c == '\''
                    {
                        break;
                    }
                    result.push(c);
                    self.advance();
                }
                None => break,
            }
        }
        result
    }

    fn read_text_until(&mut self, terminators: &[char]) -> String {
        let mut result = String::new();
        loop {
            match self.peek() {
                Some(&c) => {
                    if terminators.contains(&c) || c == '{' || c == '}' || c == '@' || c == '\n' {
                        break;
                    }
                    result.push(c);
                    self.advance();
                }
                None => break,
            }
        }
        result
    }

    pub fn next_token(&mut self) -> Token {
        self.start_pos = self.pos;
        self.start_line = self.line;
        self.start_col = self.col;

        let c = match self.peek() {
            Some(c) => *c,
            None => return self.make_token(Token::Eof(self.make_span())),
        };

        // Handle newlines first
        if c == '\n' {
            self.advance();
            return self.make_token(Token::Newline(self.make_span()));
        }

        // Handle whitespace
        if c == ' ' || c == '\t' || c == '\r' {
            let ws = self.skip_whitespace();
            return self.make_token(Token::Whitespace(self.make_span(), ws));
        }

        // Handle interpolation and comments: {{, {{{, {--
        if c == '{' {
            // Look ahead to see what we have: {{, {{{, {--
            let mut lookahead = self.chars.clone();
            let second = lookahead.next();
            let third = lookahead.next();
            let fourth = lookahead.next();

            // eprintln!("DEBUG: c={:?}, second={:?}, third={:?}, fourth={:?}", c, second, third, fourth);

            // We have {{
            if second == Some('{') {
                // Now check the fourth character (third position after opening)
                // {{name}}  -> 0:{, 1:{, 2:n, 3:a...
                // {{{name}} -> 0:{, 1:{, 2:{, 3:n...
                match fourth {
                    Some('-') => {
                        // Handle {{-
                        self.advance(); // {
                        self.advance(); // {
                        self.advance(); // -
                        return self.make_token(Token::OpenComment(self.make_span()));
                    }
                    Some('{') => {
                        // Handle {{{
                        self.advance(); // {
                        self.advance(); // {
                        self.advance(); // {
                        return self.make_token(Token::OpenRawInterp(self.make_span()));
                    }
                    _ => {
                        // Handle {{
                        self.advance(); // first {
                        self.advance(); // second {
                        return self.make_token(Token::OpenInterp(self.make_span()));
                    }
                }
            }
            // Just a single {
            self.advance();
            return self.make_token(Token::LBrace(self.make_span()));
        }

        // Handle closing interpolation: }}, }}}
        if c == '}' {
            let mut lookahead = self.chars.clone();
            let second = lookahead.next();
            let third = lookahead.next();
            let fourth = lookahead.next();

            if second == Some('}') {
                match fourth {
                    Some('-') => {
                        // Handle --}}
                        self.advance(); // }
                        self.advance(); // }
                        self.advance(); // -
                        return self.make_token(Token::CloseComment(self.make_span()));
                    }
                    Some('}') => {
                        // Handle }}}
                        self.advance(); // }
                        self.advance(); // }
                        self.advance(); // }
                        return self.make_token(Token::CloseRawInterp(self.make_span()));
                    }
                    _ => {
                        // Handle }}
                        self.advance(); // first }
                        self.advance(); // second }
                        return self.make_token(Token::CloseInterp(self.make_span()));
                    }
                }
            }
            self.advance();
            return self.make_token(Token::RBrace(self.make_span()));
        }

        // Handle @ directive
        if c == '@' {
            self.advance();
            match self.peek() {
                Some(&'!') => {
                    self.advance();
                    return self.make_token(Token::Bang(self.make_span()));
                }
                _ => return self.make_token(Token::At(self.make_span())),
            }
        }

        // Handle string literals
        if c == '"' || c == '\'' {
            self.advance();
            let content = self.read_string(c);
            return self.make_token(Token::StringLit(self.make_span(), content));
        }

        // Handle numbers
        if c.is_ascii_digit() {
            let first = self.advance().unwrap();
            let num_str = self.read_number(first);
            return self.make_token(Token::NumLit(self.make_span(), num_str));
        }

        // Handle negative numbers
        if c == '-' {
            let (second, _) = self.peek2();
            if second.map_or(false, |c| c.is_ascii_digit()) {
                self.advance(); // consume -
                let first = self.advance().unwrap();
                let num_str = self.read_number(first);
                return self.make_token(Token::NumLit(self.make_span(), format!("-{}", num_str)));
            }
        }

        // Handle identifiers and keywords
        if c.is_alphabetic() || c == '_' {
            let first = self.advance().unwrap();
            let ident = self.read_ident(first);

            match ident.as_str() {
                "true" => return self.make_token(Token::BoolLit(self.make_span(), true)),
                "false" => return self.make_token(Token::BoolLit(self.make_span(), false)),
                "null" => return self.make_token(Token::NullLit(self.make_span())),
                _ => return self.make_token(Token::Ident(self.make_span(), ident)),
            }
        }

        // Handle text (catch-all for any other characters)
        let text = self.read_text();
        if !text.is_empty() {
            return self.make_token(Token::Text(self.make_span(), text));
        }

        // Handle punctuation
        let result = c.to_string();
        self.advance();

        match c {
            '(' => return self.make_token(Token::LParen(self.make_span())),
            ')' => return self.make_token(Token::RParen(self.make_span())),
            '[' => return self.make_token(Token::LBracket(self.make_span())),
            ']' => return self.make_token(Token::RBracket(self.make_span())),
            ',' => return self.make_token(Token::Comma(self.make_span())),
            '.' => return self.make_token(Token::Dot(self.make_span())),
            ':' => return self.make_token(Token::Colon(self.make_span())),
            '=' => return self.make_token(Token::Equals(self.make_span())),
            _ => return self.make_token(Token::Text(self.make_span(), result)),
        }
    }

    pub fn tokenize(src: &str) -> Vec<Token> {
        let mut lexer = Lexer::new(src);
        let mut tokens = Vec::new();

        loop {
            let token = lexer.next_token();
            if matches!(&token, Token::Eof(_)) {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }

        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_token() {
        let tokens = Lexer::tokenize("hello world");
        // Plain text is tokenized as identifiers + whitespace
        assert!(matches!(&tokens[0], Token::Ident(_, s) if s == "hello"));
        assert!(matches!(&tokens[1], Token::Whitespace(_, s) if s == " "));
        assert!(matches!(&tokens[2], Token::Ident(_, s) if s == "world"));
    }

    #[test]
    fn test_interpolation() {
        let tokens = Lexer::tokenize("{{name}}");
        assert!(matches!(&tokens[0], Token::OpenInterp(_)));
        assert!(matches!(&tokens[1], Token::Ident(_, s) if s == "name"));
        assert!(matches!(&tokens[2], Token::CloseInterp(_)));
    }

    #[test]
    fn test_raw_interpolation() {
        let tokens = Lexer::tokenize("{{{html}}}");
        assert!(matches!(&tokens[0], Token::OpenRawInterp(_)));
        assert!(matches!(&tokens[1], Token::Ident(_, s) if s == "html"));
        assert!(matches!(&tokens[2], Token::CloseRawInterp(_)));
    }

    #[test]
    fn test_string_escapes() {
        let tokens = Lexer::tokenize("\"hello\\nworld\"");
        assert!(matches!(&tokens[0], Token::StringLit(_, s) if s == "hello\nworld"));
    }

    #[test]
    fn test_span_tracking() {
        let tokens = Lexer::tokenize("abc");
        if let Token::Ident(span, _) = &tokens[0] {
            assert_eq!(span.start, 0);
            assert_eq!(span.end, 3);
            assert_eq!(span.line, 1);
            assert_eq!(span.col, 1);
        } else {
            panic!("Expected Ident token");
        }
    }
}
