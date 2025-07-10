use std::fs;
use std::time::{SystemTime};
use clap::{Parser, Subcommand};


// esercizio 1
fn read_save_file(){

    let file = fs::read_to_string("test.txt");

    let mut content = String::from("");

    let mut final_content = String::from("");

    match file {
        Ok(c) => {
            content = c;
        },
        Err(e) => {
            println!("Error {}", e);
        }
    }

    for _ in 0..10 {
        final_content.push_str(&content);
        final_content.push_str("\n");
    }

    let result = fs::write("text.txt", final_content);

    match result {
        Ok(_) => {
            println!("Successfully wrote the file!");
        },
        Err(e) => {
            println!("Error {}", e);
        }
    }
    

}


// esercizio 2

enum Error1{
    Simple(SystemTime),
    Complex(SystemTime, String),

}

fn print_error(e: Error1){

    match e {
        Error1::Complex(time, msg) => {
            println!("Complex error occurred at {}: {}", time, msg);
        },
        Error1::Simple(time) => {
            println!("Simple error occurred at {}", time);
        }
    }

}


// esercizio 3

pub enum MulErr {Overflow, NegativeNumber}

pub fn mul(a: i32, b: i32) -> Result<u32, MulErr> {

}


// esercizio 4
struct Node {
    name: String,
    size: u32,
    count: u32,
}
impl Node {
   

}




// battleship

const bsize: usize = 20;
pub struct Board {
    boats: [u8; 4],
    data: [[u8; bsize]; bsize],
}
pub enum Error {
    Overlap,
    OutOfBounds,
    BoatCount,
}
pub enum Boat {
    Vertical(usize),
    Horizontal(usize)
}

impl Board {
    /* crea una board vuota con una disponibilità di navi */
    pub fn new(boats: &[u8]) -> Board {
        let mut b = Board {
            boats: [0; 4],
            data: [[0; bsize]; bsize]
        };
        for i in 0.. boats.len() {
            b.boats[i] = boats[i];
        }
        for i in 0..bsize {
            for j in 0..bsize {
                b.data[i][j] = ' ' as u8;
            }
        }
        b
    }
    /* crea una board a partire da una stringa che rappresenta tutto
    il contenuto del file board.txt */
    pub fn from(s: String)->Board {
        let mut b = Board {
            boats: [0; 4],
            data: [[0; bsize]; bsize]
        };
        let chars = s.chars().collect::<Vec<char>>(); // vettore di char

        let mut i = 0;
        let mut j = 0; // indice di riga e colonna

        for c in chars {
            if c == '\n' { // carattere a capo: cambio linea e azzero la colonna
                i += 1;
                j = 0;
                continue;
            }
            if i == 0 { // prima riga, navi
                if c == ' '{
                    continue;
                }
                b.boats[j] = c as u8; // se non è spazio aggiungo le navi
                j += 1;
            } else {
                b.data[i-1][j] = c as u8; // aggiungo i caratteri alla board e cambio colonna
                j += 1;
            }

        }
        b
    }
    /* aggiunge la nave alla board, restituendo la nuova board se
    possibile */
    /* bonus: provare a *non copiare* data quando si crea e restituisce
    una nuova board con la barca, come si può fare? */
    pub fn add_boat(self, boat: Boat, pos: (usize, usize)) -> Result<Board, Error> {

        let mut new_board = self;
        let mut i = pos.0;
        let mut j = pos.1;

        match boat{
            Boat::Horizontal(n) => {
                if j + n > bsize {
                    return Err(Error::OutOfBounds);
                }
                for k in 0..n {
                    if new_board.data[i][j+k] != ' ' as u8{
                        return Err(Error::Overlap);
                    }
                }
                if new_board.boats[n-1] == 0 {
                    return Err(Error::BoatCount);
                }
                for k in 0..n {
                    new_board.data[i][j+k] = 'B' as u8;
                }
                new_board.boats[n-1] -= 1;
                Ok(new_board)
            }
            Boat::Vertical(n) => {
                if i + n > bsize {
                    return Err(Error::OutOfBounds);
                }
                for k in 0..n {
                    if new_board.data[i + k][j] != ' ' as u8{
                        return Err(Error::Overlap);
                    }
                }
                if new_board.boats[n-1] == 0 {
                    return Err(Error::BoatCount);
                }
                for k in 0..n {
                    new_board.data[i + k][j] = 'B' as u8;
                }
                new_board.boats[n-1] -= 1;
                Ok(new_board)
            }
        }

    }
    /* converte la board in una stringa salvabile su file */
    pub fn to_string(&self) -> String {
        let boats = self.boats;
        let data = self.data;
        let mut s = String::new();
        for b in boats {
            s.push(b as char);
            s.push(' ');
        }
        s.push('\n');
        for i in data {
            for j in i {
                s.push(j as char);
            }
            s.push('\n');
        }
        s
    }

}

#[derive(Parser, Debug)]
struct Args {

    file_path: String,

   #[command(subcommand)]
    command: Commands, // sottocomandi con tipi diversi
}

#[derive(Subcommand, Debug)]
enum Commands {
    New {
        ships : String, // n1,n2,n3,n4
    },
    AddBoat {
        boat: String // direzione(H/V),len,x,y
    }
}


fn main() {

    // io da comando + lettura e scruttura file
    let args = Args::parse();

    match &args.command {
        Commands::New {ships} => {
            // controllo del formato
            let mut boats = [0;4];

            let mut i = 0;
            let chars = ships.chars().collect::<Vec<char>>(); // vettore di char
            for c in chars{
                if c == ','{
                    continue;
                }
                if i > 3 {
                    println!("Too many boats!");
                    return;
                }
                if c.is_numeric() {
                    boats[i] = c as u8;
                    i += 1;
                } else {
                    println!("Unvalid format!");
                    return;
                }
            }

            let board = Board::new(&boats);

            let file = fs::write(&args.file_path, board.to_string());

            match file {
                Ok(_) => {
                    println!("Successfully wrote file");
                }
                Err(e) => {
                    println!("Error writing file: {}", e);
                }
            }

        }
        Commands::AddBoat { boat } => {

            let file = fs::read_to_string(&args.file_path);
            let mut contents = String::new();

            match file {
                Ok(content) => {
                    contents = content;
                }
                Err(e) => {
                    println!("Error reading file: {}", e);
                }
            }

            let parts: Vec<&str> = boat.split(',').collect(); // splitta la stringa

            if parts.len() != 4 {
                println!("Unvalid format! Use: <direction>,<length>,<x>,<y>");
            }

            let direction = parts[0].chars().next().unwrap(); // prende solo 'H' o 'V'
            if direction != 'H' && direction != 'V' {
                println!("Invalid direction");
                return;
            }
            let len: usize = parts[1].parse().expect("Unvalid lenght format");
            let x: usize = parts[2].parse().expect("Unvalid x format");
            let y: usize = parts[3].parse().expect("Unvalid y format");


            if len > 4 || len < 1 {
                println!("Invalid boat length");
                return;
            }

            let board = Board::from(contents);
            let mut boat = Boat::Horizontal(len);

            if direction == 'V' {
                boat = Boat::Horizontal(len);
            }

            let result = board.add_boat(boat, (y, x));

            match result {
                Ok(board) => {
                    let file = fs::write(&args.file_path, board.to_string());
                    match file {
                        Ok(_) => {
                            println!("Successfully wrote file");
                        }
                        Err(e) => {
                            println!("Error writing file: {}", e);
                        }
                    }
                }
                Err(e) => {
                    match e {
                        Error::Overlap => {
                            println!("Overlap error");
                        }
                        Error::OutOfBounds => {
                            println!("Out of bounds error");
                        }
                        Error::BoatCount => {
                            println!("Boat count error");
                        }
                    }
                }
            }


        }
    }


}
