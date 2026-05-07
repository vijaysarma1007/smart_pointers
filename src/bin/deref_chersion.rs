use std::ops::Deref;

fn main(){
    let text = String::from("Hello");
    let my_box = Box::new(text);
    outtput_text(&my_box);
}


fn outtput_text(text: &str){
    println!("{}", text);
}