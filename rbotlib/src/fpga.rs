use rbothal::*;

pub fn get_version() -> HalResult<i32> {
    hal_call!(HAL_GetFPGAVersion())
}

pub fn get_revision() -> HalResult<i64> {
    hal_call!(HAL_GetFPGARevision())
}

pub fn get_time_us() -> HalResult<u64> {
    hal_call!(HAL_GetFPGATime())
}

pub fn get_user_down() -> HalResult<bool> {
    Ok(hal_call!(HAL_GetFPGAButton())? != 0)
}