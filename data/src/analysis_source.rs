use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ImageAttributes {
    #[serde(rename = "specimenID", default)]
    pub specimen_id: String,
    #[serde(default)]
    pub objmag: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SourceImage {
    pub area: i64,
    #[serde(rename = "imageAttributes")]
    pub image_attributes: ImageAttributes,
    #[serde(rename = "physUnits")]
    pub phys_units: String,
    pub id: String,
    pub title: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "pixelSize")]
    pub pixel_size: f32,
    #[serde(rename = "dimY")]
    pub dim_y: i32,
    #[serde(rename = "dimX")]
    pub dim_x: i32,
    #[serde(rename = "imageFileSize")]
    pub image_file_size: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FOV {
    #[serde(rename = "sourceImages")]
    pub source_images: Vec<SourceImage>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnalysisSource {
    pub idk: i32,
    #[serde(rename = "publicAccess", default)]
    pub public_access: bool,
    #[serde(default)]
    pub solutions: Vec<serde_json::Value>, // Use Value for empty arrays or unknown structure
    pub created: i64,
    #[serde(rename = "FOVs")]
    pub fovs: Vec<FOV>,
    pub modified: i64,
    pub videos: Vec<serde_json::Value>, // Use Value for empty arrays or unknown structure
    pub id: String,
    pub title: String,
    pub user: String,
    #[serde(default)]
    pub tags: Vec<String>,
}
