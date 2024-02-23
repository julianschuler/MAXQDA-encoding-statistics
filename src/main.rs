use std::{fs::read_to_string, io::Error, path::PathBuf};

use clap::Parser;
use csv::ReaderBuilder;
use regex::Regex;
use serde::Deserialize;

#[derive(Parser, Debug)]
struct Args {
    text: PathBuf,
    encoding: PathBuf,
}

struct Position {
    page: usize,
    offset: usize,
}

impl<'de> Deserialize<'de> for Position {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        let regex = Regex::new(r"(?<page>[0-9]+): (?<offset>[0-9]+)").unwrap();
        let capture = regex.captures(&string).unwrap();

        Ok(Position {
            page: usize::from_str_radix(&capture["page"], 10).unwrap(),
            offset: usize::from_str_radix(&capture["offset"], 10).unwrap(),
        })
    }
}

#[derive(Deserialize)]
#[allow(unused)]
struct Record {
    #[serde(rename = "Farbe")]
    color: String,
    #[serde(rename = "Kommentar")]
    comment: String,
    #[serde(rename = "Dokumentgruppe")]
    document_group: String,
    #[serde(rename = "Dokumentname")]
    document_name: String,
    #[serde(rename = "Code")]
    code: String,
    #[serde(rename = "Anfang")]
    start: Position,
    #[serde(rename = "Ende")]
    end: Position,
    #[serde(rename = "Gewicht")]
    weight: u32,
    #[serde(rename = "Segment")]
    segment: String,
    #[serde(rename = "Bearbeitet von")]
    editor: String,
    #[serde(rename = "Bearbeitet am")]
    edit_date: String,
    #[serde(rename = "Erstellt von")]
    creator: String,
    #[serde(rename = "Erstellt am")]
    creation_date: String,
    #[serde(rename = "Fläche")]
    area: String,
    #[serde(rename = "Abdeckungsgrad %")]
    coverage: String,
}

struct Page {
    number: usize,
    offset: usize,
    text: String,
    encoded: Vec<bool>,
}

impl Page {
    fn from_segment(segment: &str) -> Self {
        let regex =
            Regex::new(r"(?<text>(?s)^.*)\(.*, S\. (?<page>\d+): (?<offset>\d+)\)").unwrap();
        let capture = regex.captures(segment).unwrap();
        let text = capture["text"].to_owned();

        let encoded = vec![false; text.len()];

        Page {
            number: usize::from_str_radix(&capture["page"], 10).unwrap(),
            offset: usize::from_str_radix(&capture["offset"], 10).unwrap(),
            text,
            encoded,
        }
    }

    fn set_encoded_range(&mut self, segment: &str) {
        if let Some(start) = self.text.find(segment) {
            for i in start..start + segment.len() {
                self.encoded[i] = true;
                // print!("{}", chars.next().unwrap());
            }
            // println!("\n");
        }

        // let mut chars = self.text.chars().skip(start.offset - self.offset);
    }

    fn get_sentence_data(&self) -> (usize, usize) {
        let mut sentences = 0;
        let mut encoded_sentences = 0;
        let mut is_sentence_encoded = false;
        for (byte, &encoded) in self.text.bytes().zip(&self.encoded) {
            if !is_sentence_encoded && encoded {
                is_sentence_encoded = true;
            }
            if byte == b'.' {
                sentences += 1;
                if is_sentence_encoded {
                    encoded_sentences += 1
                }
            }
        }

        (sentences, encoded_sentences)
    }
}

fn main() -> Result<(), Error> {
    let args = Args::parse();

    let mut pages = Vec::new();
    let text = read_to_string(args.text)?;
    for segment in text.split("\r\n\r\n") {
        pages.push(Page::from_segment(segment));
    }

    let first_page = pages.first().unwrap().number;

    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .from_path(args.encoding)?;

    for result in rdr.deserialize() {
        let record: Record = result?;

        assert_eq!(record.start.page, record.end.page);

        let current_page = pages.get_mut(record.start.page - first_page).unwrap();
        current_page.set_encoded_range(&record.segment);
    }

    let mut all_sentences = 0;
    let mut all_encoded_sentences = 0;
    for page in pages {
        let (sentences, encoded_sentences) = page.get_sentence_data();

        all_sentences += sentences;
        all_encoded_sentences += encoded_sentences;
    }

    dbg!(all_encoded_sentences, all_sentences);

    Ok(())
}
