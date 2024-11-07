use serde::{Deserialize, Serialize};
use uuid::Uuid;

// We need a mapping somewhere of uuid to original_uuids.
// All uuids need to be translated to a new uuid, only opted-info
// data should get an uuid.

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct UuidMap {
    pub uuid: Uuid,
    pub original_uuid: String,
}

impl UuidMap {
    pub fn add_uuid() {
    }
}


