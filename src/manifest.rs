use std::fs;
use std::path::Path;
use super::Asset;
use roxmltree::*;

pub struct Manifest {
    pub name: String,
    pub root_dir: String,
    pub output_dir: String,
    pub compress: bool,
    pub assets: Vec<Asset>,
    manifest_path: String,
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
                        let asset_source = asset_node.children().find(|n| n.has_tag_name("Build")).unwrap().text().unwrap_or("");
                        let asset = Asset::new(name, asset_type, asset_source);
                        asset_list.push(asset);
                    }
                }

                Self {
                    name: name_str,
                    root_dir: root_str,
                    output_dir: output_dir.to_owned(),
                    compress: compress.to_owned(),
                    assets: asset_list,
                    manifest_path: manifest_path_str,
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

    pub fn save(&self) {}

    pub fn build(&self) {}

    pub fn rebuild(&self) {}

    pub fn clean(&self) {}
}