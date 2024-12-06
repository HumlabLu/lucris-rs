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

/// Struct that holds the Hashmap converting original String uuids
/// to the "safe" Uuids.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct UuidMap {
    uuids: HashMap<String, Uuid>,
    forbidden: Vec<String>,
    // reverse mapping?
}

impl UuidMap {
    pub fn new() -> Self {
        Self {
            uuids: HashMap::new(),
            forbidden: vec![],
        }
    }

    /// Adds a new String uuid and returns a new safe_uuid
    /// as String.
    pub fn add_uuid(&mut self, uuid: &str) -> String {
        if self.uuids.contains_key(uuid) {
            eprintln!("Repeating research uuid: {}", uuid);
        }
        let safe_uuid = Uuid::new_v4();
        self.uuids.insert(uuid.to_string(), safe_uuid);
        safe_uuid.to_string()
        //uuid.to_string() //// JUST FOR TESTING; KEEP SAME UUID
    }

    pub fn add_forbidden_uuid(&mut self, uuid: &str) {
        if !self.forbidden.iter().any(|x| *x == uuid) {
            self.forbidden.push(uuid.to_string());
        }
    }

    pub fn forbidden_contains(&self, uuid: &str) -> bool {
        self.forbidden.iter().any(|x| *x == uuid)
    }

    /// Tries to look-up the uuid and return it. If the uuid is
    /// not present in the hashmap, it will be added.
    pub fn get_uuid_as_str(&mut self, uuid: &str) -> String {
        if let Some(value) = self.uuids.get(uuid) {
            return value.to_string();
            //return uuid.to_string(); //// JUST FOR TESTING; KEEP SAME UUID
        }
        self.add_uuid(uuid)
    }

    pub fn count(&self) -> usize {
        self.uuids.len()
    }
}
