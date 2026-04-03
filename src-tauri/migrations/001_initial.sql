-- Daily statistics table
CREATE TABLE IF NOT EXISTS daily_stats (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    date TEXT NOT NULL UNIQUE,
    total_keystrokes INTEGER DEFAULT 0,
    printable_chars INTEGER DEFAULT 0,
    typing_duration_seconds INTEGER DEFAULT 0,
    session_count INTEGER DEFAULT 0,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP
);

-- Session records table
CREATE TABLE IF NOT EXISTS typing_sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    start_time TEXT NOT NULL,
    end_time TEXT,
    keystrokes INTEGER DEFAULT 0,
    printable_chars INTEGER DEFAULT 0,
    avg_wpm REAL DEFAULT 0,
    max_wpm REAL DEFAULT 0,
    date TEXT NOT NULL
);

-- Indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_daily_stats_date ON daily_stats(date);
CREATE INDEX IF NOT EXISTS idx_sessions_date ON typing_sessions(date);
