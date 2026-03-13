use crate::parser::expr_parser::Expr;
use crate::parser::lexer::{Span, Token};
use crate::parser::source_map::SourceMap;
use crate::types::props::PropDecl;
use indexmap::IndexMap;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Text(String),
    Interpolation {
        expr: Expr,
        raw: bool,
    },
    Directive {
        name: String,
        args: Option<crate::parser::arg_list::ArgList>,
        body: Option<Vec<Node>>,
        span: Span,
    },
    Component {
        path: String,
        props: Option<crate::parser::arg_list::ArgList>,
        fills: HashMap<String, Vec<Node>>,
        main: Vec<Node>,
        span: Span,
    },
    Slot {
        name: Option<String>,
        default: Vec<Node>,
        scoped: Option<(String, Expr)>,
        span: Span,
    },
    Extends {
        path: String,
        span: Span,
    },
    Section {
        name: String,
        body: Vec<Node>,
    },
    Yield {
        name: String,
        span: Span,
    },
    Include {
        path: String,
        data: Option<Expr>,
        span: Span,
    },
    Push {
        stack: String,
        prepend: bool,
        body: Vec<Node>,
    },
    Stack {
        name: String,
        span: Span,
    },
    RawTransform {
        content: String,
        transform: String,
    },
}

#[derive(Debug, Clone)]
pub struct SlotDecl {
    pub name: String,
    pub optional: bool,
}

#[derive(Debug, Clone)]
pub struct Template {
    pub nodes: Vec<Node>,
    pub source_map: SourceMap,
    pub file: String,
    pub extends: Option<String>,
    pub prop_decls: Vec<PropDecl>,
    pub slot_decls: Vec<SlotDecl>,
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    file: String,
    source_map: SourceMap,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, file: String) -> Self {
        let source_map = SourceMap::new(file.clone());
        Parser {
            tokens,
            pos: 0,
            file,
            source_map,
        }
    }

    fn current(&self) -> Token {
        self.tokens
            .get(self.pos)
            .cloned()
            .unwrap_or_else(|| Token::Eof(Span::new(0, 0, 0, 0)))
    }

    fn advance(&mut self) {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
    }

    fn current_span(&self) -> Span {
        match self.current() {
            Token::Text(span, _) => span,
            Token::OpenInterp(span) => span,
            Token::OpenRawInterp(span) => span,
            Token::CloseInterp(span) => span,
            Token::CloseRawInterp(span) => span,
            Token::OpenComment(span) => span,
            Token::CloseComment(span) => span,
            Token::At(span) => span,
            Token::Bang(span) => span,
            Token::Ident(span, _) => span,
            Token::LParen(span) => span,
            Token::RParen(span) => span,
            Token::LBrace(span) => span,
            Token::RBrace(span) => span,
            Token::LBracket(span) => span,
            Token::RBracket(span) => span,
            Token::Comma(span) => span,
            Token::Dot(span) => span,
            Token::Colon(span) => span,
            Token::Equals(span) => span,
            Token::StringLit(span, _) => span,
            Token::NumLit(span, _) => span,
            Token::BoolLit(span, _) => span,
            Token::NullLit(span) => span,
            Token::Whitespace(span, _) => span,
            Token::Newline(span) => span,
            Token::Eof(span) => span,
            Token::Plus(span) => span,
            Token::Minus(span) => span,
            Token::Star(span) => span,
            Token::Slash(span) => span,
            Token::Percent(span) => span,
            Token::LAngle(span) => span,
            Token::RAngle(span) => span,
            Token::Question(span) => span,
            Token::Dollar(span) => span,
            Token::Backtick(span) => span,
        }
    }

    pub fn parse(&mut self) -> Result<Template, ParseError> {
        let mut nodes = Vec::new();

        while !matches!(self.current(), Token::Eof(_)) {
            let node = self.parse_node()?;
            nodes.push(node);
        }

        let extends = self.extract_extends(&nodes);
        let prop_decls = Vec::new(); // TODO: parse from @props directive
        let slot_decls = Vec::new(); // TODO: parse from @slots directive

        Ok(Template {
            nodes,
            source_map: self.source_map.clone(),
            file: self.file.clone(),
            extends,
            prop_decls,
            slot_decls,
        })
    }

    fn extract_extends(&self, nodes: &[Node]) -> Option<String> {
        for node in nodes {
            if let Node::Extends { path, .. } = node {
                return Some(path.clone());
            }
        }
        None
    }

    fn parse_node(&mut self) -> Result<Node, ParseError> {
        match self.current() {
            Token::Text(_, s) if !s.is_empty() => {
                let text = s.clone();
                self.advance();
                Ok(Node::Text(text))
            }
            Token::Ident(_, s) => {
                // Outside of interpolation, identifiers are just text
                let text = s.clone();
                self.advance();
                Ok(Node::Text(text))
            }
            Token::Whitespace(_, _) => {
                self.advance();
                self.parse_node()
            }
            Token::Newline(_) => {
                self.advance();
                self.parse_node()
            }
            Token::OpenInterp(_) => self.parse_interpolation(false),
            Token::OpenRawInterp(_) => self.parse_interpolation(true),
            Token::OpenComment(_) => self.parse_comment(),
            Token::At(_) => self.parse_directive(),
            _ => {
                // Skip any tokens we don't handle and move on
                self.advance();
                Ok(Node::Text("".to_string())) // Placeholder
            }
        }
    }

    fn parse_interpolation(&mut self, raw: bool) -> Result<Node, ParseError> {
        let span = self.current_span();

        // Consume OpenInterp or OpenRawInterp
        self.advance();

        // Parse the expression
        let expr = self.parse_expr()?;

        // Expect CloseInterp or CloseRawInterp
        match self.current() {
            Token::CloseInterp(_) | Token::CloseRawInterp(_) => {
                self.advance();
                Ok(Node::Interpolation { expr, raw })
            }
            _ => Err(ParseError::UnexpectedToken(
                self.current_span(),
                "}}".to_string(),
                format!("{:?}", self.current()),
            )),
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        use crate::parser::expr_parser::ExprParser;

        let start_pos = self.pos;
        let mut expr_tokens = Vec::new();

        // Collect tokens until we hit a closing token
        let mut interp_depth = 0;
        let mut paren_depth = 0;
        loop {
            match self.current() {
                Token::OpenInterp(_) | Token::OpenRawInterp(_) => {
                    interp_depth += 1;
                    expr_tokens.push(self.current());
                    self.advance();
                }
                Token::CloseInterp(_) | Token::CloseRawInterp(_) => {
                    if interp_depth == 0 {
                        break;
                    }
                    interp_depth -= 1;
                    expr_tokens.push(self.current());
                    self.advance();
                }
                Token::LParen(_) => {
                    paren_depth += 1;
                    expr_tokens.push(self.current());
                    self.advance();
                }
                Token::RParen(_) => {
                    if paren_depth > 0 {
                        paren_depth -= 1;
                        expr_tokens.push(self.current());
                        self.advance();
                    } else {
                        // This is the closing paren for the argument list
                        break;
                    }
                }
                Token::Eof(_) => break,
                Token::LBrace(_) => {
                    // Don't include block start in expression
                    break;
                }
                Token::Comma(_) => {
                    // End of this argument
                    break;
                }
                _ => {
                    expr_tokens.push(self.current());
                    self.advance();
                }
            }
        }

        let source_map = SourceMap::new(self.file.clone());
        let mut parser = ExprParser::new(&expr_tokens, &source_map);
        parser
            .parse()
            .map_err(|e| ParseError::ExprError(self.current_span(), e.to_string()))
    }

    fn parse_comment(&mut self) -> Result<Node, ParseError> {
        // Consume OpenComment
        self.advance();

        // Skip until CloseComment
        loop {
            match self.current() {
                Token::CloseComment(_) => {
                    self.advance();
                    break;
                }
                Token::Eof(_) => {
                    return Err(ParseError::UnclosedBlock {
                        directive: "comment".to_string(),
                        opened_at: self.current_span(),
                    });
                }
                _ => self.advance(),
            }
        }

        // Comments don't produce any node
        self.parse_node()
    }

    fn parse_directive(&mut self) -> Result<Node, ParseError> {
        let span = self.current_span();

        // Consume @
        self.advance();

        // Check for ! (self-closing)
        let self_closing = matches!(self.current(), Token::Bang(_));
        if self_closing {
            self.advance();
        }

        // Expect directive name
        let name = match self.current() {
            Token::Ident(_, n) => {
                let name = n.clone();
                self.advance();
                name
            }
            _ => {
                return Err(ParseError::UnexpectedToken(
                    self.current_span(),
                    "directive name".to_string(),
                    format!("{:?}", self.current()),
                ));
            }
        };

        // Parse arguments if present
        let args = if matches!(self.current(), Token::LParen(_)) {
            Some(self.parse_arg_list()?)
        } else {
            None
        };

        // Parse block body if present
        let body = if matches!(self.current(), Token::LBrace(_)) {
            Some(self.parse_block()?)
        } else {
            None
        };

        Ok(Node::Directive {
            name,
            args,
            body,
            span,
        })
    }

    fn parse_arg_list(&mut self) -> Result<crate::parser::arg_list::ArgList, ParseError> {
        // Consume (
        self.advance();

        let mut positional = Vec::new();
        let mut named = IndexMap::new();

        loop {
            match self.current() {
                Token::RParen(_) => {
                    self.advance();
                    break;
                }
                Token::Whitespace(_, _) | Token::Newline(_) => {
                    self.advance();
                }
                Token::Comma(_) => {
                    self.advance();
                }
                Token::Ident(_, _)
                | Token::NumLit(_, _)
                | Token::StringLit(_, _)
                | Token::BoolLit(_, _)
                | Token::NullLit(_) => {
                    // Parse expression starting with this token
                    let value = self.parse_expr()?;
                    positional.push(value);
                }
                _ => {
                    // Skip unknown tokens
                    self.advance();
                }
            }
        }

        Ok(crate::parser::arg_list::ArgList { positional, named })
    }

    fn parse_block(&mut self) -> Result<Vec<Node>, ParseError> {
        // Consume {
        self.advance();

        let mut nodes = Vec::new();
        let mut brace_depth = 1;

        while brace_depth > 0 {
            match self.current() {
                Token::LBrace(_) => {
                    brace_depth += 1;
                    self.advance();
                }
                Token::RBrace(_) => {
                    brace_depth -= 1;
                    if brace_depth > 0 {
                        self.advance();
                    }
                }
                Token::Eof(_) => {
                    return Err(ParseError::UnclosedBlock {
                        directive: "block".to_string(),
                        opened_at: self.current_span(),
                    });
                }
                _ => {
                    let node = self.parse_node()?;
                    nodes.push(node);
                }
            }
        }

        // Consume final }
        self.advance();

        Ok(nodes)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    UnexpectedToken(Span, String, String),
    UnclosedBlock { directive: String, opened_at: Span },
    ExprError(Span, String),
    UnknownDirective { name: String, similar: Vec<String> },
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedToken(span, expected, found) => {
                write!(
                    f,
                    "Unexpected token at {:?}: expected {}, found {}",
                    span, expected, found
                )
            }
            ParseError::UnclosedBlock {
                directive,
                opened_at,
            } => {
                write!(
                    f,
                    "Unclosed block directive '{}' opened at {:?}",
                    directive, opened_at
                )
            }
            ParseError::ExprError(span, msg) => {
                write!(f, "Expression error at {:?}: {}", span, msg)
            }
            ParseError::UnknownDirective { name, similar } => {
                write!(f, "Unknown directive '{}'", name)?;
                if !similar.is_empty() {
                    write!(f, ", did you mean {}?", similar.join(" or "))?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for ParseError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::lexer::Lexer;

    fn parse_template(src: &str) -> Result<Template, ParseError> {
        let tokens = Lexer::tokenize(src);
        let mut parser = Parser::new(tokens, "test.atom".to_string());
        parser.parse()
    }

    #[test]
    fn test_text_node() {
        let result = parse_template("Hello World");
        assert!(result.is_ok());
        let template = result.unwrap();
        // Check that we have text nodes
        assert!(template
            .nodes
            .iter()
            .any(|n| matches!(n, Node::Text(s) if s == "Hello")));
        assert!(template
            .nodes
            .iter()
            .any(|n| matches!(n, Node::Text(s) if s == "World")));
    }

    #[test]
    fn test_interpolation() {
        let result = parse_template("{{ name }}");
        assert!(result.is_ok());
        let template = result.unwrap();
        assert!(matches!(
            &template.nodes[0],
            Node::Interpolation { raw: false, .. }
        ));
    }

    #[test]
    fn test_raw_interpolation() {
        let result = parse_template("{{{ html }}}");
        assert!(result.is_ok());
        let template = result.unwrap();
        assert!(matches!(
            &template.nodes[0],
            Node::Interpolation { raw: true, .. }
        ));
    }

    #[test]
    fn test_directive() {
        let result = parse_template("@if(user.isAdmin) { Admin }");
        eprintln!("Result: {:?}", result);
        assert!(result.is_ok());
        let template = result.unwrap();
        assert!(matches!(&template.nodes[0], Node::Directive { name, .. } if name == "if"));
    }
}
