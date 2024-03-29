use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
use std::thread;
use std::thread::Thread;

/// Our channel is represented by a pair of sender and receiver
/// which can be used once each and consumed statically ensuring
/// channels are only ever used once.
/// We optimize with borrowing and using lifetimes, putting the channel construction
/// as a responsibility of the user. We can make sure an object stays on the same thread
/// by making sure its type does not implement Send, which can be achieved with the PhantomData marker type.
pub struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool
}
unsafe impl<T> Sync for Channel<T> where T: Send{}
pub struct Sender<'a, T> {
    channel: &'a Channel<T>,
    receiving_thread: Thread,
}
pub struct Receiver<'a, T>{
    channel: &'a Channel<T>,
    _no_send: PhantomData<*const ()>,
}

impl<T> Channel<T> {
    pub const fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            ready: AtomicBool::new(false),
        }
    }

    pub fn split(&mut self) -> (Sender<T>, Receiver<T>) {
        *self = Self::new();
        (
            Sender {
                channel: self,
                receiving_thread: thread::current()
            },
            Receiver {
                channel: self,
                _no_send: PhantomData
            }
        )
    }
}

impl<T> Sender<'_, T> {
    pub fn send(self, message: T) {
        unsafe { (*self.channel.message.get()).write(message) };
        self.channel.ready.store(true, Release);
        self.receiving_thread.unpark();
    }
}

impl<T> Receiver<'_, T> {
    pub fn is_ready(&self) -> bool {
        self.channel.ready.load(Relaxed)
    }
    pub fn receive(self) -> T {
        while !self.channel.ready.swap(false, Acquire) {
            thread::park();
        }
        unsafe { (*self.channel.message.get()).assume_init_read() }
    }
}

impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        if *self.ready.get_mut() {
            unsafe {
                self.message.get_mut().assume_init_drop()
            }
        }
    }
}