use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::ResearchClean;
use crate::PersonClean;

// Container for the different data files.

#[derive(Debug)]
pub struct Combined {
    pub research: HashMap<String, ResearchClean>,
    pub persons: HashMap<String, PersonClean>,
}

impl Combined {
    pub fn new(research: HashMap<String, ResearchClean>, persons: HashMap<String, PersonClean>) -> Self {
        Self {
            research,
            persons,
        }
    }

    pub fn get_research_from_uuid(self, uuid: &str) {
        if self.research.contains_key(uuid) == true {
            let research = self.research.get(uuid).unwrap();
            for p in &research.persons {
                let uuid = &p.uuid;
                if let Some(value) = self.persons.get(uuid) {
                    println!("-> person in research: {}", value);
                }
            }
        }
    }
}
