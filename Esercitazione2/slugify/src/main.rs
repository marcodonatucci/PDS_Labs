use clap::{Parser};


#[derive(Parser, Debug)]
struct Args {
    // input string
    slug_in: String,

    #[arg(short, long, default_value_t = 1)]
    repeat: u32,

    #[arg(short, long)]
    verbose: bool,
}

fn conv(c: char) -> char {

    const SUBS_I : &str = "àáâäæãåāăąçćčđďèéêëēėęěğǵḧîïíīįìıİłḿñńǹňôöòóœøōõőṕŕřßśšşșťțûüùúūǘůűųẃẍÿýžźż";
    const SUBS_O: &str = "aaaaaaaaaacccddeeeeeeeegghiiiiiiiilmnnnnoooooooooprrsssssttuuuuuuuuuwxyyzzz";

    let vec_i: Vec<_> = SUBS_I.chars().collect();
    let vec_o: Vec<_> = SUBS_O.chars().collect();

    let mut correct_c = c;

    if ! correct_c.is_ascii_alphanumeric() {
        for i in 0..vec_i.len() {
            if correct_c == vec_i[i] {
                correct_c = vec_o[i];
                return correct_c
            }
        }
        correct_c = '-';
    }
    correct_c

}

fn slugify(s: &str) -> String {
    
    let to_slug: String = s.trim().to_string();
    let mut slug = String::new();

    'outer: for c in to_slug.chars(){

        let lower_c = c.to_lowercase();

            for l in lower_c{

                let correct_c = conv(l);

                if correct_c == '-' && slug.ends_with('-'){
                    continue 'outer;
                }

                slug.push(correct_c);

            }
    }

    if slug.len() > 1 && slug.ends_with('-') {
        slug.pop();
    }

    slug

}


fn main() {

    let args = Args::parse();

    let slug = slugify(&args.slug_in);
    println!("Original: {}\nSlug: {}", args.slug_in, slug);

}

#[cfg(test)]
mod tests {
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
