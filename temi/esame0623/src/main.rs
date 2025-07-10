use std::{collections::{VecDeque}, sync::{Condvar, Mutex}};

#[derive(PartialEq)]
enum State {
    Open, 
    Closed,
}

// scrivono molti produttori e leggono molti consumatori
pub struct MpMcChannel<E: Send> {

    buffer: Mutex<(State, VecDeque<E>)>,
    size: usize,
    cv: Condvar,

}

impl<E: Send> MpMcChannel<E> {
    
    pub fn new(n: usize) -> Self {

        Self {
            buffer: Mutex::new((State::Open, VecDeque::with_capacity(n))),
            size: n,
            cv: Condvar::new(),
        }

    }
    //crea una istanza del canale basato su un buffer circolare di "n" elementi

    pub fn send(&self, e: E) -> Option<()>  {

        let mut try_lock = self.buffer.lock();
        if try_lock.is_err() {return None} // gestisce errori interni al metodo
        let mut lock = try_lock.unwrap();

        // aspetto finchè il canale è aperto e il vec è pieno (lunghezza uguale a size)
        try_lock = self.cv.wait_while(lock, |l|{l.0 == State::Open && l.1.len() == self.size});

        if try_lock.is_err() {return None} // gestisce errori interni al metodo

        lock = try_lock.unwrap();

        if lock.0 == State::Closed {return None} // il canale è stato chiuso

        lock.1.push_back(e); // pusho l'elemento in coda

        self.cv.notify_all();

        Some(())

    }
    //invia l'elemento "e" sul canale. Se il buffer circolare è pieno, attende
    //senza consumare CPU che si crei almeno un posto libero in cui depositare il valore
    //Ritorna:
    // - Some(()) se è stato possibile inserire il valore nel buffer circolare
    // - None se il canale è stato chiuso (Attenzione: la chiusura può avvenire anche
    // mentre si è in attesa che si liberi spazio) o se si è verificato un errore interno

    pub fn recv(&self) -> Option<E> {

        let mut try_lock = self.buffer.lock();
        if try_lock.is_err() {return None} // gestisce errori interni al metodo
        let mut lock = try_lock.unwrap();

        // aspetto finchè il canale è aperto e il vec è vuoto 
        try_lock = self.cv.wait_while(lock, |l|{l.0 == State::Open && l.1.len() == 0});

        if try_lock.is_err() {return None} // gestisce errori interni al metodo

        lock = try_lock.unwrap();

        if lock.0 == State::Closed && lock.1.len() == 0 {return None} // il canale è stato chiuso ed è vuoto, sennò posso continuare a ritornare gli elementi

        let result = lock.1.pop_front().unwrap(); // pusho l'elemento in coda

        self.cv.notify_all(); // avviso che si è liberato un posto

        Some(result)

    }
    //legge il prossimo elemento presente sul canale. Se il buffer circolare è vuoto,
    //attende senza consumare CPU che venga depositato almeno un valore
    //Ritorna:
    // - Some(e) se è stato possibile prelevare un valore dal buffer
    // - None se il canale è stato chiuso (Attenzione: se, all'atto della chiusura sono
    // già presenti valori nel buffer, questi devono essere ritornati, prima di indicare
    // che il buffer è stato chiuso; se la chiusura avviene mentre si è in attesa di un valore,
    // l'attesa si sblocca e viene ritornato None) o se si è verificato un errore interno.

    pub fn shutdown(&self) -> Option<()> {

        let try_lock = self.buffer.lock();
        if try_lock.is_err() {return None} // gestisce errori interni al metodo
        let mut lock = try_lock.unwrap();

        lock.0 = State::Closed;

        self.cv.notify_all(); // avviso che si è liberato un posto

        Some(())

    }
    //chiude il canale, impedendo ulteriori invii di valori.
    //Ritorna:
    // - Some(()) per indicare la corretta chiusura
    // - None in caso di errore interno all'implementazione del metodo.
    
}



fn main() {
    println!("Hello, world!");
}
