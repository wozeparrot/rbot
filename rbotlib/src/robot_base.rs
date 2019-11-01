use std::sync::atomic::{AtomicBool, Ordering};

use rbothal::*;

use crate::driverstation::*;

static ROBOT_INITED: AtomicBool = AtomicBool::new(false);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum RobotBaseError {
    HALInitFailed,
    AlreadyInited,
}

#[derive(Debug)]
pub struct RobotBase {
    pub hal_timeout: i32,
}

impl RobotBase {
    pub fn new(hal_timeout: i32) -> Result<RobotBase, RobotBaseError> {
        if ROBOT_INITED.compare_and_swap(false, true, Ordering::AcqRel) {
            return Err(RobotBaseError::AlreadyInited);
        }

        if unsafe {
            HAL_Initialize(500, 0)
        } == 0 {
            return Err(RobotBaseError::HALInitFailed);
        }

        println!("\n\n******* Robot Hardware Abstraction Layer Init *******\n\n");
        Ok(RobotBase{
            hal_timeout: hal_timeout,
        })
    }

    pub fn run(&self) {
        unsafe {
            HAL_ObserveUserProgramStarting();
        }

        println!("\n\n******* Robot Program Starting *******\n\n");
    }

    pub fn init_ds(&self) -> DriverStation {
        DriverStation::from_robot_base(self).expect("HAL Failed on DriverStation Init")
    }
}

impl Drop for RobotBase {
    fn drop(&mut self) {
        unsafe {
            HAL_ReleaseDSMutex();
        }
    }
}