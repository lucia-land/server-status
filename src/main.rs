mod cron_action; // ✅ 모듈 불러오기

use tokio::time::{interval, Duration};
use chrono::Local;
use anyhow::Result;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<()> {
    let mut ticker = interval(Duration::from_secs(60 * 10)); // 10분마다 실행

    loop {
        ticker.tick().await;
        let local_now = Local::now();
        println!("[{local_now}] Collecting cluster status...");

        match cron_action::collect_pod_status().await {
            Ok(pods) => {
                // 로그 형태로 시간정보와 함께 저장
                let log_entry = format!("[{}] Pod Status Check:\n{}\n---\n", 
                                      local_now, pods.join("\n"));
                
                // 기존 파일에 추가 (덮어쓰지 않음)
                std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open("status.log")?
                    .write_all(log_entry.as_bytes())?;
                
                println!("✅ Logged {} pod statuses to status.log", pods.len());
            }
            Err(e) => {
                let error_log = format!("[{}] ERROR: {}\n", local_now, e);
                std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open("status.log")?
                    .write_all(error_log.as_bytes())?;
                eprintln!("❌ Error: {e}");
            }
        }
    }
}
