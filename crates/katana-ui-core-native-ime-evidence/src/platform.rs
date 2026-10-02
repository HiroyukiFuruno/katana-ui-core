//! OS input-method identity. Missing platform facilities are errors by design.

use std::io;
#[cfg(target_os = "linux")]
use std::process::Command as StdCommand;
#[cfg(any(target_os = "linux", all(target_os = "macos", test)))]
mod process;

#[cfg(target_os = "macos")]
const CF_STRING_BUFFER_BYTES: usize = 512;
#[cfg(target_os = "macos")]
const CF_STRING_UTF8: u32 = 0x0800_0100;

#[cfg(target_os = "linux")]
struct ProcessService;
#[cfg(target_os = "linux")]
impl ProcessService {
    fn create_command<S: AsRef<std::ffi::OsStr>>(program: S) -> StdCommand {
        StdCommand::new(program)
    }
}

pub struct PlatformInputMethod;
const DEFAULT_LOOKUP_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(2);

impl PlatformInputMethod {
    pub fn current() -> io::Result<String> {
        #[cfg(target_os = "macos")]
        {
            macos()
        }
        #[cfg(target_os = "windows")]
        {
            windows()
        }
        #[cfg(target_os = "linux")]
        {
            linux(DEFAULT_LOOKUP_TIMEOUT)
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
        {
            Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "native IME lookup is unsupported on this OS",
            ))
        }
    }

    pub fn current_with_timeout(timeout: std::time::Duration) -> io::Result<String> {
        if timeout == DEFAULT_LOOKUP_TIMEOUT {
            return Self::current();
        }
        #[cfg(not(target_os = "linux"))]
        {
            let _ = timeout;
            Self::current()
        }
        #[cfg(target_os = "linux")]
        {
            linux(timeout)
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
        {
            Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "native IME lookup is unsupported on this OS",
            ))
        }
    }
}

#[cfg(target_os = "macos")]
fn macos() -> io::Result<String> {
    use std::ffi::{c_char, c_void};
    type Source = *const c_void;
    #[link(name = "Carbon", kind = "framework")]
    unsafe extern "C" {
        fn TISCopyCurrentKeyboardInputSource() -> Source;
        fn TISGetInputSourceProperty(source: Source, property: Source) -> Source;
        static kTISPropertyInputSourceID: Source;
    }
    #[link(name = "CoreFoundation", kind = "framework")]
    unsafe extern "C" {
        fn CFStringGetCString(
            string: Source,
            buffer: *mut c_char,
            size: isize,
            encoding: u32,
        ) -> bool;
        fn CFRelease(value: Source);
    }
    /* WHY: Carbon の標準 TIS API で現在の入力ソースを取得し、OS の自己申告値を混ぜない。 */
    let source = unsafe { TISCopyCurrentKeyboardInputSource() };
    if source.is_null() {
        return Err(io::Error::other(
            "TISCopyCurrentKeyboardInputSource returned null",
        ));
    }
    let property = unsafe { TISGetInputSourceProperty(source, kTISPropertyInputSourceID) };
    if property.is_null() {
        unsafe {
            CFRelease(source);
        }
        return Err(io::Error::other("TIS input source has no identifier"));
    }
    let mut buffer = [0i8; CF_STRING_BUFFER_BYTES];
    let ok = unsafe {
        CFStringGetCString(
            property,
            buffer.as_mut_ptr(),
            buffer.len() as isize,
            CF_STRING_UTF8,
        )
    };
    unsafe {
        CFRelease(source);
    }
    if !ok {
        return Err(io::Error::other("TIS input source identifier is not UTF-8"));
    }
    let bytes = unsafe {
        std::slice::from_raw_parts(
            buffer.as_ptr().cast::<u8>(),
            buffer.iter().position(|b| *b == 0).unwrap_or(buffer.len()),
        )
    };
    let value = String::from_utf8_lossy(bytes).trim().to_owned();
    if value.is_empty() {
        Err(io::Error::other("TIS input source identifier is empty"))
    } else {
        Ok(value)
    }
}

#[cfg(target_os = "windows")]
fn windows() -> io::Result<String> {
    use windows_sys::Win32::UI::Input::KeyboardAndMouse::GetKeyboardLayout;
    let layout = unsafe { GetKeyboardLayout(0) };
    if layout.is_null() {
        return Err(io::Error::other("GetKeyboardLayout returned zero"));
    }
    Ok(format!("HKL-0x{:016x}", layout as usize))
}

#[cfg(target_os = "linux")]
fn linux(timeout: std::time::Duration) -> io::Result<String> {
    let deadline = std::time::Instant::now()
        .checked_add(timeout)
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "input method lookup timeout overflows the monotonic clock",
            )
        })?;
    for (program, args) in [("ibus", ["engine"]), ("fcitx5-remote", ["-n"])] {
        let remaining = deadline.saturating_duration_since(std::time::Instant::now());
        if remaining.is_zero() {
            return Err(io::Error::new(
                io::ErrorKind::TimedOut,
                "input method lookup timed out",
            ));
        }
        let mut command = ProcessService::create_command(program);
        command.args(args);
        let Ok(output) = process::output_with_timeout(command, remaining) else {
            continue;
        };
        if output.status.success() {
            let value = String::from_utf8_lossy(&output.stdout).trim().to_owned();
            if !value.is_empty() {
                return Ok(value);
            }
        }
    }
    Err(io::Error::other(
        "neither ibus engine nor fcitx5-remote -n returned an input method",
    ))
}
