use serde::Deserialize;

//#[derive(Debug, Serialize, Deserialize)]

#[derive(Deserialize)]
pub struct ProjectData {
    pub project_name: String,
}
