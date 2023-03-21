// use crate::archive::arch::arch_file;
use crate::archive::arch::arch_file as arc;
use rand::Rng;

mod archive;

fn main() {
    // arch_file("SomeText.txt");
    arc("SomeText.txt");

    let mut rng = rand::thread_rng();
    let a: i32 = rng.gen();

    println!("Random number: {}", a);
}
