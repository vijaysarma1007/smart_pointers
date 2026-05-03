#[derive(Debug)]
enum LinkedListUsingBox<T> {
    Empty,
    Node {
        value: T,
        next: Box<LinkedListUsingBox<T>>,
    },
}

#[derive(Debug)]
enum LinkedListUsingReference<'a, T> {
    Empty,
    Node {
        value: T,
        next: &'a LinkedListUsingReference<'a, T>,
    },
}


fn main() {
    let second_node = LinkedListUsingReference::Node {
        value: 2,
        next: &LinkedListUsingReference::Empty,
    };

    let first_node = LinkedListUsingReference::Node {
        value: 1,
        next: &second_node,
    };

    let second_note = LinkedListUsingBox::Node {
        value: 1,
        next: Box::new(LinkedListUsingBox::Empty),
    };

    let frist_note = LinkedListUsingBox::Node {
        value: 1,
        next: Box::new(second_note),
    };

    drop(frist_note);
  
}
