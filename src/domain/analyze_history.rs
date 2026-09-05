use std::collections::HashMap;

use crate::entity::{history_analysis::HistoryAnalysis, history_entry::HistoryEntry};

pub fn analyze_history(entries: &[HistoryEntry]) -> HistoryAnalysis {
    let mut command_counts: HashMap<&str, usize> = HashMap::new();
    let mut git_commands = 0;
    let mut git_commits = 0;
    let mut cargo_commands = 0;
    let mut node_commands = 0;

    for entry in entries {
        *command_counts.entry(entry.command.as_str()).or_insert(0) += 1;

        let mut words = entry.command.split_whitespace();
        match words.next() {
            Some("git") => {
                git_commands += 1;
                if words.next() == Some("commit") {
                    git_commits += 1;
                }
            }
            Some("cargo") => cargo_commands += 1,
            Some("npm" | "pnpm" | "yarn") => node_commands += 1,
            _ => {}
        }
    }

    let unique_commands = command_counts.len();
    let mut top_commands: Vec<(String, usize)> = command_counts
        .into_iter()
        .map(|(command, count)| (command.to_owned(), count))
        .collect();

    top_commands.sort_by(|(command_a, count_a), (command_b, count_b)| {
        count_b.cmp(count_a).then_with(|| command_a.cmp(command_b))
    });
    top_commands.truncate(5);

    HistoryAnalysis {
        total_commands: entries.len(),
        unique_commands,
        git_commands,
        git_commits,
        cargo_commands,
        node_commands,
        top_commands,
    }
}

#[cfg(test)]
mod tests {
    use super::analyze_history;
    use crate::entity::history_entry::HistoryEntry;

    fn entry(command: &str) -> HistoryEntry {
        HistoryEntry {
            command: command.to_owned(),
            timestamp: None,
            duration: None,
        }
    }

    #[test]
    fn calculates_history_statistics() {
        let entries = vec![
            entry("git status"),
            entry("git status"),
            entry("git commit -m test"),
            entry("cargo test"),
            entry("pnpm test"),
        ];

        let analysis = analyze_history(&entries);

        assert_eq!(analysis.total_commands, 5);
        assert_eq!(analysis.unique_commands, 4);
        assert_eq!(analysis.git_commands, 3);
        assert_eq!(analysis.git_commits, 1);
        assert_eq!(analysis.cargo_commands, 1);
        assert_eq!(analysis.node_commands, 1);
        assert_eq!(analysis.top_commands[0], ("git status".to_owned(), 2));
    }
}
