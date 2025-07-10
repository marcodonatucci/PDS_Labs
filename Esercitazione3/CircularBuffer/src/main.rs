use std::ops::{Deref, DerefMut, Index, IndexMut};
use crate::BufferError::Full;

#[derive(Debug)]
enum BufferError {
    Full,
    Empty,
}
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct CircularBuffer<T> { // Struct per il buffer circolare GENERICA
    buffer: Vec<T>,
    head: usize,
    tail: usize,
    size: usize,
    capacity: usize,
}

impl<T: Clone + Default> CircularBuffer<T> { // implementazione generica

    pub fn new(capacity: usize) -> Self {
        CircularBuffer {
            buffer: vec![Default::default(); capacity], // inizializza il buffer con valori di default
            head: 0,
            tail: 0,
            size: 0,
            capacity,
        }
    }

    pub fn write(&mut self, item: T) -> Result<(), BufferError> {
        if self.size == self.capacity {
            return Err(Full); // buffer pieno
        }
        self.buffer[self.head] = item;
        self.head = (self.head + 1) % self.capacity; // resto della divisione, se head raggiunge capacity torna a 0
        self.size += 1;
        Ok(())
    }
    pub fn read(&mut self) -> Option<T> {
        if self.size == 0 {
            return None; // buffer vuoto
        }
        let item = self.buffer[self.tail].clone(); // clona l'elemento corrente (non posso usare il riferimento senza Copy)
        self.tail = (self.tail + 1) % self.capacity; // resto della divisione, se tail raggiunge capacity torna a 0
        self.size -= 1; // decremento la dimensione
        Some(item) // restituisco l'elemento letto
    }
    pub fn clear(&mut self) {
        self.head = 0;
        self.tail = 0;
        self.size = 0;
    }
    pub fn size(&self) -> usize{
        self.size
    }
    // può essere usata quando il buffer è pieno per forzare una
    // scrittura riscrivendo l’elemento più vecchio (leggo e scrivo)
    pub fn overwrite(&mut self, item: T) {
        if self.size == self.capacity {
            self.tail = (self.tail + 1) % self.capacity; // se il buffer è pieno, sovrascrivo l'elemento più vecchio
        } else {
            self.size += 1; // altrimenti incremento la dimensione
        }
        self.buffer[self.head] = item;
        self.head = (self.head + 1) % self.capacity; // resto della divisione, se head raggiunge capacity torna a 0
    }
    // vedi sotto*
    pub fn make_contiguous(&mut self) {
        if self.tail < self.head { // Se tail è minore di head, il buffer è già contiguo
            return; // non è necessario fare nulla
        }
        let mut new_buffer = vec![Default::default(); self.size]; // Crea un nuovo buffer con la dimensione attuale
        let mut index = 0; // Inizializza l'indice a 0
        for i in self.tail..self.capacity { // Copia gli elementi dal tail alla fine del buffer
            new_buffer[index] = self.buffer[i].clone(); // Copia l'elemento corrente nel nuovo buffer
            index += 1; // Incrementa l'indice
        }
        for i in 0..self.head { // Copia gli elementi dall'inizio del buffer fino a head
            new_buffer[index] = self.buffer[i].clone(); // Copia l'elemento corrente nel nuovo buffer
            index += 1; // Incrementa l'indice
        }
        self.buffer = new_buffer; // Sostituisce il buffer originale con il nuovo buffer
        self.head = self.size; // Imposta head alla dimensione attuale
        self.tail = 0; // Imposta tail a 0
    } // spezzo il buffer in due, la parte contigua è quella che va da tail a head, quindi piazzo
    // la metà che va da tail a capacity all'inizio del nuovo buffer e la metà che va da 0 a head alla fine
}

impl<T> Index<usize> for CircularBuffer<T> { // permette di fare indexing
    type Output = T; // attento all'output

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.size { // se l'indice è maggiore della dimensione del buffer
            panic!("Index out of bounds");
        }
        // buff[0] legge l'elemento in teta
        let real_index = (self.head + index + 1) % self.capacity; // calcolo l'indice reale
        &self.buffer[real_index]
    }
}


impl<T> IndexMut<usize> for CircularBuffer<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.size {
            panic!("Index out of bounds");
        }
        let real_index = (self.head + index + 1) % self.capacity;
        &mut self.buffer[real_index] // mutabile!
    }
}


impl<T> Deref for CircularBuffer<T> { // permette di dereferenziare il buffer a uno slice di tipo T
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        if self.size == 0 { // buffer vuoto
            &[]
        } else if self.head < self.tail {
            // dati contigui, possiamo usare una slice diretta
            &self.buffer[(self.head + 1)..self.tail]
        } else if self.tail == 0 {
            // wrap-around completo, ma dati contigui da head+1 a fine
            &self.buffer[(self.head + 1)..]
        } else {
            panic!("Buffer is not contiguous, cannot deref");
        }
    }
}

impl<T> DerefMut for CircularBuffer<T> {

    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.size == 0 { // buffer vuoto
            &mut []
        } else if self.head < self.tail {
            // dati contigui, possiamo usare una slice diretta
            &mut self.buffer[(self.head + 1)..self.tail]
        } else if self.tail == 0 {
            // wrap-around completo, ma dati contigui da head+1 a fine
            &mut self.buffer[(self.head + 1)..]
        } else {
            panic!("Buffer is not contiguous, cannot deref");
        }
    }
}

fn main() {
    let mut buffer = CircularBuffer::new(3);
    buffer.write(1).unwrap();
    buffer.write(2).unwrap();
    buffer.write(3).unwrap();

    {
        // Otteniamo un riferimento immutabile al buffer come slice
        let slice = &buffer[0];
        // Stampa il contenuto del slice
        println!("{:?}", slice);
    } // Il riferimento immutabile `slice` esce dal suo scope qui

    // Ora possiamo modificare il buffer in modo sicuro
    buffer.write(4).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use cnumbers::solution::ComplexNumber; // package cnumbers, pub mod solution, dependency ../cnumbers, workspace nella directory più alta

    #[test]
    fn test_insert_and_check_size() {
        let mut buffer = CircularBuffer::new(3);
        buffer.write(1).unwrap();
        assert_eq!(buffer.size(), 1);
    }

    #[test]
    fn test_insert_and_read() {
        let mut buffer = CircularBuffer::new(3);
        buffer.write(1).unwrap();
        assert_eq!(buffer.read(), Some(1));
    }

    #[test]
    fn test_insert_and_read_multiple() {
        let mut buffer = CircularBuffer::new(3);
        for i in 1..=3 {
            buffer.write(i).unwrap();
        }
        for i in 1..=3 {
            assert_eq!(buffer.read(), Some(i));
        }
    }

    #[test]
    fn test_head_and_tail_reset() {
        let mut buffer = CircularBuffer::new(3);
        for i in 1..=3 {
            buffer.write(i).unwrap();
        }
        for _ in 1..=3 {
            buffer.read();
        }
        assert_eq!(buffer.head, 0);
        assert_eq!(buffer.tail, 0);
    }

    #[test]
    fn test_read_empty_buffer() {
        let mut buffer: CircularBuffer<i32> = CircularBuffer::new(3);
        assert_eq!(buffer.read(), None);
    }

    #[test]
    fn test_write_full_buffer() {
        let mut buffer = CircularBuffer::new(3);
        for i in 1..=3 {
            buffer.write(i).unwrap();
        }
        assert!(buffer.write(4).is_err());
    }

    #[test]
    fn test_overwrite_full_buffer() {
        let mut buffer = CircularBuffer::new(3);
        for i in 1..=3 {
            buffer.write(i).unwrap();
        }
        buffer.overwrite(4);
        assert_eq!(buffer.read(), Some(2));
        assert_eq!(buffer.read(), Some(3));
        assert_eq!(buffer.read(), Some(4));
    }

    #[test]
    fn test_make_contiguous() {
        let mut buffer = CircularBuffer::new(3);
        buffer.write(1).unwrap();
        buffer.write(2).unwrap();
        buffer.read();
        buffer.write(3).unwrap();
        buffer.write(4).unwrap();
        buffer.make_contiguous();
        assert_eq!(buffer.head, 3);
        assert_eq!(buffer.tail, 0);
    }

    #[test]
    fn test_circular_buffer_with_complex_number() {
        let mut buffer = CircularBuffer::new(3);
        let complex1 = ComplexNumber::new(1.0, 1.0);
        let complex2 = ComplexNumber::new(2.0, 2.0);
        let complex3 = ComplexNumber::new(3.0, 3.0);

        buffer.write(complex1).unwrap();
        buffer.write(complex2).unwrap();
        buffer.write(complex3).unwrap();

        assert_eq!(buffer.read(), Some(complex1));
        assert_eq!(buffer.read(), Some(complex2));
        assert_eq!(buffer.read(), Some(complex3));
    }


}

// risposta domanda 5: si può usare il tipo Box<dyn Trait> per implementare un buffer generico eterogeneo,
// invece di inserire elementi dello stesso tipo inserisco elementi che implementano lo stesso tratto,
// perdo informazioni sul tipo di elemento, ma posso usare il trait object per accedere ai metodi del trait.
// a livello di memoria il buffer conterra per ogni elemento:
// - uno stack pointer al trait object
// - un vtable pointer che punta alla tabella dei metodi del trait
// - lo spazio del dato sullo heap