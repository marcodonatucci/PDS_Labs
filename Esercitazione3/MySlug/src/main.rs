use clap::{Parser, Subcommand};


#[derive(Parser, Debug)]
struct Args {
    // input string
    slug_in: String, // un attributo per ogni parametro che si vuole leggere
    // clap converte automaticamente al tipo indicato e darà errore se non è possibile

    #[arg(short, long, default_value_t = 1)]
    repeat: u32,

    #[arg(short, long)]
    verbose: bool,
}

fn conv(c: char) -> char {

    const SUBS_I : &str = "àáâäæãåāăąçćčđďèéêëēėęěğǵḧîïíīįìıİłḿñńǹňôöòóœøōõőṕŕřßśšşșťțûüùúūǘůűųẃẍÿýžźż";
    const SUBS_O: &str = "aaaaaaaaaacccddeeeeeeeegghiiiiiiiilmnnnnoooooooooprrsssssttuuuuuuuuuwxyyzzz";

    let vec_i = SUBS_I.chars().collect::<Vec<char>>(); // Converto la stringa in un vettore di caratteri
    let vec_o = SUBS_O.chars().collect::<Vec<char>>(); // Converto la stringa in un vettore di caratteri

    let mut correct_c = c; // Dichiaro una variabile mutabile di tipo char

    if ! correct_c.is_ascii_alphanumeric() {  // Controllo se il carattere è alfanumerico
        for i in 0..vec_i.len(){
            if correct_c == vec_i[i] {
                correct_c = vec_o[i];
                return correct_c;
            }
        }
        correct_c = '-'; // Se il carattere non è alfanumerico, lo sostituisco con un trattino
    }
    correct_c
}

fn slugify(s: &str) -> String {

    let mut to_slug = String::new(); // Dichiaro una stringa vuota di tipo String
    let mut slug = String::new(); // Dichiaro una stringa vuota di tipo String
    to_slug.push_str(s.trim());

    'outer: for c in to_slug.chars() { // Itero sui caratteri della stringa

        let lower_c = c.to_lowercase();

        for lower in lower_c {

            let correct_c = conv(lower); // Converto il carattere
            if correct_c == '-' &&  slug.ends_with('-') {
                continue 'outer; // Se il carattere è un trattino e la stringa termina con un trattino, salta il resto del ciclo
            }

            slug.push(correct_c);

        }
    }

    if slug.ends_with('-') && slug.len() > 1 {
        slug.pop(); // Se la stringa termina con un trattino e la lunghezza è maggiore di 1, rimuovi l'ultimo carattere
    }

    slug
}


// LAB3

fn is_slug(s: &str) -> bool { // Funzione per controllare se la stringa è uno slug
    let mut is_slug = true;
    let mut prev_char = ' ';
    for c in s.chars() {
        if !c.is_ascii_alphanumeric() && c != '-' {
            is_slug = false;
            break;
        }
        if c == '-' && prev_char == '-' {
            is_slug = false;
            break;
        }
        prev_char = c;
    }
    if s.ends_with('-') {
        is_slug = false;
    }
    if s.len() == 0 {
        is_slug = false;
    }
    is_slug
}

trait MySlug { // Definizione del trait MySlug
    fn to_slug1(&self) -> String;
    fn is_slug1(&self) -> bool;
}

impl MySlug for String { // Implementazione del trait MySlug per il tipo String
    fn to_slug1(&self) -> String {
        slugify(self)
    }
    fn is_slug1(&self) -> bool {
        is_slug(self)
    }
}

impl MySlug for &str{
    fn to_slug1(&self) -> String {
        slugify(self)
    }
    fn is_slug1(&self) -> bool {
        is_slug(self)
    }
}

trait Slug { // Definizione del trait Slug (niente conflitto con MySlug)
    fn to_slug(&self) -> String;
    fn is_slug(&self) -> bool;
}

// implementazione generica
impl<T> Slug for T
where T: AsRef<str> { // accetto tutti i tipi che possono essere deferenziati in stringa
    // vanno bene tutti i tipi che implementano questo tratto (String, &str e personalizzati)

    fn to_slug(&self) -> String {
        slugify(self.as_ref()) // self è di tipo &T, T implementa AsRef<str>, Quindi self.as_ref() restituisce un &str, is_slug() vuole un &str, quindi siamo a posto ✅
    }

    fn is_slug(&self) -> bool {
        is_slug(self.as_ref())
    }
}

fn main() {

    // let args = Args::parse(); // Parsing degli argomenti della riga di comando

    // if args.verbose {
        // println!("Verbose mode is on");
    // }

    // let slug = slugify(&args.slug_in);
    // println!("Original: {}\nSlug: {}", args.slug_in, slug);

    let s1 = String::from("Hello String");
    let s2 = "hello-slice";

    println!("{}", s1.is_slug()); // false
    println!("{}", s2.is_slug()); // true

    let s3: String = s1.to_slug();
    let s4: String = s2.to_slug();

    println!("s3:{} s4:{}", s3, s4); // stampa: s3:hello-string s4:hello-slice

}

#[cfg(test)]
mod tests { // modulo di testing fuori dal main
    use super::*;

    #[test]
    fn test_conversione_lettera_accentata() {
        assert_eq!(conv('à'), 'a');
    }

    #[test]
    fn test_conversione_lettera_non_accentata() {
        assert_eq!(conv('b'), 'b');
    }

    #[test]
    fn test_conversione_lettera_non_ammessa_sconosciuta() {
        assert_eq!(conv('@'), '-');
    }

    #[test]
    fn conversione_lettera_non_compresa_nella_lista(){
        assert_eq!(conv('ῶ'), '-');
    }

    #[test]
    fn test_stringa_separata_da_spazio(){
        assert_eq!(slugify("hello world"), "hello-world");
    }

    #[test]
    fn test_stringa_caratteri_accentati(){
        assert_eq!(slugify("città"), "citta");
    }

    #[test]
    fn test_stringa_vuota(){
        assert_eq!(slugify(""), "");
    }

    #[test]
    fn test_stringa_con_spazi_consecutivi(){
        assert_eq!(slugify("a  b  c"), "a-b-c");
    }

    #[test]
    fn test_stringa_con_caratteri_non_validi_consecutivi(){
        assert_eq!(slugify("a b@# c?"), "a-b-c");
    }

    #[test]
    fn test_stringa_solo_caratteri_non_validi(){
        assert_eq!(slugify("@#?"), "-");
    }

    #[test]
    fn test_stringa_con_spazio_alla_fine(){
        assert_eq!(slugify("hello "), "hello");
    }

    #[test]
    fn test_stringa_con_caratteri_non_validi_alla_fine(){
        assert_eq!(slugify("ciao@#"), "ciao");
    }
}
