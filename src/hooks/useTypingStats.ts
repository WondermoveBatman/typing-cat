import { useEffect, useCallback } from "react";
import { listen } from "@tauri-apps/api/event";
import { useStatsStore } from "../stores/statsStore";
import {
  getCurrentStats,
  getWeeklyStats,
  startTracking,
  stopTracking,
  isTracking as checkIsTracking,
} from "../services/tauriCommands";

export function useTypingStats() {
  const {
    currentStats,
    weeklyStats,
    isTracking,
    isLoading,
    setCurrentStats,
    setWeeklyStats,
    setIsTracking,
    setIsLoading,
  } = useStatsStore();

  // Fetch current stats
  const fetchCurrentStats = useCallback(async () => {
    try {
      const stats = await getCurrentStats();
      setCurrentStats(stats);
    } catch (error) {
      console.error("Failed to fetch current stats:", error);
    }
  }, [setCurrentStats]);

  // Fetch weekly stats
  const fetchWeeklyStats = useCallback(async () => {
    try {
      setIsLoading(true);
      const stats = await getWeeklyStats();
      setWeeklyStats(stats);
    } catch (error) {
      console.error("Failed to fetch weekly stats:", error);
    } finally {
      setIsLoading(false);
    }
  }, [setWeeklyStats, setIsLoading]);

  // Toggle tracking
  const toggleTracking = useCallback(async () => {
    try {
      if (isTracking) {
        await stopTracking();
        setIsTracking(false);
      } else {
        await startTracking();
        setIsTracking(true);
      }
    } catch (error) {
      console.error("Failed to toggle tracking:", error);
    }
  }, [isTracking, setIsTracking]);

  // Initialize
  useEffect(() => {
    const init = async () => {
      const tracking = await checkIsTracking();
      setIsTracking(tracking);
      await fetchCurrentStats();
      await fetchWeeklyStats();
    };
    init();

    // Listen for tracking events from tray
    const unlistenStart = listen("start-tracking", async () => {
      await startTracking();
      setIsTracking(true);
    });

    const unlistenStop = listen("stop-tracking", async () => {
      await stopTracking();
      setIsTracking(false);
    });

    // Poll for stats updates
    const interval = setInterval(fetchCurrentStats, 1000);

    return () => {
      clearInterval(interval);
      unlistenStart.then((fn) => fn());
      unlistenStop.then((fn) => fn());
    };
  }, [fetchCurrentStats, fetchWeeklyStats, setIsTracking]);

  return {
    currentStats,
    weeklyStats,
    isTracking,
    isLoading,
    toggleTracking,
    fetchWeeklyStats,
  };
}
