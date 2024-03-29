#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]

use std::ffi::CStr;
use std::os::raw::c_char;

use rbothal::HAL_MatchType::*;
use rbothal::*;

use crate::joystick::{JoystickAxis, JoystickButton, JoystickError, JoystickPort, JoystickPOV};
use crate::robot_base::RobotBase;

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

    pub fn get_alliance(&self) -> HalResult<Alliance> {
        match hal_call!(HAL_GetAllianceStation())? {
            HAL_AllianceStationID::kRed1 | HAL_AllianceStationID::kRed2 | HAL_AllianceStationID::kRed3 => Ok(Alliance::Red),
            HAL_AllianceStationID::kBlue1 | HAL_AllianceStationID::kBlue2 | HAL_AllianceStationID::kBlue3 => Ok(Alliance::Blue),
            _ => Err(HalError(0)),
        }
    }

    pub fn get_station(&self) -> HalResult<i32> {
        match hal_call!(HAL_GetAllianceStation())? {
            HAL_AllianceStationID::kBlue1 | HAL_AllianceStationID::kRed1 => Ok(1),
            HAL_AllianceStationID::kBlue2 | HAL_AllianceStationID::kRed2 => Ok(2),
            HAL_AllianceStationID::kBlue3 | HAL_AllianceStationID::kRed3 => Ok(3),
            _ => Err(HalError(0))
        }
    }

    pub fn get_robot_state(&self) -> RobotState {
        let mut control_word: HAL_ControlWord = Default::default();

        unsafe {
            HAL_GetControlWord(&mut control_word);
        }

        if control_word.enabled() != 0 {
            if control_word.autonomous() != 0 {
                RobotState::Autonomous
            } else if control_word.test() != 0 {
                RobotState::Test
            } else {
                RobotState::Teleop
            }
        } else if control_word.eStop() != 0 {
            RobotState::EStop
        } else {
            RobotState::Disabled
        }
    }

    pub fn is_ds_attached(&self) -> bool {
        let mut control_word: HAL_ControlWord = Default::default();

        unsafe {
            HAL_GetControlWord(&mut control_word);
        }

        control_word.dsAttached() != 0
    }

    pub fn is_fms_attached(&self) -> bool {
        let mut control_word: HAL_ControlWord = Default::default();

        unsafe {
            HAL_GetControlWord(&mut control_word);
        }

        control_word.fmsAttached() != 0
    }

    pub fn get_game_message(&self) -> Vec<u8> {
        let mut info: HAL_MatchInfo = Default::default();

        unsafe {
            HAL_GetMatchInfo(&mut info);
        }

        info.gameSpecificMessage[0..info.gameSpecificMessageSize as usize].to_vec()
    }

    pub fn wait_for_data(&self) {
        unsafe {
            HAL_WaitForDSData();
        }
    }

    pub fn wait_for_data_timeout(&self, timeout: f64) {
        unsafe {
            HAL_WaitForDSDataTimeout(timeout);
        }
    }
}