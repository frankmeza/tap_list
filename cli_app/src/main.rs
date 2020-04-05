use std::io;

mod ws_client;

fn main() {
    println!("\n Choose a beer to pour:\n");

    loop {
        // creates receiver variable for beer to pour
        let mut number_choice = String::new();
        io::stdin().read_line(&mut number_choice).unwrap().to_string();

        let beer_choice = number_choice.as_str().trim();
        println!("You chose {}", beer_choice);
    }
}
