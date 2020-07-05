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
fn create_art_from_file_dont_exist_throw_error() -> Result<(), ()> {
        let painting = match Art::new_from_file("input/dontexist.txt".to_string()) {
                Err(e) if e == std::io::ErrorKind::NotFound => Ok(()),
                _ => Err(()),
        };
        painting
}

#[test]
fn painting_can_hold_chars() -> Result<(), ()> {
        let painting = match Art::new_blank(10, 10) {
                Ok(o) => o,
                Err(e) => panic!(e),
        };
        let painting = match painting.insert_characters_randomly('*', 20) {
                Ok(_o) => Ok(()),
                _ => Err(()),
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

#[test]
fn small_paint_to_large_one_throw_error() -> Result<(), ()> {
        let big = match Art::new_blank(10, 10) {
                Ok(o) => o,
                Err(e) => panic!(e),
        };
        let small = match Art::new_blank(1, 1) {
                Ok(o) => o,
                Err(e) => panic!(e),
        };
        let big = match big.insert_art(&small, 0, 0) {
                Ok(_o) => Ok(()),
                _ => Err(()),
        };
        big
}

#[test]
fn large_paint_to_small_one_throw_error() -> Result<(), ()> {
        let big = match Art::new_blank(10, 10) {
                Ok(o) => o,
                Err(e) => panic!(e),
        };
        let small = match Art::new_blank(1, 1) {
                Ok(o) => o,
                Err(e) => panic!(e),
        };
        let small = match small.insert_art(&big, 0, 0) {
                Err(e) if e == std::io::ErrorKind::UnexpectedEof => Ok(()),
                _ => Err(()),
        };
        small
}