use crate::parser::expr_parser::Expr;
use indexmap::IndexMap;

#[derive(Debug, Clone, PartialEq)]
pub struct ArgList {
    pub positional: Vec<Expr>,
    pub named: IndexMap<String, Expr>,
}

impl ArgList {
    pub fn new() -> Self {
        ArgList {
            positional: Vec::new(),
            named: IndexMap::new(),
        }
    }

    pub fn required_expr(&self, index: usize) -> Result<&Expr, String> {
        self.positional
            .get(index)
            .ok_or_else(|| format!("Missing required positional argument at index {}", index))
    }

    pub fn optional_expr(&self, index: usize) -> Option<&Expr> {
        self.positional.get(index)
    }

    pub fn required_string(&self, key: &str) -> Result<String, String> {
        self.named
            .get(key)
            .and_then(|e| {
                if let Expr::Str(_, s) = e {
                    Some(s.clone())
                } else {
                    None
                }
            })
            .ok_or_else(|| format!("Missing or invalid required argument '{}'", key))
    }

    pub fn optional_string(&self, key: &str) -> Option<String> {
        self.named.get(key).and_then(|e| {
            if let Expr::Str(_, s) = e {
                Some(s.clone())
            } else {
                None
            }
        })
    }

    pub fn first(&self) -> Option<&Expr> {
        self.positional.first().or(self.named.values().next())
    }
}

impl Default for ArgList {
    fn default() -> Self {
        Self::new()
    }
}
