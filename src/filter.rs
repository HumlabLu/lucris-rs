use crate::json_research::ResearchClean;
use clap::ValueEnum;
use regex::{escape, RegexSet};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, ValueEnum)]
pub enum FilterMode {
    #[value(name = "keep")]
    KeepMatching,
    #[value(name = "delete")]
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

pub fn _filter_research_by_keyword(
    research: &mut HashMap<String, ResearchClean>,
    keywords: Vec<String>,
    mode: FilterMode,
) {
    let keywords: HashSet<String> = keywords
        .into_iter()
        .map(|keyword| keyword.trim().to_lowercase())
        .filter(|keyword| !keyword.is_empty())
        .collect();

    research.retain(|_, item| {
        let matches = item.has_keyword(&keywords);
        match mode {
            FilterMode::KeepMatching => matches,
            FilterMode::DeleteMatching => !matches,
        }
    });
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

// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json_research::read_research_jsonl;
    use crate::uuid_map::UuidMap;
    use std::path::{Path, PathBuf};

    fn make_test_path(file_name: &str) -> PathBuf {
        let project_root = env!("CARGO_MANIFEST_DIR");
        let data_path = Path::new(project_root)
            .join("tests")
            .join("data")
            .join(file_name);
        data_path
    }

    #[test]
    fn test_filter_research_by_abstract() {
        let data_path = make_test_path("pjb_research.jsonl");

        // Read the original ResearchJson.
        let read_umap = UuidMap::new();
        let (research_json, _) =
            read_research_jsonl(data_path.to_str().expect("Invalid path."), &read_umap)
                .expect("Failed to read test data.");

        // Convert it to ResearchClean.
        let mut conversion_umap = UuidMap::new();
        let research_clean = ResearchClean::try_from_with_locale_umap(
            &research_json[0],
            "en_GB",
            &mut conversion_umap,
        )
        .expect("Failed to create ResearchClean");

        // Create hashmap uuid -> ResearchClean.
        let uuid = research_clean.get_uuid().to_owned();
        let rs_clean = HashMap::from([(uuid, research_clean)]);

        eprintln!("{:?}", rs_clean);

        let mut rs_keep = rs_clean.clone();
        filter_research_by_abstract(
            &mut rs_keep,
            vec!["language modeling".to_owned()],
            FilterMode::KeepMatching,
        )
        .expect("Failed to compile regexp.");

        eprintln!(">>\n{:?}\n<<", rs_keep);

        // Should still contain one element.
        assert_eq!(rs_keep.len(), 1);

        // Here we have a match, but we do not keep the item.
        let mut rs_delete = rs_clean.clone();
        filter_research_by_abstract(
            &mut rs_delete,
            vec!["approximations".to_owned()],
            FilterMode::DeleteMatching,
        )
        .expect("Failed to compile regular expressions");

        assert!(rs_delete.is_empty());

        // Boundary test, sub-match should not match, so
        // result should be empty.
        let mut rs_sub = rs_clean.clone();
        filter_research_by_abstract(
            &mut rs_sub,
            vec!["approx".to_owned()],
            FilterMode::KeepMatching,
        )
        .expect("Failed to compile regular expressions");

        assert!(rs_sub.is_empty());
    }
}
