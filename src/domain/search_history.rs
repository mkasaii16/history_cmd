use crate::entity::history_entry::HistoryEntry;

pub fn search_history<'a>(entries: &'a [HistoryEntry], query: &str) -> Vec<&'a HistoryEntry> {
    let query = query.to_lowercase();

    entries
        .iter()
        .filter(|entry| entry.command.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::search_history;
    use crate::entity::history_entry::HistoryEntry;

    fn entry(command: &str) -> HistoryEntry {
        HistoryEntry {
            command: command.to_owned(),
            timestamp: None,
            duration: None,
        }
    }

    #[test]
    fn finds_matching_commands_case_insensitively() {
        let entries = vec![
            entry("git status"),
            entry("git commit -m test"),
            entry("cargo test"),
        ];

        let results = search_history(&entries, "GIT");

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].command, "git status");
        assert_eq!(results[1].command, "git commit -m test");
    }

    #[test]
    fn returns_an_empty_list_when_nothing_matches() {
        let entries = vec![entry("cargo check"), entry("git status")];

        let results = search_history(&entries, "docker");

        assert!(results.is_empty());
    }
}
