use conllu::graph::Sentence;
use conllu::io::ReadSentence;
use conllu::io::Reader;
use conllu::token::Tokens;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::LineWriter;

pub fn read_sentences(filename: &str) -> Vec<Sentence> {
    Reader::new(BufReader::new(File::open(filename).unwrap()))
        .sentences()
        .map(|s| s.unwrap())
        .collect()
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct POS {
    word: String,
    tag: String,
}

impl std::fmt::Display for POS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{} {}", self.word, self.tag)
    }
}

fn main() {
    // let file = File::create("poem.txt")?;
    // let mut file = LineWriter::new(file);
    let mut pos_elements = HashSet::new();
    let sentences = read_sentences("testdata/ud-spanish.conllu");
    for sentence in sentences {
        let tokens = sentence.tokens();
        for token in tokens {
            match token.upos() {
                None => println!("received none"),
                Some(upos) => {
                    let pos = POS {
                        word: String::from(token.form()),
                        tag: String::from(upos),
                    };
                    pos_elements.insert(pos);
                }
            }
        }
    }
    println!("unique pos elements {}", pos_elements.len());
    for pos in &pos_elements {
        println!("{}", pos);
    }

    let mut tags: Vec<String> = pos_elements.into_iter().map(|pos| pos.tag).collect();
    tags.sort();
    tags.dedup();
    println!("unique_tags : {:#?}", tags);

    //TODO: split into train / dataset
}
