#![macro_use]

use std::borrow::Cow;
use std::error::Error;
use std::ffi::CStr;
use std::fmt;

use crate::*;

#[derive(Copy, Clone)]
pub struct HalError(pub i32);

impl HalError {
    pub fn message(&self) -> Cow<str> {
        let const_char_ptr = unsafe {
            HAL_GetErrorMessage(self.0)
        };
        let c_str = unsafe {
            CStr::from_ptr(const_char_ptr)
        };

        c_str.to_string_lossy()
    }
}

impl fmt::Debug for HalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "HalError {{ {} }}", self.message())
    }
}

impl fmt::Display for HalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: \"{}\"!", self.message())
    }
}

impl Error for HalError {
    fn description(&self) -> &str {
        "Error in the HAL"
    }
}

impl From<i32> for HalError {
    fn from(code: i32) -> Self {
        HalError(code)
    }
}

pub type HalResult<T> = Result<T, HalError>;

#[must_use]
#[derive(Copy, Clone, Debug)]
pub struct HalMaybe<T> {
    ret: T,
    err: Option<HalError>,
}

impl<T> HalMaybe<T> {
    pub fn new(ret: T, err: Option<HalError>) -> HalMaybe<T> {
        HalMaybe { ret, err }
    }

    pub fn ok(self) -> T {
        self.ret
    }

    pub fn has_err(&self) -> bool {
        self.err.is_some()
    }

    pub fn err(&self) -> Option<HalError> {
        self.err
    }

    pub fn into_res(self) -> Result<T, HalError> {
        if let Some(x) = self.err {
            Err(x)
        } else {
            Ok(self.ret)
        }
    }
}

#[macro_export]
macro_rules! hal_call {
    ($function:ident($($arg:expr),* $(,)?)) => {{
        let mut status = 0;
        let result = unsafe { $function($(
            $arg,
        )* &mut status as *mut i32) };
        if status == 0 { Ok(result) } else { Err(HalError::from(status)) }
    }};
    ($namespace:path, $function:ident($($arg:expr),*)) => {{
        let mut status = 0;
        let result = unsafe { $namespace::$function($(
            $arg,
        )* &mut status as *mut i32) };
        if status == 0 { Ok(result) } else { Err(HalError::from(status)) }
    }};
}

#[macro_export]
macro_rules! maybe_hal_call {
    ($function:ident($($arg:expr),* $(,)?)) => {{
        let mut status = 0;
        let result = unsafe { $function($(
            $arg,
        )* &mut status as *mut i32) };
        HalMaybe::new(
            result,
            if status == 0 {
                None
            } else {
                Some(HalError::from(status))
            }
        )
    }};
    ($namespace:path, $function:ident($($arg:expr),*)) => {{
        let mut status = 0;
        let result = unsafe { $namespace::$function($(
            $arg,
        )* &mut status as *mut i32) };
        HalMaybe::new(
            result,
            if status == 0 {
                None
            } else {
                Some(HalError::from(status))
            }
        )
    }};
}