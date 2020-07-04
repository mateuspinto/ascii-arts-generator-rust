#[cfg(test)]
use super::Art;
#[test]
fn create_art_from_file() {
        let _painting = match Art::new_from_file("input/x.txt".to_string()) {
                Ok(o) => o,
                Err(e) => panic!(e),
        };
}

#[test]
fn more_chars_than_painting_len_throw_error() {
        let painting = Art::new_blank(10, 10);
        let _painting = match painting.insert_characters_randomly('*', 101) {
                Ok(o) => panic!(o),
                Err(e) => e,
        };
}
