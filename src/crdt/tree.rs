use crate::crdt::chunk::{Node as Chunk, T};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct Node<'a> {
    pub keys: Vec<i32>,
    pub children: Vec<Rc<RefCell<Node<'a>>>>,
    pub is_leaf: bool,
    pub chunk: Option<&'a Chunk<T>>,
}

impl<'a> Node<'a> {
    pub fn new() -> Node<'a> {
        Node {
            keys: vec![],
            children: vec![],
            is_leaf: true,
            chunk: None,
        }
    }
}

pub struct Btree<'a> {
    pub root: Rc<RefCell<Node<'a>>>,
    pub t: usize,
}

impl<'a> Btree<'a> {
    pub fn new(t: usize) -> Btree<'a> {
        Btree {
            root: Rc::new(RefCell::new(Node::new())),
            t: t,
        }
    }

    pub fn insert(&mut self, val: i32) {
        let mut root = self.root.clone();

        if root.borrow().keys.len() == (2 * self.t) - 1 {
            let mut temp = Rc::new(RefCell::new(Node::new()));
            temp.borrow_mut().children.push(self.root.clone()); // Add root as the first child
            self.root = temp.clone();
            self.split_child(temp, 0);
        }

        self.insert_non_full(self.root.clone(), val);
    }

    pub fn insert_non_full(&mut self, par: Rc<RefCell<Node>>, val: i32) {
        let mut p = par.clone();
        let mut parent = p.borrow_mut();
        //println!("{:?}", parent.keys.len());

        let mut i = parent.keys.len();
        if parent.is_leaf {
            parent.keys.push(0);
            while i > 0 && val < parent.keys[i - 1] {
                parent.keys[i] = parent.keys[i - 1];
                i = i - 1;
            }
            parent.keys[i] = val;
        } else {
            while i > 0 && val < parent.keys[i - 1] {
                i = i - 1;
            }
            if parent.children[i].borrow_mut().keys.len() == (2 * self.t) - 1 {
                self.split_child(par.clone(), i);
                if val > parent.keys[i] {
                    i = i + 1;
                }
            }
            self.insert_non_full(parent.children[i].clone(), val)
        }
    }

    pub fn split_child(&self, nod: Rc<RefCell<Node>>, i: usize) {
        let mut t = self.t;
        let mut node = nod.borrow_mut();
        let mut y0 = node.children[i].clone();
        let mut y = y0.borrow_mut();
        let mut z0 = Rc::new(RefCell::new(Node::new()));
        let mut z = z0.borrow_mut();
        z.is_leaf = y.is_leaf;
        node.children.insert(i + 1, z0.clone());
        node.keys.insert(i, y.keys[t - 1].clone());
        z.keys = y.keys[t..(2 * t - 1)].to_vec();
        y.keys = y.keys[0..(2 * t - 1)].to_vec();
        if !y.is_leaf {
            z.children = y.children[t..2 * t].to_vec();
            y.children = y.children[0..t - 1].to_vec();
        }
    }

    fn print_tree(&self, node: &Rc<RefCell<Node>>, level: usize) {
        print!("Level {}: ", level);
        let node_borrow = node.borrow(); // Get an immutable reference

        for key in &node_borrow.keys {
            print!("{} ", key);
        }
        println!();

        if !node_borrow.children.is_empty() {
            for child in &node_borrow.children {
                self.print_tree(child, level + 1);
            }
        }
    }

    pub fn estimate_memory_usage(&self) -> usize {
        // This is a rough estimate and doesn't account for all details like internal data structures.
        std::mem::size_of_val(self)
            + self.root.borrow().keys.capacity() * std::mem::size_of::<i32>()
            + self.root.borrow().children.capacity() * std::mem::size_of::<Rc<RefCell<Node>>>()
    }
}

#[test]
pub fn main() {
    let start_time = Instant::now();
    let mut btree = Btree::new(123);
    let x = 100000;
    for i in 0..x {
        btree.insert(i);
    }
    btree.print_tree(&btree.root, 0);
    let elapsed_time = start_time.elapsed();
    println!("Time taken: {:?}", elapsed_time);
    println!(
        "estimated memory usage for {:?} nodes is {:?} bytes",
        x,
        btree.estimate_memory_usage()
    )
}
