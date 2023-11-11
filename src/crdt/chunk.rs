// Backup-fork of https://play.rust-lang.org/?gist=c3db81ec94bf231b721ef483f58deb35
use std::cell::RefCell;
use std::fmt::Display;
use std::rc::{Rc, Weak};

// The node type stores the data and two pointers.
//
// It uses Option to represent nullability in safe Rust. It has zero overhead
// over a null pointer due to the NonZero optimization.
//
// It uses an Rc (Reference Counted) pointer to give ownership of the next node
// to the current node. And a Weak (weak Reference Counted) pointer to reference
// the previous node without owning it.
//
// It uses RefCell for interior mutability. It allows mutation through
// shared references.

/*
using weak references on prev pointer as we want to drop current node if prev node drops

a------->b- - - ->a

in this case next pointer of a is b so next pointer on a which stores value of b is strong reference while
b also stores reference of a as prev which is week . hence if a is dropped b gets dropped automatically but if b gets dropped a wont get dropped




*/

type Id = (u32, u32, u32);
pub type T = String;

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub data: String,
    pub prev: Option<Weak<RefCell<Node<T>>>>,
    pub next: Option<Rc<RefCell<Node<T>>>>,
    pub id: Id,
    pub is_deleted: bool,
    pub length: u32,
    pub start: u32,
}

impl<T> Node<T> {
    // Constructs a node with some `data` initializing prev and next to null.
    pub fn new(data: String, id: Id, is_deleted: bool, start: u32, length: u32) -> Self {
        Self {
            data,
            prev: None,
            next: None,
            id,
            is_deleted,
            start,
            length,
        }
    }

    // Appends `data` to the chain of nodes. The implementation is recursive
    // but one could rewrite it to use a while-let imperative loop instead
    // without too much effort.
    pub fn append(
        node: &mut Rc<RefCell<Node<T>>>,
        data: String,
        id: Id,
        is_deleted: bool,
        start: u32,
        length: u32,
    ) -> Option<Rc<RefCell<Node<T>>>> {
        let is_last = node.borrow().next.is_none();
        if is_last {
            // If the current node is the last one, create a new node,
            // set its prev pointer to the current node, and store it as
            // the node after the current one.
            let mut new_node = Node::new(data, id, is_deleted, start, length);
            new_node.prev = Some(Rc::downgrade(&node));
            let rc = Rc::new(RefCell::new(new_node));
            node.borrow_mut().next = Some(rc.clone());
            Some(rc)
        } else {
            // Not the last node, just continue traversing the list:
            if let Some(ref mut next) = node.borrow_mut().next {
                Self::append(next, data, id, is_deleted, start, length)
            } else {
                None
            }
        }
    }
}

// The doubly-linked list with pointers to the first and last nodes in the list.
pub struct List<T> {
    first: Option<Rc<RefCell<Node<T>>>>,
    last: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> List<T> {

    // Constructs an empty list.
    pub fn new() -> Self {
        Self {
            first: None,
            last: None,
        }
    }





    // Appends a new node to the list, handling the case where the list is empty.
    pub fn append(&mut self, data: String, id: Id, is_deleted: bool, start: u32, length: u32) {
        if let Some(ref mut next) = self.first {
            self.last = Node::append(next, data, id, is_deleted, start, length);
        } else {
            let f = Rc::new(RefCell::new(Node::new(data, id, is_deleted, start, length)));
            self.first = Some(f.clone());
            self.last = Some(f);
        }
    }






    pub fn insert(
        &mut self,
        index: u32,
        data: String,
        id: Id,
        is_deleted: bool,
        start: u32,
        length: u32,
    ) {
        if index == 0 {
            if let Some(first_elem) = &self.first {
                let new_node =
                    Rc::new(RefCell::new(Node::new(data, id, is_deleted, start, length)));
                new_node.borrow_mut().next = Some(first_elem.clone());
                first_elem.borrow_mut().prev = Some(Rc::downgrade(&new_node).clone());
            } else {
                self.append(data, id, is_deleted, start, length);
            }
        } else {
            let mut i = 0;
            let mut cur_node_ref = self.first.clone();
            while i < index {
                if let Some(cur_node) = cur_node_ref {
                    cur_node_ref = cur_node.borrow().next.clone();
                    i = i + 1;
                } else {
                    self.append(data, id, is_deleted, start, length);
                    return;
                }
            }

            match cur_node_ref {
                Some(cur_node) => {
                    let new_node =
                        Rc::new(RefCell::new(Node::new(data, id, is_deleted, start, length)));
                    let prev_node = cur_node.borrow().prev.clone();

                    new_node.borrow_mut().next = Some(cur_node.clone());
                    new_node.borrow_mut().prev = prev_node.clone();
                    //Some(Rc::downgrade(&cur_node).clone());
                    cur_node.borrow_mut().prev = Some(Rc::downgrade(&new_node).clone());

                    if let Some(prev_node) = prev_node {
                        prev_node.upgrade().unwrap().borrow_mut().next = Some(new_node.clone());
                    }
                }
                None => {}
            }
        }
    }
}









// Pretty-printing
impl<T: Display> Display for List<T> {
    fn fmt(&self, w: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(w, "[")?;
        let mut node = self.first.clone();
        while let Some(n) = node {
            write!(w, "{}", n.borrow().data)?;
            node = n.borrow().next.clone();
            if node.is_some() {
                write!(w, ", ")?;
            }
        }
        write!(w, "]")
    }
}










#[test]
fn main() {
    let mut list: List<T> = List::new();
    println!("{}", list);

    list.insert(12, "hello".to_owned(), (0, 0, 0), false, 0, 5);
    list.append("hello".to_owned(), (0, 1, 0), false, 5, 5);
    //list.append("hello".to_owned(),(0,1,0),false,5,5);
    list.append("hello".to_owned(), (0, 2, 0), false, 10, 5);
    list.append("hello".to_owned(), (0, 3, 0), false, 15, 5);
    list.append("hello".to_owned(), (0, 4, 0), false, 20, 5);
    list.append("hello".to_owned(), (0, 5, 0), false, 25, 5);
    list.append("hello".to_owned(), (0, 6, 0), false, 30, 5);

    list.insert(2, "ooooo".to_owned(), (0, 7, 0), false, 10, 5);
    println!("{}", list);
}
