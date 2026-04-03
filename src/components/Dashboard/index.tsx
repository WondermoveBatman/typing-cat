import { useEffect, useState } from "react";
import { useTypingStats } from "../../hooks/useTypingStats";
import { checkAccessibility } from "../../services/tauriCommands";
import { StatCard } from "./StatCard";

export function Dashboard() {
  const { currentStats, weeklyStats, isTracking, toggleTracking } = useTypingStats();
  const [accessibilityGranted, setAccessibilityGranted] = useState<boolean | null>(null);

  useEffect(() => {
    // Check accessibility permission on mount
    checkAccessibility().then((status) => {
      setAccessibilityGranted(status.granted);
    });
  }, []);

  const handleRequestPermission = async () => {
    const status = await checkAccessibility();
    setAccessibilityGranted(status.granted);
  };

  const formatNumber = (num: number) => {
    return num.toLocaleString();
  };

  const totalWeeklyKeystrokes = weeklyStats.reduce(
    (sum, day) => sum + day.total_keystrokes,
    0
  );

  return (
    <div className="dashboard">
      <div className="dashboard-header">
        <h1>Keystroke Counter</h1>
        <button
          className={`tracking-btn ${isTracking ? "active" : ""}`}
          onClick={toggleTracking}
          disabled={accessibilityGranted === false}
        >
          {isTracking ? "추적 중지" : "추적 시작"}
        </button>
      </div>

      {/* Permission Alert */}
      {accessibilityGranted === false && (
        <div className="permission-alert">
          <div className="alert-icon">⚠️</div>
          <div className="alert-content">
            <div className="alert-title">접근성 권한 필요</div>
            <div className="alert-message">
              키보드 입력을 감지하려면 접근성 권한이 필요합니다.
              <br />
              시스템 설정 → 개인정보 보호 및 보안 → 손쉬운 사용에서 앱을 허용해주세요.
            </div>
            <button className="alert-btn" onClick={handleRequestPermission}>
              권한 설정 열기
            </button>
          </div>
        </div>
      )}

      <div className="stats-grid">
        <StatCard
          title="현재 세션"
          value={formatNumber(currentStats.total_keystrokes)}
          subtitle="타자 수"
        />
        <StatCard
          title="WPM"
          value={currentStats.wpm.toFixed(1)}
          subtitle="분당 단어"
        />
        <StatCard
          title="CPM"
          value={currentStats.cpm.toFixed(0)}
          subtitle="분당 글자"
        />
        <StatCard
          title="이번 주"
          value={formatNumber(totalWeeklyKeystrokes)}
          subtitle="총 타자 수"
        />
      </div>

      <div className="status-indicator">
        <span
          className={`status-dot ${currentStats.is_session_active ? "active" : ""}`}
        />
        <span>{currentStats.is_session_active ? "타이핑 중..." : "대기 중"}</span>
      </div>

      {weeklyStats.length > 0 && (
        <div className="weekly-chart">
          <h3>주간 통계</h3>
          <div className="chart-bars">
            {weeklyStats.map((day) => {
              const maxKeystrokes = Math.max(...weeklyStats.map((d) => d.total_keystrokes));
              const height = maxKeystrokes > 0
                ? (day.total_keystrokes / maxKeystrokes) * 100
                : 0;

              return (
                <div key={day.date} className="chart-bar-container">
                  <div
                    className="chart-bar"
                    style={{ height: `${height}%` }}
                    title={`${day.date}: ${formatNumber(day.total_keystrokes)}`}
                  />
                  <span className="chart-label">
                    {new Date(day.date).toLocaleDateString("ko-KR", { weekday: "short" })}
                  </span>
                </div>
              );
            })}
          </div>
        </div>
      )}
    </div>
  );
}
