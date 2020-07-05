mod art;

fn main() {
    let blank = match art::Art::new_blank(10, 100) {
        Ok(o) => o,
        Err(e) => panic!(e),
    };

    let xis = match art::Art::new_from_file("input/x.txt".to_string()) {
        Ok(o) => o,
        Err(e) => panic!(e),
    };

    let new = match blank.insert_art(&xis, 0, 0) {
        Ok(o) => o,
        Err(e) => panic!("{:?}", e),
    };

    println!("{}", new);
}
