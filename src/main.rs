use std::env;

fn main() {
    let params: Vec<_> = env::args().collect();
    println!("{}", params[1]);
}
