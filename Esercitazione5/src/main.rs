#![allow(warnings)]

pub mod mem_inspect {

    // dump object info:
    // size, address, bytes
    pub fn dump_object<T>(obj: &T) {
        let ptr = obj as *const T as *const u8;
        let _size = size_of::<T>();
        let _ptr = ptr as usize;
        println!("Object size: {_size}; address: {_ptr:x}");

        dump_memory(ptr, _size);
    }

    // dump memory info
    pub fn dump_memory(start: *const u8, size: usize) {
        let bytes = unsafe { std::slice::from_raw_parts(start, size) };

        println!("Bytes:");
        for (i, byte) in bytes.iter().enumerate() {
            print!("{:02x} ", byte);
            if i % 8 == 7 {
                println!();
            }
        }
        println!()
    }

    #[test]
    fn dump_object_example() {
        let s = "hello".to_string();
        dump_object(&s);

        let b = Box::new(s);
        // before running try to answer:
        // 1. what is the size of b?
        // 2. what is the content of b?
        dump_object(&b);

        // how to the the pointer of the wrapped object?
        let ptr = b.as_ref() as *const String as *const u8;
        println!("Pointer: {ptr:?}");

        assert!(true);
    }
}


pub mod List1 {
    use std::mem;

    #[derive(Clone)]
    pub enum Node<T> {
        Cons(T, Box<Node<T>>),
        Nil,
    }

    pub struct List<T> {
        head: Node<T>,
    }

    impl<T: Clone> List<T> {
        pub fn new() -> Self {
            return Self { head: Node::Nil }
        }

        // insert a new element at the beginning of the list
        // you may encouter a problem with the borrow checker while trying to move self.head to a new variable
        // problem:
        // 1. you need to build a new list Node (elem: elem, self.head)
        // 2. but you can't move self.head, because self.head would be undefined
        // 3. you can't copy it either, because Box can't be copied
        // solution: use mem::replace to move the value of self.head into a new variable and replace it with Nil
        // 4. let self.head point to the new created node
        pub fn push(&mut self, elem: T) {
            let old_head = std::mem::replace(&mut self.head, Node::Nil);
            self.head = Node::Cons(elem, Box::new(old_head));

        }

        // pop the first element of the list and return it
        fn pop(&mut self) -> Option<T> {
            // sposto il valore della head nella nuova variabile e rimpiazzo con nil 
            let old_head = std::mem::replace(&mut self.head, Node::Nil);
            match old_head {
                Node::Nil => {
                    return None;
                },
                Node::Cons(element, next_element) => {
                    // la nuova testa è il nodo subito dopo 
                    self.head = *next_element.clone(); //deref Box restituisce il valore

                    return Some(element.clone());

                }
            }
        }

        // return a referece to the first element of the list
        pub fn peek(&self) -> Option<&T> {
            match &self.head { // prendo il riferimento perchè non posso prendere possesso di un elemento di una struct dalla reference 
                Node::Nil => None,
                Node::Cons(element, next_element ) => Some(element)
            }
        }

        // uncomment after having implemented the ListIter struct
        // return an interator over the list values
        fn iter(&self) -> ListIter<T> {

            return ListIter { next: Some(&self.head) };

        }

        // take the first n elements of the list and return a new list with them
        pub fn take(&mut self, n: usize) -> List<T> {
            
            let mut new_list = List::new();

            for i in 0..n {
                let element = self.pop().unwrap(); //usa match se vuoi gestire il None!
                new_list.push(element);
            }

            new_list

        }
    }

    struct ListIter<'a, T> {
        // implement the iterator trait for ListIter
        next: Option<&'a Node<T>>,
    }
    
    impl<'a, T> Iterator for ListIter<'a, T> {
        type Item = &'a T;
    
        fn next(&mut self) -> Option<Self::Item> {
            match self.next {
                Some(Node::Cons(elem, next)) => {
                    self.next = Some(next);
                    Some(elem)
                },
                _ => None,
            }
        }
    }
  // something that may be useful for the iterator implementation:
   //  let a = Some(T);
   //  let b = &a;
   //  match b { Some(i) => ... } // here i is a reference to T
}

pub mod List2 {
    use std::{cell::RefCell, rc::Rc};


    pub struct DNode<T> {
        elem: T,
        next: NodeLink<T>,
        prec: NodeLink<T>,
    }

    type NodeLink<T> = Option<Rc<RefCell<DNode<T>>>>;

    pub struct DList<T> {
        head: NodeLink<T>,
        tail: NodeLink<T>
    }

    // for this implementattion, since we are using option, take a look at the take method in Option<T>.
    // It allows to move the value of the option into another option and replace it with None
    // let mut a = Some(5);
    // let b = a.take(); // a is now None and b is Some(5)
    impl<T> DList<T> {
        pub fn new() -> Self {
            return Self { head: None, tail: None }
        }

        // insert a new element at the beginning of the list
        // you may encouter a problem with the borrow checker while trying to move self.head to a new variable
        // problem:
        // 1. you need to build a new list Node (elem: elem, self.head)
        // 2. but you can't move self.head, because self.head would be undefined
        // 3. you can't copy it either, because Box can't be copied
        // solution: use mem::replace to move the value of self.head into a new variable and replace it with Nil
        // 4. let self.head point to the new created node
        pub fn push_front(&mut self, elem: T) {
            let new_head = Rc::new(RefCell::new(DNode { // creo la nuova head
                elem: elem,
                next: self.head.clone(), // metto il ref alla vecchia head
                prec: None,
            }));

            match self.head.take() { // take prende il valore della option
                Some(old_head) => {
                    old_head.borrow_mut().prec = Some(new_head.clone());  // assegno alla vecchia head la nuova come precedente (clone aggiunge uno strong ref alla Rc)
                    self.head = Some(new_head); // il campo head prende possesso della nuova head
                },
                None => {
                    self.tail = Some(new_head.clone()); // diventa anche la coda! (la lista era vuota) 

                    self.head = Some(new_head); // diventa la nuova testa
                },
                
            }

        }

        pub fn push_back(&mut self, elem: T) {
            let new_tail = Rc::new(RefCell::new(DNode { // creo la nuova head
                elem: elem,
                next: None, 
                prec: self.tail.clone(), // metto il ref alla tail
            }));

            match self.tail.take() {
                Some(old_tail) => {
                    old_tail.borrow_mut().next = Some(new_tail.clone());  
                    self.tail = Some(new_tail);  
                },
                None => {
                    self.head = Some(new_tail.clone()); 
                    self.tail = Some(new_tail); 
                },
                
            }

        }

        // pop the first element of the list and return it
        fn pop_front(&mut self) -> Option<T> {
            
            self.head.take().map(|old_head| { // map?
                if let Some(next) = old_head.borrow_mut().next.take() { // se c'è un next
                    next.borrow_mut().prec = None; // toglie il riferimento alla head
                    self.head = Some(next); // diventa la nuova head
                } else {
                    self.tail.take(); // lascia il None nella tail 
                }
                Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem // è così...
            })
        }

        fn pop_back(&mut self) -> Option<T> {
            self.tail.take().map(|old_tail| {
                if let Some(prec) = old_tail.borrow_mut().prec.take() {
                    prec.borrow_mut().next = None;
                    self.tail = Some(prec);
                } else {
                    self.head.take();
                }
                Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
            })
        }

        fn popn(&mut self, n: usize) -> Option<T> {
            // sposto il valore della head nella nuova variabile e rimpiazzo con nil 
            let mut current = self.head.clone();

            for _ in 0..n {
                current = current?.borrow().next.clone(); // clono il prossimo nodo fino a quello che mi serve (n)
            }

            current.map(|node| { //map sul nodo corrente

                {
                let mut node_ref = node.borrow_mut(); // creo un riferimento mutabile al nodo 

                if let Some(prec) = node_ref.prec.take() {
                    prec.borrow_mut().next = node_ref.next.clone(); // se c'è un prec metto il suo next a quello dopo di current
                } else {
                    self.head = node_ref.next.clone(); // se non c'è prec allora sono in testa e assegno quello dopo a head
                }
                
                // stessa cosa per il next
                if let Some(next) = node_ref.next.take() {
                    next.borrow_mut().prec = node_ref.prec.clone();     
                } else {
                    self.tail = node_ref.prec.clone();
                }
            } // il borrow di node esce dallo scope e si può ritornare
                Rc::try_unwrap(node).ok().unwrap().into_inner().elem
            })
        }
    }
}

pub mod dlist {
// *****
// double linked list suggestions:
// the node has both a next and a prev link

// type NodeLink = ???
// typer NodeBackLink = ???
// struct DNode<T> {
//     elem: T,
//     prev: NodeBackLink,  // which type do we use here?
//     next: NodeLink, // which type do we use here?
// }

// struct DList {
// head: NodeLink,
// tail: NodeLink
// }

// use Rc, since we need more than one reference to the same node. 
// You need to both strong and weak references

// For mutating the list and changing the next and prev fields we also need to be able to mutate the node, 
// therefore we can use RefCell too (as for the tree at lesson)

// how to access content of Rc<RefCell<T>>:
// es let a = Rc::new(RefCell::new(5));
// let mut x = (*a).borrow_mut();  // with (*a) we dereference the Rc, with (*a).borrow_mut() we get a mutable reference to the content of the RefCell
// *x = 6; // we can now change the content of the RefCell

// hint for pop: you can return either a reference to the value or take the value out of the Rc, 
// but usually it is not possible to take out the value from an Rc since it may be referenced elsewhere.
// if you can guarantee it's the only reference to the value  you can use Rc::try_unwrap(a).unwrap().into_inner() to get the value
// it first takes out the value from the Rc, then it tries to unwrap the value from the Result, and finally it takes the inner value from the Result
// see here
// https://stackoverflow.com/questions/70404603/how-to-return-the-contents-of-an-rc
// otherwise you can impose the COPY trait on T 

// other hint that may be useful: Option<T> has a default clone implementation which calls the clone of T. Therefore:
// Some(T).clone() ->  Some(T.clone())
// None.clone() -> None


}

pub fn main() {

}
