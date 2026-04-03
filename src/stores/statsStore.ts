import { create } from "zustand";
import type { CurrentStats, DailyStats } from "../types/stats";

interface StatsState {
  currentStats: CurrentStats;
  weeklyStats: DailyStats[];
  monthlyStats: DailyStats[];
  activeTab: 'weekly' | 'monthly';
  isTracking: boolean;
  isLoading: boolean;

  setCurrentStats: (stats: CurrentStats) => void;
  setWeeklyStats: (stats: DailyStats[]) => void;
  setMonthlyStats: (stats: DailyStats[]) => void;
  setActiveTab: (tab: 'weekly' | 'monthly') => void;
  setIsTracking: (tracking: boolean) => void;
  setIsLoading: (loading: boolean) => void;
}

export const useStatsStore = create<StatsState>((set) => ({
  currentStats: {
    total_keystrokes: 0,
    printable_chars: 0,
    wpm: 0,
    cpm: 0,
    is_session_active: false,
  },
  weeklyStats: [],
  monthlyStats: [],
  activeTab: 'weekly',
  isTracking: false,
  isLoading: false,

  setCurrentStats: (stats) => set({ currentStats: stats }),
  setWeeklyStats: (stats) => set({ weeklyStats: stats }),
  setMonthlyStats: (stats) => set({ monthlyStats: stats }),
  setActiveTab: (tab) => set({ activeTab: tab }),
  setIsTracking: (tracking) => set({ isTracking: tracking }),
  setIsLoading: (loading) => set({ isLoading: loading }),
}));
