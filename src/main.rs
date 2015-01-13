#![feature(plugin)]
#[plugin] #[nolink]
extern crate regex_macros;
extern crate regex;


use std::io::File;
use std::ascii::AsciiExt;

use std::collections::HashMap;

fn lowercase(s : &str) -> String {
    return s.to_ascii_lowercase();
}

fn source() -> String {
    match File::open(&Path::new("gettysburg.txt")).read_to_string() {
        Ok(text) => lowercase(text.as_slice()),
        Err(err) => panic!("Cannot open model data: {}", err),
    }
}

fn word_counts(t: &str) -> HashMap<&str, i32>  {
    let mut words = HashMap::<&str, i32>::new();

    for (start, end) in regex!("[a-z]+").find_iter(t) {
        let key = &t[start..end];
        let new_count = match words.get_mut(key) {
            Some(count) => *count  + 1,
            None => 1,
        };
        words.insert(key, new_count);
        println!("{}", key);
    }

    words
}

fn main() {
    let s = source();  // separate line to persuade borrow-checker
    let t = s.as_slice();

    let ws = word_counts(t);

}
