use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::ResearchJson;

// We need a mapping somewhere of uuid to original_uuids.
// All uuids need to be translated to a new uuid, only opted-info
// data should get an uuid.

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Combined {
    pub uuid: Uuid,
    pub original_uuid: String,
}

// Test From<T> implementation. A builder which takes multiple structs
// is probably better.
impl From<&ResearchJson> for Combined {
    fn from(research_json: &ResearchJson) -> Self {
        // maybe a match on the uuid?
        Combined {
            original_uuid: research_json.uuid.clone().unwrap(), // Assume we do have a uuid...
            uuid: Uuid::new_v4(),
        } 
    }
}

