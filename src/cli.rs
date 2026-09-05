use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "himd",
    version,
    author = "Moein Kasaei",
    about = "Turn your shell history into useful insights,by Moein Kasaei",
    long_about = "Your shell remembers. himd makes sense of it.\n\n\
        himd reads your local Bash or Zsh history file, parses its commands, and lets \
        you inspect usage statistics or search previous commands. It never executes \
        commands from your history.\n\n\
        Run himd without a command to show the same output as `himd stats`.",
    after_help = "QUICK START:\n  \
        himd\n  \
        himd stats\n  \
        himd search git\n  \
        himd search \"git commit\" --limit 10\n  \
        himd version\n\n\
        HISTORY FILE:\n  \
        himd uses $HISTFILE when it is set. Otherwise it reads ~/.bash_history or \
        ~/.zsh_history based on your current shell."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Show statistics and the most recent commands
    #[command(
        long_about = "Analyze all parsed history entries and print:\n  \
            - total and unique command counts\n  \
            - Git, Git commit, Cargo, and Node command counts\n  \
            - the 5 most frequently repeated exact commands\n  \
            - the 10 most recent commands\n\n\
            Repeated commands are counted separately. Unique commands are compared \
            using their complete text.",
        after_help = "EXAMPLES:\n  himd stats\n  himd"
    )]
    Stats,

    /// Search command history (case-insensitive)
    #[command(
        long_about = "Search every parsed history entry using a case-insensitive substring match.\n\n\
            The value shown as `Found` is the total number of matching history \
            entries, including repeated commands. The --limit option only controls \
            how many of the newest matches are printed; it does not change the \
            `Found` count.",
        after_help = "EXAMPLES:\n  \
            himd search git\n  \
            himd search \"git commit\"\n  \
            himd search git commit --limit 5\n  \
            himd search docker -l 50\n  \
            himd search \"cargo run\" --limit 10 --copy\n\n\
            Both `git commit` and \"git commit\" become the same search query."
    )]
    Search {
        /// Text to find in each command
        #[arg(required = true, num_args = 1.., value_name = "QUERY")]
        query: Vec<String>,

        /// Maximum number of newest matches to print
        #[arg(short, long, default_value_t = 20)]
        limit: usize,

        /// Copy the displayed matches to the system clipboard
        #[arg(long)]
        copy: bool,
    },

    /// Show the installed himd version
    #[command(
        long_about = "Print the installed himd version from Cargo.toml.",
        after_help = "EXAMPLES:\n  himd version\n  himd --version\n  himd -V"
    )]
    Version,
}
