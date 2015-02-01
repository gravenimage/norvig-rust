#![allow(unstable)]
#![feature(plugin)]
#![feature(slicing_syntax)]
#[plugin]

extern crate regex_macros;
extern crate regex;
extern crate test;
extern crate core;

use std::io::File;
use std::ascii::AsciiExt;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;

struct StringSet(pub HashSet<String>);

impl fmt::Display for StringSet {
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

fn  make_set(s: String) -> HashSet<String> {
    let mut set = HashSet::new(); set.insert(s);
    set
}

fn candidates(word: &str, model : &HashMap<&str, i32>) -> HashSet<String> {
    let known_as_set = known(make_set(word.to_string()), model);
    if known_as_set.len() > 0 { return known_as_set }

    let known_edits1  = known(edits1(word), model);
    if known_edits1.len() > 0 { return known_edits1 }

    let known_edits2 = known_edits2(word, model);
    if known_edits2.len() > 0 { return known_edits2 }

    return make_set(word.to_string())
}

fn count<'a>(word: &str, model: &'a HashMap<&str, i32>) -> i32 {
    match model.get(word) {
        Some(count) => count.clone(),
        None => 1
    }
}

fn correct<'a>(word : &str, model : &'a HashMap<&str, i32>) -> String {
    let mut candidates = candidates(word, model).into_iter().collect::<Vec<String>>();
    candidates.sort_by(|b, a|
                       count(&a[], model).cmp(&count(&b[], model)));
    candidates[0].clone()
}

fn main() {
    let s = source();  // separate line to persuade borrow-checker
    let t = &s[];

    let ws = word_counts(t);
    println!("Corrected: {}", correct("ther", &ws));

//    println!("{}", StringSet(candidates("ther", &ws)));
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

#[bench]
fn bench_main(b: &mut test::Bencher) {
    main()
}
