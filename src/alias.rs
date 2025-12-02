use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Aliases {
    pub aliases: HashMap<String, String>,
}
