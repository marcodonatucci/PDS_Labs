// WARNING:
// - the lifetimes are not set correctly, you have to set them to make it compile
// - you have also to implemment missing functions and fix the code
// - *** see test test functions in the code for usage examples

use std::io;
use std::fs::File;
use std::io::BufRead;

// (1) LineEditor: implement functionality
pub struct LineEditor {
    lines: Vec<String>,
}

impl LineEditor {
    pub fn new(s: String) -> Self {
        let l = LineEditor {
            lines: s.lines().map(|l| l.to_string()).collect(), // map each line to a String
        };
        l
        }

    // create a new LineEditor from a file
    pub fn from_file(file_name: &str) -> Result<Self, io::Error> {
        let file = File::open(file_name)?;
        let reader = io::BufReader::new(file);
        let mut lines = Vec::new();
        for line in reader.lines() {
            lines.push(line?);
        }
        Ok(LineEditor { lines }) // se funziona crea un nuovo LineEditor
    }

    pub fn all_lines(&self) -> Vec<&str> {
        self.lines.iter().map(|l| l.as_str()).collect()
    }

    pub fn replace(&mut self, line: usize, start: usize, end: usize, subst: &str) {
        // replace the text in the line
        if line < self.lines.len() {
            let l = &mut self.lines[line];
            let start = start.min(l.len()); // cerca il minimo tra start e la lunghezza della stringa
            let end = end.min(l.len()); // cerca il minimo tra end e la lunghezza della stringa
            // così non vado fuori dagli indici
            let new_line = format!("{}{}{}", &l[..start], subst, &l[end..]);
            // formatta con la sottostringa iniziale fino a start, poi la nuova sottostringa
            // poi quella finale da end in poi
            self.lines[line] = new_line;
        }
    }

    pub fn get_lines(&self) -> Vec<&String> {
        let v = self.lines.iter().collect();
        v
    }

    pub fn set_line(&mut self, line: usize, new_line: String) -> Result<&mut String, io::Error> {
        if line < self.lines.len() {
            self.lines[line] = new_line;
        }
        Ok(&mut self.lines[line])
    }
}


// (2) Match contains the information about the match. Fix the lifetimes
// repl will contain the replacement.
// It is an Option because it may be not set yet or it may be skipped
struct Match<'a> {
    pub line: usize,
    pub start: usize,
    pub end: usize,
    pub text: &'a str, // deve vivere almeno quanto la struct Match
    pub repl: Option<String>,
}

// use the crate "regex" to find the pattern and its method find_iter for iterating over the matches
// modify if necessary, this is just an example for using a regex to find a pattern
fn find_example<'a>(lines: &[&'a str], pattern: &'a str) -> Vec<Match<'a>> { // si aspetta i lifetimes
    let mut matches = Vec::new();
    let re = regex::Regex::new(pattern).expect("Invalid regex");
    for (line_idx, line) in lines.iter().enumerate() {
        for mat in re.find_iter(line) {
            matches.push(Match {
                line: line_idx,
                start: mat.start(),
                end: mat.end(),
                text: &line[mat.start()..mat.end()],
                repl: None,
            });
        }
    }
    matches
}

// (3) Fix the lifetimes of the FindReplace struct
// (4) implement the Finder struct
struct FindReplace<'a> {
    lines: Vec<&'a str>,
    pattern: String,
    matches: Vec<Match<'a>>, // matches dura quanto FindReplace
}

impl<'a> FindReplace<'a> { // lifetime anonimo
    pub fn new(lines: Vec<&'a str>, pattern: &'a str) -> Self {
        let matches = find_example(&lines, pattern);
        let mut finder = FindReplace {
            lines: lines.clone(),
            pattern: pattern.to_string(),
            matches: matches,
        };
        finder
    }

    // return all the matches
    pub fn matches(&self) -> &Vec<Match> {
        &self.matches
    }

    // apply a function to all matches and allow to accept them and set the repl
    // useful for promptig the user for a replacement
    pub fn apply(&mut self, fun: impl Fn(&mut Match) -> bool) {
        for m in &mut self.matches {
            if fun(m) {
                // if the function returns true, we accept the match
                // and set the repl
                m.repl = Some("some repl".to_string());
            } else {
                // if the function returns false, we skip the match
                m.repl = None;
            }
        }
    }
}


//(5) how FindReplace should work together with the LineEditor in order
// to replace the matches in the text
#[test]
fn test_find_replace() {
    let s = "Hello World.\nA second line full of text.";
    let mut editor = LineEditor::new(s.to_string());

    let lines_owned: Vec<String> = editor.all_lines().iter().map(|&l| l.to_string()).collect(); // Copia i dati
    let lines: Vec<&str> = lines_owned.iter().map(|s| s.as_str()).collect(); // Crea un Vec<&str> dai dati copiati

    let mut finder = FindReplace::new(lines.clone(), "ll"); // Usa il Vec<&str> clonato


    // find all the matches and accept them
    finder.apply(|m| {
        println!("{} {} {} {}", m.line, m.start, m.end, m.text);
        m.repl = Some("some repl".to_string());
        true
    });

    // now let's replace the matches
    // why this loop won't work?
    //for m: Match in finder.matches() { // non funziona perchè matches() ritorna un riferimento
    //    editor.replace(m.line, m.start, m.end, &m.repl.as_ref().expect("repl not set"));
    //}

    // alternate method: why this one works?

    let mut subs = Vec::new();
    for m in finder.matches() {
        subs.push( /** add match if repl is set */
        (m.line, m.start, m.end, m.repl.as_ref().expect("repl not set").as_str()));
    }

    for (line, start, end, subst) in subs {
        editor.replace(line, start, end, subst);
    }

}


// (6) sometimes it's very expensive to find all the matches at once before applying
// the changes
// we can implement a lazy finder that finds just the next match and returns it
// each call to next() will return the next match
// this is a naive implementation of an Iterarator

#[derive(Debug, Clone, Copy)]
struct FinderPos {
    pub line: usize,
    pub offset: usize,
}

struct LazyFinder<'a> {
    lines: Vec<&'a str>,
    pattern: String,
    pos: Option<FinderPos>,
}

impl<'a> LazyFinder<'a> {
        // Crea un nuovo LazyFinder con le linee e il pattern specificati
        pub fn new(lines: Vec<&'a str>, pattern: &'a str) -> Self {
            // Inizializza la posizione iniziale come la prima riga e offset 0
            let pos = Some(FinderPos { line: 0, offset: 0 });
            LazyFinder {
                lines, // Le linee di testo da analizzare
                pattern: pattern.to_string(), // Il pattern da cercare, convertito in String
                pos, // La posizione iniziale per la ricerca
            }
        }

    // remember:
    // return None if there are no more matches
    // return Some(Match) if there is a match
    // each time save the position of the match for the next call
    pub fn next<'b>(&'b mut self) -> Option<Match<'a>>  where
        'a: 'b, // Assicura che il lifetime 'a sia più lungo di 'b
        {
        // Se non ci sono più righe da analizzare, restituisci None
        if self.pos.is_none() {
            return None;
        }

        // Ottieni la posizione corrente
        let pos = self.pos.unwrap();
        let line = self.lines.get(pos.line).copied()?; // Usa `.copied()` per ottenere un riferimento valido

        // Trova il pattern nella riga corrente
        let re = regex::Regex::new(&self.pattern).expect("Invalid regex");
        if let Some(mat) = re.find(&line[pos.offset..]) {
            // Se trovi un match, crea un nuovo Match e aggiorna la posizione
            let m = Match {
                line: pos.line,
                start: pos.offset + mat.start(),
                end: pos.offset + mat.end(),
                text: &line[pos.offset + mat.start()..pos.offset + mat.end()],
                repl: None,
            };

            // Aggiorna la posizione per il prossimo match
            self.pos = Some(FinderPos {
                line: pos.line,
                offset: pos.offset + mat.end(),
            });

            Some(m) // Restituisci il match trovato
        } else {
            // Se non trovi un match, passa alla riga successiva
            self.pos = Some(FinderPos {
                line: pos.line + 1,
                offset: 0,
            });
            self.next() // Chiama ricorsivamente next() per cercare nella prossima riga
        }

    }
}

// (7) example of how to use the LazyFinder
#[test]
fn test_lazy_finder() {
    let s = "Hello World.\nA second line full of text.";
    let mut editor = LineEditor::new(s.to_string());

    let lines = editor.all_lines();
    let mut finder = LazyFinder::new(lines, "ll");

    // find all the matches and accept them
    while let Some(m) = finder.next() {
        println!("{} {} {} {}", m.line, m.start, m.end, m.text);
    }
}


// (8) now you have everything you need to implement the real Iterator

struct FindIter<'a> {
    lines: Vec<&'a str>,
    pattern: String,

}

impl<'a> FindIter<'a> {
    pub fn new(lines: Vec<&'a str>, pattern: &'a str) -> Self {
        let mut finder = FindIter {
            lines: lines.clone(),
            pattern: pattern.to_string(),
        };
        finder
    }
}

impl<'a> Iterator for FindIter<'a> {
    type Item = Match<'a>; // <== we inform the Iterator that we return a Match

    fn next<'b>(&'b mut self) -> Option<Self::Item> {
        // Se non ci sono più righe da analizzare, restituisci None
        if self.lines.is_empty() {
            return None;
        }

        // Ottieni la riga corrente
        let line = self.lines.remove(0); // Rimuovi la riga corrente dalla lista

        // Trova il pattern nella riga corrente
        let re = regex::Regex::new(&self.pattern).expect("Invalid regex");
        if let Some(mat) = re.find(line) {
            // Se trovi un match, crea un nuovo Match e restituiscilo
            Some(Match {
                line: 0,
                start: mat.start(),
                end: mat.end(),
                text: &line[mat.start()..mat.end()],
                repl: None,
            })
        } else {
            // Se non trovi un match, chiama next() ricorsivamente per cercare nella prossima riga
            self.next()
        }
    }
}

// (9) test the find iterator
#[test]
fn test_find_iter() {
    let s = "Hello World.\nA second line full of text.";
    let mut editor = LineEditor::new(s.to_string());

    let lines = editor.all_lines();
    let mut finder = FindIter::new(lines, "ll");

    // find all the matches and accept them
    for m in finder {
        println!("{} {} {} {}", m.line, m.start, m.end, m.text);

    }
}


fn main() {}
