use std::ffi::CStr;
use std::os::raw::c_char;

use rbothal::HAL_MatchType::*;
use rbothal::*;

use crate::robot_base::RobotBase;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum JoystickError {
    PortDNE,
    ButtonUnplugged,
    AxisUnplugged,
    AxisDNE,
    PovDNE,
    PovUnplugged,
}

#[derive(Copy, Clone, Debug)]
pub struct JoystickPort(i32);

impl JoystickPort {
    pub fn new(port: u8) -> Result<JoystickPort, JoystickError> {
        if port as u32 >= HAL_kMaxJoysticks {
            return Err(JoystickError::PortDNE);
        }
        Ok(JoystickPort(i32::from(port)))
    }
}

#[derive(Copy, Clone, Debug)]
pub struct JoystickButton(i32);

impl JoystickButton {
    pub fn new(button: u8) -> Result<JoystickButton, JoystickError> {
        Ok(JoystickButton(i32::from(button)))
    }
}

#[derive(Copy, Clone, Debug)]
pub struct JoystickPOV(i32);

impl JoystickPOV {
    pub fn new(pov: u8) -> Result<JoystickPOV, JoystickError> {
        if pov as u32 >= HAL_kMaxJoystickPOVs {
            return Err(JoystickError::PovDNE);
        }
        Ok(JoystickPOV(i32::from(pov)))
    }
}

#[derive(Copy, Clone, Debug)]
pub struct JoystickAxis(i32);

impl JoystickAxis {
    pub fn new(axis: u8) -> Result<JoystickAxis, JoystickError> {
        if axis as u32 >= HAL_kMaxJoystickAxes {
            return Err(JoystickError::AxisDNE);
        }
        Ok(JoystickAxis(i32::from(axis)))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Alliance {
    Red,
    Blue,
    None,
}

#[derive(Debug, Copy, Clone)]
enum MatchType {
    Practice,
    Qualification,
    Elimination,
    None,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum RobotState {
    Disabled,
    Autonomous,
    Teleop,
    Test,
    EStop,
}

#[derive(Debug, Clone)]
struct MatchData {
    event_name: String,
    game_specific_message: Vec<u8>,
    match_number: u16,
    replay_number: u8,
    match_type: MatchType,
}

impl From<HAL_MatchInfo> for MatchData {
    fn from(info: HAL_MatchInfo) -> MatchData {
        let mut cs = info.eventName;
        cs[cs.len() - 1] = 0;

        MatchData {
            event_name: unsafe {
                CStr::from_ptr(&cs as *const c_char)
            }.to_string_lossy().into_owned(),
            game_specific_message: info.gameSpecificMessage[0..info.gameSpecificMessageSize as usize].to_vec(),
            match_number: info.matchNumber,
            replay_number: info.replayNumber,
            match_type: match info.matchType {
                HAL_kMatchType_practice => MatchType::Practice,
                HAL_kMatchType_qualification => MatchType::Qualification,
                HAL_kMatchType_elimination => MatchType::Elimination,
                _ => MatchType::None,
            },
        }
    }
}

#[derive(Clone, Debug)]
pub struct DriverStation<'a>(&'a RobotBase);

impl<'a> DriverStation<'a> {
    #[inline]
    pub(crate) fn from_robot_base(base: &'a RobotBase) -> Result<DriverStation, ()> {
        if unsafe {
            HAL_Initialize(base.hal_timeout, 0)
        } == 0 {
            return Err(());
        }

        Ok(DriverStation(base))
    }

    #[inline]
    pub fn get_button_pressed(&self, port: JoystickPort, button: JoystickButton) -> Result<bool, JoystickError> {
        let mut buttons: HAL_JoystickButtons = Default::default();

        unsafe {
            HAL_GetJoystickButtons(port.0, &mut buttons);
        }

        if button.0 >= buttons.count as i32 {
            return Err(JoystickError::ButtonUnplugged);
        }

        Ok(buttons.buttons & (1 << button.0) != 0)
    }

    #[inline]
    pub fn get_stick_axis(&self, port: JoystickPort, axis: JoystickAxis) -> Result<f32, JoystickError> {
        let mut axes: HAL_JoystickAxes = Default::default();

        unsafe {
            HAL_GetJoystickAxes(port.0, &mut axes);
        }

        if axis.0 > i32::from(axes.count) {
            return Err(JoystickError::AxisUnplugged);
        }

        Ok(axes.axes[axis.0 as usize])
    }

    #[inline]
    pub fn get_stick_pov(&self, port: JoystickPort, pov: JoystickPOV) -> Result<i32, JoystickError> {
        let mut povs: HAL_JoystickPOVs = Default::default();

        unsafe {
            HAL_GetJoystickPOVs(port.0, &mut povs);
        }

        if pov.0 > i32::from(povs.count) {
            return Err(JoystickError::PovUnplugged);
        }

        Ok(povs.povs[pov.0 as usize] as i32)
    }
}