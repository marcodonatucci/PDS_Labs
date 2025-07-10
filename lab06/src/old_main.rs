use std::time::Instant;


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

pub fn find_primes1(limit: u64, n_threads: u64) -> Vec<u64> {
    // condividere una variabile counter tra ogni thread, ogni thread a turno incrementa
    // counter e verifica se quel numero è primo, se è primo lo memorizza altrimenti lo
    // scarta; alla fine restituisce i numeri primi che ha trovato

    let primes = std::sync::Arc::new(std::sync::Mutex::new(vec![]));
    let mut handles = vec![];
    for i in 0..n_threads {

        let primes = primes.clone();

        let thread_id = i; // per visibilità nel print

        let handle = std::thread::spawn(move || {

            let mut counter = thread_id;

            while counter < limit {


                if is_prime(counter) {

                    primes.lock().unwrap().push(counter);
                }
                counter += n_threads;
            }


        });

        handles.push(handle);

    }

    for handle in handles {
        handle.join().unwrap();
    }

    let mut primes = primes.lock().unwrap();

    primes.sort();

    primes.dedup();

    primes.to_vec()

}

pub fn find_primes2(limit: u64, n_threads: u64) -> Vec<u64> {
    // non condividere nulla, ogni thread conta a partire da 2,3,4,5,...n a limit modulo n e
    // verifica quel numero; in questo modo ogni thread proverà dei numeri differenti
    // (perché dividere in blocchi contigui sarebbe meno efficiente?)
    // Es con tre thread uno rispettivamente verificherà:
    // 2 5 8 11 …
    // 3 6 9 12 …
    // 4 7 10 13 …

    let primes = std::sync::Arc::new(std::sync::Mutex::new(vec![]));
    let mut handles = vec![];

    for i in 0..n_threads {

        let primes = primes.clone();

        let thread_id = i; // per visibilità nel print

        let handle = std::thread::spawn(move || {

            let mut counter = 2 + thread_id;

            while counter < limit {

                if is_prime(counter) {
                    primes.lock().unwrap().push(counter);
                }
                counter += n_threads;
            }

        });

        handles.push(handle);

    }

    for handle in handles {
        handle.join().unwrap();
    }

    let mut primes = primes.lock().unwrap();

    primes.sort();

    primes.dedup();

    primes.to_vec()
}


fn main() {
    let limit = 1_000_000;
    let max_threads = 16;

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
        let primes1 = find_primes1(limit, n_threads);
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
        }

        println!();
    }
}

