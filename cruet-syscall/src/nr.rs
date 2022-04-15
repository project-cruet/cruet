#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use crate::SyscallArg;
include!(concat!(env!("OUT_DIR"), "/unistd.rs"));
