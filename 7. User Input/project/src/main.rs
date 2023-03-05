use std::io;

fn main() {
    let mut input = String::new();
    println!("Write smth: ");
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            print!("You wrote {}", input);
        }
        Err(e) => {
            print!("Opps! Smth went wrong {}", e);
        }
    }

}
