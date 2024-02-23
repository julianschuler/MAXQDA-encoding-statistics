use std::{fs::read_to_string, io::Error, path::PathBuf};

use clap::Parser;
use csv::ReaderBuilder;
use serde::Deserialize;

#[derive(Parser, Debug)]
struct Args {
    paths: Vec<PathBuf>,
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
    start: String,
    #[serde(rename = "Ende")]
    end: String,
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

struct EncodedText {
    text: String,
    encoded: Vec<bool>,
}

impl EncodedText {
    fn from_text(text: String) -> Self {
        let encoded = vec![false; text.len()];
        let text = text.replace("\r", "");

        Self { text, encoded }
    }

    fn set_encoding(&mut self, segment: &str) {
        if let Some(start) = self.text.find(&segment) {
            for i in start..start + segment.len() {
                self.encoded[i] = true;
            }
        } else {
            eprintln!("unable to find '{segment}'")
        }
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
                    encoded_sentences += 1;
                    is_sentence_encoded = false;
                }
            }
        }

        (sentences, encoded_sentences)
    }
}

fn main() -> Result<(), Error> {
    let args = Args::parse();

    // Assumption: For each given path there exists a txt and csv file
    for mut path in args.paths {
        // Read text from file
        path.set_extension("txt");
        let text = read_to_string(&path)?;
        let mut encoded_text = EncodedText::from_text(text);

        // Read encoded segments and set as encoded in encoded_text
        path.set_extension("csv");
        let mut reader = ReaderBuilder::new().delimiter(b';').from_path(&path)?;
        for result in reader.deserialize() {
            let record: Record = result?;
            encoded_text.set_encoding(&record.segment);
        }

        // Print out results
        let (sentences, encoded_sentences) = encoded_text.get_sentence_data();
        println!("\n{}:", path.file_stem().unwrap().to_str().unwrap());
        println!("  Encoded sentences: {}", encoded_sentences);
        println!("  Total sentences: {}", sentences);
        println!(
            "  Encoding percentage: {:.2}%",
            100.0 * encoded_sentences as f32 / sentences as f32
        );
    }

    Ok(())
}
