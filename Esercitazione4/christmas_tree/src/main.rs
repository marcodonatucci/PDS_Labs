use std::collections::{HashMap};

pub struct Albero {
    relazioni: HashMap<String, Vec<String>>, // Relazioni padre-figlio
    stati: HashMap<String, bool>,            // Stato dell'interruttore di ogni nodo
}

impl Albero {
    pub fn new() -> Albero {
        Albero {
            relazioni: HashMap::new(),
            stati: HashMap::new(),
        }
    }
    // nota: aggiustare mutabilità dove necessario gestire errori in caso
    // di collisioni, valori mancanti
    // aggiungi un nodo figlio del nodo father
    pub fn add(&mut self, father: &str, node: &str) {
        self.relazioni.insert(father.to_string(), vec![node.to_string()]);
        self.stati.insert(node.to_string(), false); // inizializza lo stato dell'interruttore a false (spento)
    }
    // togli un nodo e tutti gli eventuali rami collegati
    pub fn remove(&mut self, node: &str) {
        self.relazioni.remove(node);
        self.stati.remove(node);
    }
    // commuta l’interruttore del nodo (che può essere on off) e restituisci il nuovo valore
    pub fn toggle(&mut self, node: &str) -> bool {
        // controlla se il nodo esiste
        if let Some(stato) = self.stati.get(node) {
            // commuta lo stato
            // controlla lo stato di tutti i nodi fino alla radice, deve essere on
            if let Some(padre) = self.relazioni.keys().find(|&k| self.relazioni[k].contains(&node.to_string())) {
                if let Some(stato_padre) = self.stati.get(padre) {
                    if !stato_padre {
                        return false; // se il padre è spento, non posso accendere il figlio
                    }
                }
            }
            let nuovo_stato = !stato;
            // aggiorna lo stato nel dizionario
            self.stati.insert(node.to_string(), nuovo_stato);
            return nuovo_stato;
        }
        false // se il nodo non esiste, restituisci false
    }
    // restituisci se la luce è accesa e spenta
    pub fn peek(&self, node: &str) -> bool {
        // controlla se il nodo esiste
        if let Some(stato) = self.stati.get(node) {
            return *stato;
        }
        false // se il nodo non esiste, restituisci false
    }
}


fn main() {
    let mut albero = Albero::new();

    // Aggiungiamo alcuni nodi
    albero.add("root", "figlio1");
    albero.add("figlio1", "figlio2");

    println!("Stato iniziale figlio1: {}", albero.peek("figlio1")); // false
    println!("Stato iniziale figlio2: {}", albero.peek("figlio2")); // false

    // Commuta lo stato di figlio1
    let nuovo_stato1 = albero.toggle("figlio1");
    println!("Nuovo stato figlio1 dopo toggle: {}", nuovo_stato1); // true

    // Commuta di nuovo per tornare allo stato originale
    let nuovo_stato2 = albero.toggle("figlio1");
    println!("Nuovo stato figlio1 dopo secondo toggle: {}", nuovo_stato2); // false

    // Peek su un nodo non esistente
    println!("Stato nodo inesistente: {}", albero.peek("inesistente")); // false

    // Rimuoviamo un nodo
    albero.remove("figlio1");

    // Dopo la rimozione
    println!("Stato figlio1 dopo rimozione: {}", albero.peek("figlio1")); // false
    println!("Stato figlio2 dopo rimozione padre: {}", albero.peek("figlio2")); // true (non è stato rimosso direttamente!)

    // Nota: figlio2 rimane nello stato perché non è stato direttamente rimosso.
}
