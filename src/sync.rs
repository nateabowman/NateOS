use core::sync::atomic::{AtomicBool, Ordering};
use spin::Mutex;

pub struct SpinLock<T> {
    locked: AtomicBool,
    data: Mutex<T>,
}

impl<T> SpinLock<T> {
    pub const fn new(data: T) -> Self {
        SpinLock {
            locked: AtomicBool::new(false),
            data: Mutex::new(data),
        }
    }

    pub fn lock(&self) -> SpinLockGuard<T> {
        while self.locked.compare_and_swap(false, true, Ordering::Acquire) {
            core::hint::spin_loop();
        }
        SpinLockGuard { lock: self }
    }
}

pub struct SpinLockGuard<'a, T> {
    lock: &'a SpinLock<T>,
}

impl<'a, T> Drop for SpinLockGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Ordering::Release);
    }
}

impl<'a, T> core::ops::Deref for SpinLockGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.lock.data.get() }
    }
}

impl<'a, T> core::ops::DerefMut for SpinLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.lock.data.get() }
    }
}

pub struct Mutex<T> {
    inner: spin::Mutex<T>,
}

impl<T> Mutex<T> {
    pub const fn new(data: T) -> Self {
        Mutex {
            inner: spin::Mutex::new(data),
        }
    }

    pub fn lock(&self) -> spin::MutexGuard<T> {
        self.inner.lock()
    }
}

pub struct Semaphore {
    count: AtomicBool,
    waiters: Mutex<usize>,
}

impl Semaphore {
    pub const fn new() -> Self {
        Semaphore {
            count: AtomicBool::new(true),
            waiters: Mutex::new(0),
        }
    }

    pub fn wait(&self) {
        while !self.count.compare_and_swap(true, false, Ordering::Acquire) {
            *self.waiters.lock() += 1;
            core::hint::spin_loop();
        }
    }

    pub fn signal(&self) {
        self.count.store(true, Ordering::Release);
    }
}

