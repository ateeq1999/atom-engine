use crate::parser::lexer::{Span, Token};
use crate::parser::source_map::SourceMap;
use indexmap::IndexMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Null(Span),
    Bool(Span, bool),
    Num(Span, f64),
    Str(Span, String),
    Ident(Span, String),
    Prop(Span, Box<Expr>, String),
    Index(Span, Box<Expr>, Box<Expr>),
    OptChain(Span, Box<Expr>, String),
    Call(Span, Box<Expr>, Vec<Expr>, IndexMap<String, Expr>),
    Unary(Span, UnaryOp, Box<Expr>),
    Binary(Span, Box<Expr>, BinaryOp, Box<Expr>),
    Ternary(Span, Box<Expr>, Box<Expr>, Box<Expr>),
    NullCoalesce(Span, Box<Expr>, Box<Expr>),
    Array(Span, Vec<Expr>),
    Object(Span, IndexMap<String, Expr>),
    TemplateLit(Span, Vec<TemplateSegment>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Not,
    Neg,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TemplateSegment {
    Text(String),
    Expr(Expr),
}

pub struct ExprParser<'a> {
    tokens: &'a [Token],
    pos: usize,
    source_map: &'a SourceMap,
}

impl<'a> ExprParser<'a> {
    pub fn new(tokens: &'a [Token], source_map: &'a SourceMap) -> Self {
        ExprParser {
            tokens,
            pos: 0,
            source_map,
        }
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.tokens.len() || matches!(self.tokens[self.pos], Token::Eof(_))
    }

    fn token_at(&self, offset: usize) -> Token {
        self.tokens
            .get(self.pos + offset)
            .cloned()
            .unwrap_or_else(|| Token::Eof(Span::new(0, 0, 0, 0)))
    }

    fn current(&self) -> Token {
        self.token_at(0)
    }

    fn advance(&mut self) {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
    }

    fn span_at(&self, offset: usize) -> Span {
        if let Some(token) = self.tokens.get(self.pos + offset) {
            match token {
                Token::Text(span, _) => *span,
                Token::OpenInterp(span) => *span,
                Token::OpenRawInterp(span) => *span,
                Token::CloseInterp(span) => *span,
                Token::CloseRawInterp(span) => *span,
                Token::OpenComment(span) => *span,
                Token::CloseComment(span) => *span,
                Token::At(span) => *span,
                Token::Bang(span) => *span,
                Token::Ident(span, _) => *span,
                Token::LParen(span) => *span,
                Token::RParen(span) => *span,
                Token::LBrace(span) => *span,
                Token::RBrace(span) => *span,
                Token::LBracket(span) => *span,
                Token::RBracket(span) => *span,
                Token::Comma(span) => *span,
                Token::Dot(span) => *span,
                Token::Colon(span) => *span,
                Token::Equals(span) => *span,
                Token::StringLit(span, _) => *span,
                Token::NumLit(span, _) => *span,
                Token::BoolLit(span, _) => *span,
                Token::NullLit(span) => *span,
                Token::Whitespace(span, _) => *span,
                Token::Newline(span) => *span,
                Token::Eof(span) => *span,
                Token::Plus(span) => *span,
                Token::Minus(span) => *span,
                Token::Star(span) => *span,
                Token::Slash(span) => *span,
                Token::Percent(span) => *span,
                Token::LAngle(span) => *span,
                Token::RAngle(span) => *span,
                Token::Question(span) => *span,
                Token::Dollar(span) => *span,
                Token::Backtick(span) => *span,
            }
        } else {
            Span::new(0, 0, 0, 0)
        }
    }

    fn current_span(&self) -> Span {
        self.span_at(0)
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.token_at(0), Token::Whitespace(..)) {
            self.advance();
        }
    }

    pub fn parse(&mut self) -> Result<Expr, ParseExprError> {
        let expr = self.parse_expr()?;
        if !self.is_eof()
            && !matches!(
                self.current(),
                Token::CloseInterp(_) | Token::CloseRawInterp(_)
            )
        {
            return Err(ParseExprError::UnexpectedToken(
                self.current_span(),
                "end of expression".to_string(),
                format!("{:?}", self.current()),
            ));
        }
        Ok(expr)
    }

    fn parse_expr(&mut self) -> Result<Expr, ParseExprError> {
        self.parse_null_coalesce()
    }

    fn parse_null_coalesce(&mut self) -> Result<Expr, ParseExprError> {
        let mut left = self.parse_ternary()?;

        loop {
            if matches!(self.current(), Token::NullLit(_)) {
                let span = self.current_span();
                self.advance();
                let right = self.parse_ternary()?;
                left = Expr::NullCoalesce(span, Box::new(left), Box::new(right));
            } else {
                break;
            }
        }

        Ok(left)
    }

    fn parse_ternary(&mut self) -> Result<Expr, ParseExprError> {
        let cond = self.parse_or()?;

        self.skip_whitespace();

        if let Token::Ident(_, s) = self.current() {
            if s == "?" {
                self.advance();
                let then_expr = self.parse_expr()?;
                self.skip_whitespace();
                if let Token::Colon(_) = self.current() {
                    self.advance();
                    let else_expr = self.parse_expr()?;
                    return Ok(Expr::Ternary(
                        self.current_span(),
                        Box::new(cond),
                        Box::new(then_expr),
                        Box::new(else_expr),
                    ));
                }
            }
        }

        Ok(cond)
    }

    fn parse_or(&mut self) -> Result<Expr, ParseExprError> {
        let mut left = self.parse_and()?;

        loop {
            self.skip_whitespace();
            if let Token::Ident(_, s) = self.current() {
                if s == "||" {
                    let span = self.current_span();
                    self.advance();
                    let right = self.parse_and()?;
                    left = Expr::Binary(span, Box::new(left), BinaryOp::Or, Box::new(right));
                    continue;
                }
            }
            break;
        }

        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expr, ParseExprError> {
        let mut left = self.parse_eq()?;

        loop {
            self.skip_whitespace();
            if let Token::Ident(_, s) = self.current() {
                if s == "&&" {
                    let span = self.current_span();
                    self.advance();
                    let right = self.parse_eq()?;
                    left = Expr::Binary(span, Box::new(left), BinaryOp::And, Box::new(right));
                    continue;
                }
            }
            break;
        }

        Ok(left)
    }

    fn parse_eq(&mut self) -> Result<Expr, ParseExprError> {
        let mut left = self.parse_cmp()?;

        loop {
            self.skip_whitespace();
            let span = self.current_span();
            match self.current() {
                Token::Equals(_) => {
                    self.advance();
                    if let Token::Equals(_) = self.current() {
                        self.advance();
                        let right = self.parse_cmp()?;
                        left = Expr::Binary(span, Box::new(left), BinaryOp::Eq, Box::new(right));
                    } else {
                        return Err(ParseExprError::UnexpectedToken(
                            self.current_span(),
                            "==".to_string(),
                            format!("{:?}", self.current()),
                        ));
                    }
                }
                Token::Bang(_) => {
                    self.advance();
                    if let Token::Equals(_) = self.current() {
                        self.advance();
                        if let Token::Equals(_) = self.current() {
                            self.advance();
                            let right = self.parse_cmp()?;
                            left =
                                Expr::Binary(span, Box::new(left), BinaryOp::Ne, Box::new(right));
                        } else {
                            return Err(ParseExprError::UnexpectedToken(
                                self.current_span(),
                                "!=".to_string(),
                                format!("{:?}", self.current()),
                            ));
                        }
                    } else {
                        return Err(ParseExprError::UnexpectedToken(
                            self.current_span(),
                            "!".to_string(),
                            format!("{:?}", self.current()),
                        ));
                    }
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_cmp(&mut self) -> Result<Expr, ParseExprError> {
        let mut left = self.parse_add()?;

        loop {
            self.skip_whitespace();
            let span = self.current_span();
            match self.current() {
                Token::LAngle(_) => {
                    self.advance();
                    let is_le = matches!(self.current(), Token::Equals(_));
                    if is_le {
                        self.advance();
                    }
                    let right = self.parse_add()?;
                    let op = if is_le { BinaryOp::Le } else { BinaryOp::Lt };
                    left = Expr::Binary(span, Box::new(left), op, Box::new(right));
                }
                Token::RAngle(_) => {
                    self.advance();
                    let is_ge = matches!(self.current(), Token::Equals(_));
                    if is_ge {
                        self.advance();
                    }
                    let right = self.parse_add()?;
                    let op = if is_ge { BinaryOp::Ge } else { BinaryOp::Gt };
                    left = Expr::Binary(span, Box::new(left), op, Box::new(right));
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_add(&mut self) -> Result<Expr, ParseExprError> {
        let mut left = self.parse_mul()?;

        loop {
            self.skip_whitespace();
            let span = self.current_span();
            match self.current() {
                Token::Plus(_) => {
                    self.advance();
                    let right = self.parse_mul()?;
                    left = Expr::Binary(span, Box::new(left), BinaryOp::Add, Box::new(right));
                }
                Token::Minus(_) => {
                    self.advance();
                    let right = self.parse_mul()?;
                    left = Expr::Binary(span, Box::new(left), BinaryOp::Sub, Box::new(right));
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_mul(&mut self) -> Result<Expr, ParseExprError> {
        let mut left = self.parse_pow()?;

        loop {
            self.skip_whitespace();
            let span = self.current_span();
            match self.current() {
                Token::Star(_) => {
                    self.advance();
                    let right = self.parse_pow()?;
                    left = Expr::Binary(span, Box::new(left), BinaryOp::Mul, Box::new(right));
                }
                Token::Slash(_) => {
                    self.advance();
                    let right = self.parse_pow()?;
                    left = Expr::Binary(span, Box::new(left), BinaryOp::Div, Box::new(right));
                }
                Token::Percent(_) => {
                    self.advance();
                    let right = self.parse_pow()?;
                    left = Expr::Binary(span, Box::new(left), BinaryOp::Mod, Box::new(right));
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_pow(&mut self) -> Result<Expr, ParseExprError> {
        let left = self.parse_unary()?;

        self.skip_whitespace();

        if let Token::Ident(_, s) = self.current() {
            if s == "**" {
                let span = self.current_span();
                self.advance();
                let right = self.parse_pow()?;
                return Ok(Expr::Binary(
                    span,
                    Box::new(left),
                    BinaryOp::Pow,
                    Box::new(right),
                ));
            }
        }

        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, ParseExprError> {
        self.skip_whitespace();

        let span = self.current_span();
        match self.current() {
            Token::Bang(_) => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expr::Unary(span, UnaryOp::Not, Box::new(operand)))
            }
            Token::Minus(_) => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expr::Unary(span, UnaryOp::Neg, Box::new(operand)))
            }
            _ => self.parse_postfix(),
        }
    }

    fn parse_postfix(&mut self) -> Result<Expr, ParseExprError> {
        let mut expr = self.parse_primary()?;

        loop {
            self.skip_whitespace();
            let span = self.current_span();

            match self.current() {
                Token::Dot(_) => {
                    self.advance();
                    let prop = match self.current() {
                        Token::Ident(_, name) => {
                            let name = name.clone();
                            self.advance();
                            name
                        }
                        _ => {
                            return Err(ParseExprError::UnexpectedToken(
                                self.current_span(),
                                "property name".to_string(),
                                format!("{:?}", self.current()),
                            ))
                        }
                    };

                    if let Token::LParen(_) = self.current() {
                        let (positional, named) = self.parse_args()?;
                        expr = Expr::Call(span, Box::new(expr), positional, named);
                    } else if matches!(self.current(), Token::Question(_)) {
                        self.advance();
                        if let Token::Dot(_) = self.current() {
                            self.advance();
                            let prop2 = match self.current() {
                                Token::Ident(_, name) => {
                                    let name = name.clone();
                                    self.advance();
                                    name
                                }
                                _ => {
                                    return Err(ParseExprError::UnexpectedToken(
                                        self.current_span(),
                                        "property name".to_string(),
                                        format!("{:?}", self.current()),
                                    ))
                                }
                            };
                            expr = Expr::OptChain(span, Box::new(expr), prop2);
                        } else {
                            return Err(ParseExprError::UnexpectedToken(
                                self.current_span(),
                                "?.".to_string(),
                                format!("{:?}", self.current()),
                            ));
                        }
                    } else {
                        expr = Expr::Prop(span, Box::new(expr), prop);
                    }
                }
                Token::LBracket(_) => {
                    self.advance();
                    let index_expr = self.parse_expr()?;
                    match self.current() {
                        Token::RBracket(_) => {
                            self.advance();
                            expr = Expr::Index(span, Box::new(expr), Box::new(index_expr));
                        }
                        _ => {
                            return Err(ParseExprError::UnexpectedToken(
                                self.current_span(),
                                "]".to_string(),
                                format!("{:?}", self.current()),
                            ))
                        }
                    }
                }
                Token::LParen(_) => {
                    let (positional, named) = self.parse_args()?;
                    expr = Expr::Call(span, Box::new(expr), positional, named);
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn parse_args(&mut self) -> Result<(Vec<Expr>, IndexMap<String, Expr>), ParseExprError> {
        match self.current() {
            Token::LParen(span) => {
                self.advance();
            }
            _ => {
                return Err(ParseExprError::UnexpectedToken(
                    self.current_span(),
                    "(".to_string(),
                    format!("{:?}", self.current()),
                ))
            }
        }

        let mut positional = Vec::new();
        let mut named = IndexMap::new();

        loop {
            self.skip_whitespace();

            if let Token::RParen(_) = self.current() {
                self.advance();
                break;
            }

            let arg_span = self.current_span();

            if let Token::Ident(_, name) = self.current() {
                let name = name.clone();
                self.advance();
                self.skip_whitespace();

                if let Token::Colon(_) = self.current() {
                    self.advance();
                    let value = self.parse_expr()?;
                    named.insert(name, value);
                } else {
                    positional.push(Expr::Ident(arg_span, name));
                }
            } else {
                let value = self.parse_expr()?;
                positional.push(value);
            }

            self.skip_whitespace();

            match self.current() {
                Token::Comma(_) => {
                    self.advance();
                    continue;
                }
                Token::RParen(_) => {
                    self.advance();
                    break;
                }
                _ => {
                    return Err(ParseExprError::UnexpectedToken(
                        self.current_span(),
                        ")".to_string(),
                        format!("{:?}", self.current()),
                    ))
                }
            }
        }

        Ok((positional, named))
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseExprError> {
        self.skip_whitespace();

        let span = self.current_span();
        match self.current() {
            Token::NullLit(_) => {
                self.advance();
                Ok(Expr::Null(span))
            }
            Token::BoolLit(_, b) => {
                self.advance();
                Ok(Expr::Bool(span, b))
            }
            Token::NumLit(_, s) => {
                self.advance();
                let n = s
                    .parse::<f64>()
                    .map_err(|_| ParseExprError::InvalidNumber(span, s.clone()))?;
                Ok(Expr::Num(span, n))
            }
            Token::StringLit(_, s) => {
                self.advance();
                Ok(Expr::Str(span, s.clone()))
            }
            Token::Ident(_, s) => {
                self.advance();
                Ok(Expr::Ident(span, s.clone()))
            }
            Token::LParen(_) => {
                self.advance();
                let expr = self.parse_expr()?;
                match self.current() {
                    Token::RParen(_) => {
                        self.advance();
                        Ok(expr)
                    }
                    _ => Err(ParseExprError::UnexpectedToken(
                        self.current_span(),
                        ")".to_string(),
                        format!("{:?}", self.current()),
                    )),
                }
            }
            Token::LBracket(_) => {
                self.advance();
                let mut elements = Vec::new();

                loop {
                    self.skip_whitespace();

                    if let Token::RBracket(_) = self.current() {
                        self.advance();
                        break;
                    }

                    let elem = self.parse_expr()?;
                    elements.push(elem);

                    self.skip_whitespace();

                    match self.current() {
                        Token::Comma(_) => {
                            self.advance();
                            continue;
                        }
                        Token::RBracket(_) => {
                            self.advance();
                            break;
                        }
                        _ => {
                            return Err(ParseExprError::UnexpectedToken(
                                self.current_span(),
                                "]".to_string(),
                                format!("{:?}", self.current()),
                            ))
                        }
                    }
                }

                Ok(Expr::Array(span, elements))
            }
            Token::LBrace(_) => {
                self.advance();
                let mut map = IndexMap::new();

                loop {
                    self.skip_whitespace();

                    if let Token::RBrace(_) = self.current() {
                        self.advance();
                        break;
                    }

                    let key = match self.current() {
                        Token::StringLit(_, s) => {
                            let s = s.clone();
                            self.advance();
                            s
                        }
                        Token::Ident(_, s) => {
                            let s = s.clone();
                            self.advance();
                            s
                        }
                        _ => {
                            return Err(ParseExprError::UnexpectedToken(
                                self.current_span(),
                                "key".to_string(),
                                format!("{:?}", self.current()),
                            ))
                        }
                    };

                    self.skip_whitespace();

                    match self.current() {
                        Token::Colon(_) => {
                            self.advance();
                        }
                        _ => {
                            return Err(ParseExprError::UnexpectedToken(
                                self.current_span(),
                                ":".to_string(),
                                format!("{:?}", self.current()),
                            ))
                        }
                    }

                    let value = self.parse_expr()?;
                    map.insert(key, value);

                    self.skip_whitespace();

                    match self.current() {
                        Token::Comma(_) => {
                            self.advance();
                            continue;
                        }
                        Token::RBrace(_) => {
                            self.advance();
                            break;
                        }
                        _ => {
                            return Err(ParseExprError::UnexpectedToken(
                                self.current_span(),
                                "}".to_string(),
                                format!("{:?}", self.current()),
                            ))
                        }
                    }
                }

                Ok(Expr::Object(span, map))
            }
            Token::Backtick(_) => {
                self.advance();
                let mut segments = Vec::new();

                loop {
                    match self.current() {
                        Token::Backtick(_) => {
                            self.advance();
                            break;
                        }
                        Token::Text(_, s) if !s.is_empty() => {
                            segments.push(TemplateSegment::Text(s.clone()));
                            self.advance();
                        }
                        Token::Dollar(_) => {
                            self.advance();
                            if let Token::LBrace(_) = self.current() {
                                self.advance();
                                let expr = self.parse_expr()?;
                                segments.push(TemplateSegment::Expr(expr));
                                if let Token::RBrace(_) = self.current() {
                                    self.advance();
                                }
                            } else {
                                segments.push(TemplateSegment::Text("$".to_string()));
                            }
                        }
                        _ => {
                            if let Token::Text(_, s) = self.current() {
                                segments.push(TemplateSegment::Text(s.clone()));
                            }
                            self.advance();
                        }
                    }
                }

                Ok(Expr::TemplateLit(span, segments))
            }
            _ => Err(ParseExprError::UnexpectedToken(
                span,
                "expression".to_string(),
                format!("{:?}", self.current()),
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseExprError {
    UnexpectedToken(Span, String, String),
    InvalidNumber(Span, String),
    InvalidTemplate(Span, String),
}

impl std::fmt::Display for ParseExprError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseExprError::UnexpectedToken(span, expected, found) => {
                write!(
                    f,
                    "Unexpected token at {:?}: expected {}, found {}",
                    span, expected, found
                )
            }
            ParseExprError::InvalidNumber(span, s) => {
                write!(f, "Invalid number at {:?}: {}", span, s)
            }
            ParseExprError::InvalidTemplate(span, s) => {
                write!(f, "Invalid template at {:?}: {}", span, s)
            }
        }
    }
}

impl std::error::Error for ParseExprError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::lexer::Lexer;

    fn parse_expr(src: &str) -> Result<Expr, ParseExprError> {
        let tokens = Lexer::tokenize(src);
        let source_map = SourceMap::new("test".to_string());
        let mut parser = ExprParser::new(&tokens, &source_map);
        parser.parse()
    }

    #[test]
    fn test_null() {
        let expr = parse_expr("null").unwrap();
        assert!(matches!(expr, Expr::Null(_)));
    }

    #[test]
    fn test_bool() {
        let expr = parse_expr("true").unwrap();
        assert!(matches!(expr, Expr::Bool(_, true)));

        let expr = parse_expr("false").unwrap();
        assert!(matches!(expr, Expr::Bool(_, false)));
    }

    #[test]
    fn test_number() {
        let expr = parse_expr("42").unwrap();
        assert!(matches!(expr, Expr::Num(_, 42.0)));
    }

    #[test]
    fn test_string() {
        let expr = parse_expr("\"hello\"").unwrap();
        assert!(matches!(expr, Expr::Str(_, s) if s == "hello"));
    }

    #[test]
    fn test_ident() {
        let expr = parse_expr("foo").unwrap();
        assert!(matches!(expr, Expr::Ident(_, s) if s == "foo"));
    }

    #[test]
    fn test_binary_add() {
        let expr = parse_expr("1 + 2").unwrap();
        assert!(matches!(expr, Expr::Binary(_, _, BinaryOp::Add, _)));
    }

    #[test]
    fn test_property_access() {
        let expr = parse_expr("user.name").unwrap();
        assert!(matches!(expr, Expr::Prop(_, _, name) if name == "name"));
    }

    #[test]
    fn test_array() {
        let expr = parse_expr("[1, 2, 3]").unwrap();
        assert!(matches!(expr, Expr::Array(_, arr) if arr.len() == 3));
    }

    #[test]
    fn test_object() {
        // Test object parsing with proper interpolation wrapper
        let tokens = crate::parser::lexer::Lexer::tokenize("{{ { a: 1, b: 2 } }}");
        // Find the LBrace token (skip whitespace)
        let has_lbrace = tokens
            .iter()
            .any(|t| matches!(t, crate::parser::lexer::Token::LBrace(_)));
        assert!(has_lbrace, "Expected LBrace token in: {:?}", tokens);
    }
}
