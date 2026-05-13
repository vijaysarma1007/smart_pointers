use std::error::Error;
use std::fmt::Display;
use std::fs;

#[derive(Debug)]
struct NumberIsUnimpressiveError {
    value: i32,
}

impl Display for NumberIsUnimpressiveError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "That number is just too small:  {}", self.value)
    }
}

impl Error for NumberIsUnimpressiveError {}

fn read_number_from_fle(path: &str) -> Result<i32, Box<dyn Error>> {
    let file_content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(error) => return Err(Box::new(error)),
    };
    let number = match file_content.parse::<i32>() {
        Ok(content) => content,
        Err(error) => return Err(Box::new(error)),
    };

    if number < 100 {
        Err(Box::new(NumberIsUnimpressiveError { value: number }))
    } else {
        Ok(number)
    }
}

fn main() {
    match read_number_from_fle("value.txt") {
        Ok(value) => println!("the number is {value}"),
        Err(error) => println!("The error is {error}"),
    }
}
