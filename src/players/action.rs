#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Action {
    Check,
    Fold,
    Call,
    Raise(u32), // Always positive since it is unsigned
    AllIn(u32),
}

impl Action {
    // Returns the amount of chips in a raise
    pub fn amount(&self) -> u32 {
        match self {
            Action::Raise(amount) => *amount,
            Action::AllIn(amount) => *amount,
            _ => 0,
        }
    }
}
