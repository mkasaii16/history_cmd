#[derive(Debug)]
pub struct HistoryAnalysis {
    pub total_commands: usize,
    pub unique_commands: usize,
    pub git_commands: usize,
    pub git_commits: usize,
    pub cargo_commands: usize,
    pub node_commands: usize,
    pub top_commands: Vec<(String, usize)>,
}
