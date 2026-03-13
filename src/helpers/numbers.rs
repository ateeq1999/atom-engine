use crate::types::value::Value;

pub fn num_round(n: f64, decimals: Option<i32>) -> Value {
    let d = decimals.unwrap_or(0);
    let multiplier = 10_f64.powi(d);
    Value::Num((n * multiplier).round() / multiplier)
}

pub fn num_floor(n: f64) -> Value {
    Value::Num(n.floor())
}

pub fn num_ceil(n: f64) -> Value {
    Value::Num(n.ceil())
}

pub fn num_abs(n: f64) -> Value {
    Value::Num(n.abs())
}

pub fn num_clamp(n: f64, min: f64, max: f64) -> Value {
    Value::Num(n.max(min).min(max))
}

pub fn num_to_fixed(n: f64, decimals: i32) -> Value {
    Value::Str(format!("{:.width$}", n, width = decimals as usize))
}

pub fn num_format(n: f64) -> Value {
    Value::Str(
        format!("{:.0}", n)
            .chars()
            .rev()
            .collect::<Vec<_>>()
            .chunks(3)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join(",")
            .chars()
            .rev()
            .collect(),
    )
}

pub fn num_currency(n: f64, symbol: Option<&str>) -> Value {
    let sym = symbol.unwrap_or("$");
    Value::Str(format!("{}{:.2}", sym, n))
}

pub fn num_percent(n: f64, decimals: Option<i32>) -> Value {
    let d = decimals.unwrap_or(0);
    Value::Str(format!("{:.width$}%", n * 100.0, width = d as usize))
}

pub fn num_is_even(n: f64) -> Value {
    Value::Bool(n as i64 % 2 == 0)
}

pub fn num_is_odd(n: f64) -> Value {
    Value::Bool(n as i64 % 2 != 0)
}

pub fn num_pad(n: f64, width: usize, ch: Option<char>) -> Value {
    let pad_char = ch.unwrap_or('0');
    Value::Str(
        format!("{:<width$}", n.to_string(), width = width).replace(' ', &pad_char.to_string()),
    )
}

pub fn num_min(a: f64, b: f64) -> Value {
    Value::Num(a.min(b))
}

pub fn num_max(a: f64, b: f64) -> Value {
    Value::Num(a.max(b))
}

pub fn num_random(min: i64, max: i64) -> Value {
    use std::time::{SystemTime, UNIX_EPOCH};
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;
    let range = (max - min + 1) as u64;
    let result = (seed % range) as i64 + min;
    Value::Num(result as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round() {
        assert_eq!(num_round(3.14159, Some(2)), Value::Num(3.14));
    }

    #[test]
    fn test_floor() {
        assert_eq!(num_floor(3.9), Value::Num(3.0));
    }

    #[test]
    fn test_ceil() {
        assert_eq!(num_ceil(3.1), Value::Num(4.0));
    }

    #[test]
    fn test_abs() {
        assert_eq!(num_abs(-5.0), Value::Num(5.0));
    }

    #[test]
    fn test_clamp() {
        assert_eq!(num_clamp(5.0, 0.0, 10.0), Value::Num(5.0));
        assert_eq!(num_clamp(-1.0, 0.0, 10.0), Value::Num(0.0));
        assert_eq!(num_clamp(15.0, 0.0, 10.0), Value::Num(10.0));
    }

    #[test]
    fn test_is_even() {
        assert!(num_is_even(4.0).is_truthy());
        assert!(!num_is_even(3.0).is_truthy());
    }

    #[test]
    fn test_is_odd() {
        assert!(num_is_odd(3.0).is_truthy());
        assert!(!num_is_odd(4.0).is_truthy());
    }

    #[test]
    fn test_min() {
        assert_eq!(num_min(3.0, 5.0), Value::Num(3.0));
    }

    #[test]
    fn test_max() {
        assert_eq!(num_max(3.0, 5.0), Value::Num(5.0));
    }
}
