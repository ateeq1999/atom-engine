use crate::error::RenderError;
use crate::parser::expr_parser::{BinaryOp, Expr, UnaryOp};
use crate::renderer::scope::Scope;
use crate::types::value::Value;

pub struct EvalCtx<'r> {
    pub scope: &'r Scope,
    pub debug: bool,
}

impl<'r> EvalCtx<'r> {
    pub fn new(scope: &'r Scope) -> Self {
        EvalCtx {
            scope,
            debug: false,
        }
    }

    pub fn with_debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }
}

pub fn eval_expr(expr: &Expr, ctx: &EvalCtx) -> Result<Value, RenderError> {
    match expr {
        Expr::Null(_) => Ok(Value::Null),

        Expr::Bool(_, b) => Ok(Value::Bool(*b)),

        Expr::Num(_, n) => Ok(Value::Num(*n)),

        Expr::Str(_, s) => Ok(Value::Str(s.clone())),

        Expr::Ident(_, name) => ctx
            .scope
            .get(name)
            .ok_or_else(|| RenderError::UndefinedVariable { name: name.clone() }),

        Expr::Prop(_, object, prop) => {
            let obj = eval_expr(object, ctx)?;
            if let Value::Object(map) = obj {
                Ok(map.get(prop).cloned().unwrap_or(Value::Null))
            } else {
                Ok(Value::Null)
            }
        }

        Expr::Index(_, object, index) => {
            let obj = eval_expr(object, ctx)?;
            let idx = eval_expr(index, ctx)?;

            match (obj, idx) {
                (Value::Array(arr), Value::Num(n)) => {
                    let i = n as usize;
                    if i < arr.len() {
                        Ok(arr[i].clone())
                    } else {
                        Ok(Value::Null)
                    }
                }
                (Value::Object(map), Value::Str(key)) => {
                    Ok(map.get(&key).cloned().unwrap_or(Value::Null))
                }
                _ => Ok(Value::Null),
            }
        }

        Expr::OptChain(_, object, prop) => {
            let obj = eval_expr(object, ctx)?;
            if let Value::Null = obj {
                return Ok(Value::Null);
            }
            if let Value::Object(map) = obj {
                Ok(map.get(prop).cloned().unwrap_or(Value::Null))
            } else {
                Ok(Value::Null)
            }
        }

        Expr::Call(_, callee, args, _named) => {
            // For now, just evaluate the callee and return null
            // Full helper dispatch comes in Phase 9
            Ok(Value::Null)
        }

        Expr::Unary(_, op, operand) => {
            let val = eval_expr(operand, ctx)?;
            match op {
                UnaryOp::Not => Ok(Value::Bool(!val.is_truthy())),
                UnaryOp::Neg => {
                    if let Value::Num(n) = val {
                        Ok(Value::Num(-n))
                    } else {
                        Ok(Value::Num(0.0))
                    }
                }
            }
        }

        Expr::Binary(_, left, op, right) => {
            let l = eval_expr(left, ctx)?;
            let r = eval_expr(right, ctx)?;
            eval_binary_op(&l, op, &r)
        }

        Expr::Ternary(_, cond, then, else_) => {
            let c = eval_expr(cond, ctx)?;
            if c.is_truthy() {
                eval_expr(then, ctx)
            } else {
                eval_expr(else_, ctx)
            }
        }

        Expr::NullCoalesce(_, left, right) => {
            let l = eval_expr(left, ctx)?;
            if matches!(l, Value::Null) {
                eval_expr(right, ctx)
            } else {
                Ok(l)
            }
        }

        Expr::Array(_, elements) => {
            let mut result = Vec::new();
            for e in elements {
                result.push(eval_expr(e, ctx)?);
            }
            Ok(Value::Array(result))
        }

        Expr::Object(_, fields) => {
            use indexmap::IndexMap;
            let mut map = IndexMap::new();
            for (k, v) in fields {
                let key = k.clone();
                let value = eval_expr(v, ctx)?;
                map.insert(key, value);
            }
            Ok(Value::Object(map))
        }

        Expr::TemplateLit(_, segments) => {
            use crate::parser::expr_parser::TemplateSegment;
            let mut result = String::new();
            for seg in segments {
                match seg {
                    TemplateSegment::Text(s) => result.push_str(s),
                    TemplateSegment::Expr(e) => {
                        let val = eval_expr(e, ctx)?;
                        result.push_str(&val.coerce_str());
                    }
                }
            }
            Ok(Value::Str(result))
        }
    }
}

fn eval_binary_op(left: &Value, op: &BinaryOp, right: &Value) -> Result<Value, RenderError> {
    match op {
        BinaryOp::Add => {
            if let (Value::Str(s), v) = (left.clone(), right.clone()) {
                Ok(Value::Str(format!("{}{}", s, v.coerce_str())))
            } else if let (Value::Num(a), Value::Num(b)) = (left.clone(), right.clone()) {
                Ok(Value::Num(a + b))
            } else {
                Ok(Value::Str(format!(
                    "{}{}",
                    left.coerce_str(),
                    right.coerce_str()
                )))
            }
        }
        BinaryOp::Sub => {
            if let (Value::Num(a), Value::Num(b)) = (left.clone(), right.clone()) {
                Ok(Value::Num(a - b))
            } else {
                Ok(Value::Num(0.0))
            }
        }
        BinaryOp::Mul => {
            if let (Value::Num(a), Value::Num(b)) = (left.clone(), right.clone()) {
                Ok(Value::Num(a * b))
            } else {
                Ok(Value::Num(0.0))
            }
        }
        BinaryOp::Div => {
            if let (Value::Num(a), Value::Num(b)) = (left.clone(), right.clone()) {
                if b == 0.0 {
                    Err(RenderError::DivisionByZero)
                } else {
                    Ok(Value::Num(a / b))
                }
            } else {
                Ok(Value::Num(0.0))
            }
        }
        BinaryOp::Mod => {
            if let (Value::Num(a), Value::Num(b)) = (left.clone(), right.clone()) {
                if b == 0.0 {
                    Err(RenderError::DivisionByZero)
                } else {
                    Ok(Value::Num(a % b))
                }
            } else {
                Ok(Value::Num(0.0))
            }
        }
        BinaryOp::Pow => {
            if let (Value::Num(a), Value::Num(b)) = (left.clone(), right.clone()) {
                Ok(Value::Num(a.powf(b)))
            } else {
                Ok(Value::Num(0.0))
            }
        }
        BinaryOp::Eq => Ok(Value::Bool(left == right)),
        BinaryOp::Ne => Ok(Value::Bool(left != right)),
        BinaryOp::Lt => {
            if let (Value::Num(a), Value::Num(b)) = (left.clone(), right.clone()) {
                Ok(Value::Bool(a < b))
            } else {
                Ok(Value::Bool(left.coerce_str() < right.coerce_str()))
            }
        }
        BinaryOp::Le => {
            if let (Value::Num(a), Value::Num(b)) = (left.clone(), right.clone()) {
                Ok(Value::Bool(a <= b))
            } else {
                Ok(Value::Bool(left.coerce_str() <= right.coerce_str()))
            }
        }
        BinaryOp::Gt => {
            if let (Value::Num(a), Value::Num(b)) = (left.clone(), right.clone()) {
                Ok(Value::Bool(a > b))
            } else {
                Ok(Value::Bool(left.coerce_str() > right.coerce_str()))
            }
        }
        BinaryOp::Ge => {
            if let (Value::Num(a), Value::Num(b)) = (left.clone(), right.clone()) {
                Ok(Value::Bool(a >= b))
            } else {
                Ok(Value::Bool(left.coerce_str() >= right.coerce_str()))
            }
        }
        BinaryOp::And => Ok(Value::Bool(left.is_truthy() && right.is_truthy())),
        BinaryOp::Or => Ok(Value::Bool(left.is_truthy() || right.is_truthy())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::expr_parser::{BinaryOp, Expr, UnaryOp};
    use crate::parser::lexer::Span;

    fn eval_expr_test(expr: Expr) -> Result<Value, RenderError> {
        let scope = Scope::new();
        let ctx = EvalCtx::new(&scope);
        eval_expr(&expr, &ctx)
    }

    #[test]
    fn test_eval_null() {
        let result = eval_expr_test(Expr::Null(Span::new(0, 0, 1, 1))).unwrap();
        assert!(matches!(result, Value::Null));
    }

    #[test]
    fn test_eval_bool() {
        let result = eval_expr_test(Expr::Bool(Span::new(0, 0, 1, 1), true)).unwrap();
        assert!(matches!(result, Value::Bool(true)));
    }

    #[test]
    fn test_eval_number() {
        let result = eval_expr_test(Expr::Num(Span::new(0, 0, 1, 1), 42.0)).unwrap();
        assert!(matches!(result, Value::Num(42.0)));
    }

    #[test]
    fn test_eval_string() {
        let result = eval_expr_test(Expr::Str(Span::new(0, 0, 1, 1), "hello".to_string())).unwrap();
        assert!(matches!(result, Value::Str(s) if s == "hello"));
    }

    #[test]
    fn test_eval_unary_not() {
        let result = eval_expr_test(Expr::Unary(
            Span::new(0, 0, 1, 1),
            UnaryOp::Not,
            Box::new(Expr::Bool(Span::new(0, 0, 1, 1), false)),
        ))
        .unwrap();
        assert!(matches!(result, Value::Bool(true)));
    }

    #[test]
    fn test_eval_unary_neg() {
        let result = eval_expr_test(Expr::Unary(
            Span::new(0, 0, 1, 1),
            UnaryOp::Neg,
            Box::new(Expr::Num(Span::new(0, 0, 1, 1), 5.0)),
        ))
        .unwrap();
        assert!(matches!(result, Value::Num(-5.0)));
    }

    #[test]
    fn test_eval_add_numbers() {
        let result = eval_expr_test(Expr::Binary(
            Span::new(0, 0, 1, 1),
            Box::new(Expr::Num(Span::new(0, 0, 1, 1), 1.0)),
            BinaryOp::Add,
            Box::new(Expr::Num(Span::new(0, 0, 1, 1), 2.0)),
        ))
        .unwrap();
        assert!(matches!(result, Value::Num(3.0)));
    }

    #[test]
    fn test_eval_add_string() {
        let result = eval_expr_test(Expr::Binary(
            Span::new(0, 0, 1, 1),
            Box::new(Expr::Str(Span::new(0, 0, 1, 1), "hello".to_string())),
            BinaryOp::Add,
            Box::new(Expr::Str(Span::new(0, 0, 1, 1), " world".to_string())),
        ))
        .unwrap();
        assert!(matches!(result, Value::Str(s) if s == "hello world"));
    }

    #[test]
    fn test_eval_comparison_lt() {
        let result = eval_expr_test(Expr::Binary(
            Span::new(0, 0, 1, 1),
            Box::new(Expr::Num(Span::new(0, 0, 1, 1), 1.0)),
            BinaryOp::Lt,
            Box::new(Expr::Num(Span::new(0, 0, 1, 1), 2.0)),
        ))
        .unwrap();
        assert!(result.is_truthy());
    }

    #[test]
    fn test_eval_ternary_true() {
        let result = eval_expr_test(Expr::Ternary(
            Span::new(0, 0, 1, 1),
            Box::new(Expr::Bool(Span::new(0, 0, 1, 1), true)),
            Box::new(Expr::Str(Span::new(0, 0, 1, 1), "yes".to_string())),
            Box::new(Expr::Str(Span::new(0, 0, 1, 1), "no".to_string())),
        ))
        .unwrap();
        assert!(matches!(result, Value::Str(s) if s == "yes"));
    }

    #[test]
    fn test_eval_null_coalesce_null() {
        let result = eval_expr_test(Expr::NullCoalesce(
            Span::new(0, 0, 1, 1),
            Box::new(Expr::Null(Span::new(0, 0, 1, 1))),
            Box::new(Expr::Str(Span::new(0, 0, 1, 1), "default".to_string())),
        ))
        .unwrap();
        assert!(matches!(result, Value::Str(s) if s == "default"));
    }

    #[test]
    fn test_eval_array() {
        let result = eval_expr_test(Expr::Array(
            Span::new(0, 0, 1, 1),
            vec![
                Expr::Num(Span::new(0, 0, 1, 1), 1.0),
                Expr::Num(Span::new(0, 0, 1, 1), 2.0),
                Expr::Num(Span::new(0, 0, 1, 1), 3.0),
            ],
        ))
        .unwrap();
        assert!(matches!(result, Value::Array(arr) if arr.len() == 3));
    }

    #[test]
    fn test_eval_object() {
        use indexmap::IndexMap;
        let mut map = IndexMap::new();
        map.insert("a".to_string(), Expr::Num(Span::new(0, 0, 1, 1), 1.0));
        map.insert("b".to_string(), Expr::Num(Span::new(0, 0, 1, 1), 2.0));

        let result = eval_expr_test(Expr::Object(Span::new(0, 0, 1, 1), map)).unwrap();
        assert!(matches!(result, Value::Object(m) if m.len() == 2));
    }
}
