import { useEffect, useState } from "react";
import { useTypingStats } from "../../hooks/useTypingStats";
import { checkAccessibility } from "../../services/tauriCommands";
import { StatCard } from "./StatCard";
import { BarChart, Bar, LineChart, Line, XAxis, Tooltip, ResponsiveContainer } from 'recharts';

export function Dashboard() {
  const { currentStats, weeklyStats, monthlyStats, activeTab, setActiveTab, isTracking, toggleTracking } = useTypingStats();
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

  // Custom Tooltip for Recharts
  const CustomTooltip = ({ active, payload }: any) => {
    if (active && payload && payload.length) {
      const data = payload[0].payload;
      return (
        <div style={{
          background: 'var(--bg-card)',
          padding: '8px 12px',
          borderRadius: '6px',
          border: '1px solid var(--border)',
          fontSize: '12px'
        }}>
          <div style={{ color: 'var(--text-secondary)', marginBottom: '4px' }}>
            {new Date(data.date).toLocaleDateString('ko-KR')}
          </div>
          <div style={{ color: 'var(--text-primary)', fontWeight: '600' }}>
            {formatNumber(data.total_keystrokes)} 타자
          </div>
        </div>
      );
    }
    return null;
  };

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

      {(weeklyStats.length > 0 || monthlyStats.length > 0) && (
        <div className="weekly-chart">
          {/* Tab Buttons */}
          <div className="chart-tabs">
            <button
              className={`tab-btn ${activeTab === 'weekly' ? 'active' : ''}`}
              onClick={() => setActiveTab('weekly')}
            >
              주간
            </button>
            <button
              className={`tab-btn ${activeTab === 'monthly' ? 'active' : ''}`}
              onClick={() => setActiveTab('monthly')}
            >
              월간
            </button>
          </div>

          {/* Weekly Chart */}
          {activeTab === 'weekly' && weeklyStats.length > 0 && (
            <ResponsiveContainer width="100%" height={120}>
              <BarChart data={weeklyStats}>
                <XAxis
                  dataKey="date"
                  tickFormatter={(d) => new Date(d).toLocaleDateString('ko-KR', { weekday: 'short' })}
                  stroke="var(--text-secondary)"
                  fontSize={10}
                />
                <Tooltip content={<CustomTooltip />} />
                <Bar dataKey="total_keystrokes" fill="#6366f1" radius={[4, 4, 0, 0]} />
              </BarChart>
            </ResponsiveContainer>
          )}

          {/* Monthly Chart */}
          {activeTab === 'monthly' && monthlyStats.length > 0 && (
            <ResponsiveContainer width="100%" height={120}>
              <LineChart data={monthlyStats}>
                <XAxis
                  dataKey="date"
                  tickFormatter={(d) => new Date(d).getDate().toString()}
                  stroke="var(--text-secondary)"
                  fontSize={10}
                />
                <Tooltip content={<CustomTooltip />} />
                <Line
                  type="monotone"
                  dataKey="total_keystrokes"
                  stroke="#6366f1"
                  strokeWidth={2}
                  dot={{ fill: '#6366f1', r: 3 }}
                  activeDot={{ r: 5 }}
                />
              </LineChart>
            </ResponsiveContainer>
          )}
        </div>
      )}
    </div>
  );
}
