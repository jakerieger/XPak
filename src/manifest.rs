use std::fs;
use std::path::Path;
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use super::Asset;

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
        let mut reader = Reader::from_str(&_manifest_xml);
        reader.config_mut().trim_text(true);

        let mut count = 0;
        let mut buf: Vec<u8> = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Err(e) => panic!("Error at position {}: {:?}", reader.error_position(), e),
                Ok(Event::Eof) => break,
                Ok(Event::Start(e)) => {
                    match e.name().as_ref() {
                        b"OutputDir" => println!("OutputDir"),
                        b"Compress" => println!("Compress"),
                        _ => (),
                    }
                }
                _ => (),
            }
            buf.clear();
        }

        Self {
            name: name_str,
            root_dir: root_str,
            output_dir: String::new(),
            compress: false,
            assets: Vec::new(),
            manifest_path: manifest_path_str,
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