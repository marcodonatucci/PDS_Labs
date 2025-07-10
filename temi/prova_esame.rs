use std::{sync::{Mutex, Arc, Condvar}, thread, time::Duration};

pub struct CountDownLatch {
    counter: Mutex<usize>, // contatore protetto da mutex per essere condiviso in più thread (lo chiudiamo in un Arc)
    cv: Condvar // condvar per la notify di wait_zero
}


impl CountDownLatch {
    pub fn new(n: usize) -> Self {
        Self {
            counter: Mutex::new(n),
            cv: Condvar::new()
        }
    }
    // wait zero aspetta al massimo timeout ms
    // se esce per timeout ritorna Err altrimenti Ok

    pub fn wait_zero(&self, timeout: Option<std::time::Duration>) -> Result<(),()> {

        let mut count = self.counter.lock().unwrap(); // 1) let mut MutexGuard 

        while *count > 0 { // 2) ciclo per controllare la variabile (deref), per notifiche spurie

            if let Some(duration) = timeout  { // se c'è un timeout impostato definisco un WaitTimeoutResult
                let to_result;
                (count, to_result) = self.cv.wait_timeout(count, duration).unwrap(); // riottengo il guard e il result
                if to_result.timed_out() {
                    return Err(()); // se è timedout ritorna Err
                }
            } else { 
                count = self.cv.wait(count).unwrap(); // 3) la wait rilascia il lock, mette in attesa e alla notify ritorna il 
                                                            // mutexguard con il nuovo valore
            } 
        }
        Ok(())
    }

    pub fn count_down(&self) {
        let mut counter = self.counter.lock().unwrap(); // lock mutabile sul counter
        *counter -= 1; // diminuisco di uno (deref per accedere al valore del mutexguard)
        self.cv.notify_all(); // la cv fa notify_all, sta ai thread in wait controllare che la condizione sia verificata (notifica spuria)
 
    }
}

pub fn doSomeWork(description: &str) {
    thread::sleep(Duration::from_millis(100));
    println!("Working on {}", description);
    thread::sleep(Duration::from_millis(100));
}

pub fn demo_latch() {
    let mut handles = vec![];

    let driver_ready = Arc::new(CountDownLatch::new(1)); // non sono mut, uno prepara il driver l'altro lo rilascia (10 volte)
    let driver_release = Arc::new(CountDownLatch::new(10));
     
    for _ in 0..10 {
        let driver_ready = Arc::clone(&driver_ready); // gli arc si clonano sempre prima di usarli (con ref originale)
        let driver_release = Arc::clone(&driver_release);

        let h = thread::spawn(move ||{ // move !
            driver_ready.wait_zero(None).unwrap(); // aspetto che sia pronto il driver
            doSomeWork("(2) lavoro che necessita driver");
            driver_release.count_down(); // rilascio un driver
            doSomeWork("(3) altro lavoro che non necessita driver");
        });
        handles.push(h);
    }

    doSomeWork("(1) prepapara il driver");
    driver_ready.count_down(); // il driver è pronto
    doSomeWork("(4) rilascia il driver");
    driver_release.wait_zero(None).unwrap();

    for h in handles {
        let _ = h.join();
    }

}
    

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_count_down_latch() {
    demo_latch();
}

