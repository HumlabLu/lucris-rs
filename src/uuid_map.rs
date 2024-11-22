use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

// Maybe use: https://docs.rs/short-uuid/latest/short_uuid/

// We need a mapping somewhere of uuid to original_uuids.
// All uuids need to be translated to a new uuid, only opted-info
// data should get an uuid.

// Can we keep extra meta data? Like: this uuid is a person, connects to
// this and that research? We need to look up persons via uuid in the
// research data, how to we do this? A new struct with uuid -> ResearchJson?
// This can also be done in a "real" database (select from persons where ...).

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct UuidMap {
    uuids: HashMap<String, Uuid>,
    // reverse mapping?
}

impl UuidMap {
    pub fn new() -> Self {
        Self {
            uuids: HashMap::new(),
        }
    }

    pub fn add_uuid(&mut self, uuid: &str) -> String {
        if self.uuids.contains_key(uuid) {
            eprintln!("Repeating research uuid: {}", uuid);
        }
        let safe_uuid = Uuid::new_v4();
        self.uuids.insert(uuid.to_string(), safe_uuid);
        safe_uuid.to_string()
    }

    // Looks it up and returns the safe-uuid. Adds it if not present.
    pub fn get_uuid_as_str(&mut self, uuid: &str) -> String {
        if let Some(value) = self.uuids.get(uuid) {
            return value.to_string();
        }
        self.add_uuid(uuid)
    }

    pub fn count(&self) -> usize {
        self.uuids.len()
    }
}
