use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct DeviceInfo {
    #[serde(rename = "type")]
    pub device_type: String,
    pub detail: String,
    pub model: String,
}

/// 本地 IP 缓存
static LOCAL_IP_CACHE: Mutex<(Option<String>, Option<Instant>)> = Mutex::new((None, None));

/// 获取本地局域网 IP（优先使用 local-ip-address crate，无弹窗风险）
pub fn get_local_ip(cache_seconds: u64) -> String {
    let mut cache = LOCAL_IP_CACHE.lock().unwrap();
    let now = Instant::now();

    if let (Some(ip), Some(time)) = (&cache.0, cache.1) {
        if now.duration_since(time) < Duration::from_secs(cache_seconds) {
            return ip.clone();
        }
    }

    let ip = local_ip_address::local_ip()
        .map(|ip| ip.to_string())
        .unwrap_or_else(|_| "127.0.0.1".to_string());

    *cache = (Some(ip.clone()), Some(now));
    ip
}

/// 从字符串中提取 "key 数字" 格式的版本号，例如 "iphone os 14_2" -> "14.2"
fn extract_version_after(haystack: &str, key: &str) -> Option<String> {
    let lower = haystack.to_lowercase();
    let start = lower.find(key)? + key.len();
    let rest = &lower[start..];
    let end = rest
        .find(|c: char| !(c.is_ascii_digit() || c == '.' || c == '_'))
        .unwrap_or(rest.len());
    let version = &rest[..end];
    if version.is_empty() {
        None
    } else {
        Some(version.replace('_', "."))
    }
}

/// 解析 User-Agent 获取设备信息
pub fn parse_user_agent(ua: &str) -> DeviceInfo {
    if ua.is_empty() {
        return DeviceInfo {
            device_type: "Unknown".into(),
            detail: "未知设备".into(),
            model: "Unknown".into(),
        };
    }

    let ua_lower = ua.to_lowercase();

    // iPhone
    if ua_lower.contains("iphone") {
        let ios_version = extract_version_after(&ua_lower, "iphone os ")
            .unwrap_or_else(|| "Unknown".into());
        return DeviceInfo {
            device_type: "iPhone".into(),
            detail: format!("iOS {ios_version}"),
            model: "iPhone".into(),
        };
    }

    // iPad
    if ua_lower.contains("ipad") {
        let os_version = extract_version_after(&ua_lower, "os ")
            .unwrap_or_else(|| "Unknown".into());
        return DeviceInfo {
            device_type: "iPad".into(),
            detail: format!("iPadOS {os_version}"),
            model: "iPad".into(),
        };
    }

    // Android
    if ua_lower.contains("android") {
        let os_version = extract_version_after(&ua_lower, "android ")
            .unwrap_or_else(|| "Unknown".into());

        let model = ua_lower
            .find(" build")
            .and_then(|end| {
                let before = &ua_lower[..end];
                before
                    .rfind(';')
                    .map(|start| before[start + 1..].trim().to_string())
            })
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "Android".into());

        let device_type = if ua_lower.contains("mobile") {
            "Android"
        } else {
            "Android平板"
        };

        return DeviceInfo {
            device_type: device_type.into(),
            detail: format!("Android {os_version}"),
            model,
        };
    }

    // Windows
    if ua_lower.contains("windows") {
        return DeviceInfo {
            device_type: "Windows PC".into(),
            detail: "Windows".into(),
            model: "PC".into(),
        };
    }

    // Mac
    if ua_lower.contains("macintosh") || ua_lower.contains("mac os") {
        return DeviceInfo {
            device_type: "Mac".into(),
            detail: "macOS".into(),
            model: "Mac".into(),
        };
    }

    // Linux
    if ua_lower.contains("linux") {
        return DeviceInfo {
            device_type: "Linux".into(),
            detail: "Linux".into(),
            model: "Linux".into(),
        };
    }

    DeviceInfo {
        device_type: "Unknown".into(),
        detail: ua.chars().take(50).collect(),
        model: "Unknown".into(),
    }
}

/// 仿 werkzeug secure_filename：去除路径分隔符与危险字符
pub fn secure_filename(name: &str) -> String {
    let name = name.replace(['/', '\\'], "_");
    name.chars()
        .filter(|c| c.is_alphanumeric() || "._- ".contains(*c))
        .collect::<String>()
        .trim_start_matches('.')
        .to_string()
}

/// 安全路径检查：防止路径遍历攻击
pub fn safe_path(base_dir: &str, filename: &str) -> Option<PathBuf> {
    let base = Path::new(base_dir).canonicalize().ok()?;
    let name = secure_filename(filename);
    if name.is_empty() {
        return None;
    }
    let full = base.join(&name).canonicalize().ok()?;
    if full.starts_with(&base) {
        Some(full)
    } else {
        None
    }
}
