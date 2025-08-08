/*
 * Copyright 2025 Cory Parent
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! Utility functions and helpers for the Syneidesis coordination ecosystem

use chrono::{DateTime, Utc};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use crate::error::{CoreError, CoreResult};

/// Generate a unique identifier with a prefix
pub fn generate_id(prefix: &str) -> String {
    let uuid = Uuid::new_v4();
    format!("{}-{}", prefix, uuid.simple())
}

/// Generate a unique identifier without a prefix
pub fn generate_uuid() -> String {
    Uuid::new_v4().to_string()
}

/// Validate an identifier string
pub fn validate_id(id: &str) -> CoreResult<()> {
    if id.is_empty() {
        return Err(CoreError::invalid_identifier("Identifier cannot be empty"));
    }

    if id.len() > 255 {
        return Err(CoreError::invalid_identifier("Identifier too long"));
    }

    // Check for valid characters (alphanumeric, hyphens, underscores)
    if !id
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    {
        return Err(CoreError::invalid_identifier(
            "Identifier contains invalid characters",
        ));
    }

    Ok(())
}

/// Get current timestamp
pub fn timestamp_now() -> DateTime<Utc> {
    Utc::now()
}

/// Get current timestamp as Unix timestamp
pub fn unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Get current timestamp as Unix timestamp with milliseconds
pub fn unix_timestamp_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

/// Convert Unix timestamp to DateTime
pub fn from_unix_timestamp(timestamp: u64) -> DateTime<Utc> {
    DateTime::from_timestamp(timestamp as i64, 0).unwrap_or_else(Utc::now)
}

/// Convert Unix timestamp with milliseconds to DateTime
pub fn from_unix_timestamp_ms(timestamp_ms: u64) -> DateTime<Utc> {
    let secs = timestamp_ms / 1000;
    let nsecs = ((timestamp_ms % 1000) * 1_000_000) as u32;
    DateTime::from_timestamp(secs as i64, nsecs).unwrap_or_else(Utc::now)
}

/// Calculate duration between two timestamps
pub fn duration_between(start: DateTime<Utc>, end: DateTime<Utc>) -> Duration {
    if end >= start {
        end.signed_duration_since(start)
            .to_std()
            .unwrap_or_default()
    } else {
        start
            .signed_duration_since(end)
            .to_std()
            .unwrap_or_default()
    }
}

/// Check if a timestamp is recent (within the given duration)
pub fn is_recent(timestamp: DateTime<Utc>, duration: Duration) -> bool {
    let now = Utc::now();
    let diff = now.signed_duration_since(timestamp);
    diff.to_std().map(|d| d <= duration).unwrap_or(false)
}

/// Check if a timestamp is expired (older than the given duration)
pub fn is_expired(timestamp: DateTime<Utc>, duration: Duration) -> bool {
    !is_recent(timestamp, duration)
}

/// Format duration as human-readable string
pub fn format_duration(duration: Duration) -> String {
    let secs = duration.as_secs();
    let mins = secs / 60;
    let hours = mins / 60;
    let days = hours / 24;

    if days > 0 {
        format!("{}d {}h {}m {}s", days, hours % 24, mins % 60, secs % 60)
    } else if hours > 0 {
        format!("{}h {}m {}s", hours, mins % 60, secs % 60)
    } else if mins > 0 {
        format!("{}m {}s", mins, secs % 60)
    } else {
        format!("{secs}s")
    }
}

/// Parse duration from string (e.g., "30s", "5m", "2h", "1d")
pub fn parse_duration(s: &str) -> CoreResult<Duration> {
    let s = s.trim();
    if s.is_empty() {
        return Err(CoreError::validation("Duration string cannot be empty"));
    }

    let (value_str, unit) = s.split_at(s.len() - 1);
    let value: u64 = value_str
        .parse()
        .map_err(|_| CoreError::validation(format!("Invalid duration value: {value_str}")))?;

    let duration = match unit {
        "s" => Duration::from_secs(value),
        "m" => Duration::from_secs(value * 60),
        "h" => Duration::from_secs(value * 3600),
        "d" => Duration::from_secs(value * 86400),
        _ => {
            return Err(CoreError::validation(format!(
                "Invalid duration unit: {unit}. Expected s, m, h, or d"
            )))
        }
    };

    Ok(duration)
}

/// Generate a random string of specified length
pub fn random_string(length: usize) -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

/// Generate a random numeric string of specified length
pub fn random_numeric_string(length: usize) -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| rng.gen_range(0..10).to_string())
        .collect()
}

/// Check if a string contains only alphanumeric characters
pub fn is_alphanumeric(s: &str) -> bool {
    s.chars().all(|c| c.is_alphanumeric())
}

/// Check if a string contains only numeric characters
pub fn is_numeric(s: &str) -> bool {
    s.chars().all(|c| c.is_numeric())
}

/// Truncate a string to the specified length
pub fn truncate_string(s: &str, max_length: usize) -> String {
    if s.len() <= max_length {
        s.to_string()
    } else {
        format!("{}...", &s[..max_length.saturating_sub(3)])
    }
}

/// Convert bytes to human-readable format
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: [&str; 4] = ["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.2} {}", size, UNITS[unit_index])
}

/// Convert human-readable bytes format back to bytes
pub fn parse_bytes(s: &str) -> CoreResult<u64> {
    let s = s.trim();
    if s.is_empty() {
        return Err(CoreError::validation("Byte string cannot be empty"));
    }

    // Find the last non-digit character to determine the unit
    let mut unit_start = s.len();
    for (i, c) in s.char_indices().rev() {
        if !c.is_ascii_digit() && c != '.' {
            unit_start = i;
        } else {
            break;
        }
    }

    let value_str = &s[..unit_start];
    let unit = &s[unit_start..].trim();

    let value: f64 = value_str
        .parse()
        .map_err(|_| CoreError::validation(format!("Invalid byte value: {s}")))?;

    let bytes = match unit.to_uppercase().as_str() {
        "B" => value as u64,
        "K" | "KB" => (value * 1024.0) as u64,
        "M" | "MB" => (value * 1024.0 * 1024.0) as u64,
        "G" | "GB" => (value * 1024.0 * 1024.0 * 1024.0) as u64,
        _ => {
            return Err(CoreError::validation(format!(
                "Invalid byte unit: {unit}. Expected B, KB, MB, or GB"
            )))
        }
    };

    Ok(bytes)
}

/// Retry a function with exponential backoff
pub async fn retry_with_backoff<F, Fut, T, E>(
    mut f: F,
    max_retries: usize,
    initial_delay: Duration,
) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
    E: std::fmt::Debug,
{
    let mut delay = initial_delay;
    let mut last_error = None;

    for attempt in 0..=max_retries {
        match f().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                last_error = Some(e);
                if attempt < max_retries {
                    tokio::time::sleep(delay).await;
                    delay *= 2; // Exponential backoff
                }
            }
        }
    }

    Err(last_error.unwrap())
}

// Note: Debounce function removed for now due to complexity with async closures
// This can be re-implemented later if needed with proper async trait bounds

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_id() {
        let id = generate_id("test");
        assert!(id.starts_with("test-"));
        assert_eq!(id.len(), 37); // "test-" + 32 hex chars
    }

    #[test]
    fn test_validate_id() {
        assert!(validate_id("valid-id").is_ok());
        assert!(validate_id("valid_id").is_ok());
        assert!(validate_id("valid123").is_ok());
        assert!(validate_id("").is_err());
        assert!(validate_id(&"a".repeat(256)).is_err());
        assert!(validate_id("invalid@id").is_err());
    }

    #[test]
    fn test_timestamp_functions() {
        let now = timestamp_now();
        let unix = unix_timestamp();
        let unix_ms = unix_timestamp_ms();

        assert!(unix > 0);
        assert!(unix_ms > unix * 1000);
        assert!(from_unix_timestamp(unix) <= now);
        assert!(from_unix_timestamp_ms(unix_ms) <= now);
    }

    #[test]
    fn test_duration_functions() {
        let start = Utc::now();
        let end = start + chrono::Duration::seconds(30);
        let duration = duration_between(start, end);

        assert_eq!(duration.as_secs(), 30);
        assert!(is_recent(start, Duration::from_secs(1)));
        assert!(!is_expired(start, Duration::from_secs(1)));
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(Duration::from_secs(30)), "30s");
        assert_eq!(format_duration(Duration::from_secs(90)), "1m 30s");
        assert_eq!(format_duration(Duration::from_secs(3661)), "1h 1m 1s");
        assert_eq!(format_duration(Duration::from_secs(90000)), "1d 1h 0m 0s");
    }

    #[test]
    fn test_parse_duration() {
        assert_eq!(parse_duration("30s").unwrap(), Duration::from_secs(30));
        assert_eq!(parse_duration("5m").unwrap(), Duration::from_secs(300));
        assert_eq!(parse_duration("2h").unwrap(), Duration::from_secs(7200));
        assert_eq!(parse_duration("1d").unwrap(), Duration::from_secs(86400));
        assert!(parse_duration("").is_err());
        assert!(parse_duration("30x").is_err());
    }

    #[test]
    fn test_string_utilities() {
        assert!(is_alphanumeric("abc123"));
        assert!(!is_alphanumeric("abc-123"));
        assert!(is_numeric("123"));
        assert!(!is_numeric("123a"));
        assert_eq!(truncate_string("hello world", 8), "hello...");
        assert_eq!(truncate_string("short", 10), "short");
    }

    #[test]
    fn test_byte_formatting() {
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(1048576), "1.00 MB");
        assert_eq!(parse_bytes("1.00 KB").unwrap(), 1024);
        assert_eq!(parse_bytes("1.00 MB").unwrap(), 1048576);
        assert!(parse_bytes("").is_err());
        assert!(parse_bytes("1.00 TB").is_err());
    }

    #[tokio::test]
    async fn test_retry_with_backoff() {
        let attempts = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let attempts_clone = attempts.clone();

        let result = retry_with_backoff(
            move || {
                let attempts = attempts_clone.clone();
                async move {
                    let current_attempt =
                        attempts.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;
                    if current_attempt < 3 {
                        Err("temporary error")
                    } else {
                        Ok("success")
                    }
                }
            },
            5,
            Duration::from_millis(10),
        )
        .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "success");
    }
}
