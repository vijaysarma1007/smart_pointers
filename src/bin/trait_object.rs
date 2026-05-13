trait Wearable {
    fn wear(&self) -> String;
}

#[derive(Debug)]
struct Pants {
    fabric: String,
    waist_size: u32,
}

impl Wearable for Pants {
    fn wear(&self) -> String {
        format!("{} {} pants", self.waist_size, self.fabric)
    }
}

#[derive(Debug)]
struct Tie {
    color: String,
}

impl Wearable for Tie {
    fn wear(&self) -> String {
        format!("{} tie", self.color)
    }
}

fn main() {
    let pants = Pants {
        fabric: "Cotton".to_string(),
        waist_size: 34,
    };

    let tie = Tie {
        color: "Red".to_string(),
    };

    let outfit: Vec<Box<dyn Wearable>> = vec![Box::new(pants), Box::new(tie)];

    for item in outfit {
        println!(" Putting on the {}", item.wear());
    }
}
