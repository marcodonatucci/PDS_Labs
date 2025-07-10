// Il pattern Producer/Consumer è un paradigma di programmazione concorrente dove:
// - Uno o più producer generano dati e li inseriscono in un buffer condiviso
// - Uno o più consumer prelevano e processano questi dati dal buffer
// 
// Concetti chiave implementati:
// 1. Sincronizzazione: gestita tramite Mutex e Condition Variables
// 2. Backpressure: il producer si blocca quando il buffer è pieno
// 3. Efficienza: il consumer si blocca quando il buffer è vuoto
// 4. Terminazione: gestita tramite il segnale Stop

use std::collections::VecDeque;
use std::sync::{Arc, Mutex, Condvar};
use std::time::Duration;
use rand::Rng;
use std::thread;

// Enum che rappresenta gli elementi nel canale:
// - Value(T): contiene un valore effettivo
// - Stop: segnale di terminazione
#[derive(Debug)]
pub enum Item<T> {
    Value(T),
    Stop,
}

// Struttura principale del canale che contiene:
// - buffer: VecDeque per implementare la coda FIFO
// - capacity: limita la dimensione del buffer (backpressure)
// - is_closed: flag per gestire la terminazione
pub struct MyChannel<T> {
    // Arc permette di condividere in modo thread-safe:
    // - Il buffer protetto da Mutex
    // - Due Condition Variables per la sincronizzazione
    buffer: Arc<(Mutex<VecDeque<Item<T>>>, Condvar, Condvar)>,
    capacity: usize,
    is_closed: Arc<Mutex<bool>>,
}

impl<T> MyChannel<T> {
    pub fn new(capacity: usize) -> Self {
        MyChannel {
            buffer: Arc::new((
                Mutex::new(VecDeque::with_capacity(capacity)),
                Condvar::new(), // not_full: sveglia i producer quando c'è spazio
                Condvar::new(), // not_empty: sveglia i consumer quando ci sono dati
            )),
            capacity,
            is_closed: Arc::new(Mutex::new(false)),
        }
    }

    // write() implementa la logica del producer:
    // 1. Verifica se il canale è chiuso
    // 2. Se il buffer è pieno, si blocca sulla condition variable not_full
    // 3. Inserisce il valore e notifica i consumer
    pub fn write(&self, value: T) -> Result<(), &'static str> {
        let (buffer, not_full, not_empty) = &*self.buffer;
        let mut buffer_guard = buffer.lock().unwrap();
        
        // Check if channel is closed
        if *self.is_closed.lock().unwrap() {
            return Err("Channel is closed");
        }

        // Implementazione della backpressure:
        // Il thread si blocca se il buffer è pieno
        while buffer_guard.len() >= self.capacity {
            buffer_guard = not_full.wait(buffer_guard).unwrap();
            if *self.is_closed.lock().unwrap() {
                return Err("Channel is closed");
            }
        }

        buffer_guard.push_back(Item::Value(value));
        not_empty.notify_one(); // Sveglia un consumer in attesa
        Ok(())
    }

    // read() implementa la logica del consumer:
    // 1. Se trova un valore, lo restituisce e notifica i producer
    // 2. Se trova Stop, lo rimette nel buffer e segnala la terminazione
    // 3. Se il buffer è vuoto, si blocca sulla condition variable not_empty
    pub fn read(&self) -> Result<T, &'static str> {
        let (buffer, not_full, not_empty) = &*self.buffer;
        let mut buffer_guard = buffer.lock().unwrap();

        loop {
            match buffer_guard.pop_front() {
                Some(Item::Value(value)) => {
                    not_full.notify_one(); // Sveglia un producer in attesa
                    return Ok(value);
                }
                Some(Item::Stop) => {
                    // Rimette Stop nel buffer per altri consumer
                    buffer_guard.push_front(Item::Stop);
                    return Err("Channel is stopped");
                }
                None => {
                    // Se il buffer è vuoto e il canale è chiuso, termina
                    if *self.is_closed.lock().unwrap() && buffer_guard.is_empty() {
                        return Err("Channel is closed and empty");
                    }
                    // Altrimenti si blocca in attesa di nuovi dati
                    buffer_guard = not_empty.wait(buffer_guard).unwrap();
                }
            }
        }
    }

    // close() gestisce la terminazione ordinata:
    // 1. Marca il canale come chiuso
    // 2. Inserisce il segnale Stop
    // 3. Sveglia tutti i consumer in attesa
    pub fn close(&self) {
        let (buffer, _, not_empty) = &*self.buffer;
        let mut buffer_guard = buffer.lock().unwrap();
        *self.is_closed.lock().unwrap() = true;
        buffer_guard.push_back(Item::Stop);
        not_empty.notify_all(); // Sveglia tutti i consumer
    }
}

// Il main dimostra l'uso del canale con:
// - Un producer che genera valori con delay casuali
// - Un consumer che li processa con delay fisso
// - Corretta gestione della terminazione
fn main() {
    let channel = Arc::new(MyChannel::new(5)); // Buffer di dimensione 5
    let N = 20; // Numero di valori da produrre

    // Clone per il producer
    let producer_channel = channel.clone();
    
    // Thread producer
    let producer = thread::spawn(move || {
        let mut rng = rand::thread_rng();
        
        for i in 0..N {
            // Simula lavoro random
            thread::sleep(Duration::from_millis(rng.gen_range(100..500)));
            
            match producer_channel.write(i) {
                Ok(_) => println!("Produced: {}", i),
                Err(e) => {
                    println!("Producer error: {}", e);
                    break;
                }
            }
        }
        
        println!("Producer closing channel");
        producer_channel.close();
    });

    // Thread consumer
    let consumer = thread::spawn(move || {
        loop {
            match channel.read() {
                Ok(value) => println!("Consumed: {}", value),
                Err(e) => {
                    println!("Consumer finished: {}", e);
                    break;
                }
            }
            
            // Simula processamento
            thread::sleep(Duration::from_millis(200));
        }
    });

    // Attendi che entrambi i thread terminino
    producer.join().unwrap();
    consumer.join().unwrap();
    
    println!("All done!");
}
