#![allow(unstable)]
#![feature(plugin)]
#![feature(slicing_syntax)]
#[plugin]


extern crate regex_macros;
extern crate regex;
extern crate test;
extern crate core;

use std::old_io::File;
use std::old_io::BufferedReader;
use std::ascii::AsciiExt;
use std::collections::HashMap;
use std::collections::HashSet;
//use std::fmt;
use std::os;

/*
struct StringSet<'a>(pub &'a HashSet<String>);

impl <'a> fmt::Display for StringSet<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let StringSet(ref x) = *self;
        write!(f, "[");
        // TODO write 'zip/enumerate-type iter returning (first?, value)
        let mut first = true;
        for s in x.iter() {
            if first {
                write!(f, "{}", s);
            }
            else {
                write!(f, ",{}", s);
            }
            first = false;
        }
        write!(f, "]")
    }
}

fn pv(v : &Vec<String>) {
    for s in v.iter() {
       println!("{}", s );
    }
}
*/
fn lowercase(s : &str) -> String {
    return s.to_ascii_lowercase();
}

/// a lowercase string of the model text
fn source() -> String {
    match File::open(&Path::new("mobydick.txt")).read_to_string() {
        Ok(text) => lowercase(&text[]),
        Err(err) => panic!("Cannot open model data: {}", err),
    }
}

// calculates the word-frequencies in the supplied text
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

// returns a tuple of the input string split
// e.g split("abcd", 2) -> ("ab","cd")
fn split(s: &str, pos: usize) -> (&str, &str) {
    (&s[0..pos], &s[pos..s.len()])
}

fn alphabet() -> &'static str {
    "abcdefghijklmnopqrstuvwxyz"
}

// a vec of all the split pairs of a word
fn splits(s: &str) -> Vec<(&str, &str)> {
    let mut v = Vec::<(&str, &str)>::new();
    for pos in 0..s.len()+1 {
        v.push(split(s, pos));
    }
    v
}

// all the possible single-char deletes
fn deletes<'a, 'b>(splits : & 'a Vec<(& 'b str, & 'b str)>) ->  Vec<String> {
    let mut v = Vec::<String>::new();
    for &(a, b) in splits.iter() {
        if b.len() > 0 {
            v.push(a.to_string() +  &b[1..b.len()]);
        }
    }
    v
}

// all the possible single-char transposes
fn transposes<'a, 'b>(splits : & 'a Vec<(& 'b str, & 'b str)>) ->  Vec<String> {
    let mut v = Vec::<String>::new();
    for &(a, b) in splits.iter() {
        if b.len() > 1 {
            v.push(a.to_string() + &b[1..2] + &b[0..1] + &b[2..b.len()]);
        }
    }
    v
}

// all the possible single-char inserts
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

// all the possible single-char replacements
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

// consumes all the values in the vec and inserts them into the set
fn insert_all(set : &mut HashSet<String>, vec : Vec<String>) {
    // TODO there must be a very generic way to do this! FromIter?
    for s in vec.into_iter() {
        (*set).insert(s);
    }
}

// a set of all the strings in 1-edit distance
fn edits1(word: &str) -> HashSet<String> {
    let mut set = HashSet::<String>::new();
    let splits = splits(word);
    insert_all(&mut set, deletes(&splits));
    insert_all(&mut set, inserts(&splits));
    insert_all(&mut set, transposes(&splits));
    insert_all(&mut set, replaces(&splits));

    set
}

// return only the words known in the model
// TODO words should be an iterator?
// TODO return should be the word passed in - no need to realloc?
fn known<'a>(words: HashSet<String>, model: &HashMap<&str, i32>) -> HashSet<String> {
    let mut set = HashSet::<String>::new();
    for word in words.into_iter() {
        if model.contains_key(word.as_slice()) {
            set.insert(word);
        }
    }
    set
}

// all the known words within 2-edit distance
fn known_edits2(word: &str, model: &HashMap<&str, i32>) -> HashSet<String> {
    let mut set = HashSet::<String>::new();
    for e1 in edits1(word).iter() {
        for e2 in edits1(&e1[]).iter() {
            if model.contains_key(&e2[]) {
                set.insert(e2.to_string());
            }
        }
    }

    set
}

// simple set constructor
// TODO - there must be a better way of doing this?
fn make_set(s: String) -> HashSet<String> {
    let mut set = HashSet::new(); set.insert(s);
    set
}

// all the candidate words for a supplied word.
// each stage (word, edit1, edit2) is infinitely more likely
// than the next
fn candidates(word: &str, model : &HashMap<&str, i32>) -> HashSet<String> {
    let known_as_set = known(make_set(word.to_string()), model);
    if known_as_set.len() > 0 { return known_as_set }

    let known_edits1  = known(edits1(word), model);
    if known_edits1.len() > 0 { return known_edits1 }

    let known_edits2 = known_edits2(word, model);
    if known_edits2.len() > 0 { return known_edits2 }

    return make_set(word.to_string())
}

// returns the word frequency, but gives *all* words a minimum frequency
// of 1
fn count<'a>(word: &str, model: &'a HashMap<&str, i32>) -> i32 {
    match model.get(word) {
        Some(count) => count.clone(),
        None => 1
    }
}

// returns the corrected word given a model
fn correct<'a>(word : &str, model : &'a HashMap<&str, i32>) -> String {
    let mut candidates = candidates(word, model).into_iter().collect::<Vec<String>>();
    //pv(&candidates);
    candidates.sort_by(|b, a|
                       count(&a[], model).cmp(&count(&b[], model)));
    candidates[0].clone()
}

fn main() {
    let s = source();  // separate line to persuade borrow-checker
    let t = &s[];

    let ws = word_counts(t);

    let argv = os::args();
    let input = argv.get(1).unwrap();
    let path = Path::new(&input[]);
    let mut reader = BufferedReader::new(File::open(&path));

    for line in reader.lines() {
        let w = line.unwrap();
        let wtrimmed = w.trim();
        println!("{} -> {}", wtrimmed,  correct(wtrimmed, &ws));
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
