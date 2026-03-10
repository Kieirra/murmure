use std::path::Path;
use tauri::command;

fn validate_murmure_extension(file_path: &str) -> Result<(), String> {
    let path = Path::new(file_path);
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("murmure") => Ok(()),
        _ => Err("File must have a .murmure extension".to_string()),
    }
}

#[command]
pub fn read_config_file(file_path: String) -> Result<String, String> {
    validate_murmure_extension(&file_path)?;
    std::fs::read_to_string(&file_path).map_err(|e| e.to_string())
}

#[command]
pub fn write_config_file(file_path: String, content: String) -> Result<(), String> {
    validate_murmure_extension(&file_path)?;
    std::fs::write(&file_path, content).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_validate_murmure_extension_valid() {
        assert!(validate_murmure_extension("/tmp/config.murmure").is_ok());
    }

    #[test]
    fn test_validate_murmure_extension_invalid() {
        assert!(validate_murmure_extension("/tmp/config.json").is_err());
        assert!(validate_murmure_extension("/tmp/config").is_err());
        assert!(validate_murmure_extension("/tmp/config.txt").is_err());
    }

    #[test]
    fn test_write_and_read_config_file() {
        let path = "/tmp/test-murmure-rw.murmure";
        let content = r#"{"version":1}"#.to_string();

        let write_result = write_config_file(path.to_string(), content.clone());
        assert!(write_result.is_ok());

        let read_result = read_config_file(path.to_string());
        assert!(read_result.is_ok());
        assert_eq!(read_result.unwrap(), content);

        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_read_config_file_rejects_non_murmure() {
        let result = read_config_file("/tmp/config.json".to_string());
        assert!(result.is_err());
        assert!(result.unwrap_err().contains(".murmure"));
    }

    #[test]
    fn test_write_config_file_rejects_non_murmure() {
        let result = write_config_file("/tmp/config.json".to_string(), "{}".to_string());
        assert!(result.is_err());
        assert!(result.unwrap_err().contains(".murmure"));
    }

    #[test]
    fn test_read_config_file_nonexistent() {
        let result = read_config_file("/tmp/nonexistent-file.murmure".to_string());
        assert!(result.is_err());
    }
}
