mod cli;
mod domain;
mod entity;
mod output;

use clap::Parser;
use cli::{Cli, Command};
use domain::{
    analyze_history::analyze_history, get_history::history_path, parse_zsh_line::parse_zsh_line,
    search_history::search_history,
};
use entity::history_entry::HistoryEntry;
use output::{print_analysis, print_search_results};

use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let command = Cli::parse().command;

    if matches!(&command, Some(Command::Version)) {
        println!("himd {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    let path = history_path()?;
    // from_utf8_lossy prevents the program from crashing
    // when the history contains invalid UTF-8.
    let bytes = fs::read(&path)?;
    let content = String::from_utf8_lossy(&bytes);

    let entries: Vec<HistoryEntry> = content.lines().filter_map(parse_zsh_line).collect();

    match command {
        Some(Command::Search { query, limit }) => {
            let query = query.join(" ");
            let results = search_history(&entries, &query);
            print_search_results(&query, &results, limit);
        }
        Some(Command::Stats) | None => {
            let analysis = analyze_history(&entries);
            print_analysis(&analysis, &entries);
        }
        Some(Command::Version) => unreachable!(),
    }

    Ok(())
}
