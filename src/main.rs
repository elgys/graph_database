mod database;

use database::{deserilize::Deserilizer, serilize::serilizer};
use std::{
    fs::File,
    io::{Read, Write},
};

fn main() {
    let num: i16 = -189;
    let word: String = String::from("deadBringer");

    let mut buffer: [u8; 15] = [0; 15];
    serilizer::i16_serilize(num, &mut buffer[0..2]);
    serilizer::uft8_serilize(&word, &mut buffer[2..15]);

    let mut file = File::create("test.grp").unwrap();
    let result = file.write_all(&buffer);
    println!("{:?}", result);

    let mut read = File::open("test.grp").unwrap();
    let mut text: Vec<u8> = Vec::new();
    read.read_to_end(&mut text).unwrap();
    let readnum = Deserilizer::i16_deserilize(&text[0..2]).unwrap();
    let readword = Deserilizer::uft8_deserilize(&text[2..15], 13);
    println!("{} {}", readnum, readword);
}
