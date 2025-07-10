use std::{
    collections::HashMap,
    hash::Hash,
    sync::{Arc, RwLock},
    time::{Duration, Instant},
};

pub struct Cache<K: Eq + Hash, V> {
    map: RwLock<HashMap<K, (Arc<V>, Instant)>>, // valore (thread safe) + tempo di scadenza 
}

impl<K: Eq + Hash, V> Cache<K, V> {
    /// Crea una nuova istanza vuota
    pub fn new() -> Self {
        Self {
            map: RwLock::new(HashMap::new()),
        }
    }

    /// Numero di coppie presenti nella cache (comprese le scadute)
    pub fn size(&self) -> usize {
        self.map.read().unwrap().len()
    }

    /// Inserisce o sovrascrive la coppia (k, v) con durata `d`
    pub fn put(&self, k: K, v: V, d: Duration) {
        let mut map = self.map.write().unwrap();
        let expiration = Instant::now() + d;

        // Inserisco la nuova entry
        map.insert(k, (Arc::new(v), expiration));

        // Pulizia chiavi scadute
        map.retain(|_, (_, expiry)| *expiry > Instant::now());
    }

    /// Rinnova la durata della chiave, se esiste ed è valida
    pub fn renew(&self, k: &K, d: Duration) -> bool {
        let mut map = self.map.write().unwrap();
        let now = Instant::now();

        if let Some((val, expiry)) = map.get_mut(k) {
            if *expiry > now {
                *expiry = now + d;
                return true;
            } else {
                map.remove(k); // rimuovi scaduto
            }
        }

        false
    }

    /// Restituisce il valore associato alla chiave se non scaduto
    pub fn get(&self, k: &K) -> Option<Arc<V>> {
        let map = self.map.read().unwrap();
        let now = Instant::now();

        match map.get(k) {
            Some((val, expiry)) if *expiry > now => Some(Arc::clone(val)),
            _ => None,
        }
    }
}


fn main() {
    println!("Hello, world!");
}
