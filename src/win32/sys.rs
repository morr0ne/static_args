#![allow(non_snake_case, non_camel_case_types)]

use core::ffi::{c_int, c_void};

extern "C" {
    // pub fn memcmp(left: *const c_void, right: *const c_void, len: usize) -> c_int;
    pub fn wcslen(s: PCWSTR) -> usize;
}

/// A pointer to a null-terminated string of 16-bit Unicode characters.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PWSTR(pub *mut u16);

impl PWSTR {
    /// Construct a null `PWSTR`.
    pub const fn null() -> Self {
        Self(core::ptr::null_mut())
    }

    /// Returns a raw pointer to the `PWSTR`.
    pub const fn as_ptr(&self) -> *mut u16 {
        self.0
    }

    /// Checks whether the `PWSTR` is null.
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    /// String data without the trailing 0.
    ///
    /// # Safety
    ///
    /// The `PWSTR`'s pointer needs to be valid for reads up until and including the next `\0`.
    pub unsafe fn as_wide(&self) -> &[u16] {
        let len = wcslen(PCWSTR::from_raw(self.0));
        core::slice::from_raw_parts(self.0, len)
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PCWSTR(pub *const u16);

impl PCWSTR {
    /// Construct a new `PCWSTR` from a raw pointer
    pub const fn from_raw(ptr: *const u16) -> Self {
        Self(ptr)
    }

    /// Construct a null `PCWSTR`
    pub const fn null() -> Self {
        Self(core::ptr::null())
    }

    /// Returns a raw pointer to the `PCWSTR`
    pub const fn as_ptr(&self) -> *const u16 {
        self.0
    }

    /// Checks whether the `PCWSTR` is null
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    /// String data without the trailing 0
    ///
    /// # Safety
    ///
    /// The `PCWSTR`'s pointer needs to be valid for reads up until and including the next `\0`.
    pub unsafe fn as_wide(&self) -> &[u16] {
        let len = wcslen(*self);
        core::slice::from_raw_parts(self.0, len)
    }
}

#[repr(C)]
pub struct PEB {
    pub Reserved1: [u8; 2],
    pub BeingDebugged: u8,
    pub Reserved2: [u8; 1],
    pub Reserved3: [*mut c_void; 2],
    pub Ldr: *mut PEB_LDR_DATA,
    pub ProcessParameters: *mut RTL_USER_PROCESS_PARAMETERS,
    pub Reserved4: [*mut c_void; 3],
    pub AtlThunkSListPtr: *mut c_void,
    pub Reserved5: *mut c_void,
    pub Reserved6: u32,
    pub Reserved7: *mut c_void,
    pub Reserved8: u32,
    pub AtlThunkSListPtr32: u32,
    pub Reserved9: [*mut c_void; 45],
    pub Reserved10: [u8; 96],
    pub PostProcessInitRoutine: PPS_POST_PROCESS_INIT_ROUTINE,
    pub Reserved11: [u8; 128],
    pub Reserved12: [*mut c_void; 1],
    pub SessionId: u32,
}

impl Copy for PEB {}

impl Clone for PEB {
    fn clone(&self) -> Self {
        *self
    }
}

#[repr(C)]
pub struct PEB_LDR_DATA {
    pub Reserved1: [u8; 8],
    pub Reserved2: [*mut c_void; 3],
    pub InMemoryOrderModuleList: LIST_ENTRY,
}
impl Copy for PEB_LDR_DATA {}

impl Clone for PEB_LDR_DATA {
    fn clone(&self) -> Self {
        *self
    }
}

#[repr(C)]
pub struct LIST_ENTRY {
    pub Flink: *mut LIST_ENTRY,
    pub Blink: *mut LIST_ENTRY,
}

impl ::core::marker::Copy for LIST_ENTRY {}

impl ::core::clone::Clone for LIST_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}

#[repr(C)]
pub struct RTL_USER_PROCESS_PARAMETERS {
    pub Reserved1: [u8; 16],
    pub Reserved2: [*mut c_void; 10],
    pub ImagePathName: UNICODE_STRING,
    pub CommandLine: UNICODE_STRING,
}

impl ::core::marker::Copy for RTL_USER_PROCESS_PARAMETERS {}

impl ::core::clone::Clone for RTL_USER_PROCESS_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}

#[repr(C)]
pub struct UNICODE_STRING {
    pub Length: u16,
    pub MaximumLength: u16,
    pub Buffer: PWSTR,
}

impl ::core::marker::Copy for UNICODE_STRING {}

impl ::core::clone::Clone for UNICODE_STRING {
    fn clone(&self) -> Self {
        *self
    }
}

pub type PPS_POST_PROCESS_INIT_ROUTINE = ::core::option::Option<unsafe extern "system" fn()>;
