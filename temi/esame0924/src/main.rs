// Domanda 1 
// prendo un borrow della string2 nella funzione fun1, che mi servirà per 
// calcolare la size della slice ritornata attraverso la sua len, il problema
// è che non vive abbastanza perchè viene rilasciata alla fine dello scope in cui 
// è definita, non permettendo l'utilizzo di result nella println
// si èuò correggere spostando la print nello scope di definizione o rimuovendolo
// stessa cosa per la seconda funzione

// Domanda 2
// L'errore riguarda la condivisione su più thread del mutex n, non è possibile
// condividerlo in quanto non implementa il tratto copy, quindi alla prima iterazione 
// il thread cerca di prenderne il possesso ma essendo heap allocated quando il secondo thread
// tenta di accedere non può perchè è già posseduto da un altro. La soluzione è implementare un 
// Arc, clonarlo prima di ogni spawn e passarne il riferimento al thread con la keyword move

// Domanda 3
// Rc inizializzato a 5, stampato, poi clonato e stampato (5,5).
// si prova a prendere un riferiment mutabile ma nello scope lo strong count è 2,
// perchè c'è anche copia, quindi non viene concesso e stampa wrong A.
// quando finisce lo scope e viene droppata copia si aumenta il valore di 10 e stampa 15

use std::time::Duration;
use std::sync::{Arc, Mutex, Condvar};

#[derive(PartialEq, Eq, Debug)]
pub enum WaitResult {
    Success,
    Timeout,
    Canceled
}

 struct LatchState { // contiene le informazioni sullo stato da utilizzare con la condvar
    count: usize,
    canceled: bool,
}

pub struct Latch {
    state: Arc<(Mutex<LatchState>, Condvar)>, // lo stato è condiviso con Arc e viene gestito tramite condvar 
}

pub trait CancelableLatch { // creo il tratto
    fn new(count: usize) -> Self;
    fn count_down(&self);
    fn cancel(&self);
    fn wait(&self) -> WaitResult;
    fn wait_timeout(&self, d: Duration) -> WaitResult;
}

impl CancelableLatch for  Latch { // implemento il tratto per la struct
    fn new(count: usize) -> Self {
        Self {
            state: Arc::new((
                Mutex::new(LatchState {
                    count,
                    canceled: false,
                }),
                Condvar::new(),
            )),
        }
    }

    fn count_down(&self) {
        let (lock, cvar) = &*self.state; // prendo lock e condvar con una tupla (deref da arc)
        let mut c = lock.lock().unwrap(); 

        if c.count > 0 {
            c.count -= 1; // decremento count se è > 0
            if c.count == 0 {
                cvar.notify_all(); // se count è zero notifico tutti
            }
        } 

    }

    fn cancel(&self) {
        let (lock, cvar) = &*self.state; // prendo lock e condvar con una tupla (deref da arc)
        let mut state = lock.lock().unwrap(); 
        state.canceled = true; // set dello stato e notifica a tutti
        cvar.notify_all();
    }

    fn wait(&self) -> WaitResult {
        let (lock, cvar) = &*self.state; // prendo lock e condvar con una tupla (deref da arc)
        let mut state = lock.lock().unwrap(); 
        while state.count > 0 && !state.canceled {
            state = cvar.wait(state).unwrap();
        }
        
        if state.canceled {
            WaitResult::Canceled
        } else {
            WaitResult::Success
        }
    }

    fn wait_timeout(&self, d:Duration) -> WaitResult {
        let (lock, cvar) = &*self.state; // prendo lock e condvar con una tupla (deref da arc)
        let mut state = lock.lock().unwrap(); 
        while state.count > 0 && !state.canceled {

            let to_result; // definisci sempre il waitto
            (state, to_result) = cvar.wait_timeout(state, d).unwrap(); // tupla guard + to 
            
            if to_result.timed_out() && !state.canceled && state.count > 0 {
                return WaitResult::Timeout;
            }
        }

        if state.canceled {
            WaitResult::Canceled
        } else {
            WaitResult::Success
        }
        
    }
}

pub fn main() {}
