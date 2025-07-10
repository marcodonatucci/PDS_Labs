// paste this file into main.rs
use std::env;
use std::fs;

enum Letter {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
    I = 8,
    J = 9,
    K = 10,
    L = 11,
    M = 12,
    N = 13,
    O = 14,
    P = 15,
    Q = 16,
    R = 17,
    S = 18,
    T = 19,
    U = 20,
    V = 21,
    W = 22,
    X = 23,
    Y = 24,
    Z = 25
}

fn stats(text: &str) -> [u32; 26] {
    let mut counts= [0; 26]; // inizializzo l'array a 0
    // definisco un enum che mappa le lettere agli indici
    for c in text.chars() {
        if c.is_alphabetic() {
            let c = c.to_ascii_lowercase();
            let index= match c {
                'a' => Letter::A,
                'b' => Letter::B,
                'c' => Letter::C,
                'd' => Letter::D,
                'e' => Letter::E,
                'f' => Letter::F,
                'g' => Letter::G,
                'h' => Letter::H,
                'i' => Letter::I,
                'j' => Letter::J,
                'k' => Letter::K,
                'l' => Letter::L,
                'm' => Letter::M,
                'n' => Letter::N,
                'o' => Letter::O,
                'p' => Letter::P,
                'q' => Letter::Q,
                'r' => Letter::R,
                's' => Letter::S,
                't' => Letter::T,
                'u' => Letter::U,
                'v' => Letter::V,
                'w' => Letter::W,
                'x' => Letter::X,
                'y' => Letter::Y,
                'z' => Letter::Z,
                _ => Letter::A,
            };

            counts[index as usize] += 1; // accedo al valore numerico della lettera con 'as'
            // usize è il tipo di dato per gli indici degli array e degli slice


        }
    }
    counts
}

fn is_pangram(counts: &[u32]) -> bool {
    if counts.len() == 25 {
        return false; // se l'array non è lungo 26 non è un pangramma
    }
    for i in counts{ // è già un puntatore
        if *i == 0{ // accedo al valore
            return false;
        }
    }
    true
}

// call this function from main
// load here the contents of the file
pub fn run_pangram() {

    let args: Vec<String> = env::args().collect(); // prendo gli argomenti della riga di comando
    if args.len() < 2 { // se non ci sono argomenti
        println!("Usage: {} <filename>", args[0]);
        return;
    }
    let filename = &args[1];

    let contents = fs::read_to_string(&filename); // restituisce un tipo Result<String, Error>
    if contents.is_err() {
        println!("Error reading file: {}", filename);
        return;
    }

    let contents = contents.unwrap(); // estraggo il contenuto del file dal result

    let stats_array = stats(&contents); // calcolo le statistiche
    let result: bool = is_pangram(&stats_array); // verifico se è un pangramma dalle statistiche

    match result {
        true => println!("{} is a pangram.", &contents),
        false => println!("{} is not a pangram.", &contents),
    }

    println!("a {:?}", stats_array[0]);
    println!("b {:?}", stats_array[1]);
    println!("c {:?}", stats_array[2]);
    println!("d {:?}", stats_array[3]);
    println!("e {:?}", stats_array[4]);
    println!("f {:?}", stats_array[5]);
    println!("g {:?}", stats_array[6]);
    println!("h {:?}", stats_array[7]);
    println!("i {:?}", stats_array[8]);
    println!("j {:?}", stats_array[9]);
    println!("k {:?}", stats_array[10]);
    println!("l {:?}", stats_array[11]);
    println!("m {:?}", stats_array[12]);
    println!("n {:?}", stats_array[13]);
    println!("o {:?}", stats_array[14]);
    println!("p {:?}", stats_array[15]);
    println!("q {:?}", stats_array[16]);
    println!("r {:?}", stats_array[17]);
    println!("s {:?}", stats_array[18]);
    println!("t {:?}", stats_array[19]);
    println!("u {:?}", stats_array[20]);
    println!("v {:?}", stats_array[21]);
    println!("w {:?}", stats_array[22]);
    println!("x {:?}", stats_array[23]);
    println!("y {:?}", stats_array[24]);
    println!("z {:?}", stats_array[25]);

}


// please note, code has been splittend in simple functions in order to make testing easier

#[cfg(test)] // this is a test module
mod tests
{
    // tests are separated modules, yuou must import the code you are testing
    use super::*;

    #[test]
    fn test_all_ones() {
        let counts = [1; 26];
        assert!(is_pangram(&counts));
    }

    #[test]
    fn test_some_zeros() {
        let mut counts = [0; 26];
        counts[0] = 0;
        counts[1] = 0;
        assert!(!is_pangram(&counts));
    }

    #[test]
    fn test_increasing_counts() {
        let mut counts = [0; 26];
        for i in 0..26 {
            counts[i] = i as u32 + 1;
        }
        assert!(is_pangram(&counts));
    }

    #[test]
    fn test_wrong_size()  {
        let counts = [1; 25];
        assert!(!is_pangram(&counts));
    }

    #[test]
    fn test_stats_on_full_alphabet() {
        let counts = stats("abcdefghijklmnopqrstuvwxyz");
        for c in counts {
            assert!(c == 1);
        }
    }

    #[test]
    fn test_stats_on_empty_string() {
        let counts = stats("");
        for c in counts {
            assert!(c == 0);
        }
    }

    #[test]
    fn test_stats_missing_char() {
        let counts = stats("abcdefghijklmnopqrstuvwxy");
        for c in counts.iter().take(25) {
            assert!(*c == 1);
        }
        assert!(counts[25] == 0);

    }

    #[test]
    fn test_stats_on_full_tring() {
        let contents = "The quick brown fox jumps over the lazy dog";
        let counts = stats(contents);
        for c in counts {
            assert!(c > 0);
        }
    }

    #[test]
    fn test_stats_with_punctuation() {
        let contents = "The quick brown fox jumps over the lazy dog!";
        let counts = stats(contents);
        for c in counts {
            assert!(c > 0);
        }
    }

    #[test]
    fn test_missing_char_on_full_string() {
        let contents = "The quick brown fox jumps over the laz* dog";
        let counts = stats(contents);
        println!("{:?}", counts);
        for (i, c) in counts.iter().enumerate() {
            if i == 24 {
                assert!(*c == 0);
            } else {
                assert!(*c > 0);
            }

        }
    }

    #[test]
    fn test_is_pangram() {
        let counts = stats("The quick brown fox jumps over the lazy dog");
        assert!(is_pangram(&counts));
    }
}

fn main() {
    run_pangram();
}

