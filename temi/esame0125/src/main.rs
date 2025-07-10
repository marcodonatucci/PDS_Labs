// domanda 1
//
// Viene creta un istanza della struct Modulo che implementa il tratto Calculate.
// Viene in seguito definita la variabile tobj che ha come tipo il riferimento a un oggetto che implementa il tratto Calculate.
// Alla variabile viene assegnato il riferimento all'istanza del modulo e viene utilizzata in seguito la funzione add
// implementata dal tratto Calculate. Il risultato della add è salvato in result e viene restituito tramite una println
//
// l'occupazione in memoria è: 
// 8 byte per mod3 su stack,
// 16 byte per tobj su stack -> 8 data ptr + 8 vtable ptr
// 8 byte per result su stack

// domanda 2
// la funzione lambda increment_n prende un riferimento mutabile a count per incrementarlo,
// avviene un errore di compilazione nel momento in cui la println subito dopo la increment
// tenta di richiedere il riferimetno in lettura a count per stamparlo ma count ha ancora un riferimento mutabile 
// attivo. Un modo per correggere questo errore e mantenere il comportamento richiesto sarebbe
// spostare la print direttamente all'interno della lambda

// domanda 3
// Viene definita una variabile mut a che viene mossa all'interno della lambda count_a.
// questa funzione incrementa a e la stampa. 
// a viene inizializzata a 0 e la closure attraverso la keywork move prende possesso di una copia locale di a,
// poi il suo valore viene cambiato a 2 ma questo vale solo per la a relativa al main, non a quella definita nello stato della closure.
// la funzione call_x incrementa a di 2 due volte quindi viene stampato 1 2 1 2 visto che anche la closure implementa il tratto copy (le sue variabili lo implementano)
// la print a riga 12 fa riferimento alla a esterna alla closure quindi stamperà 2. 
// dopo l'ultima call_x lo stato è ancora a 0 e viene stampato 1 2 3 4, e l'ultima print si riferisce sempre alla
// a definita nel main quindi 2.
// la closure viene copiata ad ogni invocazione della funzione call_x

use std::{sync::{atomic::AtomicBool, Arc, Condvar, Mutex}, thread::{self, JoinHandle}, time::{Duration, Instant}};


struct DelayedExecutor<F: FnOnce()+Send+'static> {
    buffer: Arc<(Mutex<Vec<(F, Instant)>>, Condvar)>, // vec che contiene il tipo F (tupla con instant per controllare il tempo), in un mutex per accesso condiviso e condvar per la wait
    is_open: AtomicBool, // tipo bool atomico per segnalare la chiusura 
    jh: Option<JoinHandle<()>> // joinhandle del thread che andrà ad eseguire le task
}

impl<F: FnOnce()+Send+'static> DelayedExecutor<F> {

    pub fn new() -> Self {
        
        let buffer = Arc::new((Mutex::new(Vec::<(F, Instant)>::new()), Condvar::new()));
        let is_open = true.into(); // atomic, non va clonato!
        let buffer_c = Arc::clone(&buffer); // clono la lista di task 

        let jh = thread::spawn(move || { // spawn del thread che esegue le funzioni
            loop { // il thread deve continuare ad eseguire la funzione finchè viene chiuso
                let (lock, cv) = &*buffer_c;
                let mut buffer = lock.lock().unwrap();

                while  buffer.len() == 0 && is_open {
                    buffer = cv.wait(buffer).unwrap(); // aspetto finchè avro un elemento nella lista e il canale sarà apeto
                }

                if !is_open {
                    break; // se è stato chiuso fermo il loop ed esco dal thread
                }

                let (task, i) = buffer.last().unwrap(); // prendo il primo elemento

                if *i < Instant::now() {

                    let (real_task, real_i) = buffer.pop().unwrap();

                    drop(buffer); // rilascio il lock per non tenerlo troppo tempo

                    real_task(); // eseguo la task

                    buffer = lock.lock().unwrap(); // riacquisisco il lock

                } else {
                    let d = i.duration_since(Instant::now()); // calcolo la durata da aspettare
                    let to_result; 
                    (buffer, to_result) = cv.wait_timeout(buffer, d).unwrap(); // aspetto fino a timeout o finchè non ci sarà una task più urgente

                }


            }});

        Self { buffer: buffer, is_open: is_open.into(), jh: Some(jh) }
    }

    pub fn execute(&mut self, f:F, delay: Duration) -> bool {
        let (lock, cv) = &*self.buffer;
        let mut buffer = lock.lock().unwrap();

        if !*self.is_open.get_mut() {
            return false;
        }

        let i = Instant::from(Instant::now() + delay);

        buffer.push((f, i)); // push della nuova task

        buffer.sort_by(|a, b| {b.1.cmp(&a.1)}); // sort in base alle i

        cv.notify_one(); // notifica

        return true;

    }

    pub fn close(&mut self, drop_pending_tasks: bool) {

        let (lock, cv) = &*self.buffer;
        let mut buffer = lock.lock().unwrap();

        self.is_open = false.into();

        if drop_pending_tasks {
            buffer.clear();
        }

        cv.notify_one();
       
    }
}

impl<F: FnOnce()+Send+'static> Drop for DelayedExecutor<F> {
    fn drop(&mut self) {
        self.close(true); // chiudo tutto e pulisco il vec
        self.jh.take().unwrap().join().unwrap(); // aspetto che venga eseguito tutto
    }
}

pub fn main() {

}
