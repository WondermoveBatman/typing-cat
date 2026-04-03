import { invoke } from "@tauri-apps/api/core";
import type { CurrentStats, DailyStats } from "../types/stats";

export async function getCurrentStats(): Promise<CurrentStats> {
  return invoke<CurrentStats>("get_current_stats");
}

export async function getDailyStats(date?: string): Promise<DailyStats | null> {
  return invoke<DailyStats | null>("get_daily_stats", { date });
}

export async function getWeeklyStats(): Promise<DailyStats[]> {
  return invoke<DailyStats[]>("get_weekly_stats");
}

export async function getMonthlyStats(): Promise<DailyStats[]> {
  return invoke<DailyStats[]>("get_monthly_stats");
}

export async function startTracking(): Promise<boolean> {
  return invoke<boolean>("start_tracking");
}

export async function stopTracking(): Promise<boolean> {
  return invoke<boolean>("stop_tracking");
}

export async function isTracking(): Promise<boolean> {
  return invoke<boolean>("is_tracking");
}

export interface AccessibilityStatus {
  granted: boolean;
  platform: string;
}

export async function checkAccessibility(): Promise<AccessibilityStatus> {
  return invoke<AccessibilityStatus>("check_accessibility");
}
