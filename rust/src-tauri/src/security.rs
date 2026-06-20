use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// 登录尝试限流器：按 IP 记录失败次数，达到上限后锁定一段时间。
pub struct LoginRateLimiter {
    max_attempts: usize,
    lockout_seconds: u64,
    attempts: Mutex<HashMap<String, (usize, Instant)>>,
}

impl LoginRateLimiter {
    pub fn new() -> Self {
        Self {
            max_attempts: 5,
            lockout_seconds: 300,
            attempts: Mutex::new(HashMap::new()),
        }
    }

    pub fn is_locked(&self, client_ip: &str) -> bool {
        let mut attempts = self.attempts.lock().unwrap();
        if let Some((count, last_attempt)) = attempts.get(client_ip) {
            if *count >= self.max_attempts {
                if last_attempt.elapsed() < Duration::from_secs(self.lockout_seconds) {
                    return true;
                }
                // 锁已过期，清除记录
                attempts.remove(client_ip);
            }
        }
        false
    }

    pub fn record_failure(&self, client_ip: &str) {
        let mut attempts = self.attempts.lock().unwrap();
        let entry = attempts
            .entry(client_ip.to_string())
            .or_insert((0, Instant::now()));
        entry.0 += 1;
        entry.1 = Instant::now();
    }

    pub fn record_success(&self, client_ip: &str) {
        let mut attempts = self.attempts.lock().unwrap();
        attempts.remove(client_ip);
    }

    pub fn get_remaining_lockout(&self, client_ip: &str) -> u64 {
        let attempts = self.attempts.lock().unwrap();
        if let Some((count, last_attempt)) = attempts.get(client_ip) {
            if *count >= self.max_attempts {
                let elapsed = last_attempt.elapsed();
                let remaining = Duration::from_secs(self.lockout_seconds).saturating_sub(elapsed);
                return remaining.as_secs();
            }
        }
        0
    }
}
