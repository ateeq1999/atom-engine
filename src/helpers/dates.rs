use crate::types::value::Value;
use chrono::{DateTime, Datelike, Duration, Timelike, Utc};

pub fn date_format(dt: &DateTime<Utc>, pattern: &str) -> Value {
    let result = pattern
        .replace("YYYY", &dt.format("%Y").to_string())
        .replace("MM", &dt.format("%m").to_string())
        .replace("DD", &dt.format("%d").to_string())
        .replace("HH", &dt.format("%H").to_string())
        .replace("mm", &dt.format("%M").to_string())
        .replace("ss", &dt.format("%S").to_string());
    Value::Str(result)
}

pub fn date_to_iso(dt: &DateTime<Utc>) -> Value {
    Value::Str(dt.to_rfc3339())
}

pub fn date_to_unix(dt: &DateTime<Utc>) -> Value {
    Value::Num(dt.timestamp() as f64)
}

pub fn date_add(dt: &DateTime<Utc>, n: i64, unit: &str) -> Value {
    let duration = match unit {
        "years" => Duration::days(n * 365),
        "months" => Duration::days(n * 30),
        "weeks" => Duration::weeks(n),
        "days" => Duration::days(n),
        "hours" => Duration::hours(n),
        "minutes" => Duration::minutes(n),
        "seconds" => Duration::seconds(n),
        _ => Duration::days(n),
    };
    Value::Date(*dt + duration)
}

pub fn date_sub(dt: &DateTime<Utc>, n: i64, unit: &str) -> Value {
    date_add(dt, -n, unit)
}

pub fn date_start_of(dt: &DateTime<Utc>, unit: &str) -> Value {
    match unit {
        "year" => Value::Date(
            dt.with_month(1)
                .unwrap()
                .with_day(1)
                .unwrap()
                .with_hour(0)
                .unwrap()
                .with_minute(0)
                .unwrap()
                .with_second(0)
                .unwrap(),
        ),
        "month" => Value::Date(
            dt.with_day(1)
                .unwrap()
                .with_hour(0)
                .unwrap()
                .with_minute(0)
                .unwrap()
                .with_second(0)
                .unwrap(),
        ),
        "day" => Value::Date(
            dt.with_hour(0)
                .unwrap()
                .with_minute(0)
                .unwrap()
                .with_second(0)
                .unwrap(),
        ),
        "hour" => Value::Date(dt.with_minute(0).unwrap().with_second(0).unwrap()),
        _ => Value::Date(*dt),
    }
}

pub fn date_end_of(dt: &DateTime<Utc>, unit: &str) -> Value {
    match unit {
        "year" => Value::Date(
            dt.with_month(12)
                .unwrap()
                .with_day(31)
                .unwrap()
                .with_hour(23)
                .unwrap()
                .with_minute(59)
                .unwrap()
                .with_second(59)
                .unwrap(),
        ),
        "month" => {
            let last_day = (*dt - Duration::days(1)).day();
            Value::Date(
                dt.with_day(last_day)
                    .unwrap()
                    .with_hour(23)
                    .unwrap()
                    .with_minute(59)
                    .unwrap()
                    .with_second(59)
                    .unwrap(),
            )
        }
        "day" => Value::Date(
            dt.with_hour(23)
                .unwrap()
                .with_minute(59)
                .unwrap()
                .with_second(59)
                .unwrap(),
        ),
        _ => Value::Date(*dt),
    }
}

pub fn date_is_before(dt: &DateTime<Utc>, other: &DateTime<Utc>) -> Value {
    Value::Bool(dt < other)
}

pub fn date_is_after(dt: &DateTime<Utc>, other: &DateTime<Utc>) -> Value {
    Value::Bool(dt > other)
}

pub fn date_is_same(dt: &DateTime<Utc>, other: &DateTime<Utc>, _unit: Option<&str>) -> Value {
    Value::Bool(dt.timestamp() == other.timestamp())
}

pub fn date_diff(dt: &DateTime<Utc>, other: &DateTime<Utc>, unit: &str) -> Value {
    let diff = (*dt - *other).num_seconds();
    let result = match unit {
        "years" => diff / (365 * 24 * 60 * 60),
        "months" => diff / (30 * 24 * 60 * 60),
        "days" => diff / (24 * 60 * 60),
        "hours" => diff / (60 * 60),
        "minutes" => diff / 60,
        "seconds" => diff,
        _ => diff,
    };
    Value::Num(result as f64)
}

pub fn date_to_relative(dt: &DateTime<Utc>) -> Value {
    let now = Utc::now();
    let diff = now.signed_duration_since(*dt);

    if diff.num_seconds() < 60 {
        Value::Str("just now".to_string())
    } else if diff.num_minutes() < 60 {
        let m = diff.num_minutes();
        Value::Str(format!("{} minute{} ago", m, if m == 1 { "" } else { "s" }))
    } else if diff.num_hours() < 24 {
        let h = diff.num_hours();
        Value::Str(format!("{} hour{} ago", h, if h == 1 { "" } else { "s" }))
    } else if diff.num_days() < 30 {
        let d = diff.num_days();
        Value::Str(format!("{} day{} ago", d, if d == 1 { "" } else { "s" }))
    } else if diff.num_days() < 365 {
        Value::Str(format!("{} months ago", diff.num_days() / 30))
    } else {
        Value::Str(format!("{} years ago", diff.num_days() / 365))
    }
}

pub fn now() -> Value {
    Value::Date(Utc::now())
}

pub fn today() -> Value {
    Value::Date(
        Utc::now()
            .date_naive()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc(),
    )
}

pub fn parse_date(s: &str, _fmt: Option<&str>) -> Value {
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        Value::Date(dt.with_timezone(&Utc))
    } else if let Ok(naive) = chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d") {
        Value::Date(naive.and_hms_opt(0, 0, 0).unwrap().and_utc())
    } else {
        Value::Null
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_now() {
        let result = now();
        assert!(matches!(result, Value::Date(_)));
    }

    #[test]
    fn test_today() {
        let result = today();
        assert!(matches!(result, Value::Date(_)));
    }

    #[test]
    fn test_date_format() {
        let dt = Utc::now();
        let result = date_format(&dt, "YYYY-MM-DD");
        assert!(matches!(result, Value::Str(_)));
    }

    #[test]
    fn test_date_to_iso() {
        let dt = Utc::now();
        let result = date_to_iso(&dt);
        assert!(matches!(result, Value::Str(_)));
    }

    #[test]
    fn test_date_add() {
        let dt = Utc::now();
        let result = date_add(&dt, 1, "days");
        assert!(matches!(result, Value::Date(_)));
    }

    #[test]
    fn test_parse_date() {
        let result = parse_date("2024-01-15", None);
        assert!(matches!(result, Value::Date(_)));
    }
}
