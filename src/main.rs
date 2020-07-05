mod art;

fn main() {
    let xis = match art::Art::new_from_file("input/x.txt".to_string()) {
        Ok(o) => o,
        Err(e) => panic!(e),
    };

    let blank = art::Art::new_blank(10, 100);

    let blank = match blank.insert_art(&xis, 20, 2) {
        Ok(o) => o,
        Err(e) => panic!(e),
    };

    let blank = match blank.insert_characters_randomly('*', 20) {
        Ok(o) => o,
        Err(e) => panic!(e),
    };

    println!("{}", blank);
}
