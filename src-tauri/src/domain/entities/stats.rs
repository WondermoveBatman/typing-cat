use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

/// Represents a single keystroke event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeystrokeEvent {
    pub key: String,
    pub timestamp: DateTime<Utc>,
    pub is_printable: bool,
}

/// Current session typing statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypingStats {
    pub total_keystrokes: u64,
    pub printable_chars: u64,
    pub session_start: Option<DateTime<Utc>>,
    pub last_keystroke: Option<DateTime<Utc>>,
}

impl TypingStats {
    pub fn new() -> Self {
        Self {
            total_keystrokes: 0,
            printable_chars: 0,
            session_start: None,
            last_keystroke: None,
        }
    }

    pub fn record_keystroke(&mut self, is_printable: bool) {
        let now = Utc::now();

        // Check if session should be reset (5 min idle)
        if let Some(last) = self.last_keystroke {
            if now - last > Duration::minutes(5) {
                self.reset_session();
            }
        }

        // Start new session if needed
        if self.session_start.is_none() {
            self.session_start = Some(now);
        }

        self.total_keystrokes += 1;
        if is_printable {
            self.printable_chars += 1;
        }
        self.last_keystroke = Some(now);
    }

    pub fn reset_session(&mut self) {
        self.total_keystrokes = 0;
        self.printable_chars = 0;
        self.session_start = None;
        self.last_keystroke = None;
    }

    /// Calculate Characters Per Minute
    pub fn calculate_cpm(&self) -> f64 {
        if let (Some(start), Some(end)) = (self.session_start, self.last_keystroke) {
            let duration_minutes = (end - start).num_seconds() as f64 / 60.0;
            if duration_minutes > 0.0 {
                return self.printable_chars as f64 / duration_minutes;
            }
        }
        0.0
    }

    /// Calculate Words Per Minute (standard: 5 chars = 1 word)
    pub fn calculate_wpm(&self) -> f64 {
        self.calculate_cpm() / 5.0
    }

    /// Check if session is currently active
    pub fn is_session_active(&self) -> bool {
        if let Some(last) = self.last_keystroke {
            let idle_duration = Utc::now() - last;
            return idle_duration < Duration::minutes(5);
        }
        false
    }
}

impl Default for TypingStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Daily aggregated statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyStats {
    pub date: String,
    pub total_keystrokes: u64,
    pub printable_chars: u64,
    pub typing_duration_seconds: u64,
    pub session_count: u32,
}

impl DailyStats {
    pub fn calculate_avg_wpm(&self) -> f64 {
        if self.typing_duration_seconds > 0 {
            let duration_minutes = self.typing_duration_seconds as f64 / 60.0;
            (self.printable_chars as f64 / duration_minutes) / 5.0
        } else {
            0.0
        }
    }
}
