use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct FragmentLineage {
    pub transformation_step: String,
    pub predecessors: Vec<String>,
}
