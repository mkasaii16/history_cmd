use std::{env, error::Error, fs, io, path::PathBuf};

#[derive(Debug)]
struct HistoryEntry {
    command: String,
    timestamp: Option<u64>,
    duration: Option<u64>,
}

fn parse_zsh_line(line: &str) -> Option<HistoryEntry> {
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

fn main() -> Result<(), Box<dyn Error>> {
    let path = history_path()?;

    // from_utf8_lossy prevents the program from crashing
    // when the history contains invalid UTF-8.
    let bytes = fs::read(&path)?;
    let content = String::from_utf8_lossy(&bytes);

    let entries: Vec<HistoryEntry> = content.lines().filter_map(parse_zsh_line).collect();

    for entry in entries.iter().rev().take(20) {
        println!(
            "{} | used at: {:?} | duration: {:?}s",
            entry.command, entry.timestamp, entry.duration
        );
    }

    Ok(())
}
