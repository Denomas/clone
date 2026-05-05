//! Shared compatibility helpers for cross-libc (glibc vs musl) builds.

/// Wrapper for `libc::pthread_t` to implement `Send`.
///
/// On musl, `pthread_t` is `*mut c_void` which doesn't implement `Send`.
/// This wrapper allows safely sending thread IDs through channels.
pub(crate) struct SendPthreadT(pub libc::pthread_t);
unsafe impl Send for SendPthreadT {}

/// Platform-correct ioctl request type.
/// glibc uses `c_ulong`, musl uses `c_int`.
#[cfg(target_env = "gnu")]
pub(crate) type IoctlReq = libc::c_ulong;
#[cfg(not(target_env = "gnu"))]
pub(crate) type IoctlReq = libc::c_int;

/// Convert a u32 ioctl number to the platform's IoctlReq type.
/// On musl (c_int), this reinterprets the bits (safe for ioctl numbers
/// where the high bit is set by _IOW/_IOR macros).
#[inline(always)]
#[allow(overflowing_literals)]
pub(crate) fn ioctl_req(n: u32) -> IoctlReq {
    #[cfg(target_env = "gnu")]
    {
        n as IoctlReq
    }
    #[cfg(not(target_env = "gnu"))]
    {
        n as i32
    }
}
