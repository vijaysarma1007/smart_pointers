#[derive(Debug)]
enum LinkedList<T> {
    Empty,
    Node { value: T, next: Box<LinkedList<T>> },
}

fn main() {
    let list = LinkedList::<i32>::Node {
        value: 1,
        next: Box::new(LinkedList::Node {
            value: 2,
            next: Box::new(LinkedList::Node {
                value: 3,
                next: Box::new(LinkedList::Empty),
            }),
        }),
    };

    println!("{list:#?}");

    let im_with_you = LinkedList::<String>::Node {
        value: String::from("I'm with you"),
        next: Box::new(LinkedList::Empty),
    };

    let skter_boi = LinkedList::<String>::Node {
        value: String::from("Skter boi"),
        next: Box::new(im_with_you),
    };

    let complicated = LinkedList::<String>::Node {
        value: String::from("Complicated"),
        next: Box::new(skter_boi),
    };

    println!("{:#?}", complicated);
}
