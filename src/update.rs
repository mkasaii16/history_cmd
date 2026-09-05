use std::{
    sync::mpsc::{self, Receiver},
    thread,
    time::Duration,
};

use update_informer::{Check, registry};

const GITHUB_REPOSITORY: &str = "mkasaii16/history_cmd";
const UPDATE_COMMAND: &str = "brew upgrade himd";

pub type UpdateCheck = Receiver<Option<String>>;

pub fn check_in_background() -> UpdateCheck {
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        let latest_version = update_informer::new(
            registry::GitHub,
            GITHUB_REPOSITORY,
            env!("CARGO_PKG_VERSION"),
        )
        .interval(Duration::ZERO)
        .timeout(Duration::from_secs(2))
        .check_version()
        .ok()
        .flatten()
        .map(|version| version.to_string());

        let _ = sender.send(latest_version);
    });

    receiver
}

pub fn print_notice(check: UpdateCheck) {
    // The update check must never make the CLI feel slow. The HTTP request keeps
    // running in its background thread if this small display budget expires.
    if let Ok(Some(latest_version)) = check.recv_timeout(Duration::from_millis(750)) {
        eprintln!("\n{}", update_notice(&latest_version));
    }
}

fn update_notice(latest_version: &str) -> String {
    format!(
        "🚀 A new himd version is available: {} → {latest_version}\n\
         ✨ Update it with: {UPDATE_COMMAND}",
        env!("CARGO_PKG_VERSION")
    )
}

#[cfg(test)]
mod tests {
    use super::update_notice;

    #[test]
    fn notice_contains_versions_and_upgrade_command() {
        let notice = update_notice("9.9.9");

        assert!(notice.contains(env!("CARGO_PKG_VERSION")));
        assert!(notice.contains("9.9.9"));
        assert!(notice.contains("brew upgrade himd"));
        assert!(notice.contains('🚀'));
    }
}
