#![allow(dead_code)]
use rand::Rng;
use std::fmt::{Display, Error, Formatter};
use std::fs::File;
use std::io::prelude::*;

mod tests;

const BLANK_SPACE: char = '.';

pub struct Art {
    grid: Vec<char>,
    widht: usize,
}

impl Art {
    pub fn height(&self) -> usize {
        self.grid.len() / self.widht
    }
    pub fn new_blank(height: usize, widht: usize) -> Art {
        Art {
            grid: vec![BLANK_SPACE; height * widht],
            widht: widht,
        }
    }

    pub fn new_from_file(filename: String) -> Result<Art, std::io::ErrorKind> {
        let mut file = match File::open(filename) {
            Ok(o) => o,
            Err(_e) => return Err(std::io::ErrorKind::NotFound),
        };
        let mut file_contents = String::new();
        match file.read_to_string(&mut file_contents) {
            Ok(o) => o,
            Err(_e) => return Err(std::io::ErrorKind::InvalidData),
        };

        let mut data: Vec<char> = Vec::new();
        let mut widht: usize = 1;

        for x in file_contents.char_indices() {
            if x.1 == '\n' {
                widht += 1;
            } else {
                data.push(x.1);
            }
        }

        Ok(Art {
            grid: data,
            widht: widht,
        })
    }

    pub fn insert_characters_randomly(
        &self,
        character: char,
        quantity: usize,
    ) -> Result<Art, std::io::ErrorKind> {
        if quantity > self.grid.len() {
            return Err(std::io::ErrorKind::UnexpectedEof);
        }
        let mut new_painting = self.clone();
        let mut generator = rand::thread_rng();
        let mut misses = 0;
        let mut hits = 0;

        loop {
            let position = generator.gen_range(0, new_painting.grid.len());
            if new_painting.grid[position] == BLANK_SPACE {
                new_painting.grid[position] = character;
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

    pub fn insert_art(
        &self,
        to_be_pasted: &Art,
        x: usize,
        y: usize,
    ) -> Result<Art, std::io::ErrorKind> {
        if y * to_be_pasted.widht + x > self.grid.len() {
            // Art doens't fit in destination
            return Err(std::io::ErrorKind::UnexpectedEof);
        }
        for j in 0..to_be_pasted.height() {
            for i in 0..to_be_pasted.widht {
                if self.grid[(j + y) * self.widht + (i + x)] != BLANK_SPACE
                    && to_be_pasted.grid[j * to_be_pasted.widht + i] != BLANK_SPACE
                {
                    // There is a conflict in some character
                    return Err(std::io::ErrorKind::InvalidData);
                }
            }
        }
        let mut new_painting = self.clone();
        for j in 0..to_be_pasted.height() {
            for i in 0..to_be_pasted.widht {
                if to_be_pasted.grid[j * to_be_pasted.widht + i] != BLANK_SPACE {
                    new_painting.grid[(j + y) * new_painting.widht + (i + x)] =
                        to_be_pasted.grid[j * to_be_pasted.widht + i];
                }
            }
        }
        Ok(new_painting)
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
        let mut painting = String::new();

        for (i, x) in self.grid.iter().enumerate() {
            painting.push(*x);
            if (i + 1) % self.widht == 0 && (i + 1) != self.grid.len() {
                painting.push('\n');
            }
        }

        write!(f, "{}", painting)
    }
}
