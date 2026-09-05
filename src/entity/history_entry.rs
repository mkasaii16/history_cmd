#[derive(Debug)]
pub struct HistoryEntry {
    pub command: String,
    pub timestamp: Option<u64>,
    pub duration: Option<u64>,
}
