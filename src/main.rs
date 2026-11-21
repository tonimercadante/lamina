use std::{env, fs::File, io::Read, path::Path};

fn main() {
    let args: Vec<_> = env::args().collect();
    let file_path = match args.get(1) {
        Some(fp) => fp,
        None => {
            eprintln!("Usage: lamina <file_path>");
            return;
        }
    };
    let path = Path::new(file_path);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} \n{}", display, s),
    }
}
