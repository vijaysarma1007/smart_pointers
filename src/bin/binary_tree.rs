use std::cmp::Ordering;

#[derive(Debug)]
enum BinarySearchTree {
    Empty,
    Node {
        value: i32,
        left: Box<BinarySearchTree>,
        right: Box<BinarySearchTree>,
    },
}

impl BinarySearchTree {
    fn new() -> Self {
        BinarySearchTree::Empty
    }

    fn insert(&mut self, new_value: i32) {
        match self {
            BinarySearchTree::Empty => {
                *self = BinarySearchTree::Node {
                    value: new_value,
                    left: Box::new(BinarySearchTree::Empty),
                    right: Box::new(BinarySearchTree::Empty),
                }
            },
            BinarySearchTree::Node { value, left, right } => {
                   match new_value.cmp(value) {
                    Ordering::Equal => (),
                    Ordering::Less => left.insert(new_value),
                    Ordering::Greater => right.insert(new_value),
                   }
                   
            }
        }
    }

    fn contains(&self, target:i32) -> bool {
        match self{
            BinarySearchTree::Empty => false,
            BinarySearchTree::Node { value, left, right } => {
                match  target.cmp(value) {
                    Ordering::Equal => true,
                    Ordering::Less => left.contains(target),
                    Ordering::Greater => right.contains(target)
                }
            }
        }
    }
}

fn main() {
    let mut tree = BinarySearchTree::new();
    tree.insert(5);
    tree.insert(8);
    tree.insert(2);
    tree.insert(11);
    println!("{:#?}", tree);

    let contains = tree.contains(12);
    println!("contains: {:#?}", contains);
}
