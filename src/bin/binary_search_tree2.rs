use std::cmp::Ordering;

#[derive(Debug)]
enum BinarySearchTree {
    Empty,
    Node {
        value: i32,
        level: i32, // Represents the depth of the node
        left: Box<BinarySearchTree>,
        right: Box<BinarySearchTree>,
    },
}

impl BinarySearchTree {
    fn new() -> Self {
        BinarySearchTree::Empty
    }

    // Public insert method (starts at level 0)
    fn insert(&mut self, new_value: i32) {
        self.insert_at_level(new_value, 0);
    }

    // Helper method to handle the recursive level incrementing
    fn insert_at_level(&mut self, new_value: i32, current_level: i32) {
        match self {
            BinarySearchTree::Empty => {
                *self = BinarySearchTree::Node {
                    value: new_value,
                    left: Box::new(BinarySearchTree::Empty),
                    right: Box::new(BinarySearchTree::Empty),
                    level: current_level, // Set the calculated level
                }
            }
            BinarySearchTree::Node {
                value, left, right, ..
            } => {
                match new_value.cmp(value) {
                    Ordering::Equal => (), // Value already exists
                    Ordering::Less => left.insert_at_level(new_value, current_level + 1),
                    Ordering::Greater => right.insert_at_level(new_value, current_level + 1),
                }
            }
        }
    }

    fn contains(&self, target: i32) -> bool {
        match self {
            BinarySearchTree::Empty => false,
            BinarySearchTree::Node {
                value, left, right, ..
            } => match target.cmp(value) {
                Ordering::Equal => true,
                Ordering::Less => left.contains(target),
                Ordering::Greater => right.contains(target),
            },
        }
    }
}

fn main() {
    let mut tree = BinarySearchTree::new();
    tree.insert(5); // Level 0 (Root)
    tree.insert(8);
    tree.insert(3); // Level 1
    tree.insert(2); // Level 1
    tree.insert(11); // Level 2

    println!("{:#?}", tree);

    let target = 11;
    println!("Contains {}: {}", target, tree.contains(target));
}
