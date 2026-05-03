fn main() {
    let sushi = String::from("yellowtail");
    let sushi_raw_pointer1 = &raw const sushi;
    println!("{:?}", sushi_raw_pointer1);

    unsafe {
        println!("{}", *sushi_raw_pointer1);
    }
}
