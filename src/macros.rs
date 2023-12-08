#![macro_use]

/// Macro to get c strings from literals without runtime overhead
/// Literal must not contain any interior nul bytes!
#[macro_export]
macro_rules! c_str {
    ($literal:expr) => {
        std::ffi::CStr::from_bytes_with_nul_unchecked(concat!($literal, "\0").as_bytes())
    };
}

/// Get offset to struct member, similar to `offset_of` in C/C++
/// From <https://stackoverflow.com/questions/40310483/how-to-get-pointer-offset-in-bytes/40310851#40310851>
#[macro_export]
macro_rules! offset_of {
    ($ty:ty, $field:ident) => {
        &(*(std::ptr::null() as *const $ty)).$field as *const _ as usize
    };
}
