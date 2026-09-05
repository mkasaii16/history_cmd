use std::{
    io::{self, Write},
    process::{Command, Stdio},
};

use crate::entity::history_entry::HistoryEntry;

pub fn format_commands_for_clipboard(results: &[&HistoryEntry], limit: usize) -> String {
    results
        .iter()
        .rev()
        .take(limit)
        .map(|entry| entry.command.as_str())
        .collect::<Vec<&str>>()
        .join("\n")
}

pub fn copy_to_clipboard(content: &str) -> io::Result<()> {
    if content.is_empty() {
        return Ok(());
    }

    #[cfg(target_os = "macos")]
    return pipe_to_command("pbcopy", &[], content);

    #[cfg(target_os = "windows")]
    return pipe_to_command("clip", &[], content);

    #[cfg(target_os = "linux")]
    {
        if pipe_to_command("wl-copy", &[], content).is_ok() {
            return Ok(());
        }
        if pipe_to_command("xclip", &["-selection", "clipboard"], content).is_ok() {
            return Ok(());
        }
        return pipe_to_command("xsel", &["--clipboard", "--input"], content);
    }

    #[allow(unreachable_code)]
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "clipboard is not supported on this operating system",
    ))
}

fn pipe_to_command(program: &str, args: &[&str], content: &str) -> io::Result<()> {
    let mut child = Command::new(program)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;

    child
        .stdin
        .as_mut()
        .ok_or_else(|| io::Error::new(io::ErrorKind::BrokenPipe, "clipboard input unavailable"))?
        .write_all(content.as_bytes())?;

    let status = child.wait()?;
    if status.success() {
        Ok(())
    } else {
        Err(io::Error::other(format!(
            "clipboard command `{program}` failed"
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::format_commands_for_clipboard;
    use crate::entity::history_entry::HistoryEntry;

    fn entry(command: &str) -> HistoryEntry {
        HistoryEntry {
            command: command.to_owned(),
            timestamp: None,
            duration: None,
        }
    }

    #[test]
    fn formats_only_the_newest_limited_results() {
        let entries = [entry("git status"), entry("git add ."), entry("git push")];
        let results = entries.iter().collect::<Vec<&HistoryEntry>>();

        let content = format_commands_for_clipboard(&results, 2);

        assert_eq!(content, "git push\ngit add .");
    }
}
