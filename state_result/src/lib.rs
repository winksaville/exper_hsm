pub type StateFnsHdl = usize;

pub enum StateResult {
    NotHandled,
    Handled,
    TransitionTo(StateFnsHdl),
}
