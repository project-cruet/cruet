#![no_std]
#![feature(core_ffi_c)]

pub mod nr;
pub type SyscallArg = ::core::ffi::c_long;

#[cfg(target_arch = "x86_64")]
mod x86_64;

#[cfg(target_arch = "x86_64")]
pub use x86_64::*;

#[cfg(target_arch = "aarch64")]
mod aarch64;

#[cfg(target_arch = "aarch64")]
pub use aarch64::*;

/// Macro helper to make a syscall
/// ## Example:
/// ```rust
/// use cruet_syscall::*;
/// let _tid = unsafe { syscall!(nr::__NR_gettid) };
/// ```
#[macro_export]
macro_rules! syscall {
    ($num:expr) => {
        $crate::syscall0($num)
    };
    ($num:expr, $arg1:expr) => {
        $crate::syscall1($num, $arg1)
    };
    ($num:expr, $arg1:expr, $arg2:expr) => {
        $crate::syscall2($num, $arg1, $arg2)
    };
    ($num:expr, $arg1:expr, $arg2:expr, $arg3:expr) => {
        $crate::syscall3($num, $arg1, $arg2, $arg3)
    };
    ($num:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr) => {
        $crate::syscall4($num, $arg1, $arg2, $arg3, $arg4)
    };
    ($num:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr) => {
        $crate::syscall5($num, $arg1, $arg2, $arg3, $arg4, $arg5)
    };
    ($num:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr, $arg6:expr) => {
        $crate::syscall6($num, $arg1, $arg2, $arg3, $arg4, $arg5, $arg6)
    };
}

#[cfg(test)]
mod tests {
    extern crate std;

    use std::*;

    #[test]
    fn it_gets_pid() {
        unsafe {
            let tid = syscall!(crate::nr::__NR_gettid);
            println!("tid: {}", tid);
        }
    }

    #[test]
    fn it_yields_schedule() {
        unsafe {
            let res = syscall!(crate::nr::__NR_sched_yield);
            assert_eq!(res, 0);
        }
    }
}
