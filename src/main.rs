mod cli;
mod clipboard;
mod domain;
mod entity;
mod output;
mod update;

use clap::Parser;
use cli::{Cli, Command};
use clipboard::{copy_to_clipboard, format_commands_for_clipboard};
use domain::{
    analyze_history::analyze_history, get_history::history_path, parse_zsh_line::parse_zsh_line,
    search_history::search_history,
};
use entity::history_entry::HistoryEntry;
use output::{print_analysis, print_search_results};

use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let update_check = update::check_in_background();
    let command = Cli::parse().command;

    if matches!(&command, Some(Command::Version)) {
        println!("himd {}", env!("CARGO_PKG_VERSION"));
        update::print_notice(update_check);
        return Ok(());
    }

    let path = history_path()?;
    // from_utf8_lossy prevents the program from crashing
    // when the history contains invalid UTF-8.
    let bytes = fs::read(&path)?;
    let content = String::from_utf8_lossy(&bytes);

    let entries: Vec<HistoryEntry> = content.lines().filter_map(parse_zsh_line).collect();

    match command {
        Some(Command::Search { query, limit, copy }) => {
            let query = query.join(" ");
            let results = search_history(&entries, &query);
            print_search_results(&query, &results, limit);

            if copy {
                let clipboard_content = format_commands_for_clipboard(&results, limit);
                if clipboard_content.is_empty() {
                    println!("\n📋 Nothing to copy.");
                } else {
                    copy_to_clipboard(&clipboard_content)?;
                    println!(
                        "\n📋 Copied {} command(s) to the clipboard.",
                        results.len().min(limit)
                    );
                }
            }
        }
        Some(Command::Stats) | None => {
            let analysis = analyze_history(&entries);
            print_analysis(&analysis, &entries);
        }
        Some(Command::Version) => unreachable!(),
    }

    update::print_notice(update_check);

    Ok(())
}
