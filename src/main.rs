use std::fs;

fn main() {
    let a = "hello rust world";

    println!("Hello, world! {}", a);

    let b = "rust world";
    println!("Hello, world! {}", b);
    println!("Hello, world!");

    let c = 1;
    println!("Hello, world! {}", c);

    let ss = a.len();
    for i in 0..10 {
        println!("{2} '{0}' length is {1}", a, ss, i);
    }
    let text = fs::read_to_string("D:/DataHub/StudyRust/src/test.d").unwrap();
    println!("{}", text);

    eat_at_restaurant();
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}
pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}
