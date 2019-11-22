use rbothal::*;

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
pub struct JoystickPort(pub i32);

impl JoystickPort {
    pub fn new(port: u8) -> Result<JoystickPort, JoystickError> {
        if port as u32 >= HAL_kMaxJoysticks {
            return Err(JoystickError::PortDNE);
        }
        Ok(JoystickPort(i32::from(port)))
    }
}

#[derive(Copy, Clone, Debug)]
pub struct JoystickButton(pub i32);

impl JoystickButton {
    pub fn new(button: u8) -> Result<JoystickButton, JoystickError> {
        Ok(JoystickButton(i32::from(button)))
    }
}

#[derive(Copy, Clone, Debug)]
pub struct JoystickPOV(pub i32);

impl JoystickPOV {
    pub fn new(pov: u8) -> Result<JoystickPOV, JoystickError> {
        if pov as u32 >= HAL_kMaxJoystickPOVs {
            return Err(JoystickError::PovDNE);
        }
        Ok(JoystickPOV(i32::from(pov)))
    }
}

#[derive(Copy, Clone, Debug)]
pub struct JoystickAxis(pub i32);

impl JoystickAxis {
    pub fn new(axis: u8) -> Result<JoystickAxis, JoystickError> {
        if axis as u32 >= HAL_kMaxJoystickAxes {
            return Err(JoystickError::AxisDNE);
        }
        Ok(JoystickAxis(i32::from(axis)))
    }
}

struct Joystick {
    port: JoystickPort,
}

impl Joystick {
    pub fn new(p: u8) -> Result<Joystick, JoystickError> {
        Ok(Joystick {
            port: JoystickPort::new(p)?,
        })
    }
}