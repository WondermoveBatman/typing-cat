export interface CurrentStats {
  total_keystrokes: number;
  printable_chars: number;
  wpm: number;
  cpm: number;
  is_session_active: boolean;
}

export interface DailyStats {
  date: string;
  total_keystrokes: number;
  printable_chars: number;
  typing_duration_seconds: number;
  session_count: number;
}
