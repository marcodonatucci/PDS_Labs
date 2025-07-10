use std::result::{Result};
use std::sync::mpsc::{channel, Receiver, SendError, Sender};
use std::sync::Mutex;


pub struct MultiChannel { // lista di senders... (nel mutex!)
    senders: Mutex<Vec<Sender<u8>>>,
}

impl MultiChannel {

    pub fn new() -> Self {
        Self { 
            senders: Mutex::new(Vec::<Sender<u8>>::new()),
        }
    }
    // crea un nuovo canale senza alcun ricevitore collegato

    pub fn subscribe(&self) -> Receiver<u8> {
        // si crea un canale con channel() e ritorna una tupla di sender e receiver
        let (tx, rx): (Sender<u8>, Receiver<u8>) = channel(); 
        let mut senders = self.senders.lock().unwrap();

        senders.push(tx); // aggiungo il sender al canale

        rx // ritorno il receiver

    }
    // collega un nuovo ricevitore al canale: da quando
    // questo metodo viene invocato, gli eventuali byte
    // inviati al canale saranno recapitati al ricevitore.
    // Se il ricevitore viene eliminato, il canale
    // continuerà a funzionare inviando i propri dati
    // ai ricevitori restanti (se presenti), altrimenti
    // ritornerà un errore

    pub fn send(&self, data: u8) -> Result<(), SendError<u8>> {

        let senders = self.senders.lock().unwrap();

        if senders.len() == 0 { // se è vuoto ritorno l'errore (meglio is_empty()) !!!
            return Err(SendError(data));
        } else {
        
        for s in senders.iter() { // con iter guardo i riferimenti e non prendo possesso !!! 
            let result = s.send(data); // metodo per inoltrare i dati
        }

        Ok(()) 
    }

    // non rimuovi i sender che hanno il receiver eliminato (quelli che danno Error al send): 

    // guard.retain(|s|s.send(data).is_ok());

    // mantiene nella lista solo quelli per cui il send è andato bene !!! 
        
    }
    // invia a tutti i sottoscrittori un byte
    // se non c'è alcun sottoscrittore, notifica l'errore
    // indicando il byte che non è stato trasmesso

}

#[test]
pub fn test() {

    let channel = MultiChannel::new();
    let r1 = channel.subscribe();
    let r2 = channel.subscribe();
    let r3 = channel.subscribe();

    let result1 = channel.send(4);

    assert_eq!([4,4,4], [r1.recv().unwrap() as i32, r2.recv().unwrap() as i32, r3.recv().unwrap() as i32]);
    assert_eq!(Ok(()), result1);

    println!("channel len: {}", channel.senders.lock().unwrap().len());

    drop(r1);

    let result2 = channel.send(2);

    assert_eq!([2, 2], [r2.recv().unwrap() as i32, r3.recv().unwrap() as i32]);
    assert_eq!(Ok(()), result2);

    println!("channel len: {}", channel.senders.lock().unwrap().len());

}
fn main() {
    println!("Hello, world!");
}
