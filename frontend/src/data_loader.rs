use serde_json::Value;
use std::path::Path;
use anyhow::Result;
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use std::collections::HashMap;

pub static DATA_CACHE: Lazy<RwLock<HashMap<String, Value>>> = Lazy::new(|| RwLock::new(HashMap::new()));

/// Load JSON data from a file
pub fn load_json(path: &str) -> Result<Value> {
    let cache = DATA_CACHE.read();
    if let Some(cached) = cache.get(path) {
        return Ok(cached.clone());
    }
    drop(cache);

    let content = std::fs::read_to_string(path)?;
    let value: Value = serde_json::from_str(&content)?;
    
    let mut cache = DATA_CACHE.write();
    cache.insert(path.to_string(), value.clone());
    Ok(value)
}

/// Load TOML data from a file
pub fn load_toml(path: &str) -> Result<Value> {
    let cache = DATA_CACHE.read();
    if let Some(cached) = cache.get(path) {
        return Ok(cached.clone());
    }
    drop(cache);

    let content = std::fs::read_to_string(path)?;
    let table: toml::Table = toml::from_str(&content)?;
    let value = serde_json::to_value(table)?;
    
    let mut cache = DATA_CACHE.write();
    cache.insert(path.to_string(), value.clone());
    Ok(value)
}

/// Load YAML data from a file
pub fn load_yaml(path: &str) -> Result<Value> {
    let cache = DATA_CACHE.read();
    if let Some(cached) = cache.get(path) {
        return Ok(cached.clone());
    }
    drop(cache);

    let content = std::fs::read_to_string(path)?;
    let yaml: serde_yaml::Value = serde_yaml::from_str(&content)?;
    let value = serde_json::to_value(yaml)?;
    
    let mut cache = DATA_CACHE.write();
    cache.insert(path.to_string(), value.clone());
    Ok(value)
}

/// Auto-detect and load data based on file extension
pub fn load_data(path: &str) -> Result<Value> {
    let path_obj = Path::new(path);
    match path_obj.extension().and_then(|ext| ext.to_str()) {
        Some("json") => load_json(path),
        Some("toml") => load_toml(path),
        Some("yaml") | Some("yml") => load_yaml(path),
        _ => Err(anyhow::anyhow!("Unsupported file format: {}", path)),
    }
}

/// Get data from cache without loading from disk
pub fn get_cached(key: &str) -> Option<Value> {
    DATA_CACHE.read().get(key).cloned()
}

/// Clear the entire cache
pub fn clear_cache() {
    DATA_CACHE.write().clear();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_operations() {
        clear_cache();
        
        let key = "test_key";
        let value = json!({"test": "data"});
        
        DATA_CACHE.write().insert(key.to_string(), value.clone());
        assert_eq!(get_cached(key), Some(value));
        
        clear_cache();
        assert_eq!(get_cached(key), None);
    }
}
