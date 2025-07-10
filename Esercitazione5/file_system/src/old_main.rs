use std::cell::{RefCell, Ref};
use std::rc::{Rc, Weak};
use std::path::PathBuf;

use std::fs;
use walkdir::WalkDir;

#[derive(Debug)]
enum FSItem {
    File(File),
    Directory(Directory),
    SymLink(SymLink),
}

#[derive(Debug)]
struct File {
    name: String,
    parent: Weak<RefCell<FSItem>>,
}

#[derive(Debug)]
struct Directory {
    name: String,
    children: Vec<Rc<RefCell<FSItem>>>,
    parent: Option<Weak<RefCell<FSItem>>>,
}

#[derive(Debug)]
struct SymLink {
    name: String,
    target: PathBuf,
    parent: Weak<RefCell<FSItem>>,
}


struct FileSystem {
    root: Rc<RefCell<FSItem>>,
    current_dir: Rc<RefCell<FSItem>>,
}


impl FileSystem {
    // crea un nuovo FS vuoto
    pub fn new() -> Self{
        let root = Rc::new(RefCell::new(FSItem::Directory(Directory {
            name: "/".to_string(),
            children: Vec::new(),
            parent: None,
        })));

        FileSystem {
            root: root.clone(),
            current_dir: root,
        }
    }

    // crea un nuovo FS replicando la struttura su disco
    /*pub fn from_disk() -> Self {

    }*/

    // cambia la directory corrente, path come in tutti gli altri metodi
    // può essere assoluto o relativo;
    // es: “ ../sibling” vuol dire torna su di uno e scendi in sibling
    pub fn change_dir(&mut self, path: String) -> Result<(), String> {
        match self.resolve_path(&path) {
            Some(dir) => {
                if let FSItem::Directory(_) = &*dir.borrow() {
                    self.current_dir = dir.clone();
                    Ok(())
                } else {
                    Err("Non è una directory".into())
                }
            }
            None => Err("Path non trovato".into()),
        }
    }

    // crea la dir in memoria e su disco
    pub fn make_dir(&self, path: String, name: String) -> Result<(), String> {
        let parent = self.resolve_path(&path).ok_or("Path non trovato")?;
        let new_dir = Directory {
            name: name.clone(),
            children: vec![],
            parent: Some(Rc::downgrade(&parent)),
        };
        let new_node = Rc::new(RefCell::new(FSItem::Directory(new_dir)));
        if let FSItem::Directory(ref mut dir) = *parent.borrow_mut() {
            dir.children.push(new_node);
            Ok(())
        } else {
            Err("Il path non è una directory".into())
        }
    }

    // crea un file vuoto in memoria e su disco
    pub fn make_file(&self, path: String, name: String) -> Result<(), String> {
        let parent = self.resolve_path(&path).ok_or("Path non trovato")?;
        let new_file = File {
            name: name.clone(),
            parent: Rc::downgrade(&parent),
        };
        let new_node = Rc::new(RefCell::new(FSItem::File(new_file)));
        if let FSItem::Directory(ref mut dir) = *parent.borrow_mut() {
            dir.children.push(new_node);
            Ok(())
        } else {
            Err("Il path non è una directory".into())
        }
    }

    // rinonima file / dir in memoria e su disco
    pub fn rename(&self, path: String, new_name: String) -> Result<(), String> {
        let item = self.resolve_path(&path).ok_or("Path non trovato")?;
        match &mut *item.borrow_mut() {
            FSItem::File(file) => {
                file.name = new_name;
            }
            FSItem::Directory(dir) => {
                dir.name = new_name;
            }
            FSItem::SymLink(symlink) => {
                symlink.name = new_name;
            }
        }
        Ok(())
    }

    // cerca l’elemento indicato dal path e restituisci un riferimento
    pub fn find(&self, path: String) -> Result<Rc<RefCell<FSItem>>, String> {
        let item = self.resolve_path(&path).ok_or("Path non trovato")?;
        Ok(Rc::clone(&item))
    }

    fn resolve_path(&self, path: &str) -> Option<Rc<RefCell<FSItem>>> {
        let mut current = if path.starts_with("/") {
            Rc::clone(&self.root)
        } else {
            Rc::clone(&self.current_dir)
        };

        let components = path.split('/').filter(|p| !p.is_empty());

        for part in components {
            match part {
                "." => continue,
                ".." => {
                    let parent = {
                        let current_borrow = current.borrow();
                        if let FSItem::Directory(dir) = &*current_borrow {
                            if let Some(parent_weak) = &dir.parent {
                                parent_weak.upgrade()
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    };
                    if let Some(p) = parent {
                        current = p;
                    }
                }
                name => {
                    let child = {
                        let current_borrow = current.borrow();
                        if let FSItem::Directory(dir) = &*current_borrow {
                            dir.children.iter().find(|c| match &*c.borrow() {
                                FSItem::Directory(d) => d.name == name,
                                FSItem::File(f) => f.name == name,
                                FSItem::SymLink(l) => l.name == name,
                            }).map(Rc::clone)
                        } else {
                            None
                        }
                    };
                    if let Some(c) = child {
                        current = c;
                    } else {
                        return None;
                    }
                }
            }
        }

        Some(current)
    }


}

fn main() {
    // Esempio di utilizzo
    let mut fs = FileSystem::new();
    fs.make_dir("/".to_string(), "home".to_string()).unwrap();
    fs.make_dir("/home".to_string(), "user".to_string()).unwrap();
    fs.make_file("/home/user".to_string(), "file.txt".to_string()).unwrap();
    fs.change_dir("/home/user".to_string()).unwrap();
    fs.rename("file.txt".to_string(), "new_file.txt".to_string()).unwrap();
    fs.make_dir("..".to_string(), "documents".to_string()).unwrap();
    fs.make_file("/home/documents".to_string(), "doc.txt".to_string()).unwrap();
    fs.change_dir("..".to_string()).unwrap();
    fs.make_file("/home/documents".to_string(), "doc2.txt".to_string()).unwrap();
    fs.change_dir("/home/documents".to_string()).unwrap();
    fs.make_file("/home/documents".to_string(), "doc3.txt".to_string()).unwrap();
    fs.make_file("/home/documents".to_string(), "doc4.txt".to_string()).unwrap();
    fs.make_file("/home/documents".to_string(), "doc5.txt".to_string()).unwrap();
    fs.rename("doc5.txt".to_string(), "renamed_doc.txt".to_string()).unwrap();

    // Stampa la struttura del file system
    println!("{:#?}", fs.root.borrow());
    println!("{:#?}", fs.current_dir.borrow());

}
