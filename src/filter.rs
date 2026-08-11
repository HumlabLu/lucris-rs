use crate::json_research::ResearchClean;
use regex::{escape, RegexSet};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy)]
pub enum FilterMode {
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

impl ResearchClean {
    pub fn has_keyword(&self, keywords: &HashSet<String>) -> bool {
        self.keywords
            .iter()
            .any(|keyword| keywords.contains(&keyword.trim().to_lowercase()))
    }
}

pub fn filter_research_by_person(
    research: &mut HashMap<String, ResearchClean>,
    names: Vec<String>,
    mode: FilterMode,
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
            FilterMode::KeepMatching => matches,
            FilterMode::DeleteMatching => !matches,
        }
    });
}

pub fn filter_research_by_keyword(
    research: &mut HashMap<String, ResearchClean>,
    keywords: Vec<String>,
    keep_matching: bool,
) {
    let keywords: HashSet<String> = keywords
        .into_iter()
        .map(|keyword| keyword.trim().to_lowercase())
        .filter(|keyword| !keyword.is_empty())
        .collect();

    research.retain(|_, item| item.has_keyword(&keywords) == keep_matching);
}

impl ResearchClean {
    pub fn abstract_matches_any(&self, patterns: &RegexSet) -> bool {
        patterns.is_match(&self.abstract_text)
    }
}

// Use a similar list to names. We use regexen so we can use \b boundary.
pub fn filter_research_by_abstract(
    research: &mut HashMap<String, ResearchClean>,
    terms: Vec<String>,
    mode: FilterMode,
) -> Result<(), regex::Error> {
    let patterns: Vec<String> = terms
        .into_iter()
        .map(|term| term.trim().to_owned())
        .filter(|term| !term.is_empty())
        .map(|term| {
            // Escape regex characters.
            let escaped_term = escape(&term);

            // (?i) for case-insensitive.
            // \b provides the word boundaries.
            format!(r"(?i)\b{}\b", escaped_term)
        })
        .collect();

    // Compile once.
    let patterns = RegexSet::new(patterns)?;

    research.retain(|_, item| {
        let matches = item.abstract_matches_any(&patterns);

        match mode {
            FilterMode::KeepMatching => matches,
            FilterMode::DeleteMatching => !matches,
        }
    });

    Ok(())
}
