use serde::{Deserialize, Serialize}

#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    pub name: String,
    pub runner: String,
    pub dependencies: Vec<String>,
    pub launch: Launch,
}
#[derive(Debug,Serialize, Deserialize)]
pub struct Launch {
    pub executable: String,
}
