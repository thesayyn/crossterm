#[cfg(all(unix, feature = "event-stream"))]
pub(crate) use unix::waker::Waker;
#[cfg(all(target_arch = "wasm32", feature = "event-stream"))]
pub(crate) use wasm::waker::Waker;
#[cfg(all(windows, feature = "event-stream"))]
pub(crate) use windows::waker::Waker;

#[cfg(unix)]
pub(crate) mod unix;
#[cfg(windows)]
pub(crate) mod windows;

#[cfg(target_arch = "wasm32")]
pub(crate) mod wasm {
    pub(crate) mod waker {
        #[derive(Debug)]
        pub struct Waker;

        impl Waker {
            pub fn wake(&self) {}
        }
    }
}
