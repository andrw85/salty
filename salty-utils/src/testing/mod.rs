use crate::storage::Workspace;
pub trait Testing {
    fn default() -> Self;
}

pub fn reset_salty_home() {
    let _ = Workspace::new().reset_workspace();
}
