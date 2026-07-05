use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::utils::{safe_path, secure_filename};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UploadMeta {
    device: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    time: chrono::DateTime<chrono::Utc>,
}

pub struct FileManager {
    pub upload_folder: String,
}

impl FileManager {
    pub fn new(upload_folder: String) -> Self {
        let fm = Self { upload_folder };
        fm.ensure_folders();
        fm
    }

    pub fn ensure_folders(&self) {
        if let Err(e) = fs::create_dir_all(&self.upload_folder) {
            eprintln!("[FileManager] create_dir_all {} failed: {e}", self.upload_folder);
        }
    }

    fn metadata_path(&self) -> PathBuf {
        Path::new(&self.upload_folder).join(".metadata.json")
    }

    fn load_metadata(&self) -> HashMap<String, UploadMeta> {
        let path = self.metadata_path();
        if !path.exists() {
            return HashMap::new();
        }
        fs::read_to_string(&path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    fn save_metadata(&self, meta: &HashMap<String, UploadMeta>) {
        let path = self.metadata_path();
        if let Ok(s) = serde_json::to_string_pretty(meta) {
            let _ = fs::write(path, s);
        }
    }

    pub fn save_upload_meta(&self, filename: &str, device_type: &str) {
        let mut meta = self.load_metadata();
        meta.insert(
            filename.to_string(),
            UploadMeta {
                device: device_type.to_string(),
                time: chrono::Utc::now(),
            },
        );
        self.save_metadata(&meta);
    }

    fn remove_from_metadata(&self, filenames: &[String]) {
        let mut meta = self.load_metadata();
        for name in filenames {
            meta.remove(name);
        }
        self.save_metadata(&meta);
    }

    pub fn list_files(&self) -> Vec<serde_json::Value> {
        let mut files = Vec::new();
        let folder = Path::new(&self.upload_folder);
        let upload_meta = self.load_metadata();
        if let Ok(entries) = fs::read_dir(folder) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if name.starts_with('.') {
                        continue;
                    }
                    if let Ok(meta) = entry.metadata() {
                        let info = upload_meta.get(&name);
                        files.push(serde_json::json!({
                            "name": name,
                            "size": meta.len(),
                            "modified": meta.modified().ok()
                                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                                .map(|d| d.as_secs_f64())
                                .unwrap_or(0.0),
                            "source": "uploaded",
                            "device": info.map(|m| m.device.clone()).unwrap_or_else(|| "Unknown".into())
                        }));
                    }
                }
            }
        }
        files
    }

    pub fn unique_filepath(&self, filename: &str) -> PathBuf {
        let safe = secure_filename(filename);
        let folder = Path::new(&self.upload_folder);
        let mut path = folder.join(&safe);
        let stem = Path::new(&safe)
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "file".into());
        let ext = Path::new(&safe)
            .extension()
            .map(|s| format!(".{}", s.to_string_lossy()))
            .unwrap_or_default();
        let mut counter = 1;
        while path.exists() {
            path = folder.join(format!("{stem}_{counter}{ext}"));
            counter += 1;
        }
        path
    }

    pub fn find_file_path(&self, filename: &str) -> Option<PathBuf> {
        if let Some(path) = safe_path(&self.upload_folder, filename) {
            if path.exists() && path.is_file() {
                return Some(path);
            }
        }
        None
    }

    pub fn delete_files(&self, filenames: &[String]) -> usize {
        let mut deleted = 0;
        for name in filenames {
            if let Some(path) = self.find_file_path(name) {
                if fs::remove_file(&path).is_ok() {
                    deleted += 1;
                }
            }
        }
        self.remove_from_metadata(filenames);
        deleted
    }

    pub fn get_disk_info(&self) -> Option<serde_json::Value> {
        let folder = Path::new(&self.upload_folder);
        let total = fs2::total_space(folder).ok()?;
        let free = fs2::available_space(folder).ok()?;
        let used = total - free;
        Some(serde_json::json!({
            "total": total,
            "used": used,
            "free": free
        }))
    }

    pub fn save_chunk(
        &self,
        file_id: &str,
        chunk_index: usize,
        total_chunks: usize,
        filename: &str,
        data: &[u8],
    ) -> (bool, usize, Option<String>) {
        let temp_dir = Path::new(&self.upload_folder).join(format!(".temp_{file_id}"));
        if let Err(e) = fs::create_dir_all(&temp_dir) {
            eprintln!("[FileManager] create temp dir failed: {e}");
            return (false, 0, None);
        }

        let chunk_path = temp_dir.join(format!("chunk_{chunk_index}"));
        if fs::write(&chunk_path, data).is_err() {
            return (false, 0, None);
        }

        let uploaded_chunks = match fs::read_dir(&temp_dir) {
            Ok(entries) => entries
                .flatten()
                .filter(|e| e.file_name().to_string_lossy().starts_with("chunk_"))
                .count(),
            Err(_) => 0,
        };

        if uploaded_chunks == total_chunks {
            let final_path = self.unique_filepath(filename);
            match fs::File::create(&final_path) {
                Ok(mut outfile) => {
                    let mut ok = true;
                    for i in 0..total_chunks {
                        let chunk_file = temp_dir.join(format!("chunk_{i}"));
                        match fs::read(&chunk_file) {
                            Ok(bytes) => {
                                if outfile.write_all(&bytes).is_err() {
                                    ok = false;
                                    break;
                                }
                            }
                            Err(_) => {
                                ok = false;
                                break;
                            }
                        }
                    }
                    if ok {
                        let _ = fs::remove_dir_all(&temp_dir);
                        let saved_name = final_path.file_name().map(|s| s.to_string_lossy().to_string());
                        return (true, uploaded_chunks, saved_name);
                    }
                }
                Err(e) => eprintln!("[FileManager] create final file failed: {e}"),
            }
        }

        (false, uploaded_chunks, None)
    }

    pub fn cleanup_temp_dirs(&self, max_age_hours: u64) {
        let folder = Path::new(&self.upload_folder);
        if !folder.exists() {
            return;
        }
        let now = std::time::SystemTime::now();
        let max_age = std::time::Duration::from_secs(max_age_hours * 3600);
        if let Ok(entries) = fs::read_dir(folder) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if name.starts_with(".temp_") {
                        if let Ok(meta) = entry.metadata() {
                            if let Ok(created) = meta.created() {
                                if now.duration_since(created).unwrap_or_default() > max_age {
                                    if let Err(e) = fs::remove_dir_all(&path) {
                                        eprintln!("[Cleanup] Failed to remove {path:?}: {e}");
                                    } else {
                                        println!("[Cleanup] Removed old temp dir: {path:?}");
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
