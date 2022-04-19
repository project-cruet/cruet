#![feature(core_intrinsics)]
#![feature(untagged_unions)]
#![feature(cmpxchg16b_target_feature)]
#![cfg_attr(test, feature(thread_id_value))]
#![no_std]

extern crate alloc;

use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};

#[repr(C, align(16))]
struct Separated<T> {
    ptr: AtomicPtr<T>,
    token: AtomicUsize,
}

pub union ProtectedPtr<T> {
    atomic: UnsafeCell<i128>,
    separated: Separated<T>,
}

unsafe impl<T> Send for ProtectedPtr<T> {}

unsafe impl<T> Sync for ProtectedPtr<T> {}

impl<T> Clone for ProtectedPtr<T> {
    fn clone(&self) -> Self {
        unsafe {
            ProtectedPtr::new(
                self.separated.ptr.load(Ordering::Relaxed),
                self.separated.token.load(Ordering::Relaxed),
            )
        }
    }
}

pub struct Snapshot<T> {
    old: Separated<T>,
    parent: *mut ProtectedPtr<T>,
}

impl<T> ProtectedPtr<T> {
    pub const fn default() -> Self {
        Self {
            separated: Separated {
                ptr: AtomicPtr::new(core::ptr::null_mut()),
                token: AtomicUsize::new(0),
            },
        }
    }

    pub const fn new(ptr: *mut T, token: usize) -> Self {
        Self {
            separated: Separated {
                ptr: AtomicPtr::new(ptr),
                token: AtomicUsize::new(token),
            },
        }
    }

    pub fn initialize(&self, x: *mut T) {
        unsafe {
            self.separated.ptr.store(x, Ordering::Relaxed);
            self.separated.token.store(0, Ordering::Relaxed);
        }
    }

    pub fn read(&self) -> Snapshot<T> {
        let old = unsafe {
            Separated {
                ptr: AtomicPtr::new(self.separated.ptr.load(Ordering::Relaxed)),
                token: AtomicUsize::new(self.separated.token.load(Ordering::Relaxed)),
            }
        };
        Snapshot {
            old,
            parent: self as *const ProtectedPtr<T> as *mut ProtectedPtr<T>,
        }
    }
}

impl<T> Snapshot<T> {
    pub fn ptr(&self) -> *mut T {
        self.old.ptr.load(Ordering::Relaxed)
    }

    pub fn try_store(&self, value: *mut T) -> bool {
        unsafe { self.store_impl(value) }
    }

    #[cfg_attr(target_arch = "x86_64", target_feature(enable = "cmpxchg16b"))]
    unsafe fn store_impl(&self, value: *mut T) -> bool {
        let token = (*self.parent).separated.token.load(Ordering::Relaxed);

        let xchg = ProtectedPtr::new(value, token.wrapping_add(1));
        let old = ProtectedPtr::new(
            self.old.ptr.load(Ordering::Relaxed),
            self.old.token.load(Ordering::Relaxed),
        );

        let addr = &(*self.parent).atomic;

        let (_, res) = core::intrinsics::atomic_cxchgweak_acqrel_failrelaxed(
            addr.get(),
            *old.atomic.get(),
            *xchg.atomic.get(),
        );
        res
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use std::boxed::Box;
    use super::*;

    struct Record {
        data: *mut Node,
    }

    unsafe impl Send for Record {}

    unsafe impl Sync for Record {}

    struct Node {
        content: usize,
        next: ProtectedPtr<Node>,
    }

    impl Node {
        fn new(content: usize) -> Self {
            Node {
                content,
                next: ProtectedPtr::default(),
            }
        }

        fn alloc(data: usize) -> *mut Self {
            Box::leak(Box::new(Node::new(data)))
        }

        fn alloc_link(content: usize, next: ProtectedPtr<Node>) -> *mut Self {
            Box::leak(Box::new(Node {
                content,
                next,
            }))
        }

        unsafe fn dealloc(node: &ProtectedPtr<Node>) {
            Box::from_raw(node.read().ptr());
        }

        unsafe fn dealloc_from(node: &ProtectedPtr<Node>) {
            let ptr = node.read().ptr();
            if !ptr.is_null() {
                Self::dealloc_from(&(*ptr).next);
                Self::dealloc(node);
            }
        }

        fn take(node: &ProtectedPtr<Node>) -> Option<usize> {
            unsafe {
                loop {
                    let snapshot = node.read();
                    if snapshot.ptr().is_null() {
                        return None;
                    }
                    let next = (*snapshot.ptr()).next.read();
                    let data = Some((*snapshot.ptr()).content);
                    if snapshot.try_store(next.ptr()) {
                        std::println!("popped {}", data.unwrap());
                        return data;
                    };
                    std::thread::yield_now();
                }
            }
        }

        fn push(node: &ProtectedPtr<Node>, record: &std::sync::Mutex<std::vec::Vec<Record>>, data: usize) {
            loop {
                let snapshot = node.read();
                let new = Node::alloc_link(data, node.clone());
                if snapshot.try_store(new) {
                    std::println!("pushed {}", data);
                    record.lock().unwrap().push(Record { data: new });
                    break;
                };
                unsafe {
                    Box::from_raw(new);
                }
                std::thread::yield_now();
            }
        }
    }

    #[test]
    fn it_handles_basic_operations() {
        let head = ProtectedPtr::default();
        head.initialize(Node::alloc(0));
        let cmp = head.read();
        cmp.try_store(Node::alloc_link(1, head.clone()));

        unsafe {
            assert_eq!((*head.read().ptr()).content, 1);
            assert_eq!((*(*head.read().ptr()).next.read().ptr()).content, 0);
            Node::dealloc_from(&head);
        }
    }

    #[test]
    fn it_handles_concurrent_operations() {
        let counter = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let head = std::sync::Arc::new(ProtectedPtr::default());
        let record = std::sync::Arc::new(std::sync::Mutex::new(std::vec::Vec::new()));
        let mut handles = std::vec::Vec::new();
        for i in 0..=100 {
            {
                let counter = counter.clone();
                let head = head.clone();
                handles.push(std::thread::spawn(move || {
                    std::thread::yield_now();
                    while counter.load(std::sync::atomic::Ordering::Acquire) < 5050 {
                        if let Some(data) = Node::take(&head) {
                            counter.fetch_add(data, Ordering::SeqCst);
                        }
                        std::thread::yield_now();
                    }
                }));
            }
            {
                let head = head.clone();
                let record = record.clone();
                handles.push(std::thread::spawn(move || {
                    std::thread::yield_now();
                    Node::push(&head, &record, i);
                }));
            }
        }
        for i in handles {
            i.join().unwrap();
        }
        for i in record.lock().unwrap().iter() {
            unsafe {
                Box::from_raw(i.data);
            }
        }
        assert_eq!(counter.load(Ordering::Relaxed), 5050);
    }
}
