use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use sha2::{Digest, Sha256};
use yaml_rust::{Yaml, YamlEmitter, YamlLoader};

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
        let yaml_str = fs::read_to_string(&file).expect("Failed to read YAML file");
        let docs = YamlLoader::load_from_str(&yaml_str).expect("Failed to parse YAML file");
        let doc = &docs[0];
        let mut assets: HashMap<String, String> = HashMap::new();

        if let Some(cache) = doc["build_cache"].as_vec() {
            for item in cache {
                if let Some(source) = item["source"].as_str() {
                    if let Some(checksum) = item["checksum"].as_str() {
                        assets.insert(source.to_string(), checksum.to_string());
                    }
                }
            }
        }

        Self {
            assets,
        }
    }

    pub fn save_to_file(&self, root_dir: &PathBuf) -> std::io::Result<()> {
        // Create the YAML data structure
        let mut local_cache = Vec::new();

        for asset in &self.assets {
            let mut hash = yaml_rust::yaml::Hash::new();
            hash.insert(Yaml::String("source".into()), Yaml::String(asset.0.clone()));
            hash.insert(Yaml::String("checksum".into()), Yaml::String(asset.1.clone()));
            local_cache.push(Yaml::Hash(hash));
        }

        // Wrap it in the main structure
        let mut doc = yaml_rust::yaml::Hash::new();
        doc.insert(Yaml::String("build_cache".into()), Yaml::Array(local_cache));

        // Prepare for output
        let mut yaml_str = String::new();
        {
            let mut emitter = YamlEmitter::new(&mut yaml_str);
            emitter.dump(&Yaml::Hash(doc)).unwrap();
        }

        // Write to a file (or you can print it directly)
        let mut file = File::create(&root_dir.join(".build_cache"))?;
        file.write_all(yaml_str.as_bytes())?;

        Ok(())
    }

    pub fn get_checksum(&self, key: &str) -> Option<String> {
        self.assets.get(key).cloned()
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