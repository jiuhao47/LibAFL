//! # host
//! The host module is responsible for interacting with the emulator hosting the
//! target. It provides an abstraction to allow alternative implementations to
//! be used in the event a different emulator is used, or if the target
//! application is for a different operating system, then the interface for
//! interacting the host may be different.
use core::fmt::Debug;

use crate::{GuestAddr, shadow::PoisonType};

#[cfg(feature = "libc")]
pub mod libc;

#[cfg(all(feature = "linux", target_os = "linux"))]
pub mod linux;

#[repr(usize)]
#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
enum HostAction {
    CheckLoad,
    CheckStore,
    Poison,
    UserPoison,
    Unpoison,
    IsPoison,
    Alloc,
    Dealloc,
    Enable,
    Disable,
    SwapState,
}

pub trait Host: Debug + Send {
    type Error: Debug;
    fn load(start: GuestAddr, len: usize) -> Result<(), Self::Error>;
    fn store(start: GuestAddr, len: usize) -> Result<(), Self::Error>;
    fn poison(start: GuestAddr, len: usize, val: PoisonType) -> Result<(), Self::Error>;
    fn unpoison(start: GuestAddr, len: usize) -> Result<(), Self::Error>;
    fn is_poison(start: GuestAddr, len: usize) -> Result<bool, Self::Error>;
    fn swap(enabled: bool) -> Result<(), Self::Error>;
    fn track(start: GuestAddr, len: usize) -> Result<(), Self::Error>;
    fn untrack(start: GuestAddr) -> Result<(), Self::Error>;
}
