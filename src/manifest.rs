use std::fs;
use std::path::{Path, PathBuf};
use roxmltree::*;
use crate::build_cache::BuildCache;
use super::asset::*;
use super::processors;

pub struct Manifest {
    pub name: String,
    pub root_dir: String,
    pub output_dir: String,
    pub compress: bool,
    pub assets: Vec<Asset>,
    manifest_path: String,
    cache: BuildCache,
}

impl Manifest {
    pub fn new(manifest_file: &str) -> Self {
        // Get necessary path values
        let _abs_path = fs::canonicalize(manifest_file).expect("Manifest path not found");
        let _manifest_path = _abs_path.as_path();
        let _root_dir = Path::parent(&_manifest_path).expect("Parent directory");
        let _name = Path::file_name(&_manifest_path).expect("Name");

        let name_str = _name.to_str().expect("Name").to_string();
        let root_str = _root_dir.to_str().expect("Root").to_string();
        let manifest_path_str = _manifest_path.to_str().expect("Manifest path").to_string();

        // Parse manifest file
        let _manifest_xml = fs::read_to_string(manifest_file).expect("Failed to read manifest file contents.");
        let doc_result = Document::parse(&_manifest_xml);
        match doc_result {
            Ok(doc) => {
                let root = doc.root_element();
                assert_eq!(root.tag_name().name(), "PakManifest");
                let output_dir_node = root.descendants().find(|n| n.tag_name().name() == "OutputDir").unwrap();
                assert!(output_dir_node.is_element());
                let output_dir = output_dir_node.text().unwrap().to_string();
                let compress_node = root.descendants().find(|n| n.tag_name().name() == "Compress").unwrap();
                assert!(compress_node.is_element());
                let compress_str = compress_node.text().unwrap().to_string();
                let compress = compress_str == "true";

                // Process assets
                let asset_count = doc.descendants().filter(|n| n.has_tag_name("Asset")).count();
                let mut asset_list: Vec<Asset> = Vec::with_capacity(asset_count);
                for asset_node in doc.descendants().filter(|n| n.has_tag_name("Asset")) {
                    if let Some(name) = asset_node.attribute("name") {
                        let asset_type = asset_node.children().find(|n| n.has_tag_name("Type")).unwrap().text().unwrap_or("");
                        let asset_source = asset_node.children().find(|n| n.has_tag_name("Source")).unwrap().text().unwrap_or("");
                        let asset = Asset::new(name, asset_type, asset_source);
                        asset_list.push(asset);
                    }
                }

                let mut cache: BuildCache = BuildCache::new();

                // Check if a local cache file exists
                match fs::exists("build_cache") {
                    Ok(exists) => {
                        if exists {
                            let cache_file = Path::new(&root_str).join("build_cache");
                            cache = BuildCache::load_from_file(&cache_file);
                            cache.save_to_file().expect("Failed to save build cache to file");
                        }
                    }
                    Err(e) => {
                        panic!("Error locating build cache: {}", e);
                    }
                }

                Self {
                    name: name_str,
                    root_dir: root_str,
                    output_dir: output_dir.to_owned(),
                    compress: compress.to_owned(),
                    assets: asset_list,
                    manifest_path: manifest_path_str,
                    cache,
                }
            }
            Err(e) => {
                panic!("Unable to parse manifest file.\nError: {}", e);
            }
        }
    }

    pub fn to_string(&self) -> String {
        let mut value = String::new();
        value.push_str("Manifest: ");
        value.push_str(&self.name);
        value.push_str("\n");
        value.push_str("Output Dir: ");
        value.push_str(&self.output_dir);
        value.push_str("\n");
        value.push_str("Compress: ");
        if self.compress {
            value.push_str("true\n");
        } else {
            value.push_str("false\n");
        }
        value.push_str("Assets: ");
        value.push_str(&self.assets.len().to_string());
        value.push_str("\n");

        value
    }

    pub fn save_to_file(&self) {}

    fn clean_content_directory(dir: &PathBuf) -> std::io::Result<()> {
        if dir.exists() {
            fs::remove_dir_all(dir)?;
        }
        Ok(())
    }

    fn create_content_directory(&self) -> PathBuf {
        let content_dir = Path::new(&self.root_dir).join(&self.output_dir);
        match Manifest::clean_content_directory(&content_dir) {
            Ok(()) => {
                // Create the content directory
                fs::create_dir_all(&content_dir).expect("Failed to create output directory");
            }
            Err(e) => {
                panic!("Unable to clean output directory.\nError: {}", e);
            }
        }

        content_dir
    }

    fn build_asset(content_dir: &PathBuf, asset: &Asset, source_file: &PathBuf) {
        let mut output_file = content_dir.join(&asset.name);
        if !output_file.set_extension("xpak") {
            println!("Failed to set extension");
            // TODO: Try another method of appending the xpak extension
            return;
        }

        if output_file.exists() {
            fs::remove_file(&output_file).expect("Failed to remove output file");
        }

        fs::create_dir_all(&output_file.parent().unwrap()).expect("Failed to create output file subdirectories");

        let mut asset_data: Vec<u8> = Vec::new();
        match &asset.asset_type {
            AssetType::Texture => {
                let data = processors::process_texture(&source_file).expect("Failed to process texture");
                asset_data.clear();
                asset_data.extend_from_slice(&data);
            }
            AssetType::Audio => {
                let data = processors::process_audio(&source_file).expect("Failed to process audio");
                asset_data.clear();
                asset_data.extend_from_slice(&data);
            }
            AssetType::Data => {
                let data = processors::process_data(&source_file).expect("Failed to process data");
                asset_data.clear();
                asset_data.extend_from_slice(&data);
            }
        }

        // Write output file to disk
        fs::write(&output_file, &asset_data).expect("Failed to write asset");
    }

    pub fn build(&mut self) {
        let content_directory = self.create_content_directory();
        let asset_count = self.assets.len();
        let mut asset_id = 1;

        // Collect all assets that need rebuilt
        let assets_to_build: Vec<(&Asset, PathBuf)> = self.assets.iter()
            .filter_map(|asset| {
                let source_file = Path::new(&self.root_dir).join(&asset.source);

                // Check if asset needs to be rebuilt
                let rebuild = match self.cache.get_checksum(&asset.source) {
                    Some(checksum) => {
                        let current_hash = BuildCache::calculate_checksum(&source_file);
                        if current_hash != checksum {
                            self.cache.update_or_insert(&asset.source, &current_hash);
                            true
                        } else {
                            false
                        }
                    }
                    None => {
                        let current_hash = BuildCache::calculate_checksum(&source_file);
                        self.cache.update_or_insert(&asset.source, &current_hash);
                        true
                    },
                };

                if rebuild {
                    Some((asset, source_file)) // Clone the asset if it needs to be built
                } else {
                    println!("Skipping asset: {}", asset.name);
                    None
                }
            })
            .collect();

        for (asset, source_file) in assets_to_build {
            println!("  | [{}/{}] Building asset: {}", asset_id, asset_count, asset.name);
            Manifest::build_asset(&content_directory, &asset, &source_file);
            asset_id += 1;
        }

        // Update cache file
        self.cache.save_to_file().expect("Failed to save cache to disk");
    }

    pub fn rebuild(&mut self) {
        self.clean();
        self.build();
    }

    pub fn clean(&self) {
        let _ = self.create_content_directory();
    }
}