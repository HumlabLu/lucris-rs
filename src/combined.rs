use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::ResearchClean;
use crate::PersonClean;

// Container for the different data files.
//
// Maybe we should combine the research and persons (plus others) in two
// structs, ResearchClean and PersonClean. ResearchClean has persons, and
// in PersonClean we have research (or uuids?).

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

    // What should we return? String? JSONL?
    // ResearchClean has a Vec for persons, but it is only uuids
    // and names, the person_json contains much more info.
    pub fn get_research_from_uuid(self, uuid: &str) {
        if self.research.contains_key(uuid) == true {
            let research = self.research.get(uuid).unwrap();
            println!("-> {}", research);
            for p in &research.persons {
                let uuid = &p.uuid;
                if let Some(value) = self.persons.get(uuid) {
                    println!("--> person in research: {}", value);
                }
            }
        }
    }

}
