extern crate nom_bibtex;

use nom_bibtex::*;
use std::fs;

fn get_title<'a>(e: &'a Bibliography) -> &'a str {
    for (k, v) in e.tags() {
        if k == "title" {
            return v;
        }
    }
    panic!("Missing title in BibTeX entry `{}`", e.citation_key())
}

fn entry_to_string(e: &Bibliography) -> String {
    let key = e.citation_key();
    let title = get_title(e);
    return format!("{} -- {}", key, title);
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
