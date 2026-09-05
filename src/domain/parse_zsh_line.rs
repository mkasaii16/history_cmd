use crate::entity::history_entry::HistoryEntry;

pub fn parse_zsh_line(line: &str) -> Option<HistoryEntry> {
    let line = line.trim();

    if line.is_empty() {
        return None;
    }

    // Extended Zsh history:
    // : 1724000000:5;git status
    if let Some(rest) = line.strip_prefix(": ") {
        let (metadata, command) = rest.split_once(';')?;
        let (timestamp, duration) = metadata.trim().split_once(':')?;

        return Some(HistoryEntry {
            command: command.to_owned(),
            timestamp: timestamp.parse().ok(),
            duration: duration.parse().ok(),
        });
    }

    // Zsh history without metadata
    Some(HistoryEntry {
        command: line.to_owned(),
        timestamp: None,
        duration: None,
    })
}
