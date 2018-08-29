extern crate colored;
extern crate nom_bibtex;

use colored::*;
use nom_bibtex::*;
use std::fs;

fn get_key<'a>(key: &str, e: &'a Bibliography) -> &'a str {
    for (k, v) in e.tags() {
        if k == key {
            return v;
        }
    }
    panic!("Missing key `{}` in BibTeX entry `{}`", key, e.citation_key())
}

fn entry_to_string(e: &Bibliography) -> String {
    let key = e.citation_key();
    let title = get_key("title", e);
    let authors = get_key("author", e);
    return format!("{} • {} • {}", key.yellow(), title, authors.italic());
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }
    let filename = &args[1];
    let text = fs::read_to_string(filename).expect("Unable to read file");
    let bibtex = Bibtex::parse(&text).unwrap();
    let bib = bibtex.bibliographies();
    for e in bib {
        println!("{}", entry_to_string(e));
    }
}
