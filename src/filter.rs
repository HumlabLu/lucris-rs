use crate::json_research::ResearchClean;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy)]
pub enum PersonFilterMode {
    KeepMatching,
    DeleteMatching,
}

impl ResearchClean {
    pub fn has_person(&self, names: &HashSet<String>) -> bool {
        self.persons
            .iter()
            .any(|person| names.contains(&person.get_name().trim().to_lowercase()))
    }
}

pub fn filter_research_by_person(
    research: &mut HashMap<String, ResearchClean>,
    names: Vec<String>,
    mode: PersonFilterMode,
) {
    // Convert so we can use contain().
    // let names: HashSet<String> = names.into_iter().collect();
    let names: HashSet<String> = names
        .into_iter()
        .map(|name| name.trim().to_lowercase())
        .collect();

    research.retain(|_, item| {
        let matches = item.has_person(&names);

        match mode {
            PersonFilterMode::KeepMatching => matches,
            PersonFilterMode::DeleteMatching => !matches,
        }
    });
}
