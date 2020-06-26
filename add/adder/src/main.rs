use add_one;
use rand;

fn main() {
    let num = rand::random::<i32>();
    println!("Hello, world! {} pluse one is {}!",
        num,
        add_one::add_one(num));
}
