use std::{sync::{Arc, Mutex}, thread, time::Instant};
use itertools::{Itertools};


pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u64) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn find_primes(limit: u64, n_threads: u64) -> Vec<u64> {
    let shared_counter = Arc::new(Mutex::new(0)); // contatore condiviso 
    let mut threads = vec![];
    let shared_primes = Arc::new(Mutex::new(Vec::new())); // vettore di numeri primi condiviso 

    for _ in 0..n_threads {
        let counter = shared_counter.clone(); // clono le variabili condivise per muovere un riferimento in ogni thread
        let primes = shared_primes.clone();

        threads.push(thread::spawn(move || {
            loop { // loop!!! (ogni thread continua a lavorare in parallelo)
                let mut counter_guard = counter.lock().unwrap(); // ottengo il mutexGuard del contatore
                if *counter_guard >= limit { // controllo che sia nel limite (deref)
                    break;
                }
                let current = *counter_guard; // se è nel limite salvo il suo valore e lo incremento (il mutex si aggiorna)
                *counter_guard += 1;
                drop(counter_guard); // rilascia il lock esplicitamente

                if is_prime(current) {
                    let mut primes_guard = primes.lock().unwrap(); // se il valore corrente è primo lo pusho nella lista
                    primes_guard.push(current);
                }
            }
        }));
    }

    for t in threads {
        t.join().unwrap();
    }

    let primes = shared_primes.lock().unwrap();
    primes.to_vec()
}

pub fn find_primes2(limit: u64, n_threads: u64) -> Vec<u64> {

    let mut threads = vec![];
    let shared_primes = Arc::new(Mutex::new(Vec::new())); // vettore di numeri primi condiviso 

    for i in 0..n_threads {

        let primes = shared_primes.clone();
        let thread_id = i;

        threads.push(thread::spawn(move || {
                let mut counter = 2 + thread_id; // il primo numero primo è 2

                while counter < limit { // il while gestisce il looping, non un loop esterno 

                    if is_prime(counter) {
                    let mut primes_guard = primes.lock().unwrap(); // se il valore corrente è primo lo pusho nella lista
                    primes_guard.push(counter);
                }

                counter += n_threads // fuori da if, sempre incrementato 
                    
                }
        }));
    }

    for t in threads {
        t.join().unwrap();
    }

    let primes = shared_primes.lock().unwrap();
    primes.to_vec()
}

// ====== GAME ======

pub fn mk_ops(symbols: &[char], n: usize) -> Vec<String> {
    if n == 0 {
        return vec![String::new()];
    }

    let mut result = vec![];

    for &symbol in symbols {
        for perm in mk_ops(symbols, n - 1) {
            result.push(format!("{}{}", symbol, perm));
        }
    }

    result
}

pub fn prepare(s: &str) -> Vec<String> {

    let mut result = vec![];
    let ops = mk_ops(&['+', '-', '*', '/'], 4);
    
    for digit in s.chars().permutations(s.len()) {
        for op_seq in &ops {
            let mut s = String::new();
            let mut it_op = op_seq.chars();
            for d in digit.iter() {
                s.push(*d);
                if let Some(op) = it_op.next() {
                    s.push(op);
                }
            }
            result.push(s);
        }
    }   
    result
}

#[test]
fn test_mk_ops() {
    let symbols = vec!['+', '-', '*', '/'];
    let n = 4;
    let result = mk_ops(&symbols, n);
    assert_eq!(result.len(), symbols.len().pow(n as u32));

    let res = prepare("23423");
    println!("{} {:?}", res.len(), res.iter().take(n).collect::<Vec<_>>());
}

pub fn verify(v: &[String], num_threads: usize) -> Vec<String> {

    let mut results = vec![];

    let chunk_size = (v.len() + num_threads - 1) / num_threads; // dimensione delle slice

    thread::scope(|s| {

        let mut handles = Vec::new(); 
        
        for chunk in v.chunks(chunk_size) { // itero nei chunk di v di dimensione giusta
            handles.push(s.spawn( move || { // spawno il thread che verifica questo chunk (dallo scope)
                chunk.iter().filter_map(|expr| { // itero nei valori e faccio sia filter che map
                    match meval::eval_str(expr) { // eval calcola l'espressione nella stringa
                        Ok(result) => {
                            if result == 10 as f64 { // se è 10 la restituisce
                                Some(expr.clone())
                            } else {
                                None
                            }
                        },
                        _ => None,
                    }
                }).collect_vec() // ottengo il vec
            }));
        }

        for h in handles {
                results.extend(h.join().unwrap()); // allungo i result con ogni handle
        }

    });

    results
    
}

fn main() {

    let limit =  1_000_000;
    let max_threads = 16;

    let input = "74648"; // esempio di 5 cifre da usare per generare le espressioni

    let expressions = prepare(input); // genera tutte le possibili espressioni


    let logical_cores = num_cpus::get(); // include anche i core logici (hyperthreading)
    let physical_cores = num_cpus::get_physical(); // solo core fisici

    println!("🧠 Core logici disponibili: {}", logical_cores);
    println!("💪 Core fisici disponibili: {}", physical_cores);

    println!("Confronto tra find_primes1 (counter condiviso) e find_primes2 (sequenze indipendenti)");
    println!("Numero primi cercati fino a {limit}\n");

    for n_threads in 1..=max_threads {
        println!("===> THREADS: {n_threads}");

        // find_primes1
        let start1 = Instant::now();
        let primes1 = find_primes(limit, n_threads);
        let duration1 = start1.elapsed();

        println!(
            "find_primes1 - Tempo: {:?} | Primi trovati: {}",
            duration1,
            primes1.len()
        );

        // find_primes2
        let start2 = Instant::now();
        let primes2 = find_primes2(limit, n_threads);
        let duration2 = start2.elapsed();
        println!(
            "find_primes2 - Tempo: {:?} | Primi trovati: {}",
            duration2,
            primes2.len()
        );

        // Verifica correttezza (opzionale, per debugging)
        if primes1 != primes2 {
            println!("⚠️  ATTENZIONE: I risultati differiscono!");
            // è una questione di sorting, con la seconda funzione non sono in ordine!
        }

        println!();
    } 
    
    println!();
    println!("🔍 Verifica espressioni generate da: {}\nTotale combinazioni: {}\n", input, expressions.len());
    println!();

    for n_threads in 1..=max_threads {
        println!("===> THREADS: {n_threads}");

        let start = Instant::now();
        let solutions = verify(&expressions, n_threads as usize);
        let duration = start.elapsed();

        println!(
            "Tempo: {:?} | Soluzioni trovate: {}\n",
            duration,
            solutions.len()
        );
    }

}
