use std::ops::{Deref, DerefMut};

struct CustomBox<T, U> {
    data: T,
    more_data: U,
}

impl<T, U> CustomBox<T, U> {
    fn new(data: T, more_data: U) -> Self {
        Self { data, more_data }
    }
}

impl<T, U> Deref for CustomBox<T, U> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T, U> DerefMut for CustomBox<T, U> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

fn main() {
    let boxy = Box::new(3.14);
    println!("{}", *boxy);

    let mut custom_boxy = CustomBox::new(3.14, 2.13);
    *custom_boxy = 6.25;
    println!("{}", *custom_boxy);
}
