use crate::json_research::ResearchClean;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy)]
pub enum PersonFilterMode {
    KeepMatching,
    DeleteMatching,
}

pub fn filter_research_by_person(
    research: &mut HashMap<String, ResearchClean>,
    names: Vec<String>,
    mode: PersonFilterMode,
) {
    // Convert so we can use contain().
    let names: HashSet<String> = names.into_iter().collect();

    research.retain(|_, item| {
        let matches = item
            .persons
            .iter()
            .any(|person| names.contains(person.get_name()));

        match mode {
            PersonFilterMode::KeepMatching => matches,
            PersonFilterMode::DeleteMatching => !matches,
        }
    });
}
