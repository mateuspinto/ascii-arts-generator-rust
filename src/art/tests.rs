#[cfg(test)]
use super::Art;
#[test]
fn create_art_from_file() -> Result<(), String> {
        let painting = match Art::new_from_file("input/x.txt".to_string()) {
                Ok(_o) => Ok(()),
                Err(e) => Err(e),
        };
        painting
}

#[test]
fn more_chars_than_painting_len_throw_error() -> Result<(), String> {
        let painting = Art::new_blank(10, 10);
        let painting = match painting.insert_characters_randomly('*', 101) {
                Ok(_o) => Err("Excepecting error, but runned correctly".to_string()),
                Err(_e) => Ok(()),
        };
        painting
}
