#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum BugMode {
    #[default]
    None,
    RaceCondition,
    Deadlock,
}

impl std::fmt::Display for BugMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BugMode::None => write!(f, "Off"),
            BugMode::RaceCondition => write!(f, "Race Condition"),
            BugMode::Deadlock => write!(f, "Deadlock"),
        }
    }
}
