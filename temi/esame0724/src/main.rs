use std::thread;
use std::time::Duration;
use std::sync::{Arc, Condvar, Mutex, WaitTimeoutResult};



struct CountDownLock {

    counter: Mutex<usize>,
    cvar: Condvar,

}

impl CountDownLock {

    pub fn new(n: usize) -> Self {
        Self {
            counter: Mutex::new(n),
            cvar: Condvar::new(),
        }
    }

    pub fn count_down(&self) {

        let mut lock = self.counter.lock().unwrap();

        if *lock > 0 {
            *lock -= 1;
            self.cvar.notify_all();
        }
       
    }

    pub fn wait(&self) {

        let mut lock = self.counter.lock().unwrap();

        while *lock > 0 {
            lock = self.cvar.wait(lock).unwrap();
        }
    }

    pub fn wait_timeout(&self, d: Duration) -> WaitTimeoutResult {

        let mut lock = self.counter.lock().unwrap();
        let to_result;

        (lock, to_result) = self.cvar.wait_timeout_while(lock, d, |lock| {*lock > 0}).unwrap();

        to_result
    }

}

#[test]
pub fn test() {

    let cd_lock = Arc::new(CountDownLock::new(2));
    let cd_lock1 = Arc::clone(&cd_lock);
    let cd_lock2 = Arc::clone(&cd_lock);

    let h1 = thread::spawn(move || {
        cd_lock1.count_down();
        println!("thread 1 count down, actual value: {}", cd_lock1.counter.lock().unwrap());
        let result = cd_lock1.wait_timeout(Duration::from_millis(10));
        println!("Thread 1: wait_timeout result -> timed out: {}, value: {}",
            result.timed_out(),
            cd_lock1.counter.lock().unwrap());
    });

    let h2 = thread::spawn(move || {
        cd_lock2.count_down();
        println!("thread 2 count down, actual value: {}", cd_lock2.counter.lock().unwrap());
    });

    h1.join().unwrap();
    h2.join().unwrap();

    println!("CountDownLock final value: {}", cd_lock.counter.lock().unwrap());

}


fn main() {
    println!("Hello, world!");
}
