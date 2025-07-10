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
    use std::ops::Deref;

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
            Self {
                head: Node::Nil,
            }
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
        pub fn pop(&mut self) -> Option<T> {

            // move the value of the head into a new variable
            let old_head = std::mem::replace(&mut self.head, Node::Nil);
            // return the value of the head
            match old_head {
                Node::Cons(elem, next) => {
                    // replace the head with the next node
                    self.head = *next.clone(); // dereference the Box to get the value
                    // return the value of the head
                    Some(elem.clone())
                }
                Node::Nil => None,
            }
        }


        // return a referece to the first element of the list
        pub fn peek(&self) -> Option<&T> {

            match &self.head {
                Node::Cons(elem, _) => Some(elem),
                Node::Nil => None,
            }

        }

        // uncomment after having implemented the ListIter struct
        // return an interator over the list values
        pub fn iter(&self) -> ListIter<'_, T> {
            ListIter {
                next: Some(&self.head),
            }
        }

        // take the first n elements of the list and return a new list with them
        pub fn take(&mut self, n: usize) -> List<T> {

            let mut new_list = List::new();
            let mut count = 0;

            while count < n {
                match self.pop() {
                    Some(elem) => {
                        new_list.push(elem);
                        count += 1;
                    }
                    None => break,
                }
            }
            new_list
        }

}

pub struct ListIter<'a, T> {
    next: Option<&'a Node<T>>,
}


impl<'a, T> Iterator for ListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next {
            Some(Node::Cons( elem,  next)) => {
                self.next = Some(next);
                Some(elem)
            }
            _ => None,
        }
    }
}


    // something that may be useful for the iterator implementation:
    // let a = Some(T);
    // let b = &a;
    // match b { Some(i) => ... } // here i is a reference to T
}

pub mod List2 {

    use std::rc::Rc;
    use std::cell::RefCell;

    type NodeLink<T> = Option<Rc<RefCell<DNode<T>>>>;

    pub struct DNode<T> {
        elem: T,
        prev: NodeLink<T>,
        next: NodeLink<T>,
    }

    pub struct DList<T> {
        head: NodeLink<T>,
        tail: NodeLink<T>,
    }

    impl<T> DList<T> {
        pub fn new() -> Self {
            Self {
                head: None,
                tail: None,
            }
        }

        pub fn push_front(&mut self, elem: T) {
            let new_node = Rc::new(RefCell::new(DNode {
                elem,
                prev: None,
                next: self.head.clone(),
            }));

            match self.head.take() {
                Some(old_head) => {
                    old_head.borrow_mut().prev = Some(new_node.clone());
                    self.head = Some(new_node);
                }
                None => {
                    self.tail = Some(new_node.clone());
                    self.head = Some(new_node);
                }
            }
        }

        pub fn pop_front(&mut self) -> Option<T> {
            self.head.take().map(|old_head| {
                if let Some(next) = old_head.borrow_mut().next.take() {
                    next.borrow_mut().prev = None;
                    self.head = Some(next);
                } else {
                    self.tail.take();
                }
                Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
            })
        }

        pub fn push_back(&mut self, elem: T) {
            let new_node = Rc::new(RefCell::new(DNode {
                elem,
                prev: self.tail.clone(),
                next: None,
            }));

            match self.tail.take() {
                Some(old_tail) => {
                    old_tail.borrow_mut().next = Some(new_node.clone());
                    self.tail = Some(new_node);
                }
                None => {
                    self.head = Some(new_node.clone());
                    self.tail = Some(new_node);
                }
            }
        }

        pub fn pop_back(&mut self) -> Option<T> {
            self.tail.take().map(|old_tail| {
                if let Some(prev) = old_tail.borrow_mut().prev.take() {
                    prev.borrow_mut().next = None;
                    self.tail = Some(prev);
                } else {
                    self.head.take();
                }
                Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
            })
        }

        pub fn popn(&mut self, n: usize) -> Option<T> {
            let mut current = self.head.clone();
            for _ in 0..n {
                current = current?.borrow().next.clone();
            }

            current.map(|node| {
                {
                    let mut node_ref = node.borrow_mut();
                    if let Some(prev) = node_ref.prev.take() {
                        prev.borrow_mut().next = node_ref.next.clone();
                    } else {
                        self.head = node_ref.next.clone();
                    }

                    if let Some(next) = node_ref.next.take() {
                        next.borrow_mut().prev = node_ref.prev.clone();
                    } else {
                        self.tail = node_ref.prev.clone();
                    }
                }

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

fn main() {}