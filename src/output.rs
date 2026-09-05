use crate::entity::{history_analysis::HistoryAnalysis, history_entry::HistoryEntry};

pub fn print_analysis(analysis: &HistoryAnalysis, recent_entries: &[HistoryEntry]) {
    println!("=== History summary ===");
    println!("Total commands:  {}", analysis.total_commands);
    println!("Unique commands: {}", analysis.unique_commands);
    println!("Git commands:    {}", analysis.git_commands);
    println!("Git commits:     {}", analysis.git_commits);
    println!("Cargo commands:  {}", analysis.cargo_commands);
    println!("Node commands:   {}", analysis.node_commands);

    println!("\n=== Top 5 commands ===");
    for (position, (command, count)) in analysis.top_commands.iter().enumerate() {
        println!("{}. {} ({} times)", position + 1, command, count);
    }

    println!("\n=== Last 10 commands ===");
    for entry in recent_entries.iter().rev().take(10) {
        let timestamp = entry
            .timestamp
            .map(|value| value.to_string())
            .unwrap_or_else(|| "unknown".to_owned());
        let duration = entry
            .duration
            .map(|seconds| format!("{seconds}s"))
            .unwrap_or_else(|| "unknown".to_owned());

        println!(
            "{} | used at: {} | duration: {}",
            entry.command, timestamp, duration
        );
    }
}

pub fn print_search_results(query: &str, results: &[&HistoryEntry], limit: usize) {
    println!("=== Search results for \"{query}\" ===");
    println!("Found {} matching commands\n", results.len());

    for entry in results.iter().rev().take(limit) {
        println!("{}", entry.command);
    }

    if results.len() > limit {
        println!("\nShowing the {limit} most recent matches.");
    }
}
