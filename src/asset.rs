pub enum AssetType {
    Texture,
    Audio,
    Data,
}

impl AssetType {
    pub fn from(name: &str) -> AssetType {
        match name {
            "Texture" => AssetType::Texture,
            "Audio" => AssetType::Audio,
            _ => AssetType::Data,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            AssetType::Texture => String::from("Texture"),
            AssetType::Audio => String::from("Audio"),
            AssetType::Data => String::from("Data"),
        }
    }
}

pub struct Asset {
    pub name: String,
    pub asset_type: AssetType,
    pub source: String,
}

impl Asset {
    pub fn new(name: &str, asset_type: &str, source: &str) -> Asset {
        let _type = AssetType::from(asset_type);
        Asset {
            name: String::from(name),
            asset_type: _type,
            source: String::from(source),
        }
    }
}
