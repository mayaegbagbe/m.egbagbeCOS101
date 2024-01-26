use std::io::Read;
fn main() {
    let mut file = std::fs::File::create("High quality categories of drink").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

