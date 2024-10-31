use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use sha2::{Digest, Sha256};

pub struct BuildCache {
    assets: HashMap<String, String>,
}

impl BuildCache {
    pub fn new() -> Self {
        Self {
            assets: HashMap::new(),
        }
    }

    pub fn load_from_file(file: &PathBuf) -> Self {
        Self {
            assets: HashMap::new(),
        }
    }

    pub fn save_to_file(&self) {}

    pub fn get_checksum(&self, key: &str) -> Option<String> {
        self.assets.get(&key).cloned()
    }

    pub fn calculate_checksum(source: &PathBuf) -> String {
        let mut hasher = Sha256::new();
        let mut file = File::open(source).expect("Failed to open source file for reading.");
        let mut buffer = [0; 4096];
        loop {
            let n = file.read(&mut buffer).expect("Failed to read source file.");
            if n == 0 {
                break;
            }
            hasher.update(&buffer[..n]);
        }
        let result = hasher.finalize();
        let hash = format!("{:x}", result);
        return hash;
    }

    pub fn update_or_insert(&mut self, key: &str, value: &str) {
        self.assets.insert(key.to_string(), value.to_string());
    }

    pub fn clear(&mut self) {
        self.assets.clear();
    }
}