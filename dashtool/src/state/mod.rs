use std::collections::HashMap;

use gix::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct State {
    pub commits: HashMap<String, ObjectId>,
}
