use rbothal::*;

pub fn is_browned_out() -> HalResult<bool> {
    Ok(hal_call!(HAL_GetBrownedOut())? != 0)
}

pub fn is_system_active() -> HalResult<bool> {
    Ok(hal_call!(HAL_GetSystemActive())? != 0)
}

pub fn get_battery_voltage() -> HalResult<f64> {
    hal_call!(HAL_GetVinVoltage())
}

pub fn get_battery_current() -> HalResult<f64> {
    hal_call!(HAL_GetVinCurrent())
}