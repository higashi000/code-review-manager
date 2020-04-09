use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ReviewData {
    pub project_name: String,
    file_path: String,
    point_out: String,
    func_name: String,
    judgment: bool,
}
