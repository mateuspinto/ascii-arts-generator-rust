#![allow(dead_code)]
use rand::Rng;
use std::fmt::{Display, Error, Formatter};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod tests;

const BLANK_SPACE: char = '.';

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub struct Art {
    grid: Vec<char>,
    widht: usize,
}

impl Art {
    fn height(&self) -> usize {
        self.grid.len() / self.widht
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.widht + x
    }

    fn get_char(&self, x: usize, y: usize) -> char {
        self.grid[self.get_index(x, y)]
    }

    fn is_char_blank(&self, x: usize, y: usize) -> bool {
        self.get_char(x, y) == BLANK_SPACE
    }

    fn get_size(&self) -> (usize, usize) {
        (self.widht, self.height())
    }

    fn get_increased_size(&self, x_offset: usize, y_offset: usize) -> (usize, usize) {
        let mut size = self.get_size();
        size.0 += x_offset;
        size.1 += y_offset;
        size
    }

    fn size_greater_or_equal_than(&self, to_be_compared: &Art) -> bool {
        let size_a = self.get_size();
        let size_b = to_be_compared.get_size();

        if size_a.0 >= size_b.0 && size_a.1 >= size_b.1 {
            true
        } else {
            false
        }
    }

    fn size_greater_or_equal_than_increased(
        &self,
        to_be_compared: &Art,
        x_offset: usize,
        y_offset: usize,
    ) -> bool {
        let size_a = self.get_size();
        let size_b = to_be_compared.get_increased_size(x_offset, y_offset);

        if size_a.0 >= size_b.0 && size_a.1 >= size_b.1 {
            true
        } else {
            false
        }
    }

    fn has_colisions(&self, to_be_tested: &Art, x_offset: usize, y_offset: usize) -> bool {
        for j in 0..to_be_tested.height() {
            for i in 0..to_be_tested.widht {
                if !self.is_char_blank(i + x_offset, j + y_offset)
                    && !to_be_tested.is_char_blank(i, j)
                {
                    return true;
                }
            }
        }
        false
    }

    fn raw_new_blank(height: usize, widht: usize) -> Art {
        Art {
            grid: vec![BLANK_SPACE; height * widht],
            widht: widht,
        }
    }

    pub fn new_blank(height: usize, widht: usize) -> Result<Art, std::io::ErrorKind> {
        if height == 0 || widht == 0 {
            Err(std::io::ErrorKind::UnexpectedEof)
        } else {
            Ok(Art::raw_new_blank(height, widht))
        }
    }

    pub fn new_from_file(filename: String) -> Result<Art, std::io::ErrorKind> {
        let mut data: Vec<char> = Vec::new();
        let mut height: usize = 0;

        if let Ok(lines) = read_lines(filename) {
            for line in lines {
                if let Ok(raw_line) = line {
                    for c in raw_line.chars() {
                        data.push(c);
                    }
                    height += 1;
                } else {
                    return Err(std::io::ErrorKind::InvalidData);
                }
            }

            let widht = data.len() / height;
            Ok(Art {
                grid: data,
                widht: widht,
            })
        } else {
            Err(std::io::ErrorKind::NotFound)
        }
    }

    pub fn insert_characters_randomly(
        &self,
        character: char,
        quantity: usize,
    ) -> Result<Art, std::io::ErrorKind> {
        if quantity > self.grid.len() {
            // Art doens't fit in destination
            return Err(std::io::ErrorKind::UnexpectedEof);
        }
        let mut new_painting = self.clone();
        let mut generator = rand::thread_rng();
        let mut misses = 0;
        let mut hits = 0;

        loop {
            let x = generator.gen_range(0, self.widht);
            let y = generator.gen_range(0, self.height());

            if new_painting.is_char_blank(x, y) {
                new_painting.grid[self.get_index(x, y)] = character;
                hits += 1;
            } else {
                misses += 1;
            }

            if hits == quantity {
                return Ok(new_painting);
            } else if misses == 2 * new_painting.grid.len() {
                return Err(std::io::ErrorKind::InvalidData);
            }
        }
    }

    fn raw_insert_art(&self, to_be_pasted: &Art, x: usize, y: usize) -> Art {
        let mut new_painting = self.clone();
        for j in 0..to_be_pasted.height() {
            for i in 0..to_be_pasted.widht {
                if !to_be_pasted.is_char_blank(i, j) {
                    new_painting.grid[self.get_index(x + i, y + j)] = to_be_pasted.get_char(i, j);
                }
            }
        }
        new_painting
    }

    pub fn insert_art(
        &self,
        to_be_pasted: &Art,
        x: usize,
        y: usize,
    ) -> Result<Art, std::io::ErrorKind> {
        if !self.size_greater_or_equal_than_increased(to_be_pasted, x, y) {
            // Art doens't fit in destination
            return Err(std::io::ErrorKind::UnexpectedEof);
        }
        if self.has_colisions(to_be_pasted, x, y) {
            // Art has colisions between source and destination
            return Err(std::io::ErrorKind::InvalidData);
        }
        Ok(self.raw_insert_art(to_be_pasted, x, y))
    }

    pub fn insert_art_randomly(
        &self,
        to_be_pasted: &Art,
        quantity: usize,
    ) -> Result<Art, std::io::ErrorKind> {
        let mut new_painting = self.clone();
        let mut generator = rand::thread_rng();
        let mut misses = 0;
        let mut hits = 0;

        loop {
            let x = generator.gen_range(0, self.widht - to_be_pasted.widht);
            let y = generator.gen_range(0, self.height() - to_be_pasted.height());

            if !new_painting.has_colisions(to_be_pasted, x, y) {
                new_painting = new_painting.raw_insert_art(to_be_pasted, x, y);
                hits += 1;
            } else {
                misses += 1;
            }

            if hits == quantity {
                return Ok(new_painting);
            } else if misses == 100 * (new_painting.grid.len() / to_be_pasted.grid.len()) {
                return Err(std::io::ErrorKind::InvalidData);
            }
        }
    }
}

impl Clone for Art {
    fn clone(&self) -> Art {
        Art {
            grid: self.grid.clone(),
            widht: self.widht,
        }
    }
}

impl Display for Art {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut display = String::new();

        for j in 0..self.height() {
            for i in 0..self.widht {
                display.push(self.get_char(i, j));
            }
            display.push('\n');
        }

        write!(f, "{}", display)
    }
}
