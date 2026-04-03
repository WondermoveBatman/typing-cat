# Typing Cat 🐱

**macOS 전용** 메뉴바 타자 카운터 앱 - runcat에서 영감을 받아 제작

## 설치 방법

### GitHub Releases에서 다운로드

1. [Releases](https://github.com/WondermoveBatman/typing-cat/releases)에서 최신 `.dmg` 파일 다운로드
2. DMG 파일을 열고 앱을 Applications 폴더로 드래그
3. 처음 실행 시 **"확인되지 않은 개발자" 경고**가 표시됩니다
4. 앱을 **우클릭 → 열기** 클릭
5. 대화상자에서 **"열기"** 버튼 클릭

> **참고**: 이 앱은 현재 Apple Developer 계정으로 서명되지 않았습니다. 위 방법으로 안전하게 실행할 수 있습니다.

## 기능

- 실시간 타자 카운트
- WPM/CPM 타이핑 속도 측정
- 일별/주별/월별 통계
- 메뉴바 상주 (Dock 아이콘 없음)

## 지원 플랫폼

- **macOS 10.15+** (Catalina 이상)
- Apple Silicon / Intel 모두 지원

## 기술 스택

- **Framework**: Tauri v2
- **Backend**: Rust
- **Frontend**: React + TypeScript
- **Keyboard Hook**: rdev (CGEventTap)
- **Database**: SQLite (rusqlite)
- **State Management**: Zustand

## 개발 환경 설정

### 필수 요구사항

- [Rust](https://www.rust-lang.org/tools/install) (1.75+)
- [Node.js](https://nodejs.org/) (22+)
- macOS 10.15+

### 설치

```bash
# Rust 설치
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Node.js 22+ 설치 (nvm 사용 시)
nvm install 22
nvm use 22

# 의존성 설치
npm install

# 개발 서버 실행
npm run tauri dev

# 프로덕션 빌드
npm run tauri build
```

## macOS 권한 설정

키보드 입력 감지를 위해 **Accessibility 권한**이 필요합니다.

1. 앱 실행 시 권한 요청 다이얼로그가 자동으로 표시됩니다
2. 또는 수동으로: **시스템 설정** → **개인정보 보호 및 보안** → **손쉬운 사용**

## 프로젝트 구조

```
keystroke-counter/
├── src/                    # React 프론트엔드
│   ├── components/         # UI 컴포넌트
│   ├── hooks/              # React hooks
│   ├── stores/             # Zustand 상태 관리
│   ├── services/           # Tauri 커맨드 래퍼
│   └── types/              # TypeScript 타입
│
└── src-tauri/              # Rust 백엔드
    ├── src/
    │   ├── domain/         # 도메인 엔티티
    │   ├── infrastructure/ # 키보드 훅, DB, 트레이, macOS API
    │   └── presentation/   # Tauri 커맨드
    └── migrations/         # SQL 마이그레이션
```

## 라이선스

MIT
