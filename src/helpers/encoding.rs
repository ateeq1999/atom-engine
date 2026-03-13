use crate::types::value::Value;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use md5::Md5;
use sha2::{Digest, Sha256};

pub fn encode_uri(s: &str) -> Value {
    Value::Str(urlencoding::encode(s).to_string())
}

pub fn decode_uri(s: &str) -> Value {
    Value::Str(urlencoding::decode(s).unwrap_or_default().to_string())
}

pub fn base64_encode(s: &str) -> Value {
    Value::Str(BASE64.encode(s.as_bytes()))
}

pub fn base64_decode(s: &str) -> Value {
    match BASE64.decode(s.as_bytes()) {
        Ok(bytes) => Value::Str(String::from_utf8_lossy(&bytes).to_string()),
        Err(_) => Value::Null,
    }
}

pub fn sha256(s: &str) -> Value {
    let mut hasher = Sha256::new();
    hasher.update(s.as_bytes());
    let result = hasher.finalize();
    Value::Str(hex::encode(result))
}

pub fn md5(s: &str) -> Value {
    let mut hasher = Md5::new();
    hasher.update(s.as_bytes());
    let result = hasher.finalize();
    Value::Str(hex::encode(result))
}

pub fn uuid() -> Value {
    Value::Str(uuid::Uuid::new_v4().to_string())
}

pub fn random(len: usize) -> Value {
    use rand::Rng;
    let charset: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
        .chars()
        .collect();
    let mut rng = rand::thread_rng();
    let result: String = (0..len)
        .map(|_| charset[rng.gen_range(0..charset.len())])
        .collect();
    Value::Str(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_encode() {
        assert_eq!(base64_encode("hello"), Value::Str("aGVsbG8=".to_string()));
    }

    #[test]
    fn test_base64_decode() {
        assert_eq!(base64_decode("aGVsbG8="), Value::Str("hello".to_string()));
    }

    #[test]
    fn test_sha256() {
        let result = sha256("hello");
        assert!(matches!(result, Value::Str(s) if s.len() == 64));
    }

    #[test]
    fn test_md5() {
        let result = md5("hello");
        assert!(matches!(result, Value::Str(s) if s.len() == 32));
    }

    #[test]
    fn test_uuid() {
        let result = uuid();
        assert!(matches!(result, Value::Str(s) if s.len() == 36));
    }

    #[test]
    fn test_random() {
        let result = random(10);
        assert!(matches!(result, Value::Str(s) if s.len() == 10));
    }
}
