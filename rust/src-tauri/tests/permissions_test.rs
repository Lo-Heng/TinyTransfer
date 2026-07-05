// permissions_test.rs - Integration tests for Tauri permissions configuration
// Bug 1: Download functionality ACL fix verification

use std::path::Path;

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that capabilities/main.json exists and is valid JSON
    #[test]
    fn test_capabilities_file_exists() {
        let capabilities_path = Path::new("capabilities/main.json");
        assert!(capabilities_path.exists(), "capabilities/main.json should exist");
    }

    /// Test that the capabilities file has correct permissions for dialog:save
    #[test]
    fn test_dialog_save_permission() {
        let capabilities_path = Path::new("capabilities/main.json");
        let content = std::fs::read_to_string(capabilities_path)
            .expect("Should be able to read capabilities file");
        
        let json: serde_json::Value = serde_json::from_str(&content)
            .expect("Should be valid JSON");
        
        let permissions = json["permissions"].as_array()
            .expect("permissions should be an array");
        
        let permission_strings: Vec<&str> = permissions.iter()
            .filter_map(|p| p.as_str())
            .collect();
        
        // Check that dialog:allow-save is present (Tauri v2 correct format)
        assert!(
            permission_strings.contains(&"dialog:allow-save"),
            "Should have 'dialog:allow-save' permission. Found: {:?}",
            permission_strings
        );
        
        // Check that the old incorrect format is NOT present
        assert!(
            !permission_strings.contains(&"dialog:save"),
            "Should NOT have incorrect 'dialog:save' permission"
        );
        
        // Check that dialog:default is present
        assert!(
            permission_strings.contains(&"dialog:default"),
            "Should have 'dialog:default' permission for default dialog capabilities"
        );
    }

    /// Test that the capabilities file has correct permissions for fs:write
    #[test]
    fn test_fs_write_permission() {
        let capabilities_path = Path::new("capabilities/main.json");
        let content = std::fs::read_to_string(capabilities_path)
            .expect("Should be able to read capabilities file");
        
        let json: serde_json::Value = serde_json::from_str(&content)
            .expect("Should be valid JSON");
        
        let permissions = json["permissions"].as_array()
            .expect("permissions should be an array");
        
        let permission_strings: Vec<&str> = permissions.iter()
            .filter_map(|p| p.as_str())
            .collect();
        
        // Check that fs:allow-write is present (Tauri v2 correct format)
        assert!(
            permission_strings.contains(&"fs:allow-write"),
            "Should have 'fs:allow-write' permission. Found: {:?}",
            permission_strings
        );
        
        // Check that the old incorrect format is NOT present
        assert!(
            !permission_strings.contains(&"fs:write"),
            "Should NOT have incorrect 'fs:write' permission"
        );
        
        // Check that fs:default is present
        assert!(
            permission_strings.contains(&"fs:default"),
            "Should have 'fs:default' permission for default filesystem capabilities"
        );
    }

    /// Test that all required permissions are present
    #[test]
    fn test_all_required_permissions() {
        let capabilities_path = Path::new("capabilities/main.json");
        let content = std::fs::read_to_string(capabilities_path)
            .expect("Should be able to read capabilities file");
        
        let json: serde_json::Value = serde_json::from_str(&content)
            .expect("Should be valid JSON");
        
        let permissions = json["permissions"].as_array()
            .expect("permissions should be an array");
        
        let permission_strings: Vec<&str> = permissions.iter()
            .filter_map(|p| p.as_str())
            .collect();
        
        // List of required permissions for the app to function
        let required_permissions = vec![
            "core:default",
            "dialog:default",
            "dialog:allow-save",
            "fs:default",
            "fs:allow-write",
            "opener:default",
        ];
        
        for perm in required_permissions {
            assert!(
                permission_strings.contains(&perm),
                "Missing required permission: '{}'. Found: {:?}",
                perm,
                permission_strings
            );
        }
    }

    /// Test that the capability has the correct identifier and windows
    #[test]
    fn test_capability_metadata() {
        let capabilities_path = Path::new("capabilities/main.json");
        let content = std::fs::read_to_string(capabilities_path)
            .expect("Should be able to read capabilities file");
        
        let json: serde_json::Value = serde_json::from_str(&content)
            .expect("Should be valid JSON");
        
        // Check identifier
        assert_eq!(
            json["identifier"].as_str(),
            Some("main-capability"),
            "Identifier should be 'main-capability'"
        );
        
        // Check windows
        let windows = json["windows"].as_array().expect("windows should be an array");
        assert!(
            windows.contains(&serde_json::Value::String("main".to_string())),
            "Should have 'main' window in windows list"
        );
    }
}
