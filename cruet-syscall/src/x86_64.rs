use crate::SyscallArg;
use core::arch::asm;

/// ## Safety
/// This is unsafe because we invoke raw syscall from the function.
#[inline(always)]
pub unsafe fn syscall0(mut num: SyscallArg) -> SyscallArg {
    asm!(
    "syscall",
    in("rax") num,
    out("rcx") _,
    out("r11") _,
    lateout("rax") num,
    options(nostack));

    num
}

/// ## Safety
/// This is unsafe because we invoke raw syscall from the function.
#[inline(always)]
pub unsafe fn syscall1(mut num: SyscallArg, arg1: SyscallArg) -> SyscallArg {
    asm!(
    "syscall",
    in("rax") num,
    in("rdi") arg1,
    out("rcx") _,
    out("r11") _,
    lateout("rax") num,
    options(nostack));
    num
}

/// ## Safety
/// This is unsafe because we invoke raw syscall from the function.
#[inline(always)]
pub unsafe fn syscall2(mut num: SyscallArg, arg1: SyscallArg, arg2: SyscallArg) -> SyscallArg {
    asm!(
    "syscall",
    in("rax") num,
    in("rdi") arg1,
    in("rsi") arg2,
    out("rcx") _,
    out("r11") _,
    lateout("rax") num,
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
    "syscall",
    in("rax") num,
    in("rdi") arg1,
    in("rsi") arg2,
    in("rdx") arg3,
    out("rcx") _,
    out("r11") _,
    lateout("rax") num,
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
    "syscall",
    in("rax") num,
    in("rdi") arg1,
    in("rsi") arg2,
    in("rdx") arg3,
    in("r10") arg4,
    out("rcx") _,
    out("r11") _,
    lateout("rax") num,
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
    "syscall",
    in("rax") num,
    in("rdi") arg1,
    in("rsi") arg2,
    in("rdx") arg3,
    in("r10") arg4,
    in("r8") arg5,
    out("rcx") _,
    out("r11") _,
    lateout("rax") num,
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
    "syscall",
    in("rax") num,
    in("rdi") arg1,
    in("rsi") arg2,
    in("rdx") arg3,
    in("r10") arg4,
    in("r8") arg5,
    in("r9") arg6,
    out("rcx") _,
    out("r11") _,
    lateout("rax") num,
    options(nostack));
    num
}
