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

#[cfg(feature = "bevy")]
mod bevy {
    use super::RustGpuBuilderOutput;
    use bevy::{reflect::TypeUuid, utils::Uuid};

    impl TypeUuid for RustGpuBuilderOutput {
        const TYPE_UUID: Uuid =
            Uuid::from_fields(1664188495, 11437, 8530, &[15, 75, 77, 14, 32, 11, 25, 52]);
    }
}
