mod art;

fn main() {
    let mateus = match art::Art::new_from_file("input/x.txt".to_string()) {
        Ok(o) => o,
        Err(e) => panic!("{}", e),
    };

    println!("{}", mateus);
}
