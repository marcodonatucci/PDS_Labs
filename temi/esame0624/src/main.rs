use std::{sync::{Arc, Condvar, Mutex}, thread, time::Duration};


struct ExchangeState<T> { // stato da scambiare tra i due thread
    value: Option<T>, // valore come option
    has_value: bool, // flag valore
    partner_dropped: bool, // flag dropped
}

struct Exchanger<T: Send> {
    state: Arc<(Mutex<ExchangeState<T>>, Condvar)>, // stato 1
    partner_state: Arc<(Mutex<ExchangeState<T>>, Condvar)>, // stato 2 
}

impl<T: Send> Exchanger<T> {

    pub fn new() -> (Self, Self) { // ritorno due exchamge bindati tra di loro
        let state1 = Arc::new(
            (Mutex::new(
                ExchangeState {
                    value: None,
                    has_value: false,
                    partner_dropped: false,
                }), Condvar::new()));
        
        let state2 = Arc::new(
            (Mutex::new(
                ExchangeState {
                    value: None,
                    has_value: false,
                    partner_dropped: false,
                }), Condvar::new()));

        (
            Exchanger{
                state: Arc::clone(&state1), // uno ha il suo stato e il partner e il secondo è scambiato (prima i clone)
                partner_state: Arc::clone(&state2),
            },
            Exchanger{
                state: state2,
                partner_state: state1,
            }
        )
    }

    fn exchange(&self, t:T) -> Option<T> {
        { 
            let (lock, cv) = &*self.state;
            let mut state = lock.lock().unwrap(); // acquisisco il mio stato

            state.value = Some(t); // imposto il valore
            state.has_value = true;
            cv.notify_one(); // notifico il bro 
        } // droppo così l'altro può acquisirlo

        let (partner_lock, partner_cv) = &*self.partner_state;
        let mut partner_state = partner_lock.lock().unwrap(); // acquisisco lo stato del partner

        while !partner_state.has_value && !partner_state.partner_dropped { // se non ha valore o non è droppato lo aspetto 
            partner_state = partner_cv.wait(partner_state).unwrap();
        }

        if partner_state.partner_dropped { // se è droppato ritorno None
            None
        } else {
            let result = partner_state.value.take(); // sennò prendo il valore e lo ritorno 
            partner_state.has_value = false;
            result
        }
    }

}


impl<T: Send> Drop for Exchanger<T> { // implemento il drop 
    fn drop(&mut self) {
        let (lock, cv) = &*self.state;
        let mut state  = lock.lock().unwrap(); // acquisisco il mio stato
        state.value = None; // azzero tutto e imposto la flag dropped
        state.has_value = false;
        state.partner_dropped = true;
        cv.notify_one(); // notifico in caso mi stiano aspettando 
    } 
}

fn main() {

}

#[test]
pub fn test1 () {
    let (a, b) = Exchanger::new();

    // Thread A: effettua lo scambio
    let handle_a = thread::spawn(move || {
        let val = a.exchange("ciao");
        println!("Thread A received: {:?}", val);
        val 
    });

    // Thread B: termina subito (drop)
    drop(b); // Simuliamo un partner che si chiude prematuramente

    let a_val = handle_a.join().unwrap();

    assert_eq!(a_val, None);
}

// Domanda 1
// stampa (c8) e 3
// la riga 5 crea un iteratore sui riferimenti degli elementi del vettore, 
// la riga 6 applica un filter quindi accetta solo gli elementi che soddisfano la lambda (numeri pari),
// la riga 7 mappa gli elementi come tupla composta dal valore del vettore come primo elemento e valore 
// del vettore della zip come secondo elemento.
// se si omette il clone si ottiene un errore di compilazione alla riga 15 perchè 
// si cerca di utilizzare res che è stato mosso in precedenza nell'istruzione let last = res....

// Domanda 2
// Viene creato un pair Mutex e condvar all'interno di un Arc e clonato. 
// il clone viene mosso all'interno di un thread tramite la keyword move nella spawn, 
// e viene acquisito il lock al suo mutex. La variabile started viene impostata a true e 
// inviata una notifica. Il thread principale nel mentre fa una sleep di un secondo, prende i lock
// del mutex dell'altro pair, e aspetta che il valore del mutex diventi true (cvar.wait).
// il problema è che non c'è garanzia che venga eseguito il thread che invia la notifica in quanto l'istruzione 
// di join è mancante, quindi il thread principale aspetterà un tempo indefinito. Inoltre la wait non fa alcun 
// controllo sulla condizione da aspettare quindi anche in caso di join si aspetterebbe all'infinito
// Correzione: let handle = thread::spawn... , while !*started { wait }, handle.join().unwrap().

// Domanda 3
// L'errore è nella v.push(s) in quanto viene passata più volte la variabile s alla push,
// la prima volta ne prende possesso quindi dalla seconda iterazione in poi s non è più accessibile.
// La soluzione sarebbe implementare il tratto Clone per la struct S e sostituire con v.push(s.clone());