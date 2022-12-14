use core::{
    ffi::{c_char, c_int, CStr},
    ptr::null,
};

/// An iterator over the operating system arguments.
///
/// The implementation relies on certain libc implementation passing `argv` and `argc` at the start of the program via the .init_array link section.
///
/// The following platforms should work without issues:
/// - Any linux version using gnu libc, musl is unsupported
/// - Recent android version (currently only tested on android 12, older versions have not been tested but might work too)
/// - Fairly recent macos versions (older versions have not been tested but might work too)
/// - Fairly recent versions of freebsd or derivates (older versions have not been tested but might work too)
///
/// iOS/iPadOS have not been tested since I do not have a way to test on those platforms. They might work but support is unknown.
///
/// On unsupported unix platforms it will still compile but since this kind of operations are very unsafe it will just return no args.
/// If you wish to use it on unsupported unix platform you can enable the `unsafe_impl` feature but that will lead to undefined behavior.
pub struct StaticArgs {
    next: *const *const c_char,
    end: *const *const c_char,
}

impl StaticArgs {
    /// Creates an empty iterator that will yield nothing.
    pub const fn empty() -> Self {
        Self {
            next: null(),
            end: null(),
        }
    }

    /// Creates a new iterator from the provided pointers
    ///
    /// # Safety
    /// Must point to valid `argc` and `argv` pointers provided by the libc implementation.
    pub const unsafe fn new(argc: c_int, argv: *const *const c_char) -> Self {
        Self {
            next: argv,
            end: argv.offset(argc as isize),
        }
    }
}

impl Iterator for StaticArgs {
    type Item = &'static CStr;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next == self.end {
            None
        } else {
            let c_str = unsafe { CStr::from_ptr(*self.next) };
            self.next = unsafe { self.next.offset(1) };
            Some(c_str)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

#[rustversion::nightly]
impl ExactSizeIterator for StaticArgs {
    fn len(&self) -> usize {
        // This is safe because we are dealing with pointers and rust default behavior is actually worse.
        unsafe {
            // FIXME: Requires stabilization of unchecked_math. see rust-lang/rust#85122
            (self.end as usize).unchecked_sub(self.next as usize)
        }
    }
}

#[rustversion::not(nightly)]
impl ExactSizeIterator for StaticArgs {
    fn len(&self) -> usize {
        self.end as usize - self.next as usize
    }
}

static mut ARGC: c_int = 0;
static mut ARGV: *const *const c_char = null();

#[cfg(any(
    all(target_os = "linux", target_env = "gnu"),
    target_os = "macos",
    target_os = "android",
    target_os = "freebsd",
    feature = "unsafe_impl"
))]
#[cfg_attr(
    any(
        all(target_os = "linux", target_env = "gnu"),
        target_os = "freebsd",
        target_os = "android",
        feature = "unsafe_impl"
    ),
    link_section = ".init_array"
)]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
#[used]
static ARGV_INIT_ARRAY: extern "C" fn(c_int, *const *const c_char, *const *const c_char) = {
    extern "C" fn init_wrapper(
        argc: c_int,
        argv: *const *const c_char,
        _envp: *const *const c_char,
    ) {
        unsafe {
            ARGC = argc;
            ARGV = argv;
        }
    }
    init_wrapper
};

/// Returns a new iterator over the os args.
#[must_use = "Iterators do nothing unless consumed"]
pub fn static_args() -> StaticArgs {
    // Reading this statics is safe:
    // - They start out as a 0 and a null pointer
    // - They only ever get modified in the init_array section which assuming a working libc implementation it should be valid.
    if unsafe { ARGV.is_null() } {
        StaticArgs::empty()
    } else {
        unsafe { StaticArgs::new(ARGC, ARGV) }
    }
}
