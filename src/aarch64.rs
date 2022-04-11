use crate::SyscallArg;
use core::arch::asm;

/// ## Safety
/// This is unsafe because we invoke raw syscall from the function.
#[inline(always)]
pub unsafe fn syscall0(mut num: SyscallArg) -> SyscallArg {
    asm!(
    "svc 0",
    in("x8") num,
    lateout("x0") num,
    options(nostack));

    num
}

/// ## Safety
/// This is unsafe because we invoke raw syscall from the function.
#[inline(always)]
pub unsafe fn syscall1(mut num: SyscallArg, arg1: SyscallArg) -> SyscallArg {
    asm!(
    "svc 0",
    in("x8") num,
    in("x0") arg1,
    lateout("x0") num,
    options(nostack));

    num
}

/// ## Safety
/// This is unsafe because we invoke raw syscall from the function.
#[inline(always)]
pub unsafe fn syscall2(mut num: SyscallArg, arg1: SyscallArg, arg2: SyscallArg) -> SyscallArg {
    asm!(
    "svc 0",
    in("x8") num,
    in("x0") arg1,
    in("x1") arg2,
    lateout("x0") num,
    options(nostack));

    num
}

/// ## Safety
/// This is unsafe because we invoke raw syscall from the function.
#[inline(always)]
pub unsafe fn syscall3(
    mut num: SyscallArg,
    arg1: SyscallArg,
    arg2: SyscallArg,
    arg3: SyscallArg,
) -> SyscallArg {
    asm!(
    "svc 0",
    in("x8") num,
    in("x0") arg1,
    in("x1") arg2,
    in("x2") arg3,
    lateout("x0") num,
    options(nostack));

    num
}

/// ## Safety
/// This is unsafe because we invoke raw syscall from the function.
#[inline(always)]
pub unsafe fn syscall4(
    mut num: SyscallArg,
    arg1: SyscallArg,
    arg2: SyscallArg,
    arg3: SyscallArg,
    arg4: SyscallArg,
) -> SyscallArg {
    asm!(
    "svc 0",
    in("x8") num,
    in("x0") arg1,
    in("x1") arg2,
    in("x2") arg3,
    in("x3") arg4,
    lateout("x0") num,
    options(nostack));

    num
}

/// ## Safety
/// This is unsafe because we invoke raw syscall from the function.
#[inline(always)]
pub unsafe fn syscall5(
    mut num: SyscallArg,
    arg1: SyscallArg,
    arg2: SyscallArg,
    arg3: SyscallArg,
    arg4: SyscallArg,
    arg5: SyscallArg,
) -> SyscallArg {
    asm!(
    "svc 0",
    in("x8") num,
    in("x0") arg1,
    in("x1") arg2,
    in("x2") arg3,
    in("x3") arg4,
    in("x4") arg5,
    lateout("x0") num,
    options(nostack));

    num
}

/// ## Safety
/// This is unsafe because we invoke raw syscall from the function.
#[inline(always)]
pub unsafe fn syscall6(
    mut num: SyscallArg,
    arg1: SyscallArg,
    arg2: SyscallArg,
    arg3: SyscallArg,
    arg4: SyscallArg,
    arg5: SyscallArg,
    arg6: SyscallArg,
) -> SyscallArg {
    asm!(
    "svc 0",
    in("x8") num,
    in("x0") arg1,
    in("x1") arg2,
    in("x2") arg3,
    in("x3") arg4,
    in("x4") arg5,
    in("x5") arg6,
    lateout("x0") num,
    options(nostack));

    num
}
