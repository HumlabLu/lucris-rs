use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::ResearchClean;
use crate::PersonClean;
use crate::errors::CombinedError;

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
    // Since this is one uuid, we could return a (ResearchClean, Vec<PersonClean>).
    pub fn get_research_from_uuid(&self, uuid: &str) -> Result<(ResearchClean, Vec<PersonClean>), CombinedError> {
            let mut persons = Vec::new();

            // Directly attempt to get the research entry
            let research = self.research.get(uuid).ok_or(CombinedError::NoSuchUUID)?;
            println!("-> {:?}", research);

            // Iterate over the persons associated with the research
            for p in &research.persons {
                let person_uuid = &p.uuid;
                if let Some(person) = self.persons.get(person_uuid) {
                    println!("--> person in research: {:?}", person);
                    persons.push(person.clone());
                }
            }

            // Clone the research before returning
            Ok((research.clone(), persons))
        }

    pub fn get_research_from_uuid_ref(&self, uuid: &str) -> Result<(&ResearchClean, Vec<&PersonClean>), CombinedError> {
        let mut persons = Vec::new();

        let research = self.research.get(uuid).ok_or(CombinedError::NoSuchUUID)?;
        println!("-> {:?}", research);

        for p in &research.persons {
            let person_uuid = &p.uuid;
            if let Some(person) = self.persons.get(person_uuid) {
                println!("--> person in research: {:?}", person);
                persons.push(person);
            }
        }

        Ok((research, persons))
    }

    // Return all the uuids in the research HashMap. If empty we
    // return an empty vector.
    pub fn get_all_research_uuids(&self) -> Vec<&String> {
        let mut uuids = Vec::new();
        uuids = self.research.keys()
            .collect();
        uuids
    }

}
