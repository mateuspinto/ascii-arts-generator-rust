#[cfg(test)]
use super::Art;
#[test]
fn create_art_from_file() -> Result<(), ()> {
        let painting = match Art::new_from_file("input/0.txt".to_string()) {
                Ok(_o) => Ok(()),
                Err(_e) => Err(()),
        };
        painting
}

#[test]
fn more_chars_than_painting_len_throw_error() -> Result<(), ()> {
        let painting = match Art::new_blank(10, 10) {
                Ok(o) => o,
                Err(e) => panic!(e),
        };
        let painting = match painting.insert_characters_randomly('*', 101) {
                Err(e) if e == std::io::ErrorKind::UnexpectedEof => Ok(()),
                _ => Err(()),
        };
        painting
}
