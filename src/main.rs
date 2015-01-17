#![allow(unstable)]
#![feature(plugin)]
#![feature(slicing_syntax)]
#[plugin]

extern crate regex_macros;
extern crate regex;

use std::io::File;
use std::ascii::AsciiExt;
use std::collections::HashMap;
use std::collections::HashSet;

fn lowercase(s : &str) -> String {
    return s.to_ascii_lowercase();
}

fn source() -> String {
    match File::open(&Path::new("gettysburg.txt")).read_to_string() {
        Ok(text) => lowercase(&text[]),
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
    }
    words
}

// returns a tuple
fn split(s: &str, pos: usize) -> (&str, &str) {
    (&s[0..pos], &s[pos..s.len()])
}

fn alphabet() -> &'static str {
    "abcdefghijklmnopqrstuvwxyz"
}

fn splits(s: &str) -> Vec<(&str, &str)> {
    let mut v = Vec::<(&str, &str)>::new();
    for pos in 0..s.len()+1 {
        v.push(split(s, pos));
    }
    v
}

fn deletes<'a, 'b>(splits : & 'a Vec<(& 'b str, & 'b str)>) ->  Vec<String> {
    let mut v = Vec::<String>::new();
    for &(a, b) in splits.iter() {
        if b.len() > 0 {
            v.push(a.to_string() +  &b[1..b.len()]);
        }
    }
    v
}

fn transposes<'a, 'b>(splits : & 'a Vec<(& 'b str, & 'b str)>) ->  Vec<String> {
    let mut v = Vec::<String>::new();
    for &(a, b) in splits.iter() {
        if b.len() > 1 {
            v.push(a.to_string() + &b[1..2] + &b[0..1] + &b[2..b.len()]);
        }
    }
    v
}

fn inserts(splits: &Vec<(&str, &str)>) -> Vec<String> {
    let mut v = Vec::<String>::new();
    for &(a, b) in splits.iter() {
        for c in alphabet().chars() {
            let mut s = a.to_string();
            s.push(c);
            s.push_str(b);
            v.push(s);
        }
    }
    v
}

fn replaces(splits: &Vec<(&str, &str)>) -> Vec<String> {
    let mut v = Vec::<String>::new();
    for &(a, b) in splits.iter() {
        if b.len() > 0 {
            for c in alphabet().chars() {
                let mut s = a.to_string();
                s.push(c);
                s.push_str(&b[1..b.len()]);
                v.push(s);
            }
        }
    }
    v
}

fn insert_all(set : &mut HashSet<String>, vec : Vec<String>) {
    // TODO there must be a very generic way to do this! FromIter?
    for s in vec.into_iter() {
        (*set).insert(s);
    }
}

fn edits1(word: &str) -> HashSet<String> {
    let mut set = HashSet::<String>::new();
    let splits = splits(word);
    insert_all(&mut set, deletes(&splits));
    insert_all(&mut set, inserts(&splits));
    insert_all(&mut set, transposes(&splits));
    insert_all(&mut set, replaces(&splits));
    set
}

// TODO words should be an iterator?
fn known(words: &HashSet<String>, model: &HashMap<&str, i32>) -> HashSet<String> {
    let mut set = HashSet::<String>::new();
    for word in words.iter() {
        if model.contains_key(&word[]) {
            set.insert(word.to_string());
        }
    }
    set
}

fn main() {
    let s = source();  // separate line to persuade borrow-checker
    let t = &s[];

    let ws = word_counts(t);
    println!("TADA");
    for a in known(&edits1("theer"), &ws).iter() {
        println!("{}", a);
    }

}

#[test]
fn str_eq_string() {
    // tests that a &str Eq's an equivalent String
    let s1 = "abcdef";
    let s2 = "abcdef".to_string();
    assert_eq!(s1, s2);
}

#[test]
fn test_splits() {
    let expected = vec!( ("", "ab"), ("a", "b"), ("ab", ""));
    assert_eq!(splits("ab"), expected);
}

#[test]
fn test_deletes()
{
    let s = deletes(&splits("abc"));
    let expected = vec!("bc","ac", "ab");
    assert_eq!(s, expected);
}

// #[test]
// fn test_combine() {
//     let split = vec!(("", "bc"), ("a", "c"), ("ab", ""));
//     let combined = vec!("bc".to_string(), "ac".to_string(), "ab".to_string());
//     assert_eq!(combine(&split), combined);
// }
