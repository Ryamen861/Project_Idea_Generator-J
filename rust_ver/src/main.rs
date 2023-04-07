use rand::Rng;


fn main() {
    println!("Welcome to the Project Idea Generator!");

    let index = rand::thread_rng().gen_range(1..10);
    println!("the index is {}", index);
}
