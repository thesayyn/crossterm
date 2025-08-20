//! This module provides platform related functions.

#[cfg(unix)]
#[cfg(feature = "events")]
pub use self::unix::supports_keyboard_enhancement;
#[cfg(unix)]
pub(crate) use self::unix::{
    disable_raw_mode, enable_raw_mode, is_raw_mode_enabled, size, window_size,
};
#[cfg(target_arch = "wasm32")]
#[cfg(feature = "events")]
pub use self::wasm::supports_keyboard_enhancement;
#[cfg(target_arch = "wasm32")]
pub(crate) use self::wasm::{
    disable_raw_mode, enable_raw_mode, is_raw_mode_enabled, size, window_size,
};

#[cfg(windows)]
#[cfg(feature = "events")]
pub use self::windows::supports_keyboard_enhancement;
#[cfg(all(windows, test))]
pub(crate) use self::windows::temp_screen_buffer;
#[cfg(windows)]
pub(crate) use self::windows::{
    clear, disable_raw_mode, enable_raw_mode, is_raw_mode_enabled, scroll_down, scroll_up,
    set_size, set_window_title, size, window_size,
};

#[cfg(windows)]
mod windows;

#[cfg(unix)]
pub mod file_descriptor;
#[cfg(unix)]
mod unix;

#[cfg(target_arch = "wasm32")]
mod wasm {
    use crate::terminal::WindowSize;

    use std::io;

    pub fn enable_raw_mode() -> io::Result<()> {
        // TODO: implement enable_raw_mode for wasm32
        Ok(())
    }

    pub fn disable_raw_mode() -> io::Result<()> {
        // TODO: implement disable_raw_mode for wasm32
        Ok(())
    }

    pub fn is_raw_mode_enabled() -> io::Result<bool> {
        // TODO: implement is_raw_mode_enabled for wasm32
        Ok(false)
    }

    pub fn size() -> io::Result<(u16, u16)> {
        Ok((139, 27))
    }

    pub fn window_size() -> io::Result<WindowSize> {
        Ok(WindowSize {
            width: 139,
            height: 27,
            columns: 139,
            rows: 27,
        })
    }

    pub fn supports_keyboard_enhancement() -> io::Result<bool> {
        Ok(false)
    }
}
