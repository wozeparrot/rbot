#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum RobotBaseError {
    HALInitFailed,
    AlreadyInited,
}

pub struct RobotBase;

impl RobotBase {
    pub fn new() -> Result<RobotBase, RobotBaseError> {
        Err(RobotBaseError::AlreadyInited)
    }
}