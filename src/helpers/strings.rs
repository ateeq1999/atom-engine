use crate::types::value::Value;
use heck::{ToKebabCase, ToSnakeCase, ToUpperCamelCase};
use unicode_segmentation::UnicodeSegmentation;

pub fn string_upper(s: &str) -> Value {
    Value::Str(s.to_uppercase())
}

pub fn string_lower(s: &str) -> Value {
    Value::Str(s.to_lowercase())
}

pub fn string_trim(s: &str) -> Value {
    Value::Str(s.trim().to_string())
}

pub fn string_trim_start(s: &str) -> Value {
    Value::Str(s.trim_start().to_string())
}

pub fn string_trim_end(s: &str) -> Value {
    Value::Str(s.trim_end().to_string())
}

pub fn string_capitalize(s: &str) -> Value {
    let mut chars = s.chars();
    match chars.next() {
        None => Value::Str(String::new()),
        Some(first) => {
            let rest: String = chars.collect();
            Value::Str(format!("{}{}", first.to_uppercase(), rest.to_lowercase()))
        }
    }
}

pub fn string_title_case(s: &str) -> Value {
    Value::Str(s.to_upper_camel_case().to_string())
}

pub fn string_camel_case(s: &str) -> Value {
    Value::Str(s.to_upper_camel_case().to_string())
}

pub fn string_pascal_case(s: &str) -> Value {
    Value::Str(s.to_upper_camel_case().to_string())
}

pub fn string_snake_case(s: &str) -> Value {
    Value::Str(s.to_snake_case().to_string())
}

pub fn string_kebab_case(s: &str) -> Value {
    Value::Str(s.to_kebab_case().to_string())
}

pub fn string_replace(s: &str, from: &str, to: &str) -> Value {
    Value::Str(s.replace(from, to))
}

pub fn string_replace_first(s: &str, from: &str, to: &str) -> Value {
    Value::Str(s.replacen(from, to, 1))
}

pub fn string_starts_with(s: &str, prefix: &str) -> Value {
    Value::Bool(s.starts_with(prefix))
}

pub fn string_ends_with(s: &str, suffix: &str) -> Value {
    Value::Bool(s.ends_with(suffix))
}

pub fn string_contains(s: &str, needle: &str) -> Value {
    Value::Bool(s.contains(needle))
}

pub fn string_pad_start(s: &str, len: usize, ch: Option<char>) -> Value {
    let pad_char = ch.unwrap_or(' ');
    let padding = pad_char.to_string().repeat(len.saturating_sub(s.len()));
    Value::Str(format!("{}{}", padding, s))
}

pub fn string_pad_end(s: &str, len: usize, ch: Option<char>) -> Value {
    let pad_char = ch.unwrap_or(' ');
    let padding = pad_char.to_string().repeat(len.saturating_sub(s.len()));
    Value::Str(format!("{}{}", s, padding))
}

pub fn string_repeat(s: &str, n: usize) -> Value {
    Value::Str(s.repeat(n))
}

pub fn string_split(s: &str, sep: &str) -> Value {
    Value::Array(s.split(sep).map(|p| Value::Str(p.to_string())).collect())
}

pub fn string_chars(s: &str) -> Value {
    Value::Array(s.chars().map(|c| Value::Str(c.to_string())).collect())
}

pub fn string_lines(s: &str) -> Value {
    Value::Array(s.lines().map(|l| Value::Str(l.to_string())).collect())
}

pub fn string_len(s: &str) -> Value {
    Value::Num(s.graphemes(true).count() as f64)
}

pub fn string_is_empty(s: &str) -> Value {
    Value::Bool(s.is_empty())
}

pub fn string_truncate(s: &str, max_len: usize, suffix: Option<&str>) -> Value {
    let suffix_str = suffix.unwrap_or("...");
    if s.graphemes(true).count() <= max_len {
        Value::Str(s.to_string())
    } else {
        let truncated: String = s.graphemes(true).take(max_len).collect();
        Value::Str(format!("{}{}", truncated, suffix_str))
    }
}

pub fn slugify(s: &str) -> Value {
    Value::Str(s.to_kebab_case().to_string())
}

pub fn pluralize(n: i64, word: &str, plural: Option<&str>) -> Value {
    let plural_str = if n == 1 {
        word.to_string()
    } else {
        plural.unwrap_or(&format!("{}s", word)).to_string()
    };
    Value::Str(format!("{} {}", n, plural_str))
}

pub fn escape_html(s: &str) -> Value {
    Value::Str(crate::renderer::output::OutputBuffer::escape_html(s))
}

pub fn unescape_html(s: &str) -> Value {
    let result = s
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'");
    Value::Str(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upper() {
        assert_eq!(string_upper("hello"), Value::Str("HELLO".to_string()));
    }

    #[test]
    fn test_lower() {
        assert_eq!(string_lower("HELLO"), Value::Str("hello".to_string()));
    }

    #[test]
    fn test_trim() {
        assert_eq!(string_trim("  hello  "), Value::Str("hello".to_string()));
    }

    #[test]
    fn test_snake_case() {
        assert_eq!(
            string_snake_case("HelloWorld"),
            Value::Str("hello_world".to_string())
        );
    }

    #[test]
    fn test_replace() {
        assert_eq!(
            string_replace("a-b-c", "-", "_"),
            Value::Str("a_b_c".to_string())
        );
    }

    #[test]
    fn test_contains() {
        assert!(string_contains("hello world", "world").is_truthy());
    }

    #[test]
    fn test_len() {
        assert_eq!(string_len("hello"), Value::Num(5.0));
    }

    #[test]
    fn test_is_empty() {
        assert!(string_is_empty("").is_truthy());
        assert!(!string_is_empty("x").is_truthy());
    }

    #[test]
    fn test_pluralize() {
        assert_eq!(pluralize(1, "item", None), Value::Str("1 item".to_string()));
        assert_eq!(
            pluralize(2, "item", None),
            Value::Str("2 items".to_string())
        );
    }
}
