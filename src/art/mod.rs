#![allow(dead_code)]
use rand::Rng;
use std::fmt::{Display, Error, Formatter};
use std::fs::File;
use std::io::prelude::*;

const BLANK_SPACE: char = '.';

pub struct Art {
    grid: Vec<char>,
    widht: usize,
}

impl Art {
    pub fn new_blank(height: usize, widht: usize) -> Art {
        Art {
            grid: vec![BLANK_SPACE; height * widht],
            widht: widht,
        }
    }

    pub fn new_from_file(filename: String) -> Result<Art, String> {
        let mut file = match File::open(filename) {
            Ok(o) => o,
            Err(_e) => return Err(String::from("File doesn't exist!")),
        };
        let mut file_contents = String::new();
        match file.read_to_string(&mut file_contents) {
            Ok(o) => o,
            Err(_e) => return Err(String::from("File is corrupted!")),
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
        self,
        character: char,
        quantity: usize,
    ) -> Result<Art, String> {
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
                return Err(String::from("Art cannot hold all characters!"));
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
