mod art;

fn main() {
    let blank = match art::Art::new_blank(20, 150) {
        Ok(o) => o,
        Err(e) => panic!("Can't create blank: {:?}", e),
    };

    let planet0 = match art::Art::new_from_file("input/0.txt".to_string()) {
        Ok(o) => o,
        Err(e) => panic!("File error: {:?}", e),
    };

    let blank = match blank.insert_art_randomly(&planet0, 2) {
        Ok(o) => o,
        Err(e) => panic!("Place error: {:?}", e),
    };

    let planet1 = match art::Art::new_from_file("input/1.txt".to_string()) {
        Ok(o) => o,
        Err(e) => panic!("File error: {:?}", e),
    };

    let blank = match blank.insert_art_randomly(&planet1, 2) {
        Ok(o) => o,
        Err(e) => panic!("Place error: {:?}", e),
    };

    let blank = match blank.insert_characters_randomly('*', 50) {
        Ok(o) => o,
        Err(e) => panic!("Place error: {:?}", e),
    };

    println!("{}", blank);
}