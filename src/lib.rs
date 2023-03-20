use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RustGpuBuilderModules {
    Single(Vec<u8>),
    Multi(BTreeMap<String, Vec<u8>>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustGpuBuilderOutput {
    pub entry_points: Vec<String>,
    pub modules: RustGpuBuilderModules,
}
