use std::{sync::{ Condvar, Mutex}, time::{Duration, Instant}};


pub struct DelayedQueue<T:Send> {

    queue: Mutex<Vec<(T, Instant)>>,
    cv: Condvar,

}

impl<T: Send> DelayedQueue<T> {

    pub fn new() -> Self {
        Self {
            queue: Mutex::new(Vec::new()),
            cv: Condvar::new(),
        }
    }
    
    pub fn offer(&self, t:T, i: Instant) {
        let mut queue = self.queue.lock().unwrap();
        queue.push((t,i)); // aggiungo l'elemento
        queue.sort_by(|a, b|{a.1.cmp(&b.1)}); // sort per tempo in ordine crescente

        self.cv.notify_all(); // avviso

    }

    pub fn take(&self) -> Option<T> {

        let mut queue = self.queue.lock().unwrap();

        loop { // metto in un loop per la wait 

            let now = Instant::now(); 
            if queue.is_empty() { // se è vuota è none 
                return None;
            }

            let first = queue.first().unwrap();

            let timeout = Duration::from(first.1 - now); // prendo il primo e calcolo il timeout 

            if first.1 >= now {
                
                let element = queue.remove(0); // se è passato lo rimuovo e lo ritorno 
                return Some(element.0);

            } 
            let result = self.cv.wait_timeout(queue,timeout).unwrap();
            queue = result.0; // altrimenti aspetto fino al timeout o inserimento di nuovo record e si ripete
             
        }
    }

    pub fn size(&self) -> usize {
        
        let queue = self.queue.lock().unwrap();

        queue.len() as usize // ritorno la dimensione


    }

}
 

fn main() {
    println!("Hello, world!");
}
