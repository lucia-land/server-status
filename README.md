# Server Status Monitor 🚀

Kubernetes 클러스터의 Pod 상태를 주기적으로 모니터링하고 로깅하는 Rust 기반 도구입니다.

## 📋 기능

- **주기적 모니터링**: 10분마다 자동으로 Pod 상태 확인
- **상태 로깅**: 타임스탬프와 함께 `status.log` 파일에 기록
- **에러 핸들링**: 연결 실패 시에도 안전하게 에러 로그 기록
- **모든 네임스페이스**: 클러스터 전체의 Pod 상태 수집

## 🛠 설치 및 설정

### 요구사항
- **Rust** (1.70+)
- **Kubernetes 클러스터** 접근 권한
- **kubectl** 설정 완료 (`~/.kube/config`)

### 설치 방법

1. **리포지토리 클론**
   ```bash
   git clone https://github.com/your-username/server-status.git
   cd server-status
   ```

2. **의존성 설치 및 빌드**
   ```bash
   cargo build --release
   ```

3. **실행**
   ```bash
   cargo run
   ```

## 🚀 사용법

### 기본 실행
```bash
cargo run
```

### 백그라운드 실행 (Linux/macOS)
```bash
nohup cargo run > app.log 2>&1 &
```

### 출력 예시
```
[2024-10-07 14:30:00] Collecting cluster status...
✅ Logged 15 pod statuses to status.log
[2024-10-07 14:40:00] Collecting cluster status...
✅ Logged 15 pod statuses to status.log
```

## 📊 로그 형식

`status.log` 파일에 다음과 같은 형식으로 기록됩니다:

```
[2024-10-07 14:30:00] Pod Status Check:
nginx-deployment-abc123: Running
redis-master-def456: Running
mysql-server-ghi789: Pending
---

[2024-10-07 14:40:00] ERROR: Failed to connect to cluster
```

## ⚙️ 설정

현재 하드코딩된 설정값들:
- **모니터링 간격**: 10분 (`Duration::from_secs(60 * 10)`)
- **로그 파일명**: `status.log`
- **대상 리소스**: 모든 네임스페이스의 Pod

## 🔧 개발

### 프로젝트 구조
```
src/
├── main.rs          # 메인 실행 루프
└── cron_action.rs   # Kubernetes API 호출 로직
```

### 기여하기
1. Fork this repository
2. Create feature branch (`git checkout -b feature/amazing-feature`)
3. Commit changes (`git commit -m 'Add amazing feature'`)
4. Push to branch (`git push origin feature/amazing-feature`)
5. Open Pull Request

## 📝 라이선스

이 프로젝트는 [MIT License](LICENSE)를 따릅니다.

## 🐛 문제 해결

### 권한 에러
```bash
# Kubernetes 클러스터 권한 확인
kubectl auth can-i get pods --all-namespaces
```

### 연결 실패
- `~/.kube/config` 파일 확인
- 클러스터 연결 상태 점검: `kubectl cluster-info`